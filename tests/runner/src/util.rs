//! Small helpers: files, git, text. Everything here is deterministic.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub type Res<T> = Result<T, String>;

/// The fixed date every fixture commit carries, so a fixture always has the same commit ids.
pub const FIXTURE_DATE: &str = "2026-11-01T09:00:00+00:00";

pub fn write(path: &Path, content: &str) -> Res<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    }
    fs::write(path, content.as_bytes()).map_err(|e| format!("write {}: {e}", path.display()))
}

pub fn read(path: &Path) -> String {
    // Line endings are normalized: a checkout may turn LF into CRLF, and the checks compare text.
    fs::read_to_string(path).unwrap_or_default().replace("\r\n", "\n")
}

/// The environment every git call shares, the harness's and the agent's: no global or system
/// configuration, so line endings, hooks and identity come from the fixture alone.
pub fn git_env(cmd: &mut Command, empty_config: &Path) {
    cmd.env("GIT_CONFIG_GLOBAL", empty_config);
    cmd.env("GIT_CONFIG_NOSYSTEM", "1");
    cmd.env("GIT_TERMINAL_PROMPT", "0");
}

pub struct Git {
    pub empty_config: PathBuf,
}

impl Git {
    pub fn run(&self, dir: &Path, args: &[&str]) -> Res<String> {
        self.run_env(dir, args, &[])
    }

