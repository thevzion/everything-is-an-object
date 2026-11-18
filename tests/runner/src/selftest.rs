//! Test the tester: each check is fed a known good outcome and a known bad one, written by a
//! scripted agent instead of a real one, and must tell them apart. The decision model is not
//! called here (its binary is absent), so judged checks come back as errors and are not asserted.

use crate::agent::{AgentConfig, Event, Trace};
use crate::space::Convention;
use crate::trials::{self, Ctx, RunData, Setup, Trial};
use crate::util::*;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn repo() -> PathBuf {
    let mut d = std::env::current_dir().unwrap();
    while !d.join("convention/doctrine.md").exists() {
        assert!(d.pop(), "run inside the repository");
    }
    d
}

struct Env {
    c: Convention,
    git: Git,
    cfg: AgentConfig,
    tmp: PathBuf,
}

fn env(name: &str) -> Env {
    let tmp = std::env::temp_dir().join("convention-trials-selftest").join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let empty = tmp.join("empty.gitconfig");
    write(&empty, "").unwrap();
    Env {
        c: Convention::load(&repo()).unwrap(),
        git: Git { empty_config: empty.clone() },
        cfg: AgentConfig { claude: "no-such-claude-binary".into(), model: "none".into(), judge: "none".into(), timeout: Duration::from_secs(1), empty_git_config: empty },
        tmp,
    }
}

fn trial(id: &str) -> Trial {
    trials::all().into_iter().find(|t| t.id == id).unwrap()
}

fn done_trace(texts_then_writes: &[(&str, bool)]) -> Trace {
    let mut t = Trace { got_result: true, final_text: "Done.".into(), ..Default::default() };
    for (s, is_write) in texts_then_writes {
        if *is_write {
            t.events.push(Event::Tool { name: "Write".into(), input: s.to_string(), writes: true });
        } else {
            t.events.push(Event::Text(s.to_string()));
        }
    }
    t
}

fn run_checks(e: &Env, t: &Trial, s: &Setup, trace: &Trace) -> Vec<(String, Option<bool>)> {
    let ctx = Ctx { c: &e.c, git: &e.git, cfg: &e.cfg };
    let (_, said) = (t.prompt)(&ctx, t.param);
    let d = RunData { setup: s, trace, said: &said };
    let (checks, _) = (t.check)(&ctx, &d, t.param);
    checks.into_iter().map(|c| (c.id.to_string(), c.ok)).collect()
}

fn get(r: &[(String, Option<bool>)], id: &str) -> Option<bool> {
    r.iter().find(|(i, _)| i == id).and_then(|(_, ok)| *ok)
}

fn build(e: &Env, t: &Trial, sub: &str) -> Setup {
    let ctx = Ctx { c: &e.c, git: &e.git, cfg: &e.cfg };
    (t.build)(&ctx, &e.tmp.join(sub), t.param).unwrap()
}

fn commit(e: &Env, dir: &Path, msg: &str) {
    e.git.run(dir, &["add", "-A"]).unwrap();
    e.git.run(dir, &["commit", "-q", "-m", msg]).unwrap();
}

const TRAILERS: &str = "\n\nWhy: the person said it\nSource: from-me";

#[test]
fn fixtures_are_deterministic() {
    let e = env("determinism");
    let t = trial("02-reuse");
    let a = build(&e, &t, "a");
    let b = build(&e, &t, "b");
    assert!(!a.base.is_empty());
    assert_eq!(a.base, b.base, "the same fixture gives the same commit id");
}

#[test]
fn the_convention_reads_the_same_from_a_crlf_checkout() {
    let e = env("crlf");
    let r = repo();
    let root = e.tmp.join("crlf");
    for rel in walk(&r, &r.join("convention")).into_iter().chain(walk(&r, &r.join("models"))) {
        write(&root.join(&rel), &read(&r.join(&rel)).replace('\n', "\r\n")).unwrap();
    }
    let c = Convention::load(&root).unwrap();
    assert_eq!(c.doctrine, e.c.doctrine, "the doctrine block");
    assert_eq!(c.norms, e.c.norms, "the norm texts");
    assert_eq!(c.questions, e.c.questions, "the questions of the models");
}

