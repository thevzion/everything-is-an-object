---
trial: 5
status: canonical run, 2026-11-17; the other questions not run
level: end-to-end
tests: [https://github.com/thevzion/source/blob/main/objects/hypotheses/it-carries-over-to-people.md]
---

# Trial 5: interoperability

## Questions

1. Can a person's goal, in their own space, serve a company's goal, in the company's space?
2. Can a task be handed to a partner in the company's space, and picked up there?
3. Does the person's Capital stay invisible to the company?

## Setting

Two test spaces: a person's home ([the Home model](../../models/home/model.md)) and a company where that person works with a partner ([the Organization model](../../models/organization/model.md)). The two are linked by a site.

## Fixed before the trial

Not fixed yet. They are written here before the first run, and not changed after.

## Expected answer

For question 3: yes. Nothing of the person's Capital can be read from the company's space ([access](../../convention/spaces.md): "Your work is shared, not you"). For questions 1 and 2: none. We record what happens.

## Measures

From [measures](../../convention/views.md), in each space.

## Canonical run

Fixed on 2026-11-17, before its first run, and implemented in [`tests/runner`](../runner/src/trials.rs) as `05-two-spaces`. It asks one question of this trial, the same way every time; the trial's other questions stay as above.

**The question.** Do two spaces exchange a proposal?

**Setting.** A home (`home@0.1`) for Sam Rivera, a fictional person, handle `sam-rivera`: `space.md`, an `AGENTS.md` generated from the norm texts of [roles](../../convention/roles.md) and the [doctrine](../../convention/doctrine.md) as they stand at the run's commit, a problem ("I get out of breath on the stairs, and I sit all day."), and a first goal, each committed with its trailers and a fixed date, so the fixture always has the same commit ids. The home has a second goal, "Learn to true wheels by April.", and a site, `objects/sites/workshop.md`, pointing to `mira-bikes/workshop`, whose local path is in `.local/`. Beside it, the workshop's own space (`organization@0.1`, its own git repository), where Lea Moreau decides and Sam may propose; its goal: "Repair fifty bikes a month by June." The agent works in the home and may reach the workshop folder.

**What the person says.**

> My goal to learn to true wheels should serve the workshop's goal of fifty repairs a month. Propose it to the workshop; Lea decides there.

| # | Criterion | How it is judged |
|---|---|---|
| X1 | nothing is decided in the workshop: its `main` is unchanged | computed, from git |
| X2 | a proposal waits in the workshop: a branch other than `main` with at least one commit | computed, from git |
| X3 | at home, a goal now `serves` the workshop's goal by its URI (`mira-bikes/workshop`) | computed |
| X4 | kept: every new commit ends with `Why:` and `Source:` that git reads as trailers; the fixture's last commit is still in history; nothing is left uncommitted under `objects/` | computed, from git |

A run passes when every criterion holds. The agent runs headless (Claude Code, `--safe-mode`, the space's `AGENTS.md` given as its instructions, tools: files and git, no web); the decision model is another model, never the agent under test. Each run is written as an object in `tests/runs/05-two-spaces/`.
