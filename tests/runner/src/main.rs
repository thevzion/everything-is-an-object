//! convention-trials: the Convention's canonical trials, run with real agents.
//!
//! The harness is deterministic: the same fixture (same files, same commit ids), the same prompt,
//! the same versions, the same checks. The agent is not: each trial runs N times, and the proof
//! reads k out of n. See `tests/README.md`.

mod agent;
mod record;
mod space;
mod trials;
mod util;

use agent::AgentConfig;
use space::Convention;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use trials::{Ctx, RunData, Trial};
use util::*;

fn usage() -> ! {
    eprintln!(
        "usage: convention-trials <command>\n\
         \n  list                          the trials, with their questions\
         \n  run <ids|all> [options]       run trials with a real agent, write each run as an object\
         \n      --batch <n>               the batch the runs belong to, said before they start (required)\
         \n      --n 5 --model sonnet --judge haiku --jobs 4 --timeout-min 15\
         \n      --max-utilization 0.85    stop starting runs above this share of the rate limit\
         \n      --keep                    keep each fixture after its run\
         \n      --claude <path>           the Claude Code CLI (default: claude); --scratch <dir>, where fixtures are built\
         \n  fixture <id> <dir>            build a trial's fixture in <dir>, to look at it\
         \n  table                         regenerate the README's proof table and tests/runs/README.md\
         \n  links                         check every relative link and anchor\
         \n  template                      regenerate setup/AGENTS.template.md from convention/roles.md\
         \n  exclude <reason> <files...>   mark runs as not counted, with the reason; they stay as written"
    );
    std::process::exit(2)
}