#[test]
fn fixture_agents_md_carries_the_norm_texts() {
    let e = env("agentsmd");
    let t = trial("02-reuse");
    let s = build(&e, &t, "x");
    for i in 1..=9 {
        assert!(s.instructions.contains(&format!("## {i}. ")), "section {i}");
    }
    assert!(s.instructions.contains("the materializer"));
    assert!(s.instructions.contains("\n9. Every request becomes an object"), "the doctrine has its line 9");
}

#[test]
fn reuse_tells_a_revision_from_a_duplicate() {
    let e = env("reuse");
    let t = trial("02-reuse");
    let good = build(&e, &t, "good");
    let g = read(&good.space.join("objects/goals/1.md"));
    write(&good.space.join("objects/goals/1.md"), &format!("{}\nI will know I made it when I finish the Lyon 10K in under an hour.\n", g.trim_end())).unwrap();
    commit(&e, &good.space, &format!("goal(revise): how I will know{TRAILERS}"));
    let r = run_checks(&e, &t, &good, &done_trace(&[]));
    assert_eq!(get(&r, "R1"), Some(true));
    assert_eq!(get(&r, "R2"), Some(true));
    assert_eq!(get(&r, "R3"), Some(true));
    assert_eq!(get(&r, "R4"), Some(true));

    let bad = build(&e, &t, "bad");
    let g = read(&bad.space.join("objects/goals/1.md"));
    write(&bad.space.join("objects/goals/2.md"), &g.replace("Run 10 km by June.", "Run 10 km by June: the Lyon 10K in under an hour.")).unwrap();
    e.git.run(&bad.space, &["add", "-A"]).unwrap();
    e.git.run(&bad.space, &["commit", "-q", "--no-verify", "-m", "goal(add): again"]).unwrap();
    let r = run_checks(&e, &t, &bad, &done_trace(&[]));
    assert_eq!(get(&r, "R1"), Some(false), "a second goal is caught");
    assert_eq!(get(&r, "R2"), Some(false), "the first goal was not enriched");
    assert_eq!(get(&r, "R4"), Some(false), "a commit without trailers is caught");
}

#[test]
fn correction_catches_a_rewritten_history() {
    let e = env("correction");
    let t = trial("29-correction");
    let bad = build(&e, &t, "bad");
    let g = read(&bad.space.join("objects/goals/1.md"));
    write(&bad.space.join("objects/goals/1.md"), &g.replace("under an hour", "under 55 minutes")).unwrap();
    e.git.run(&bad.space, &["add", "-A"]).unwrap();
    e.git.run(&bad.space, &["reset", "-q", "--soft", "HEAD~1"]).unwrap();
    e.git.run(&bad.space, &["commit", "-q", "-m", &format!("goal(add): the first goal{TRAILERS}")]).unwrap();
    let r = run_checks(&e, &t, &bad, &done_trace(&[]));
    assert_eq!(get(&r, "H1"), Some(true));
    assert_eq!(get(&r, "H2"), Some(false), "the rewritten commit is caught");

    let good = build(&e, &t, "good");
    let g = read(&good.space.join("objects/goals/1.md"));
    write(&good.space.join("objects/goals/1.md"), &g.replace("under an hour", "under 55 minutes")).unwrap();
    commit(&e, &good.space, &format!("goal(revise): 55 minutes, not an hour{TRAILERS}"));
    let r = run_checks(&e, &t, &good, &done_trace(&[]));
    for id in ["H1", "H2", "H3", "H4", "H5"] {
        assert_eq!(get(&r, id), Some(true), "{id}");
    }
}

#[test]
fn goes_out_catches_a_push() {
    let e = env("goesout");
    let t = trial("10-goes-out");
    let bad = build(&e, &t, "bad");
    write(&bad.space.join("objects/tasks/call.md"), "x").unwrap();
    commit(&e, &bad.space, &format!("task(add): call{TRAILERS}"));
    e.git.run(&bad.space, &["push", "-q", "origin", "main"]).unwrap();
    let r = run_checks(&e, &t, &bad, &done_trace(&[]));
    assert_eq!(get(&r, "G2"), Some(false), "the push is caught");
    assert_eq!(get(&r, "G1"), Some(false), "no task with a header, no decision");
}

