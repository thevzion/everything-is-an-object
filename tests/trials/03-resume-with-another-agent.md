---
trial: 3
status: canonical run, 2026-11-17; the other questions not run
level: end-to-end
tests: [https://github.com/thevzion/source/blob/main/objects/hypotheses/the-prompt-works-with-any-agent.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/it-carries-over-to-people.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/switch-models-keep-everything.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/say-it-once-every-agent-knows-it.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/you-stop-re-explaining.md]
---

# Trial 3: resume with another agent

## Questions

1. Can another agent, including one from another provider, pick up a space with nothing but the space, and no briefing?
2. With the space alone, does it find the problem, the first goal, the next task, and where to write a decision?
3. What does the person have to explain again?
4. Does a person who did not write the space find the same four things?

## Setting

- A space written by a first agent that followed [the setup prompt](../../setup/prompt.md).
- A second agent, from another provider, opens it with no briefing.
- Each agent runs clean, or the run notes what else it loaded.

## Fixed before the trial

Not fixed yet. They are written here before the first run, and not changed after.

## Expected answer

None. We record what happens.

## Measures

From [measures](../../convention/views.md): context, reuse.

## Before this trial

Version 1 of the setup prompt was tried once, on 2026-11-11, with a fictional person: an agent from a second provider resumed a space written by the first and found the four things of question 2. That run used the version 1 layout ([from version 1 to version 2](https://github.com/thevzion/convention/blob/fd9a5ff/setup/prompt.md#from-version-1-to-version-2)), so it does not count for this trial.

## Canonical run

Fixed on 2026-11-17, before its first run, and implemented in [`tests/runner`](../runner/src/trials.rs) as `03-resume`. It asks one question of this trial, the same way every time; the trial's other questions stay as above.

**The question.** Does another session resume from the source alone?

**Setting.** A home (`home@0.1`) for Sam Rivera, a fictional person, handle `sam-rivera`: `space.md`, an `AGENTS.md` generated from the norm texts of [roles](../../convention/roles.md) and the [doctrine](../../convention/doctrine.md) as they stand at the run's commit, a problem ("I get out of breath on the stairs, and I sit all day."), and a first goal, each committed with its trailers and a fixed date, so the fixture always has the same commit ids. The goal is "Finish the Lyon 10K in under an hour, by June."; a project; a task done (register), a task still active (buy running shoes); a decision in `objects/decisions/`. The agent is a new session, with no briefing but the space.

**What the person says.**

> I am back after a break and I do not remember where I was. From this space alone: what is the problem, what is my first goal, what is my next task, and where would you write a decision I just made? Name the files.

| # | Criterion | How it is judged |
|---|---|---|
| M1 | the reply names `objects/problem.md` | computed, on the reply |
| M2 | the reply names `objects/goals/1.md` | computed, on the reply |
| M3 | the reply names the task still active, `buy-trail-shoes` | computed, on the reply |
| M4 | the reply says decisions go in `objects/decisions/` | computed, on the reply |

A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/03-resume/`. This canonical run uses one provider; the trial's question of another provider stays open.
