//! Each run written as an object, the proof table as a view of those objects, and the link check.

use crate::agent::{Event, Trace};
use crate::trials::{Check, Measures, Setup, Trial};
use crate::util::*;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Terms that must never reach a committed trace: this machine's user and paths, the operator's
/// git identity, and any line of `.local/scrub.txt`. Read at run time, never written in the code.
pub fn scrub_terms(repo: &Path) -> Vec<String> {
    let mut t: Vec<String> = Vec::new();
    for k in ["USERPROFILE", "HOME", "USERNAME", "USER"] {
        if let Ok(v) = std::env::var(k) {
            if v.len() >= 3 {
                t.push(v.clone());
                t.push(v.replace('\\', "/"));
            }
        }
    }
    for key in ["user.name", "user.email"] {
        if let Ok(o) = Command::new("git").args(["config", "--global", key]).output() {
            let v = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if v.len() >= 3 {
                t.push(v);
            }
        }
    }
    for line in read(&repo.join(".local/scrub.txt")).lines() {
        let l = line.trim();
        if l.len() >= 3 && !l.starts_with('#') {
            t.push(l.to_string());
        }
    }
    t.sort_by_key(|s| std::cmp::Reverse(s.len()));
    t.dedup();
    t
}

pub fn scrub(text: &str, fixture_root: &Path, terms: &[String]) -> String {
    let r = fixture_root.to_string_lossy().to_string();
    let mut s = text.to_string();
    for form in [r.replace('\\', "\\\\"), r.clone(), r.replace('\\', "/")] {
        if !form.is_empty() {
            s = s.replace(&form, "<fixture>");
        }
    }
    // A path may also appear in a git-bash form, /c/Users/...
    if r.len() > 2 && r.as_bytes()[1] == b':' {
        let drive = r[..1].to_lowercase();
        let posix = format!("/{}{}", drive, r[2..].replace('\\', "/"));
        s = s.replace(&posix, "<fixture>");
    }
    // A packaged app may see the same folder under another path: whatever path ends in the run's
    // own folder name is the fixture too.
    if let Some(name) = fixture_root.file_name().map(|n| n.to_string_lossy().to_string()) {
        s = replace_path_ending_in(&s, &name);
    }
    // The folder that holds every run's fixture: an agent may name it with a path it guessed.
    if let Some(parent) = fixture_root.parent() {
        let p = parent.to_string_lossy().to_string();
        if p.len() > 3 {
            for form in [p.replace('\\', "\\\\"), p.clone(), p.replace('\\', "/")] {
                s = s.replace(&form, "<scratch>");
            }
            if p.as_bytes()[1] == b':' {
                s = s.replace(&format!("/{}{}", p[..1].to_lowercase(), p[2..].replace('\\', "/")), "<scratch>");
            }
        }
    }
    for term in terms {
        s = replace_ci(&s, term, "<redacted>");
    }
    s
}

/// Replaces every path that ends in `name` (from the start of the path to the end of `name`) by
/// `<fixture>`.
pub fn replace_path_ending_in(s: &str, name: &str) -> String {
    if name.len() < 8 {
        return s.to_string();
    }
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find(name) {
        let before = &rest[..i];
        let start = before
            .rfind(|c: char| c.is_whitespace() || c == '"' || c == '`' || c == '\'' || c == '(' || c == '=' || c == ',')
            .map(|p| p + before[p..].chars().next().map(|c| c.len_utf8()).unwrap_or(1))
            .unwrap_or(0);
        out.push_str(&before[..start]);
        out.push_str("<fixture>");
        rest = &rest[i + name.len()..];
    }
    out.push_str(rest);
    out
}