    pub fn run_env(&self, dir: &Path, args: &[&str], env: &[(&str, &str)]) -> Res<String> {
        let mut cmd = Command::new("git");
        cmd.current_dir(dir).args(args);
        git_env(&mut cmd, &self.empty_config);
        for (k, v) in env {
            cmd.env(k, v);
        }
        let out = cmd.output().map_err(|e| format!("git {:?}: {e}", args))?;
        if !out.status.success() {
            return Err(format!(
                "git {:?} in {}: {}",
                args,
                dir.display(),
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        Ok(String::from_utf8_lossy(&out.stdout).trim_end().to_string())
    }

    pub fn ok(&self, dir: &Path, args: &[&str]) -> bool {
        self.run(dir, args).is_ok()
    }

    /// `git init` with a fictional identity and the commit-msg hook of the setup prompt.
    pub fn init(&self, dir: &Path, name: &str, email: &str) -> Res<()> {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        self.run(dir, &["init", "-q", "-b", "main"])?;
        self.run(dir, &["config", "user.name", name])?;
        self.run(dir, &["config", "user.email", email])?;
        self.run(dir, &["config", "core.autocrlf", "false"])?;
        let hook = "#!/bin/sh\nt=$(git interpret-trailers --parse < \"$1\")\nprintf '%s\\n' \"$t\" | grep -q '^Why: .' &&\nprintf '%s\\n' \"$t\" | grep -Eq '^Source: (from-me|inferred)' && exit 0\necho \"commit refused: end the message with Why: and Source: in one trailer block\" >&2\nexit 1\n";
        write(&dir.join(".git/hooks/commit-msg"), hook)?;
        Ok(())
    }

    /// Commits everything with a fixed date, so the same fixture gives the same commit id.
    pub fn commit_all(&self, dir: &Path, message: &str) -> Res<String> {
        self.run(dir, &["add", "-A"])?;
        let env = [("GIT_AUTHOR_DATE", FIXTURE_DATE), ("GIT_COMMITTER_DATE", FIXTURE_DATE)];
        self.run_env(dir, &["commit", "-q", "--no-verify", "-m", message], &env)?;
        self.run(dir, &["rev-parse", "HEAD"])
    }

    pub fn head(&self, dir: &Path) -> String {
        self.run(dir, &["rev-parse", "HEAD"]).unwrap_or_default()
    }
}

/// A commit after the fixture's base, with what a check needs.
#[derive(Debug, Clone)]
pub struct Commit {
    pub id: String,
    pub subject: String,
    pub body: String,
    pub trailers: String,
    pub files: Vec<String>,
}

pub fn commits_in(git: &Git, dir: &Path, range: &str) -> Res<Vec<Commit>> {
    let raw = git.run(dir, &["log", "--reverse", "--format=%H%x1f%s%x1f%B%x1e", range])?;
    let mut out = Vec::new();
    for rec in raw.split('\u{1e}') {
        let rec = rec.trim_matches(|c| c == '\n' || c == '\r');
        if rec.is_empty() {
            continue;
        }
        let parts: Vec<&str> = rec.splitn(3, '\u{1f}').collect();
        if parts.len() < 3 {
            continue;
        }
        let id = parts[0].to_string();
        let body = parts[2].to_string();
        let trailers = trailers_of(git, dir, &body);
        let files = git
            .run(dir, &["show", "--name-only", "--format=", &id])
            .unwrap_or_default()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        out.push(Commit { id, subject: parts[1].to_string(), body, trailers, files });
    }
    Ok(out)
}

/// What `git interpret-trailers --parse` reads in a message, as the hook reads it.
pub fn trailers_of(git: &Git, dir: &Path, message: &str) -> String {
    use std::io::Write;
    let mut cmd = Command::new("git");
    cmd.current_dir(dir).args(["interpret-trailers", "--parse"]);
    git_env(&mut cmd, &git.empty_config);
    cmd.stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped());
    let Ok(mut child) = cmd.spawn() else { return String::new() };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(message.as_bytes());
    }
    child
        .wait_with_output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

pub fn has_why_and_source(trailers: &str) -> bool {
    let why = trailers.lines().any(|l| l.starts_with("Why: ") && l.len() > 5);
    let source = trailers
        .lines()
        .any(|l| l.starts_with("Source: from-me") || l.starts_with("Source: inferred"));
    why && source
}

/// Every file under `dir`, as paths relative to `root` with forward slashes, sorted.
pub fn walk(root: &Path, dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            if name == ".git" || name == "target" {
                continue;
            }
            if p.is_dir() {
                stack.push(p);
            } else if let Ok(rel) = p.strip_prefix(root) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    out.sort();
    out
}

/// A header: the lines between the first two `---`, as key to values. A scalar is a list of one.
pub fn parse_header(text: &str) -> Option<(BTreeMap<String, Vec<String>>, String)> {
    let text = text.trim_start_matches('\u{feff}');
    let mut lines = text.lines();
    if lines.next()?.trim() != "---" {
        return None;
    }
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut last: Option<String> = None;
    let mut consumed = 1;
    let mut closed = false;
    for line in lines.by_ref() {
        consumed += 1;
        if line.trim() == "---" {
            closed = true;
            break;
        }
        let t = line.trim_end();
        if let Some(item) = t.trim_start().strip_prefix("- ") {
            if t.starts_with(' ') || t.starts_with('-') {
                if let Some(k) = &last {
                    map.entry(k.clone()).or_default().push(clean_value(item));
                }
                continue;
            }
        }
        if let Some((k, v)) = t.split_once(':') {
            if k.starts_with(' ') || k.contains(' ') {
                continue;
            }
            let k = k.trim().to_string();
            let v = strip_comment(v.trim());
            let vals: Vec<String> = if v.starts_with('[') && v.ends_with(']') {
                v[1..v.len() - 1]
                    .split(',')
                    .map(clean_value)
                    .filter(|s| !s.is_empty())
                    .collect()
            } else if v.is_empty() {
                Vec::new()
            } else {
                vec![clean_value(v)]
            };
            map.insert(k.clone(), vals);
            last = Some(k);
        }
    }
    if !closed {
        return None;
    }
    let body: String = text.lines().skip(consumed).collect::<Vec<_>>().join("\n");
    Some((map, body))
}

fn strip_comment(v: &str) -> &str {
    // A YAML comment starts with " #"; a question never contains one.
    match v.find(" #") {
        Some(i) => v[..i].trim_end(),
        None => v,
    }
}

fn clean_value(v: &str) -> String {
    let v = v.trim();
    let v = v.strip_prefix('"').and_then(|s| s.strip_suffix('"')).unwrap_or(v);
    let v = v.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')).unwrap_or(v);
    v.trim().to_string()
}

pub fn first(h: &BTreeMap<String, Vec<String>>, key: &str) -> String {
    h.get(key).and_then(|v| v.first()).cloned().unwrap_or_default()
}

/// Lowercase words, punctuation removed: the unit of a verbatim comparison.
pub fn words(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in text.chars() {
        let c = match c {
            '\u{2019}' | '\u{2018}' => '\'',
            _ => c,
        };
        if c.is_alphanumeric() || c == '\'' {
            cur.extend(c.to_lowercase());
        } else if !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

pub fn norm(text: &str) -> String {
    words(text).join(" ")
}

/// The person's words are kept: five of their words in a row, or, for an object shorter than
/// five words, the whole object word for word.
pub fn keeps_words(said: &str, body: &str) -> bool {
    if shares_span(said, body, 5) {
        return true;
    }
    let b = words(body);
    !b.is_empty() && b.len() < 5 && format!(" {} ", norm(said)).contains(&format!(" {} ", b.join(" ")))
}

/// True when `body` holds `n` consecutive words of `said`, in order.
pub fn shares_span(said: &str, body: &str, n: usize) -> bool {
    let s = words(said);
    let b = norm(body);
    if s.len() < n {
        return false;
    }
    let padded = format!(" {b} ");
    s.windows(n).any(|w| padded.contains(&format!(" {} ", w.join(" "))))
}

/// Quoted passages of four words or more: in straight or curly double quotes, in guillemets,
/// and blockquote lines.
pub fn quotes(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in body.lines() {
        if let Some(q) = line.trim_start().strip_prefix('>') {
            let q = q.trim();
            if words(q).len() >= 4 {
                out.push(q.to_string());
            }
        }
    }
    let pairs = [('"', '"'), ('\u{201c}', '\u{201d}'), ('\u{ab}', '\u{bb}')];
    for (open, close) in pairs {
        let mut rest = body;
        while let Some(i) = rest.find(open) {
            let after = &rest[i + open.len_utf8()..];
            let Some(j) = after.find(close) else { break };
            let q = &after[..j];
            if words(q).len() >= 4 && !q.contains('\n') {
                out.push(q.trim().to_string());
            }
            rest = &after[j + close.len_utf8()..];
        }
    }
    out
}

pub fn contains_ci(hay: &str, needle: &str) -> bool {
    hay.to_lowercase().contains(&needle.to_lowercase())
}

pub fn clip(s: &str, max_chars: usize) -> String {
    let s = s.trim();
    if s.chars().count() <= max_chars {
        return s.to_string();
    }
    let mut t: String = s.chars().take(max_chars).collect();
    t.push_str(" [...]");
    t
}

pub fn one_line(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_scalars_lists_and_comments() {
        let t = "---\ntype: goal\nmodel: goals@0.1 # a comment\nserves:\n  - objects/a.md\n  - objects/b.md\ndraws-on: [x.md, y.md]\nquestion: \"What do we want now, and how will we know?\"\n---\nBody\n";
        let (h, body) = parse_header(t).unwrap();
        assert_eq!(first(&h, "type"), "goal");
        assert_eq!(first(&h, "model"), "goals@0.1");
        assert_eq!(h["serves"], vec!["objects/a.md", "objects/b.md"]);
        assert_eq!(h["draws-on"], vec!["x.md", "y.md"]);
        assert_eq!(first(&h, "question"), "What do we want now, and how will we know?");
        assert_eq!(body.trim(), "Body");
    }

    #[test]
    fn spans_and_quotes() {
        let said = "I want to run 10 km by June, because I get out of breath.";
        assert!(shares_span(said, "Run: I want to run 10 km by June.", 5));
        assert!(!shares_span(said, "Running ten kilometres before summer.", 5));
        assert!(keeps_words("because takeaway eats my budget", "Takeaway eats my budget."));
        assert!(!keeps_words("because takeaway eats my budget", "Takeaway costs too much."));
        let q = quotes("He said \"three easy runs a week\" and\n> one long run on Sunday\n");
        assert_eq!(q.len(), 2);
    }
}
