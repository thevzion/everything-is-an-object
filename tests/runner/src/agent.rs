//! The agent under test and the decision model, both run headless through the Claude Code CLI.
//! The harness is deterministic; the agents are not, so each trial runs several times.

use crate::util::*;
use serde_json::Value;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub struct AgentConfig {
    pub claude: String,
    pub model: String,
    pub judge: String,
    pub timeout: Duration,
    pub empty_git_config: PathBuf,
}

/// The tools the agent under test is given: files and a shell; no web, no sub-agents.
pub const TOOLS: &str = "Read,Write,Edit,Glob,Grep,Bash";

/// What the shell may run without a prompt: git, and the ordinary commands that read and write
/// files. Anything else, the network included, is denied, since nobody answers a prompt here.
pub const ALLOWED: &str = "Bash(git:*) Bash(cd:*) Bash(pwd:*) Bash(ls:*) Bash(cat:*) Bash(find:*) Bash(grep:*) Bash(head:*) Bash(tail:*) Bash(wc:*) Bash(sort:*) Bash(uniq:*) Bash(cut:*) Bash(sed:*) Bash(awk:*) Bash(diff:*) Bash(echo:*) Bash(printf:*) Bash(mkdir:*) Bash(touch:*) Bash(cp:*) Bash(mv:*) Bash(chmod:*) Bash(test:*) Bash(date:*) Bash(tree:*) Bash(basename:*) Bash(dirname:*) Bash(true:*)";

/// Builds the command with a clean environment: no variable of the calling session, no global
/// git configuration, and `--safe-mode`, which loads no instruction file, memory, skill, hook,
/// plugin or MCP server. The space's `AGENTS.md` reaches the agent as appended instructions,
/// as its `CLAUDE.md` (`@AGENTS.md`) would outside safe mode.
fn base_command(cfg: &AgentConfig, cwd: &Path) -> Command {
    let mut cmd = Command::new(&cfg.claude);
    cmd.current_dir(cwd);
    for (k, _) in std::env::vars() {
        if k.starts_with("CLAUDE") || k == "AI_AGENT" || k == "BAGGAGE" {
            cmd.env_remove(&k);
        }
    }
    git_env(&mut cmd, &cfg.empty_git_config);
    cmd
}

#[derive(Debug, Clone)]
pub enum Event {
    Text(String),
    Tool { name: String, input: String, writes: bool },
    ToolError(String),
    Denied(String),
}

#[derive(Debug, Default, Clone)]
pub struct Trace {
    pub model: String,
    pub events: Vec<Event>,
    pub final_text: String,
    pub is_error: bool,
    pub cost_usd: f64,
    pub tokens_in: u64,
    pub tokens_cache_write: u64,
    pub tokens_cache_read: u64,
    pub tokens_out: u64,
    pub turns: u64,
    pub duration_ms: u64,
    pub utilization: Option<f64>,
    pub got_result: bool,
}