/// Applies the path rule to run objects already written: the folder name of each run is in its
/// file, as part of the paths the agent saw.
pub fn rescrub(repo: &Path, terms: &[String]) -> Res<usize> {
    let mut n = 0;
    for rel in walk(repo, &repo.join("tests/runs")) {
        if !rel.ends_with(".md") || rel.ends_with("README.md") {
            continue;
        }
        let p = repo.join(&rel);
        let text = read(&p);
        let mut s = text.clone();
        // Run folders are named <trial>-<date>-<nn>-<pid>; find them after "convention-trials".
        let mut names = Vec::new();
        for (i, _) in text.match_indices("convention-trials") {
            let after = &text[i + "convention-trials".len()..];
            let after = after.trim_start_matches(['/', '\\']);
            let end = after.find(|c: char| c == '/' || c == '\\' || c.is_whitespace() || c == '`' || c == '"').unwrap_or(after.len());
            let name = after[..end].to_string();
            if name.len() >= 8 && name.contains("-2026-") && !names.contains(&name) {
                names.push(name);
            }
        }
        for name in &names {
            s = replace_path_ending_in(&s, name);
        }
        for term in terms {
            s = replace_ci(&s, term, "<redacted>");
        }
        if s != text {
            write(&p, &s)?;
            n += 1;
        }
    }
    Ok(n)
}

fn replace_ci(s: &str, term: &str, by: &str) -> String {
    let lower = s.to_lowercase();
    let t = term.to_lowercase();
    if t.is_empty() || !lower.contains(&t) || lower.len() != s.len() {
        return if lower.len() != s.len() { s.replace(term, by) } else { s.to_string() };
    }
    let mut out = String::new();
    let mut i = 0;
    while let Some(p) = lower[i..].find(&t) {
        out.push_str(&s[i..i + p]);
        out.push_str(by);
        i += p + t.len();
    }
    out.push_str(&s[i..]);
    out
}

pub struct RunMeta {
    pub date: String,
    pub run: usize,
    pub batch: String,
    pub agent_model: String,
    pub cli: String,
    pub judge_model: String,
    pub convention: String,
    pub agents_md: String,
    pub prompt_hash: String,
}

pub fn outcome(trace: &Trace, checks: &[Check]) -> &'static str {
    if !trace.got_result || trace.is_error {
        return "error";
    }
    if checks.iter().any(|c| c.ok.is_none()) {
        return "error";
    }
    if checks.iter().all(|c| c.ok == Some(true)) {
        "pass"
    } else {
        "fail"
    }
}