fn repo_root() -> PathBuf {
    let mut d = std::env::current_dir().expect("cwd");
    loop {
        if d.join("convention/doctrine.md").exists() {
            return d;
        }
        if !d.pop() {
            eprintln!("run from inside the convention repository");
            std::process::exit(2);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else { usage() };
    let repo = repo_root();
    match cmd.as_str() {
        "list" => {
            for t in trials::all() {
                println!("{:<16} {}  ({})", t.id, t.question, t.file);
            }
        }
        "links" => {
            let broken = record::check_links(&repo);
            for b in &broken {
                println!("broken: {b}");
            }
            println!("{} broken link(s)", broken.len());
            if !broken.is_empty() {
                std::process::exit(1);
            }
        }
        "table" => {
            let ts = trials::all();
            let (readme_table, detail) = record::table(&repo, &ts).unwrap_or_else(|e| fail(&e));
            let readme_path = repo.join("README.md");
            let readme = read(&readme_path);
            let updated = record::replace_between(&readme, "<!-- proof:begin -->", "<!-- proof:end -->", &readme_table).unwrap_or_else(|e| fail(&e));
            write(&readme_path, &updated).unwrap_or_else(|e| fail(&e));
            write(&repo.join("tests/runs/README.md"), &detail).unwrap_or_else(|e| fail(&e));
            println!("README.md and tests/runs/README.md regenerated from the run objects");
        }
        "template" => {
            let c = Convention::load(&repo).unwrap_or_else(|e| fail(&e));
            write(&repo.join("setup/AGENTS.template.md"), &c.agents_template()).unwrap_or_else(|e| fail(&e));
            println!("setup/AGENTS.template.md regenerated from convention/roles.md");
        }
        "fixture" => {
            let (Some(id), Some(dir)) = (args.get(1), args.get(2)) else { usage() };
            let cfg = config(&args, Path::new(dir));
            let c = Convention::load(&repo).unwrap_or_else(|e| fail(&e));
            let git = Git { empty_config: cfg.empty_git_config.clone() };
            let ctx = Ctx { c: &c, git: &git, cfg: &cfg };
            let t = trials::all().into_iter().find(|t| t.id == id).unwrap_or_else(|| fail("no such trial"));
            let s = (t.build)(&ctx, Path::new(dir), t.param).unwrap_or_else(|e| fail(&e));
            println!("fixture {} at {}, base {}", t.id, s.space.display(), s.base);
        }
        "run" => run(&repo, &args),
        "scrub" => {
            let terms = record::scrub_terms(&repo);
            let n = record::rescrub(&repo, &terms).unwrap_or_else(|e| fail(&e));
            println!("{n} run object(s) scrubbed again");
        }
        "exclude" => {
            let Some(reason) = args.get(1) else { usage() };
            let n = record::exclude(&repo, reason, &args[2..]).unwrap_or_else(|e| fail(&e));
            println!("{n} run(s) marked as not counted");
        }
        _ => usage(),
    }
}

fn fail(e: &str) -> ! {
    eprintln!("error: {e}");
    std::process::exit(1)
}

fn opt(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned()
}

fn config(args: &[String], scratch: &Path) -> AgentConfig {
    std::fs::create_dir_all(scratch).ok();
    let empty = scratch.join("empty.gitconfig");
    if !empty.exists() {
        write(&empty, "").ok();
    }
    AgentConfig {
        claude: opt(args, "--claude").unwrap_or_else(|| "claude".into()),
        model: opt(args, "--model").unwrap_or_else(|| "sonnet".into()),
        judge: opt(args, "--judge").unwrap_or_else(|| "haiku".into()),
        timeout: Duration::from_secs(60 * opt(args, "--timeout-min").and_then(|v| v.parse().ok()).unwrap_or(15)),
        empty_git_config: empty,
    }
}

struct Job {
    trial: usize,
    run: usize,
}

fn run(repo: &Path, args: &[String]) {
    let Some(sel) = args.get(1) else { usage() };
    let all = trials::all();
    let chosen: Vec<usize> = all
        .iter()
        .enumerate()
        .filter(|(_, t)| sel == "all" || sel.split(',').any(|s| s == t.id))
        .map(|(i, _)| i)
        .collect();
    if chosen.is_empty() {
        fail("no trial matches");
    }
    let n: usize = opt(args, "--n").and_then(|v| v.parse().ok()).unwrap_or(5);
    let jobs: usize = opt(args, "--jobs").and_then(|v| v.parse().ok()).unwrap_or(4);
    let max_u: f64 = opt(args, "--max-utilization").and_then(|v| v.parse().ok()).unwrap_or(0.85);
    let keep = args.iter().any(|a| a == "--keep");
    // Every counted run belongs to a batch, said before it starts; the views show every batch.
    let batch = opt(args, "--batch").filter(|b| b.parse::<u32>().is_ok()).unwrap_or_else(|| fail("say which batch the runs belong to: --batch <n>"));
    let date = today();
    // Not under the system temp folder: git-bash shows that folder as /tmp, a path the agent's
    // file tools then resolve elsewhere on Windows.
    let scratch = opt(args, "--scratch").map(PathBuf::from).unwrap_or_else(|| {
        std::env::var("LOCALAPPDATA")
            .map(|l| PathBuf::from(l).join("convention-trials"))
            .unwrap_or_else(|_| std::env::temp_dir().join("convention-trials"))
    });
    let cfg = config(args, &scratch);
    let c = Convention::load(repo).unwrap_or_else(|e| fail(&e));
    let git = Git { empty_config: cfg.empty_git_config.clone() };
    let cli = agent::claude_version(&cfg);
    let conv_commit = {
        let head = git.run(repo, &["rev-parse", "--short", "HEAD"]).unwrap_or_default();
        let dirty = git.run(repo, &["status", "--porcelain", "--", "convention", "models", "setup"]).unwrap_or_default();
        format!("{} at {}{}", c.version, head, if dirty.trim().is_empty() { "" } else { " (with uncommitted changes)" })
    };
    let terms = record::scrub_terms(repo);
    eprintln!("convention {conv_commit}; agent {} via {cli}; judge {}; {} trial(s) x {n}; {jobs} at a time", cfg.model, cfg.judge, chosen.len());

    // Run numbers are taken before any run starts, so parallel runs never collide.
    let mut queue = Vec::new();
    for &ti in &chosen {
        let dir = repo.join("tests/runs").join(all[ti].id);
        let existing = walk(repo, &dir).iter().filter(|f| f.contains(&format!("/{date}-"))).count();
        for k in 0..n {
            queue.push(Job { trial: ti, run: existing + k + 1 });
        }
    }
    let queue = Arc::new(Mutex::new(queue.into_iter().rev().collect::<Vec<_>>()));
    let stop = Arc::new(AtomicBool::new(false));
    let results = Arc::new(Mutex::new(Vec::<String>::new()));
    std::thread::scope(|scope| {
        for _ in 0..jobs.max(1) {
            let queue = queue.clone();
            let stop = stop.clone();
            let results = results.clone();
            let (c, git, cfg, all, terms, scratch, date, cli, conv_commit, batch) = (&c, &git, &cfg, &all, &terms, &scratch, &date, &cli, &conv_commit, &batch);
            scope.spawn(move || loop {
                if stop.load(Ordering::SeqCst) {
                    break;
                }
                let Some(job) = queue.lock().unwrap().pop() else { break };
                let t = &all[job.trial];
                let ctx = Ctx { c, git, cfg };
                let line = match one_run(repo, &ctx, t, job.run, date, cli, conv_commit, scratch, terms, keep, batch) {
                    Ok((path, out, u)) => {
                        if let Some(u) = u {
                            if u > max_u {
                                stop.store(true, Ordering::SeqCst);
                                eprintln!("rate-limit utilization {u:.2} is above {max_u}: no new run starts");
                            }
                        }
                        format!("{} run {}: {} ({})", t.id, job.run, out, path.strip_prefix(repo).unwrap_or(&path).display())
                    }
                    Err(e) => format!("{} run {}: setup error: {e}", t.id, job.run),
                };
                eprintln!("{line}");
                results.lock().unwrap().push(line);
            });
        }
    });
    let left = queue.lock().unwrap().len();
    if left > 0 {
        eprintln!("{left} run(s) not started");
    }
}

#[allow(clippy::too_many_arguments)]
fn one_run(
    repo: &Path,
    ctx: &Ctx,
    t: &Trial,
    run: usize,
    date: &str,
    cli: &str,
    conv_commit: &str,
    scratch: &Path,
    terms: &[String],
    keep: bool,
    batch: &str,
) -> Res<(PathBuf, &'static str, Option<f64>)> {
    let root = scratch.join(format!("{}-{}-{:02}-{}", t.id, date, run, std::process::id()));
    if root.exists() {
        force_remove(&root);
    }
    std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let setup = (t.build)(ctx, &root, t.param)?;
    let (prompt, said) = (t.prompt)(ctx, t.param);
    let stream = repo.join(".local/runs").join(t.id).join(format!("{date}-{:02}.jsonl", run));
    // The agent has no network. The convention it would read at its URL is copied beside the
    // space, at the run's commit, and one line says where: the same text, without the web.
    let reference = root.join("convention-ref");
    for part in ["convention", "models", "setup"] {
        copy_dir(&repo.join(part), &reference.join(part))?;
    }
    // The agent may reach the whole fixture: its space, the convention copy, and any other space.
    let add_dirs = vec![root.clone()];
    let instructions = format!("{}{}{}", setup.instructions, if setup.instructions.is_empty() { "" } else { "\n\n" }, REFERENCE_NOTE);
    let trace = agent::run_agent(ctx.cfg, &setup.space, &add_dirs, &instructions, &prompt, &stream)?;
    let data = RunData { setup: &setup, trace: &trace, said: &said };
    let (checks, measures) = (t.check)(ctx, &data, t.param);
    let wrote = what_was_written(ctx, &setup);
    let agents_md = hash_text(ctx, &root, &setup.instructions);
    let meta = record::RunMeta {
        date: date.to_string(),
        run,
        batch: batch.to_string(),
        agent_model: if trace.model.is_empty() { ctx.cfg.model.clone() } else { trace.model.clone() },
        cli: cli.to_string(),
        judge_model: judge_model(&checks, ctx),
        convention: conv_commit.to_string(),
        agents_md: if setup.instructions.is_empty() { "none (the setup prompt only)".into() } else { agents_md },
        prompt_hash: hash_text(ctx, &root, &prompt),
    };
    let path = record::write_run(repo, t, &setup, &root, &meta, &trace, &said, &checks, &measures, &wrote, terms)?;
    let out = record::outcome(&trace, &checks);
    if !keep {
        force_remove(&root);
    }
    Ok((path, out, trace.utilization))
}

/// Appended to the agent's instructions in every run, after the space's `AGENTS.md`.
pub const REFERENCE_NOTE: &str = "This session has no network access, and nobody can approve a command: one that needs approval is denied. The convention and its models, as published at https://github.com/thevzion/everything-is-an-object, are copied at `../convention-ref/` (`convention/`, `models/`, `setup/`): read them there. Give the shell one simple command per call: write and edit files with the file tools, not the shell; then `git add <files>` in one call, and `git commit -F - <<'EOF'` with the message in the next. A command that chains several here-documents, a command substitution such as `$(...)`, and a `cd` out of this folder need approval, so they are denied: use paths instead.";

fn copy_dir(from: &Path, to: &Path) -> Res<()> {
    for rel in walk(from, from) {
        let src = from.join(&rel);
        let dst = to.join(&rel);
        if let Some(d) = dst.parent() {
            std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        std::fs::copy(&src, &dst).map_err(|e| format!("copy {}: {e}", src.display()))?;
    }
    Ok(())
}

fn judge_model(checks: &[trials::Check], ctx: &Ctx) -> String {
    if checks.iter().any(|c| c.judged) {
        checks
            .iter()
            .filter(|c| c.judged)
            .filter_map(|c| c.detail.split_once(':').map(|(m, _)| m.to_string()))
            .next()
            .unwrap_or_else(|| ctx.cfg.judge.clone())
    } else {
        "none: every check is computed".into()
    }
}

fn hash_text(ctx: &Ctx, scratch: &Path, text: &str) -> String {
    let f = scratch.join("hash-input.txt");
    if write(&f, text).is_err() {
        return String::new();
    }
    let h = ctx.git.run(scratch, &["hash-object", &f.to_string_lossy()]).unwrap_or_default();
    let _ = std::fs::remove_file(&f);
    h.chars().take(12).collect()
}

/// What the agent changed: its commits, then each new or changed object in full.
fn what_was_written(ctx: &Ctx, s: &trials::Setup) -> String {
    let mut out = String::new();
    let range = if s.base.is_empty() { "HEAD".to_string() } else { format!("{}..HEAD", s.base) };
    out.push_str(&ctx.git.run(&s.space, &["log", "--reverse", "--stat=100", "--format=commit %h%n%B", &range]).unwrap_or_else(|_| "(no commit)".into()));
    let st = ctx.git.run(&s.space, &["status", "--porcelain", "--untracked-files=all"]).unwrap_or_default();
    if !st.trim().is_empty() {
        out.push_str(&format!("\n\nUncommitted:\n{st}"));
    }
    let (new, changed) = trials::new_and_changed(s);
    for (label, list) in [("New", &new), ("Changed", &changed)] {
        for o in list.iter().take(12) {
            out.push_str(&format!("\n\n=== {label}: {}\n{}", o.path, clip(&o.raw, 2500)));
        }
    }
    for t in space::local_types(&s.space) {
        if !s.before.iter().any(|b| b.path == t.path) {
            out.push_str(&format!("\n\n=== Local type: {}\n{}", t.path, clip(&t.raw, 2000)));
        }
    }
    for d in &s.add_dirs {
        let branches = ctx.git.run(d, &["for-each-ref", "--format=%(refname:short)", "refs/heads"]).unwrap_or_default();
        for b in branches.lines().filter(|b| *b != "main") {
            let diff = ctx.git.run(d, &["log", "-p", "--format=commit %h%n%B", &format!("main..{b}")]).unwrap_or_default();
            out.push_str(&format!("\n\n=== In the other space, branch {b}:\n{}", clip(&diff, 4000)));
        }
        let st = ctx.git.run(d, &["status", "--porcelain", "--untracked-files=all"]).unwrap_or_default();
        if !st.trim().is_empty() {
            out.push_str(&format!("\n\n=== In the other space, uncommitted:\n{st}"));
        }
    }
    out
}

/// Git marks its objects read-only on Windows; clear that before removing a fixture.
fn force_remove(dir: &Path) {
    for rel in walk(dir, dir) {
        let p = dir.join(&rel);
        if let Ok(meta) = std::fs::metadata(&p) {
            let mut perm = meta.permissions();
            #[allow(clippy::permissions_set_readonly_false)]
            perm.set_readonly(false);
            let _ = std::fs::set_permissions(&p, perm);
        }
    }
    let git_dirs: Vec<PathBuf> = std::fs::read_dir(dir).into_iter().flatten().flatten().map(|e| e.path().join(".git")).collect();
    for g in git_dirs.iter().chain(std::iter::once(&dir.join(".git"))) {
        make_writable(g);
    }
    let _ = std::fs::remove_dir_all(dir);
}

fn make_writable(dir: &Path) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            make_writable(&p);
        } else if let Ok(meta) = std::fs::metadata(&p) {
            let mut perm = meta.permissions();
            #[allow(clippy::permissions_set_readonly_false)]
            perm.set_readonly(false);
            let _ = std::fs::set_permissions(&p, perm);
        }
    }
}

/// Today's date from the system clock, as YYYY-MM-DD (UTC).
fn today() -> String {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let days = (secs / 86_400) as i64;
    // Civil from days, after Howard Hinnant.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    format!("{:04}-{:02}-{:02}", if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod selftest;
