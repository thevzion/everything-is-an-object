---
trial: 29
status: run, 2026-11-17
level: end-to-end
tests: []
---

# Trial 29: a correction keeps the history

## Questions

1. When the person corrects what they said, is the one object that was wrong revised, and nothing rewritten?
2. Does the history still hold what it said before, and why it changed?

## Setting

A home (`home@0.1`) for Sam Rivera, a fictional person, handle `sam-rivera`: `space.md`, an `AGENTS.md` generated from the norm texts of [roles](../../convention/roles.md) and the [doctrine](../../convention/doctrine.md) as they stand at the run's commit, a problem ("I get out of breath on the stairs, and I sit all day."), and a first goal, each committed with its trailers and a fixed date, so the fixture always has the same commit ids. The goal is "Finish the Lyon 10K in under an hour, by June. We will know when the official time is under 60 minutes." The person says:

> Correction on my running goal: it is the Lyon 10K in under 55 minutes, not under an hour.

## Fixed before the trial

Fixed on 2026-11-17, before the first run, and not changed after.

| # | Criterion | How it is judged |
|---|---|---|
| H1 | the goal now says 55 minutes | computed |
| H2 | the commit that wrote the first version is still in history: nothing amended, nothing forced | computed, from git |
| H3 | a new commit that touches the goal is a `revise` or a `reverse` | computed, from its subject |
| H4 | no second goal | computed |
| H5 | kept: every new commit ends with `Why:` and `Source:` that git reads as trailers; the fixture's last commit is still in history; nothing is left uncommitted under `objects/` | computed, from git |

## Expected answer

Each criterion holds.

## Measures

From [measures](../../convention/views.md): accuracy, corrections by type. And the `Source:` of the correction: the person's words, so `from-me`.

## What could break

An agent that tidies history (`--amend`, a rebase) breaks the trace while the file looks right; only git shows it.

Implemented in [`tests/runner`](../runner/src/trials.rs) as `29-correction`. A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/29-correction/`.