#[allow(clippy::too_many_arguments)]
pub fn write_run(
    repo: &Path,
    trial: &Trial,
    setup: &Setup,
    fixture_root: &Path,
    meta: &RunMeta,
    trace: &Trace,
    said: &str,
    checks: &[Check],
    measures: &Measures,
    wrote: &str,
    terms: &[String],
) -> Res<PathBuf> {
    let out = outcome(trace, checks);
    let passed = checks.iter().filter(|c| c.ok == Some(true)).count();
    let short_model = meta.agent_model.replace("claude-", "");
    let dir = repo.join("tests/runs").join(trial.id);
    let path = dir.join(format!("{}-{}-{:02}.md", meta.date, short_model, meta.run));
    let checks_line = checks
        .iter()
        .map(|c| format!("{} {}", c.id, match c.ok { Some(true) => "pass", Some(false) => "fail", None => "error" }))
        .collect::<Vec<_>>()
        .join(", ");
    let mut s = String::new();
    s.push_str("---\ntype: result\nmodel: loop@0.1\nquestion: What happened?\n");
    s.push_str(&format!("serves: {}\nsource: inferred\nlifecycle: done\nevidence: observed\ndate: {}\n", trial.file, meta.date));
    s.push_str(&format!("trial: {}\nrun: {}\nbatch: {}\noutcome: {}\npassed: {}/{}\nchecks: {}\n", trial.id, meta.run, meta.batch, out, passed, checks.len(), checks_line));
    s.push_str(&format!("agent: {}\ncli: {}\njudge: {}\nconvention: {}\nagents-md: {}\nprompt: {}\nfixture: {}\n", meta.agent_model, meta.cli, meta.judge_model, meta.convention, meta.agents_md, meta.prompt_hash, if setup.base.is_empty() { "empty folder".into() } else { setup.base.clone() }));
    s.push_str(&format!(
        "tokens-in: {}\ntokens-cache-write: {}\ntokens-cache-read: {}\ntokens-out: {}\ncost-usd: {:.4}\nturns: {}\nseconds: {}\n---\n\n",
        trace.tokens_in, trace.tokens_cache_write, trace.tokens_cache_read, trace.tokens_out, trace.cost_usd, trace.turns, trace.duration_ms / 1000
    ));
    s.push_str(&format!("# {}: run {}, {}\n\n", trial.id, meta.run, out));
    s.push_str(&format!("**The question.** {}\n\n**The person said:**\n\n> {}\n\n", trial.question, said.replace('\n', "\n> ")));
    s.push_str(&format!("Written by the runner (`tests/runner`) from the fixture and the agent's stream; the criteria are in [the trial](../../../{}), fixed before its first counted run, and versioned there when the convention changed. The agent had no network: the convention, its models and the setup prompt at the run's commit were copied beside the space, and its instructions said where.\n\n", trial.file));
    s.push_str("## Checks\n\n| # | Criterion | Result | Detail |\n|---|---|---|---|\n");
    for c in checks {
        let r = match c.ok { Some(true) => "pass", Some(false) => "**fail**", None => "error" };
        let by = if c.judged { " (decision model)" } else { "" };
        s.push_str(&format!("| {} | {}{} | {} | {} |\n", c.id, c.label, by, r, one_line(&c.detail).replace('|', "\\|")));
    }
    s.push_str("\n## Measures\n\n| Measure | Value |\n|---|---|\n");
    for (k, v) in measures {
        s.push_str(&format!("| {} | {} |\n", k, one_line(v).replace('|', "\\|")));
    }
    s.push_str(&format!("| cost | ${:.4}, {} turns, {} s |\n", trace.cost_usd, trace.turns, trace.duration_ms / 1000));
    s.push_str("\n## What the agent wrote\n\n");
    s.push_str(&fence(wrote));
    s.push_str("\n## Trace\n\nThe agent's text and tool calls, in order. Paths are relative to the fixture.\n\n");
    for (i, e) in trace.events.iter().enumerate() {
        let line = match e {
            Event::Text(t) => format!("{}. text: {}", i + 1, clip(&one_line(t), 1200)),
            Event::Tool { name, input, writes } => format!("{}. {}{}: `{}`", i + 1, name, if *writes { " (writes)" } else { "" }, clip(&one_line(input), 300).replace('`', "'")),
            Event::ToolError(t) => format!("{}. tool error: {}", i + 1, t),
            Event::Denied(t) => format!("{}. denied: {}", i + 1, t),
        };
        s.push_str(&line);
        s.push('\n');
    }
    s.push_str("\n## Final reply\n\n");
    s.push_str(&fence(&trace.final_text));
    let s = scrub(&s, fixture_root, terms);
    let s = scrub(&s, &setup.space, terms);
    for t in terms {
        if contains_ci(&s, t) {
            return Err("a scrub term survived; the run is not written".into());
        }
    }
    write(&path, &s)?;
    Ok(path)
}

fn fence(t: &str) -> String {
    let t = t.trim_end();
    let mut ticks = "```".to_string();
    while t.contains(&ticks) {
        ticks.push('`');
    }
    format!("{ticks}text\n{}\n{ticks}\n", if t.is_empty() { "(nothing)" } else { t })
}

// ---------------------------------------------------------------- the proof, as a view

struct RunRow {
    file: String,
    excluded: String,
    batch: String,
    trial: String,
    outcome: String,
    date: String,
    agent: String,
    convention: String,
    cost: f64,
    tokens: u64,
    seconds: u64,
    checks: Vec<(String, String)>,
    denied: bool,
    /// For the local type: does L4 hold by its written criterion? Until pass 2 the code also
    /// asked for `serves`; a recorded failure that names only a missing `serves` holds by it.
    l4_criterion: Option<bool>,
    /// A header check of this run names an object that serves nothing yet: allowed by version 2
    /// of the header criteria, a failure by version 1.
    serves_nothing: bool,
}

