---
trial: 9
status: canonical run, 2026-11-17; the other questions not run
level: end-to-end
tests: [https://github.com/thevzion/source/blob/main/objects/hypotheses/anyone-can-start-with-seven-types.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/your-intent-is-the-source.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/you-say-it-the-source-writes-itself.md]
---

# Trial 9: placement

This trial tests the [doctrine](../../convention/doctrine.md): discern before you write.

## Questions

1. Placement: in a new space with the Organization model, where do raw intents go?
2. Restraint: does any local type double an installed type?
3. Consistency: do two agents, or two sessions, file the same intents in the same types?
4. Migration: when a model that covers a local type is installed, what does the agent do?

## Setting

A new test space with the Organization model ([organization](../../models/organization/model.md)), ten raw intents written in advance, and two agents or two sessions.

## Fixed before the trial

| Criterion | What must hold |
|---|---|
| placement | of ten raw intents, at least eight go into types of the model; the others go into local types, each with its question, marked `local` and `inferred` |
| restraint | no local type doubles an installed type |
| consistency | two agents, or two sessions, file the same intents in the same types at least eight times out of ten |
| migration | when a model that covers a local type is installed, the agent proposes the migration, applies nothing before the person agrees, and the links still resolve |

## Expected answer

Each criterion above holds.

## Measures

From [measures](../../convention/views.md): placement; and the `local-double` check of the [structure checks](../structure.md).

## What could break

Placement: everything rests on well-typed objects. If two agents file the same intent differently, the measures measure noise.

## Canonical run

Fixed on 2026-11-17, before its first counted run (the five-word rule of W3 and W4 was widened after a smoke run, as the criterion says), and implemented in [`tests/runner`](../runner/src/trials.rs) as `09-placement`. It asks one question of this trial, the same way every time; the trial's other questions stay as above.

**The question.** Does what I say become objects, in my words?

**Setting.** A home (`home@0.1`) for Sam Rivera, a fictional person, handle `sam-rivera`: `space.md`, an `AGENTS.md` generated from the norm texts of [roles](../../convention/roles.md) and the [doctrine](../../convention/doctrine.md) as they stand at the run's commit, a problem ("I get out of breath on the stairs, and I sit all day."), and a first goal, each committed with its trailers and a fixed date, so the fixture always has the same commit ids. The goal is "Run 10 km by June."

**What the person says.**

> Some things on my mind. I want to cook at home five nights a week, because takeaway eats my budget. This week I need to book the bike service before Friday. We decided with Lea to spend August in Lisbon rather than Porto, because the flights are cheaper. I would like to learn enough Portuguese to order food by then. And I wonder whether I should get a standing desk.

| # | Criterion | How it is judged |
|---|---|---|
| W1 | at least four new objects for the five intents | computed |
| W2 | every new or changed object has the header the convention asks: its fields, `question` word for word from its model (or its local type), `source`, `lifecycle` and `evidence` in their values, `serves` unless a root | computed, against `models/` |
| W3 | every new object marked `from-me` holds the person's words: at least five words in a row from what they said, or the whole object word for word when it is shorter than five words (the second part added before any counted run: a smoke run showed a four-word object, word for word, failed by the first wording) | computed |
| W4 | at least three new objects keep the person's words (five words in a row, or the whole of a shorter object) | computed |
| W5 | kept: every new commit ends with `Why:` and `Source:` that git reads as trailers; the fixture's last commit is still in history; nothing is left uncommitted under `objects/` | computed, from git |

**Version 2 of W2, from batch 3.** Fixed on 2026-11-17, before batch 3, because pass 2 changed the convention: an object may serve nothing in the space yet. In W2, "`serves` unless a root" reads "`serves` unless the object serves nothing yet, or is a root". Nothing else changes, and version 1 stays the criterion of the earlier runs. Each run of batch 3 names in W2's detail the objects that serve nothing yet, so version 1 can still be read from it.

A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/09-placement/`. This canonical run is smaller than the criteria above (ten intents, two agents); those stay fixed for the full trial.
