---
trial: 31
status: run, 2026-11-17
level: end-to-end
tests: []
---

# Trial 31: a local type

Numbered 26 until 2026-11-17, when it was renumbered: the branch `delegate-by-default` had taken trial 26 first, on 2026-11-16. Its runs moved with it, from `tests/runs/26-local-type/`.

## Questions

1. When the person asks to keep something no installed type asks about, does the agent create a local type, named by its question, without the person naming it?
2. Does that type stay apart from the installed ones?

## Setting

A home (`home@0.1`) for Sam Rivera, a fictional person, handle `sam-rivera`: `space.md`, an `AGENTS.md` generated from the norm texts of [roles](../../convention/roles.md) and the [doctrine](../../convention/doctrine.md) as they stand at the run's commit, a problem ("I get out of breath on the stairs, and I sit all day."), and a first goal, each committed with its trailers and a fixed date, so the fixture always has the same commit ids. The goal is "Run 10 km by June." The person says:

> I want to keep track of the wines in my cellar: for each bottle, the rack and shelf it sits on, and the year it should be drunk by. Here are the first three. A 2019 Rioja on rack A, shelf 2, to drink by 2027. A 2021 Muscadet on rack B, shelf 1, to drink by 2026. A 2015 Barolo on rack A, shelf 4, to drink by 2030.

No type of `home@0.1` asks about the things one keeps and where they are.

## Fixed before the trial

Fixed on 2026-11-17, before the first run, and not changed after.

| # | Criterion | How it is judged |
|---|---|---|
| L1 | a local type in `objects/types/`, with its question | computed |
| L2 | its question is not, word for word, the question of an installed type | computed, against `models/` |
| L3 | no installed type asks the same thing | a decision model, closed question, given the installed questions |
| L4 | objects of that type, `model: local`, whose `question` is the local type's, word for word | computed |
| L5 | the local type is marked `source: inferred`: the person did not name it | computed |
| L6 | kept: every new commit ends with `Why:` and `Source:` that git reads as trailers; the fixture's last commit is still in history; nothing is left uncommitted under `objects/` | computed, from git |

## Expected answer

Each criterion holds.

## Measures

From [measures](../../convention/views.md): emergence (local types); and the `local-double` check of the [structure checks](../structure.md).

## What could break

Restraint pulls the other way: an agent may file the bottles as `material` or `document`, which asks another question. The decision model, given the installed questions, decides whether one already asks the same thing.

Implemented in [`tests/runner`](../runner/src/trials.rs) as `31-local-type`. A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/26-local-type/`.