/// The batch a run belongs to: its `batch:` line. Runs written before that line existed are read
/// from their `excluded:` reason (a smoke run, batch 1), and are batch 2 when they have none.
fn batch_of(h: &BTreeMap<String, Vec<String>>) -> String {
    let b = first(h, "batch");
    if !b.is_empty() {
        return b;
    }
    let e = first(h, "excluded");
    if e.starts_with("a smoke run") {
        "smoke".into()
    } else if e.starts_with("batch 1") {
        "1".into()
    } else if e.is_empty() {
        "2".into()
    } else {
        "marked".into()
    }
}

/// The result and the detail of one check, from the run's table of checks.
fn check_row(raw: &str, id: &str) -> Option<(String, String)> {
    let prefix = format!("| {id} | ");
    let line = raw.lines().find(|l| l.starts_with(&prefix))?;
    let cells: Vec<&str> = line.split(" | ").collect();
    if cells.len() < 4 {
        return None;
    }
    Some((cells[2].trim_matches('*').to_string(), cells[3..].join(" | ").trim_end_matches(" |").trim_end_matches('|').trim().to_string()))
}

/// An L4 detail whose every fault is a missing `serves`, with at least one object.
fn l4_only_serves(detail: &str) -> bool {
    let mut parts = detail.split("; ");
    let count = parts.next().and_then(|p| p.split_whitespace().next()).and_then(|n| n.parse::<usize>().ok()).unwrap_or(0);
    let rest: Vec<&str> = parts.collect();
    count > 0 && !rest.is_empty() && rest.iter().all(|f| f.split_once(": ").map_or(false, |(_, faults)| faults.split(", ").all(|x| x == "no serves")))
}

fn load_runs(repo: &Path) -> Vec<RunRow> {
    let mut rows = Vec::new();
    for rel in walk(repo, &repo.join("tests/runs")) {
        if !rel.ends_with(".md") || rel.ends_with("README.md") {
            continue;
        }
        let raw = read(&repo.join(&rel));
        let denied = raw.lines().any(|l| l.split_once(". denied: ").map_or(false, |(n, _)| n.chars().all(|c| c.is_ascii_digit())));
        let (h, _) = match parse_header(&raw) {
            Some(x) => x,
            None => continue,
        };
        if first(&h, "type") != "result" {
            continue;
        }
        let num = |k: &str| first(&h, k).parse::<f64>().unwrap_or(0.0);
        let checks: Vec<(String, String)> = first(&h, "checks")
            .split(", ")
            .filter_map(|c| c.split_once(' ').map(|(a, b)| (a.to_string(), b.to_string())))
            .collect();
        let trial = first(&h, "trial");
        let l4_criterion = if trial.ends_with("-local-type") {
            check_row(&raw, "L4").map(|(res, det)| res == "pass" || l4_only_serves(&det))
        } else {
            None
        };
        rows.push(RunRow {
            file: rel.trim_start_matches("tests/runs/").to_string(),
            excluded: first(&h, "excluded"),
            batch: batch_of(&h),
            trial,
            outcome: first(&h, "outcome"),
            date: first(&h, "date"),
            agent: first(&h, "agent"),
            convention: first(&h, "convention"),
            cost: num("cost-usd"),
            tokens: (num("tokens-in") + num("tokens-cache-write") + num("tokens-cache-read") + num("tokens-out")) as u64,
            seconds: num("seconds") as u64,
            checks,
            denied,
            l4_criterion,
            serves_nothing: raw.contains("; serving nothing yet: "),
        });
    }
    rows
}

fn median(mut v: Vec<f64>) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 }
}

fn fmt_tokens(t: f64) -> String {
    if t >= 1_000_000.0 { format!("{:.1}M", t / 1_000_000.0) } else { format!("{}k", (t / 1000.0).round() as u64) }
}

