---
trial: 2
status: canonical run, 2026-11-17; the other questions not run
level: end-to-end
tests: [https://github.com/thevzion/source/blob/main/objects/hypotheses/it-compounds.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/your-intent-is-the-source.md]
---

# Trial 2: two turns, and reuse

## Questions

1. Does the complete loop run in two turns? Intents become objects; the person reads a projection and says what they want; that becomes objects; the projections that relied on them are flagged stale.
2. On the second turn, what does the agent reuse from the first (objects, types, links), and what does it create again?

## Setting

A test space with the Organization model ([organization](../../models/organization/model.md)), and intents written in advance for two turns.

## Fixed before the trial

Not fixed yet. They are written here before the first run, and not changed after.

## Expected answer

None. We record what happens.

## Measures

From [measures](../../convention/views.md): reuse, fit, anchoring, corrections.

## Canonical run

Fixed on 2026-11-17, before its first run, and implemented in [`tests/runner`](../runner/src/trials.rs) as `02-reuse`. It asks one question of this trial, the same way every time; the trial's other questions stay as above.

**The question.** Does a repeated signal enrich the existing object instead of adding one?

**Setting.** A home (`home@0.1`) for Sam Rivera, a fictional person, handle `sam-rivera`: `space.md`, an `AGENTS.md` generated from the norm texts of [roles](../../convention/roles.md) and the [doctrine](../../convention/doctrine.md) as they stand at the run's commit, a problem ("I get out of breath on the stairs, and I sit all day."), and a first goal, each committed with its trailers and a fixed date, so the fixture always has the same commit ids. The goal is "Run 10 km by June.", with no way to know yet.

**What the person says.**

> About my running again: I want to run 10 km by June, and I will know I made it when I finish the Lyon 10K in under an hour.

| # | Criterion | How it is judged |
|---|---|---|
| R1 | no second goal: no new file in `objects/goals/`, no new object of type goal | computed |
| R2 | `objects/goals/1.md` changed, and now says the Lyon 10K and the hour | computed |
| R3 | every new or changed object has the header the convention asks: its fields, `question` word for word from its model (or its local type), `source`, `lifecycle` and `evidence` in their values, `serves` unless a root | computed, against `models/` |
| R4 | kept: every new commit ends with `Why:` and `Source:` that git reads as trailers; the fixture's last commit is still in history; nothing is left uncommitted under `objects/` | computed, from git |

**Version 2 of R3, from batch 3.** Fixed on 2026-11-17, before batch 3, because pass 2 changed the convention: an object may serve nothing in the space yet. In R3, "`serves` unless a root" reads "`serves` unless the object serves nothing yet, or is a root". Nothing else changes, and version 1 stays the criterion of the earlier runs. Each run of batch 3 names in R3's detail the objects that serve nothing yet, so version 1 can still be read from it.

A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/02-reuse/`.
