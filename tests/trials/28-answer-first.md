---
trial: 28
status: run, 2026-11-17
level: end-to-end
tests: []
---

# Trial 28: answer first

## Questions

1. When the person asks for work, does the one who answers reply at once, saying what it understood and what is pending, before anything is written?
2. Does the work then become an object, which the last answer names?

## Setting

A home (`home@0.1`) for Sam Rivera, a fictional person, handle `sam-rivera`: `space.md`, an `AGENTS.md` generated from the norm texts of [roles](../../convention/roles.md) and the [doctrine](../../convention/doctrine.md) as they stand at the run's commit, a problem ("I get out of breath on the stairs, and I sit all day."), and a first goal, each committed with its trailers and a fixed date, so the fixture always has the same commit ids. The goal is "Finish the Lyon 10K in under an hour, by June.", served by a project. The person says:

> Make me a plan for the next two weeks to get ready for the Lyon 10K.

One agent holds every role here: the assistant, the materializer and the checker ([roles](../../convention/roles.md#how-the-assistant-works-proposed)).

## Fixed before the trial

Fixed on 2026-11-17, before the first run, and not changed after.

| # | Criterion | How it is judged |
|---|---|---|
| F1 | in the agent's stream, its first words come before its first write (a file written, a commit, a shell command that writes) | computed, from the stream |
| F2 | those first words answer the person: what it understood, what it will do or what is pending, not only a tool step such as "Let me read the file" | a decision model, closed question |
| F3 | the plan is an object: at least one new or changed object | computed |
| F4 | the last answer names a path of what was written | computed, on the reply |
| F5 | every new or changed object has the header the convention asks: its fields, `question` word for word from its model (or its local type), `source`, `lifecycle` and `evidence` in their values, `serves` unless a root | computed, against `models/` |
| F6 | kept: every new commit ends with `Why:` and `Source:` that git reads as trailers; the fixture's last commit is still in history; nothing is left uncommitted under `objects/` | computed, from git |

**Version 2 of F5, from batch 3.** Fixed on 2026-11-17, before batch 3, because pass 2 changed the convention: an object may serve nothing in the space yet. In F5, "`serves` unless a root" reads "`serves` unless the object serves nothing yet, or is a root". Nothing else changes, and version 1 stays the criterion of the earlier runs. Each run of batch 3 names in F5's detail the objects that serve nothing yet, so version 1 can still be read from it.

## Expected answer

Each criterion holds.

## Measures

Time to the first words is not measured: the stream gives the order of events, not their times.

## What could break

In a terminal the person sees the first words as they come; headless, they are in the stream, and the runner reads their order. An agent that delegates the writing to a sub-agent is not tested here: no sub-agent tool is given.

Implemented in [`tests/runner`](../runner/src/trials.rs) as `28-answer-first`. A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/28-answer-first/`.