fn list(mut v: Vec<String>) -> String {
    v.sort();
    v.dedup();
    v.join(", ")
}

/// Passes over finished runs, with the errors apart: `k/n`, or `k/n, e errors`.
fn cell(rs: &[&RunRow]) -> String {
    if rs.is_empty() {
        return "not run".into();
    }
    let k = rs.iter().filter(|r| r.outcome == "pass").count();
    let n = rs.iter().filter(|r| r.outcome == "pass" || r.outcome == "fail").count();
    let e = rs.iter().filter(|r| r.outcome == "error").count();
    if e > 0 { format!("{k}/{n}, {e} error{}", if e > 1 { "s" } else { "" }) } else { format!("{k}/{n}") }
}

/// A run that fails only on L4, where L4 holds by its written criterion, passes by that criterion.
fn passes_by_l4_criterion(r: &RunRow) -> bool {
    r.outcome == "pass" || (r.outcome == "fail" && r.l4_criterion == Some(true) && r.checks.iter().all(|(id, res)| res == "pass" || id == "L4"))
}

/// The batches run before the public history began: their run objects cite commits of the history
/// the founder keeps, which are not public.
const KEPT_HISTORY_BATCHES: u32 = 3;

/// The README's proof table and `tests/runs/README.md`, both computed from the run objects. Every
/// batch is shown, the latest first; smoke runs are listed in the runs view, not counted in a batch.
pub fn table(repo: &Path, trials: &[Trial]) -> Res<(String, String)> {
    let rows = load_runs(repo);
    let mut batches: Vec<u32> = rows.iter().filter_map(|r| r.batch.parse::<u32>().ok()).collect();
    batches.sort_by_key(|b| std::cmp::Reverse(*b));
    batches.dedup();
    let of = |t: &str, b: u32| -> Vec<&RunRow> { rows.iter().filter(|r| r.trial == t && r.batch == b.to_string()).collect() };

    let mut readme = String::from("| Question |");
    for b in &batches {
        readme.push_str(&format!(" Batch {b} |"));
    }
    readme.push_str(" Trace |\n|---|");
    for _ in &batches {
        readme.push_str("---|");
    }
    readme.push_str("---|\n");
    for t in trials {
        readme.push_str(&format!("| {} |", t.question));
        for b in &batches {
            readme.push_str(&format!(" {} |", cell(&of(t.id, *b))));
        }
        readme.push_str(&format!(" [{}](tests/runs/{}/) |\n", t.id, t.id));
    }
    readme.push('\n');
    let kept = batches.iter().any(|b| *b <= KEPT_HISTORY_BATCHES);
    for b in &batches {
        let rs: Vec<&RunRow> = rows.iter().filter(|r| r.batch == b.to_string()).collect();
        let cost: f64 = rs.iter().map(|r| r.cost).sum();
        readme.push_str(&format!(
            "- **Batch {b}**: {} runs, {}; {}; convention {}; ${cost:.2} at API prices as the CLI reports them.\n",
            rs.len(),
            list(rs.iter().map(|r| r.date.clone()).collect()),
            list(rs.iter().map(|r| r.agent.clone()).collect()),
            list(rs.iter().map(|r| r.convention.clone()).collect())
        ));
    }
    let mut notes = String::new();
    if kept {
        notes.push_str("- **Commit ids**: batches 1 to 3 cite commits of the history written before publication, which the founder keeps; the public history starts at Convention 0.3, draft. Batch 3 ran against the same text (`convention/`, `setup/`, `models/`, `AGENTS.md`) as the first public commit, apart from three lines of `convention/README.md` that drop the earlier name.\n");
    }
    if batches.iter().any(|b| *b >= 3) {
        notes.push_str("- **Header criteria**: batches 1 and 2 are checked by version 1, which asks for `serves` on every object but a root; from batch 3, by version 2, fixed before batch 3 ran, where an object may serve nothing yet. Version 1 stays computable from every run, and is read again below where it differs.\n");
    }
    // Read again, from what each run recorded: L4 by its written criterion, and the batches checked
    // by version 2 of the header criteria as version 1 would read them.
    let mut again = Vec::new();
    for t in trials.iter().filter(|t| t.id.ends_with("-local-type")) {
        let parts: Vec<String> = batches
            .iter()
            .rev()
            .filter_map(|b| {
                let rs = of(t.id, *b);
                let k = rs.iter().filter(|r| passes_by_l4_criterion(r)).count();
                let n = rs.iter().filter(|r| r.outcome == "pass" || r.outcome == "fail").count();
                let recorded = rs.iter().filter(|r| r.outcome == "pass").count();
                if k != recorded { Some(format!("{k}/{n} in batch {b}")) } else { None }
            })
            .collect();
        if !parts.is_empty() {
            again.push(format!("- **{}, read by the written criterion of L4**, which the code exceeded until pass 2 by also asking for `serves`: {}.", t.id, parts.join(", ")));
        }
    }
    for b in batches.iter().filter(|b| **b >= 3) {
        let parts: Vec<String> = trials
            .iter()
            .filter_map(|t| {
                let rs = of(t.id, *b);
                let k1 = rs.iter().filter(|r| r.outcome == "pass" && !r.serves_nothing).count();
                let n = rs.iter().filter(|r| r.outcome == "pass" || r.outcome == "fail").count();
                let k = rs.iter().filter(|r| r.outcome == "pass").count();
                if k1 != k { Some(format!("{} {k1}/{n}", t.id)) } else { None }
            })
            .collect();
        if !parts.is_empty() {
            again.push(format!("- **Batch {b}, read by version 1 of the header criteria**, which asked for `serves` on every object but a root: {}.", parts.join(", ")));
        }
    }
    for a in &again {
        notes.push_str(&a.replace("](tests/runs/", "]("));
        notes.push('\n');
    }
    if !notes.is_empty() {
        readme.push_str("- **Reading the counts**: the commit ids, the versions of the criteria, and the counts read again are in [the runs](tests/runs/README.md#reading-the-counts).\n");
    }

    let mut detail = String::from("# Runs\n\nA view of the run objects in this folder, generated by `convention-trials table`; never edited by hand. Each run is a `result` object (`loop@0.1`), under CC BY 4.0 like the rest of the text. Its checks were fixed before the first counted run, in the trial's file, and versioned there when the convention changed. Every batch is shown; the smoke runs are listed at the end.\n\n");
    detail.push_str("| Trial | Batch | Passed | Failed, with a command denied | Errors | Model | Dates | Median tokens | Median cost | Median time |\n|---|---|---|---|---|---|---|---|---|---|\n");
    let mut per_check = String::new();
    for t in trials {
        let mut lines = Vec::new();
        for b in &batches {
            let rs = of(t.id, *b);
            if rs.is_empty() {
                continue;
            }
            let e = rs.iter().filter(|r| r.outcome == "error").count();
            let d = rs.iter().filter(|r| r.outcome == "fail" && r.denied).count();
            let k = rs.iter().filter(|r| r.outcome == "pass").count();
            let n = rs.iter().filter(|r| r.outcome == "pass" || r.outcome == "fail").count();
            let mt = median(rs.iter().map(|r| r.tokens as f64).collect());
            let mc = median(rs.iter().map(|r| r.cost).collect());
            let ms = median(rs.iter().map(|r| r.seconds as f64).collect());
            detail.push_str(&format!(
                "| [{}]({}/) | {b} | {k}/{n} | {d} | {e} | {} | {} | {} | ${:.2} | {} s |\n",
                t.id,
                t.id,
                list(rs.iter().map(|r| r.agent.clone()).collect()),
                list(rs.iter().map(|r| r.date.clone()).collect()),
                fmt_tokens(mt),
                mc,
                ms as u64
            ));
            let mut by: BTreeMap<String, (usize, usize)> = BTreeMap::new();
            for r in &rs {
                for (id, res) in &r.checks {
                    let e = by.entry(id.clone()).or_default();
                    e.1 += 1;
                    if res == "pass" {
                        e.0 += 1;
                    }
                }
            }
            lines.push(format!("batch {b}: {}", by.iter().map(|(id, (p, n))| format!("{id} {p}/{n}")).collect::<Vec<_>>().join(", ")));
        }
        if !lines.is_empty() {
            per_check.push_str(&format!("- **{}**: {}\n", t.id, lines.join("; ")));
        }
    }
    detail.push_str("\nIn all, by batch:\n\n");
    for b in batches.iter().map(|b| b.to_string()).chain(std::iter::once("smoke".to_string())) {
        let rs: Vec<&RunRow> = rows.iter().filter(|r| r.batch == b).collect();
        if rs.is_empty() {
            continue;
        }
        let label = if b == "smoke" { "smoke runs".to_string() } else { format!("batch {b}") };
        detail.push_str(&format!(
            "- {label}: {} runs, {} tokens (input, cache and output), ${:.2} at API prices as the CLI reports them.\n",
            rs.len(),
            fmt_tokens(rs.iter().map(|r| r.tokens as f64).sum()),
            rs.iter().map(|r| r.cost).sum::<f64>()
        ));
    }
    detail.push_str(&format!("\n## Each check\n\nHow many runs passed each check, out of the runs that reached it, batch by batch.\n\n{per_check}"));
    if !notes.is_empty() {
        detail.push_str("\n## Reading the counts\n\nWhat to know before comparing the batches. The counts read again come from what each run recorded, without changing it.\n\n");
        detail.push_str(&notes);
    }
    let marked: Vec<&RunRow> = rows.iter().filter(|r| !r.excluded.is_empty()).collect();
    if !marked.is_empty() {
        detail.push_str("\n## Runs with a mark\n\nKept as they were written, with the mark each one carries. The smoke runs are not a batch; the runs of batch 1 are shown in their column.\n\n");
        for r in &marked {
            detail.push_str(&format!("- [{}]({}): {}, batch {}: {}\n", r.file, r.file, r.outcome, r.batch, r.excluded));
        }
    }
    if kept {
        detail.push_str("\n## Commit ids\n\nEach run keeps the commit id it was written with (`convention: <version> at <id>`). The smoke runs and batches 1 to 3 cite commits of the history written before publication, which the founder keeps; they are not in the public history, which starts at Convention 0.3, draft. Batch 3 ran against the same text (`convention/`, `setup/`, `models/`, `AGENTS.md`) as the first public commit, apart from three lines of `convention/README.md` that drop the earlier name.\n");
    }
    Ok((readme, detail))
}

