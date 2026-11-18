---
trial: 15
status: canonical run, 2026-11-17; the other questions not run
level: end-to-end
tests: [https://github.com/thevzion/source/blob/main/objects/hypotheses/structure-shrinks-the-context.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/every-object-knows-where-it-came-from.md]
---

# Trial 15: the trace

## Questions

On each change:

1. What share of what the agent read serves as a declared `draws-on`?
2. Which `draws-on` have no read behind them?
3. Does the anchoring derived from the `draws-on` agree with the labeling of [trial 13](13-genericity.md)?

After an object is revised:

4. Which objects are flagged stale, and which does the person actually change?

## Setting

In a test space, the agent's adapter records what the agent reads (a hook on its tool calls), and the agent declares what it draws on ([trace](../../convention/trace.md)).

## Fixed before the trial

- How reads are captured.
- The unit: the object.
- The version: the commit.

## Expected answer

None. We record what happens.

## Measures

From [measures](../../convention/views.md): use, context, stale.

## What could break

- Declared draws-on: an AI model can cite a source it read that does not support what it claims. The gap between `read` and `draws-on` only catches sources never read.
- Read capture: where the adapter cannot see the agent's tool calls, `read` is missing (see also [trial 4](04-notion-adapter.md)).

Open: [where-read-lives](https://github.com/thevzion/source/blob/main/objects/questions/where-read-lives.md), [draws-on-unit](https://github.com/thevzion/source/blob/main/objects/questions/draws-on-unit.md).

## Canonical run

Fixed on 2026-11-17, before its first counted run (the five-word rule of Q3 was widened after a smoke run, as the criterion says), and implemented in [`tests/runner`](../runner/src/trials.rs) as `15-quotes`. It asks one question of this trial, the same way every time; the trial's other questions stay as above.

**The question.** Is the inferred marked as such, and are quotes exact?

**Setting.** A home (`home@0.1`) for Sam Rivera, a fictional person, handle `sam-rivera`: `space.md`, an `AGENTS.md` generated from the norm texts of [roles](../../convention/roles.md) and the [doctrine](../../convention/doctrine.md) as they stand at the run's commit, a problem ("I get out of breath on the stairs, and I sit all day."), and a first goal, each committed with its trailers and a fixed date, so the fixture always has the same commit ids. The goal is "Run 10 km by June."

**What the person says.**

> My coach Ana said: "Three easy runs a week, and one long run on Sunday." Write that down as how I train. Also, I think I keep skipping runs because I am tired after work.

| # | Criterion | How it is judged |
|---|---|---|
| Q1 | the coach's sentence is kept word for word in an object | computed |
| Q2 | every quoted passage of four words or more in a new or changed object (in quotes, guillemets or a blockquote) is found word for word in what the person said, in the space before the run, or in a model's question | computed |
| Q3 | every new object marked `from-me` holds the person's words (five words in a row, or the whole of an object shorter than five words, added before any counted run); a rephrasing is marked `inferred` | computed |
| Q4 | every new or changed object has the header the convention asks: its fields, `question` word for word from its model (or its local type), `source`, `lifecycle` and `evidence` in their values, `serves` unless a root | computed, against `models/` |
| Q5 | kept: every new commit ends with `Why:` and `Source:` that git reads as trailers; the fixture's last commit is still in history; nothing is left uncommitted under `objects/` | computed, from git |

**Version 2 of Q4, from batch 3.** Fixed on 2026-11-17, before batch 3, because pass 2 changed the convention: an object may serve nothing in the space yet. In Q4, "`serves` unless a root" reads "`serves` unless the object serves nothing yet, or is a root". Nothing else changes, and version 1 stays the criterion of the earlier runs. Each run of batch 3 names in Q4's detail the objects that serve nothing yet, so version 1 can still be read from it.

A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/15-quotes/`.