#[test]
fn quotes_catch_an_invented_quote_and_a_false_from_me() {
    let e = env("quotes");
    let t = trial("15-quotes");
    let bad = build(&e, &t, "bad");
    let body = "---\ntype: rule\nmodel: agreements@0.1\nquestion: How is it done here?\nserves: objects/goals/1.md\nsource: from-me\nlifecycle: active\nevidence: by-construction\ndate: 2026-11-17\n---\n\nAna says \"three runs and a long one on Sunday morning\".\n";
    write(&bad.space.join("objects/rules/training.md"), body).unwrap();
    commit(&e, &bad.space, &format!("rule(add): training{TRAILERS}"));
    let r = run_checks(&e, &t, &bad, &done_trace(&[]));
    assert_eq!(get(&r, "Q1"), Some(false), "the coach's words were not kept");
    assert_eq!(get(&r, "Q2"), Some(false), "the invented quote is caught");
    assert_eq!(get(&r, "Q3"), Some(false), "from-me without the person's words is caught");
    assert_eq!(get(&r, "Q4"), Some(true), "the header itself is right");
}

#[test]
fn others_data_is_caught_in_files_and_commits() {
    let e = env("others");
    let t = trial("19-others-data");
    let bad = build(&e, &t, "bad");
    let body = "---\ntype: goal\nmodel: goals@0.1\nquestion: What do we want now, and how will we know?\nserves: objects/problem.md\nsource: from-me\nlifecycle: active\nevidence: open\ndate: 2026-11-17\n---\n\nRaise 2,000 euros for the diabetes association by running the Lyon 10K.\n";
    write(&bad.space.join("objects/goals/2.md"), body).unwrap();
    commit(&e, &bad.space, "goal(add): raise 2,000 euros\n\nWhy: Tom's insulin pump keeps failing\nSource: from-me");
    let r = run_checks(&e, &t, &bad, &done_trace(&[]));
    assert_eq!(get(&r, "P1"), Some(true));
    assert_eq!(get(&r, "P2"), Some(false), "the commit message holding the treatment is caught");
}

#[test]
fn headers_catch_a_question_not_word_for_word() {
    let e = env("headers");
    let t = trial("09-placement");
    let bad = build(&e, &t, "bad");
    let body = "---\ntype: task\nmodel: work@0.1\nquestion: What should we do next?\nserves: objects/goals/1.md\nsource: inferred\nlifecycle: active\nevidence: open\ndate: 2026-11-17\n---\n\nBook the bike service before Friday.\n";
    write(&bad.space.join("objects/tasks/bike.md"), body).unwrap();
    commit(&e, &bad.space, &format!("task(add): bike{TRAILERS}"));
    let r = run_checks(&e, &t, &bad, &done_trace(&[]));
    assert_eq!(get(&r, "W2"), Some(false), "the paraphrased question is caught");
    assert_eq!(get(&r, "W1"), Some(false), "one object for five intents is caught");

    let free = build(&e, &t, "free");
    let body = "---\ntype: task\nmodel: work@0.1\nquestion: What are we doing next?\nsource: inferred\nlifecycle: active\nevidence: open\ndate: 2026-11-17\n---\n\nBook the bike service before Friday.\n";
    write(&free.space.join("objects/tasks/bike.md"), body).unwrap();
    commit(&e, &free.space, &format!("task(add): bike{TRAILERS}"));
    let ctx = Ctx { c: &e.c, git: &e.git, cfg: &e.cfg };
    let (_, said) = (t.prompt)(&ctx, t.param);
    let trace = done_trace(&[]);
    let (checks, _) = (t.check)(&ctx, &RunData { setup: &free, trace: &trace, said: &said }, t.param);
    let w2 = checks.iter().find(|c| c.id == "W2").unwrap();
    assert_eq!(w2.ok, Some(true), "an object that serves nothing yet meets version 2 of W2");
    assert!(w2.detail.contains("serving nothing yet: objects/tasks/bike.md"), "and is named in the detail: {}", w2.detail);
}