/// Marks run objects as not counted, with the reason, without changing anything else in them.
pub fn exclude(repo: &Path, reason: &str, files: &[String]) -> Res<usize> {
    let mut n = 0;
    for f in files {
        let p = repo.join(f);
        let text = read(&p);
        if text.is_empty() || text.contains("\nexcluded: ") {
            continue;
        }
        let Some(i) = text.find("\noutcome: ") else { continue };
        let j = text[i + 1..].find('\n').map(|x| x + i + 1).unwrap_or(text.len());
        let updated = format!("{}\nexcluded: {}{}", &text[..j], reason, &text[j..]);
        write(&p, &updated)?;
        n += 1;
    }
    Ok(n)
}

pub fn replace_between(text: &str, begin: &str, end: &str, inner: &str) -> Res<String> {
    let a = text.find(begin).ok_or(format!("no marker {begin}"))? + begin.len();
    let b = text.find(end).ok_or(format!("no marker {end}"))?;
    Ok(format!("{}\n{}{}", &text[..a], inner, &text[b..]))
}

// ---------------------------------------------------------------- links

/// GitHub's anchor for a heading: lowercase, punctuation dropped, spaces as hyphens.
fn slug(h: &str) -> String {
    let mut s = String::new();
    for c in h.trim().chars() {
        if c.is_alphanumeric() {
            s.extend(c.to_lowercase());
        } else if c == ' ' || c == '-' {
            s.push('-');
        } else if c == '_' {
            s.push('_');
        }
    }
    s
}