impl Trace {
    pub fn first_text_index(&self) -> Option<usize> {
        self.events.iter().position(|e| matches!(e, Event::Text(t) if !t.trim().is_empty()))
    }
    pub fn first_write_index(&self) -> Option<usize> {
        self.events.iter().position(|e| matches!(e, Event::Tool { writes: true, .. }))
    }
    pub fn first_text(&self) -> String {
        self.first_text_index()
            .and_then(|i| match &self.events[i] {
                Event::Text(t) => Some(t.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }
    pub fn count_tool(&self, name: &str) -> usize {
        self.events.iter().filter(|e| matches!(e, Event::Tool { name: n, .. } if n == name)).count()
    }
}

/// A shell command that changes files or history.
fn shell_writes(cmd: &str) -> bool {
    let c = cmd
        .replace("2>&1", "")
        .replace("2>/dev/null", "")
        .replace(">/dev/null", "")
        .replace("> /dev/null", "")
        .replace("2>$null", "");
    ["git add", "git commit", "git mv", "git rm", ">", "tee ", "mkdir", "touch ", "cp ", "mv ", "New-Item", "Set-Content", "Out-File", "Add-Content"]
        .iter()
        .any(|p| c.contains(p))
}

pub fn parse_stream(path: &Path) -> Trace {
    let mut t = Trace::default();
    let text = read(path);
    for line in text.lines() {
        let Ok(e) = serde_json::from_str::<Value>(line) else { continue };
        match e["type"].as_str().unwrap_or("") {
            "system" => {
                if e["subtype"] == "init" {
                    t.model = e["model"].as_str().unwrap_or("").to_string();
                } else if e["subtype"] == "permission_denied" {
                    t.events.push(Event::Denied(e["tool_name"].as_str().unwrap_or("").to_string()));
                }
            }
            "assistant" => {
                if let Some(blocks) = e["message"]["content"].as_array() {
                    for b in blocks {
                        match b["type"].as_str().unwrap_or("") {
                            "text" => t.events.push(Event::Text(b["text"].as_str().unwrap_or("").to_string())),
                            "tool_use" => {
                                let name = b["name"].as_str().unwrap_or("").to_string();
                                let input = &b["input"];
                                let summary = match name.as_str() {
                                    "Bash" | "PowerShell" => input["command"].as_str().unwrap_or("").to_string(),
                                    "Read" | "Write" | "Edit" | "MultiEdit" | "NotebookEdit" => {
                                        input["file_path"].as_str().unwrap_or("").to_string()
                                    }
                                    "Glob" | "Grep" => format!(
                                        "{} {}",
                                        input["pattern"].as_str().unwrap_or(""),
                                        input["path"].as_str().unwrap_or("")
                                    ),
                                    _ => input.to_string(),
                                };
                                let writes = match name.as_str() {
                                    "Write" | "Edit" | "MultiEdit" | "NotebookEdit" => true,
                                    "Bash" | "PowerShell" => shell_writes(&summary),
                                    _ => false,
                                };
                                t.events.push(Event::Tool { name, input: summary, writes });
                            }
                            _ => {}
                        }
                    }
                }
            }
            "user" => {
                if let Some(blocks) = e["message"]["content"].as_array() {
                    for b in blocks {
                        if b["type"] == "tool_result" && b["is_error"] == true {
                            let c = match &b["content"] {
                                Value::String(s) => s.clone(),
                                other => other.to_string(),
                            };
                            t.events.push(Event::ToolError(clip(&one_line(&c), 200)));
                        }
                    }
                }
            }
            "rate_limit_event" => {
                if let Some(u) = e["rate_limit_info"]["utilization"].as_f64() {
                    t.utilization = Some(u);
                } else if let Some(map) = e["rate_limit_info"]["unifiedWindows"].as_object() {
                    for w in map.values() {
                        if let Some(u) = w["utilization"].as_f64() {
                            t.utilization = Some(t.utilization.map_or(u, |x: f64| x.max(u)));
                        }
                    }
                }
            }
            "result" => {
                t.got_result = true;
                t.final_text = e["result"].as_str().unwrap_or("").to_string();
                t.is_error = e["is_error"].as_bool().unwrap_or(false);
                t.cost_usd = e["total_cost_usd"].as_f64().unwrap_or(0.0);
                t.turns = e["num_turns"].as_u64().unwrap_or(0);
                t.duration_ms = e["duration_ms"].as_u64().unwrap_or(0);
                let u = &e["usage"];
                t.tokens_in = u["input_tokens"].as_u64().unwrap_or(0);
                t.tokens_cache_write = u["cache_creation_input_tokens"].as_u64().unwrap_or(0);
                t.tokens_cache_read = u["cache_read_input_tokens"].as_u64().unwrap_or(0);
                t.tokens_out = u["output_tokens"].as_u64().unwrap_or(0);
            }
            _ => {}
        }
    }
    t
}

/// Runs the agent under test once, in `cwd`, and keeps its raw stream in `stream`.
pub fn run_agent(cfg: &AgentConfig, cwd: &Path, add_dirs: &[PathBuf], instructions: &str, prompt: &str, stream: &Path) -> Res<Trace> {
    let mut cmd = base_command(cfg, cwd);
    cmd.args([
        "-p",
        "--safe-mode",
        "--model",
        &cfg.model,
        "--tools",
        TOOLS,
        "--permission-mode",
        "acceptEdits",
        "--allowedTools",
        ALLOWED,
        "--permission-prompts",
        "none",
        "--no-session-persistence",
        "--output-format",
        "stream-json",
        "--verbose",
    ]);
    for d in add_dirs {
        cmd.arg("--add-dir").arg(d);
    }
    if !instructions.is_empty() {
        cmd.arg("--append-system-prompt").arg(instructions);
    }
    // The person on whose behalf the agent works: a fictional identity, as on their own machine.
    for (k, v) in [("GIT_AUTHOR_NAME", "Sam Rivera"), ("GIT_AUTHOR_EMAIL", "sam@example.invalid"), ("GIT_COMMITTER_NAME", "Sam Rivera"), ("GIT_COMMITTER_EMAIL", "sam@example.invalid")] {
        cmd.env(k, v);
    }
    cmd.arg(prompt);
    if let Some(dir) = stream.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let out = File::create(stream).map_err(|e| e.to_string())?;
    let err = File::create(stream.with_extension("stderr.txt")).map_err(|e| e.to_string())?;
    cmd.stdout(Stdio::from(out)).stderr(Stdio::from(err)).stdin(Stdio::null());
    let mut child = cmd.spawn().map_err(|e| format!("cannot start {}: {e}", cfg.claude))?;
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => {
                if start.elapsed() > cfg.timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    let mut t = parse_stream(stream);
                    t.is_error = true;
                    return Ok(t);
                }
                std::thread::sleep(Duration::from_millis(500));
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(parse_stream(stream))
}

/// A closed question to the decision model: a different model from the agent under test, with
/// no tool and no context but the question and its material. Returns (yes, reason, model, cost).
#[allow(dead_code)]
pub struct Verdict {
    pub yes: Option<bool>,
    pub reason: String,
    pub model: String,
    pub cost_usd: f64,
}

pub fn judge(cfg: &AgentConfig, cwd: &Path, question: &str, material: &str) -> Verdict {
    let prompt = format!(
        "You are a decision model. Answer the closed question about the material below.\n\
         Reply with YES or NO as the first word, then one short sentence that says why.\n\n\
         Question: {question}\n\nMaterial:\n<<<\n{material}\n>>>\n"
    );
    let mut cmd = base_command(cfg, cwd);
    cmd.args([
        "-p",
        "--safe-mode",
        "--model",
        &cfg.judge,
        "--tools",
        "",
        "--no-session-persistence",
        "--output-format",
        "json",
        &prompt,
    ]);
    cmd.stdin(Stdio::null());
    let out = match cmd.output() {
        Ok(o) => o,
        Err(e) => return Verdict { yes: None, reason: format!("judge did not start: {e}"), model: cfg.judge.clone(), cost_usd: 0.0 },
    };
    let v: Value = serde_json::from_slice(&out.stdout).unwrap_or(Value::Null);
    let text = v["result"].as_str().unwrap_or("").trim().to_string();
    let first_word = text.split(|c: char| !c.is_alphabetic()).find(|w| !w.is_empty()).unwrap_or("").to_uppercase();
    let yes = match first_word.as_str() {
        "YES" => Some(true),
        "NO" => Some(false),
        _ => None,
    };
    let model = v["modelUsage"].as_object().and_then(|m| m.keys().next().cloned()).unwrap_or_else(|| cfg.judge.clone());
    Verdict { yes, reason: clip(&one_line(&text), 300), model, cost_usd: v["total_cost_usd"].as_f64().unwrap_or(0.0) }
}

pub fn claude_version(cfg: &AgentConfig) -> String {
    Command::new(&cfg.claude)
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into())
}