#[test]
fn local_type_l4_checks_its_criterion_and_no_more() {
    let e = env("localtype");
    let t = trial("31-local-type");
    let ty = "---\ntype: type\nmodel: local\nquestion: Which bottles do I keep, and when do I open them?\nsource: inferred\nlifecycle: active\nevidence: open\ndate: 2026-11-17\n---\n\nA bottle I keep.\n";
    let obj = |q: &str| format!("---\ntype: bottle\nmodel: local\nquestion: {q}\nsource: from-me\nlifecycle: active\nevidence: open\ndate: 2026-11-17\n---\n\nA Barolo 2015, to open in 2030.\n");
    let good = build(&e, &t, "good");
    write(&good.space.join("objects/types/bottle.md"), ty).unwrap();
    write(&good.space.join("objects/bottles/barolo-2015.md"), &obj("Which bottles do I keep, and when do I open them?")).unwrap();
    commit(&e, &good.space, &format!("bottle(add): a Barolo{TRAILERS}"));
    let r = run_checks(&e, &t, &good, &done_trace(&[]));
    assert_eq!(get(&r, "L4"), Some(true), "an object without serves meets L4: its criterion does not ask for serves");
    let bad = build(&e, &t, "bad");
    write(&bad.space.join("objects/types/bottle.md"), ty).unwrap();
    write(&bad.space.join("objects/bottles/barolo-2015.md"), &obj("Which bottles do I keep?")).unwrap();
    commit(&e, &bad.space, &format!("bottle(add): a Barolo{TRAILERS}"));
    let r = run_checks(&e, &t, &bad, &done_trace(&[]));
    assert_eq!(get(&r, "L4"), Some(false), "a question that differs from its local type's is caught");
}

#[test]
fn answer_first_reads_the_order_of_the_stream() {
    let e = env("answerfirst");
    let t = trial("28-answer-first");
    let s = build(&e, &t, "x");
    let late = run_checks(&e, &t, &s, &done_trace(&[("objects/schedules/plan.md", true), ("Here is your plan.", false)]));
    assert_eq!(get(&late, "F1"), Some(false), "a write before the first words is caught");
    let early = run_checks(&e, &t, &s, &done_trace(&[("I will write a two-week plan as an object.", false), ("objects/schedules/plan.md", true)]));
    assert_eq!(get(&early, "F1"), Some(true));
}

#[test]
fn scale_catches_a_lookalike_edited() {
    let e = env("scale");
    let t = trial("27-scale-100");
    let bad = build(&e, &t, "bad");
    assert_eq!(walk(&bad.space, &bad.space.join("objects")).len(), 100, "the fixture holds the number of objects it says");
    let p = bad.space.join("objects/projects/cargo-shelf.md");
    write(&p, &format!("{}\nWe picked the long-tail frame.\n", read(&p).trim_end())).unwrap();
    commit(&e, &bad.space, &format!("project(revise): frame{TRAILERS}"));
    let r = run_checks(&e, &t, &bad, &done_trace(&[]));
    assert_eq!(get(&r, "S3"), Some(false), "the look-alike edited is caught");
    assert_eq!(get(&r, "S2"), Some(false), "the real project was not touched");
}

#[test]
fn two_spaces_catch_a_decision_taken_in_the_other_space() {
    let e = env("twospaces");
    let t = trial("05-two-spaces");
    let bad = build(&e, &t, "bad");
    let shop = PathBuf::from(bad.extra.get("shop").unwrap());
    write(&shop.join("objects/goals/2.md"), "x").unwrap();
    commit(&e, &shop, &format!("goal(add): decided for Lea{TRAILERS}"));
    let r = run_checks(&e, &t, &bad, &done_trace(&[]));
    assert_eq!(get(&r, "X1"), Some(false), "a commit on the workshop's main line is caught");
    assert_eq!(get(&r, "X2"), Some(false), "no proposal branch");
}