fn anchors(md: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_code = false;
    for line in md.lines() {
        if line.starts_with("```") {
            in_code = !in_code;
        }
        if in_code {
            continue;
        }
        if line.starts_with('#') {
            let h = line.trim_start_matches('#');
            let base = slug(&h.replace('`', ""));
            let mut a = base.clone();
            let mut i = 1;
            while out.contains(&a) {
                a = format!("{base}-{i}");
                i += 1;
            }
            out.push(a);
        }
    }
    out
}

/// Every relative link of every Markdown file tracked or about to be: does its file exist, and
/// its anchor? Returns the broken ones.
pub fn check_links(repo: &Path) -> Vec<String> {
    let mut broken = Vec::new();
    let files: Vec<String> = walk(repo, repo).into_iter().filter(|f| f.ends_with(".md") && !f.starts_with(".local/") && !f.starts_with("tests/runner/target")).collect();
    for f in &files {
        let text = read(&repo.join(f));
        let dir = Path::new(f).parent().unwrap_or(Path::new("")).to_path_buf();
        let mut in_code = false;
        for line in text.lines() {
            if line.trim_start().starts_with("```") {
                in_code = !in_code;
            }
            if in_code {
                continue;
            }
            let mut rest = line;
            while let Some(i) = rest.find("](") {
                let after = &rest[i + 2..];
                let Some(j) = after.find(')') else { break };
                let target = after[..j].trim();
                rest = &after[j + 1..];
                if target.is_empty() || target.contains("://") || target.starts_with("mailto:") || target.contains(' ') {
                    continue;
                }
                let (file, anchor) = match target.split_once('#') {
                    Some((a, b)) => (a, Some(b)),
                    None => (target, None),
                };
                let path = if file.is_empty() { PathBuf::from(f) } else { normalize(&dir.join(file)) };
                let full = repo.join(&path);
                if !full.exists() {
                    broken.push(format!("{f}: {target} (no such file)"));
                    continue;
                }
                if let Some(a) = anchor {
                    if full.is_file() && path.extension().map_or(false, |e| e == "md") && !anchors(&read(&full)).iter().any(|x| x == a) {
                        broken.push(format!("{f}: {target} (no such anchor)"));
                    }
                }
            }
        }
    }
    broken
}

