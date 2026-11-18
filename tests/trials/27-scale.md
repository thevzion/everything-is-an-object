---
trial: 27
status: run, 2026-11-17
level: end-to-end
tests: []
---

# Trial 27: scale

## Questions

1. With 100, 1,000 and 10,000 objects in the space, does the agent still find the existing object a signal is about, instead of creating another?
2. Does it leave the look-alikes alone?
3. What does it cost, in tokens and time, as the space grows?

## Setting

A home of Sam Rivera with N objects (100, 1,000, 10,000), generated the same way every time: five goals, the project `objects/projects/cargo-bike.md` ("Build a cargo bike for the school run."), three look-alike projects (a cargo shelf, a bike rack, the kids' bikes), and the rest tasks, decisions, materials, projects and journal entries made of fixed words. All committed at once. The person says:

> Update on the cargo bike for the school run: we picked the long-tail frame, and the build starts in March.

## Fixed before the trial

Fixed on 2026-11-17, before the first run, and not changed after.

| # | Criterion | How it is judged |
|---|---|---|
| S1 | no second project for the cargo bike: no new file in `objects/projects/` | computed |
| S2 | the existing `objects/projects/cargo-bike.md` is revised, or a new object (a decision, a task) `serves` or `draws-on` it | computed |
| S3 | the look-alike projects are untouched | computed |
| S4 | every new or changed object has the header the convention asks: its fields, `question` word for word from its model (or its local type), `source`, `lifecycle` and `evidence` in their values, `serves` unless a root | computed, against `models/` |
| S5 | kept: every new commit ends with `Why:` and `Source:` that git reads as trailers; the fixture's last commit is still in history; nothing is left uncommitted under `objects/` | computed, from git |

**Version 2 of S4, from batch 3.** Fixed on 2026-11-17, before batch 3, because pass 2 changed the convention: an object may serve nothing in the space yet. In S4, "`serves` unless a root" reads "`serves` unless the object serves nothing yet, or is a root". Nothing else changes, and version 1 stays the criterion of the earlier runs. Each run of batch 3 names in S4's detail the objects that serve nothing yet, so version 1 can still be read from it.

## Expected answer

Each criterion holds at each size; the cost is recorded, not judged.

## Measures

Tokens (input, cache, output), cost, turns and time per run, by size; the number of reads and searches.

## What could break

At 10,000 objects an agent that lists or reads everything spends far more, or runs out of context; one that searches by words finds the project at once. A fixture made of repeated words is easier to search than a real life.

Implemented in [`tests/runner`](../runner/src/trials.rs) as `27-scale-100`, `27-scale-1000` and `27-scale-10000`. A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/27-scale-<N>/`.
