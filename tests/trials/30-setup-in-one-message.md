---
trial: 30
status: run, 2026-11-17
level: end-to-end
tests: []
---

# Trial 30: the setup in one message

## Questions

1. Given the setup prompt and the person's answers in one message, does an agent set up a space that follows the convention?
2. Does it write only what the person said, and mark the rest?

## Setting

An empty folder, and git with a fictional identity. The agent receives, as its prompt, everything below the line of [`setup/prompt.md`](../../setup/prompt.md) at the run's commit, then the person's answers to its five questions, at once:

> My answers, all at once. 1. Just me. 2. Call it home, under the handle sam-rivera. 3. This folder, no Notion. Yes, keep its history with git. No, I have no other space. 4. No, do not look at my files. 5. I want to run 10 km by June, because I get out of breath on the stairs. I will know when I finish the Lyon 10K.

This is the neighbouring case of the pass that wrote version 3.0 of the prompt: the ten runs of 15 Nov 2026 set up a home with version 2.2 of the prompt; this trial checks that version 3.0 still does.

## Fixed before the trial

Fixed on 2026-11-17, before the first run, and not changed after.

| # | Criterion | How it is judged |
|---|---|---|
| S1 | `space.md` says the URI `sam-rivera/home`, `convention:` with this version, and the model `home@0.1` | computed |
| S2 | `AGENTS.md` holds the nine sections in order, and the norm texts of [roles](../../convention/roles.md) word for word | computed, word by word |
| S3 | a problem and a goal, and every object's header as the convention asks | computed, against `models/` |
| S4 | no `why` and no `vision`: the person said neither | computed |
| S5 | the first commit is `space(add)`, every commit has its trailers, nothing is left uncommitted | computed, from git |

**Version 2 of S3, from batch 3.** Fixed on 2026-11-17, before batch 3, because pass 2 changed the convention: an object may serve nothing in the space yet. In S3, "as the convention asks" now includes that an object may serve nothing yet. Nothing else changes, and version 1 stays the criterion of the earlier runs. Each run of batch 3 names in S3's detail the objects that serve nothing yet, so version 1 can still be read from it.

## Expected answer

Each criterion holds.

## Measures

Whether the `commit-msg` hook was installed, and the tool's own instruction file (`CLAUDE.md`, `@AGENTS.md`).

## What could break

In one headless message the agent cannot ask its questions first; the answers come with the prompt, as a person who read the questions would give them.

Implemented in [`tests/runner`](../runner/src/trials.rs) as `30-setup`. A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/30-setup/`.