fn normalize(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in p.components() {
        match c {
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs_follow_github() {
        assert_eq!(slug(" The axes as state machines (proposed)"), "the-axes-as-state-machines-proposed");
        assert_eq!(slug(" Routines: the loop that runs alone (candidate)"), "routines-the-loop-that-runs-alone-candidate");
        assert_eq!(slug(" Section 5. Between us"), "section-5-between-us");
    }

    #[test]
    fn scrub_replaces_paths_and_terms() {
        let root = Path::new("C:\\tmp\\run-1");
        let t = scrub("cwd C:\\tmp\\run-1\\home and C:/tmp/run-1/home by Someone", root, &["someone".into()]);
        assert_eq!(t, "cwd <fixture>\\home and <fixture>/home by <redacted>");
        let t = scrub("Glob in C:\\tmp\\convention-ref, beside C:\\tmp\\run-1", root, &[]);
        assert_eq!(t, "Glob in <scratch>\\convention-ref, beside <fixture>");
    }
}

#[cfg(test)]
mod path_tests {
    use super::*;

    #[test]
    fn a_path_ending_in_the_run_folder_is_the_fixture() {
        let s = "Initialized in X:/Some/Other/Prefix/convention-trials/30-setup-2026-11-17-06-93080/home/.git/ ok";
        assert_eq!(replace_path_ending_in(s, "30-setup-2026-11-17-06-93080"), "Initialized in <fixture>/home/.git/ ok");
    }
}
