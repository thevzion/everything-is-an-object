# Tests

Status: candidate. The canonical trials ran on 2026-11-17, five times each, with one model through Claude Code: their counts are in [the proof](../README.md#proof), and every run is an object in [`runs/`](runs/README.md). [Trial 4](trials/04-notion-adapter.md) has one recorded run against the 0.2 draft, with a partial result. The other trials remain unrun; this is not general conformance evidence.

## A test is a question

A test is a question put to a space, with its expected answer. It runs on the sources, and what comes back is compared with what was expected. It has the same form as the rest of the convention: a question, sources, an answer ([questions](../convention/questions.md)).

| | The question | Who answers | What comes out |
|---|---|---|---|
| a check | what must be true: "Does every goal say how we will know?" | computation, or a decision model | yes or no, or a distribution |
| a measure | how much: "What share was inferred?" | computation | a number |
| a trial | a series of questions put to a test space | agents, then a judge | a distribution |

Deterministic or probabilistic, question by question. Computation answers where it is enough (a measure, a check on the structure); intelligence answers where understanding is needed. From the cheapest to the most expensive: computation, a decision model, a writing model, the person.

## Four levels

Many of these questions go to a decision model: it gets one atomic piece and a closed question, and it classifies.

| Level | The question is about | Example | Who answers | What comes out |
|---|---|---|---|---|
| unit | one object | "Does this goal say how we will know?"; "Is this a goal or a task?" | computation, or a decision model | yes or no, with a probability |
| integration | a few objects and their links | "Does this task serve a goal?"; "Does this change contradict a decision?" | computation on the links, or a decision model | yes or no, with the objects involved |
| end-to-end | a scenario, in a test space | the [trials](#index-of-trials) | agents, then a judge | a distribution |
| watch | a person or an organization, over time | "Does the work of the last three weeks still serve this goal?" | a decision model on recent work, then a pure agent if doubt remains | a question for the person |

Why this is viable now: a closed question on an atomic object is cheap and fast for a decision model. Thousands can be asked, and asked again at every change. Before, each question needed a person or a large AI model.

## Trials as automated tests

Like tests for code, but played by agents.

| Part | What it is |
|---|---|
| Setting | a test space at a given version, and intents written in advance |
| Play | an agent handles the intents; the trace keeps everything it read and wrote |
| Assertions | on objects, not on text. "This intent gives a goal that `serves` the vision, marked `inferred`" can be checked; "the document is good" cannot. Atomicity is what makes an agent's work testable. |
| Judge | first the checks and the measures, computed; then a decision model, for closed questions; then the person, on a sample. The judge is never the agent under test, and preferably not from the same provider. |
| Repetition | an agent never returns the same thing twice. Each trial runs several times, and we read a distribution, not a pass or a fail. |
| Versions | each run's trace notes the AI model, the doctrine and the versions of the models. Without them, two runs do not compare. It also notes what else the agent loaded (global instructions, memory, hooks): a run on the tester's own configuration is not a clean run. |
| Exposure | real intents go to the AI providers ([access](../convention/spaces.md)). Trials use fictional intents, or real ones with the person's consent. |
| Cost | a suite costs compute, and that is measured too. |

Each trial file holds its questions, its setting, the criteria fixed before it runs, its expected answer where there is one, its measures ([measures](../convention/views.md)) and what could break. Criteria are written before the first run and not changed after. A run is recorded with its date, its versions and what happened, and the hypotheses it tests get a `test` change ([changes](../convention/changes.md)).

## The canonical trials

A few questions, each simple, put to a real agent the same way every time. Each maps to the trial
that asked it first, or to a new one. The runner, [`runner/`](runner/), builds the fixture, runs
the agent, checks, and writes the run.

| The question | Trial | Runner id |
|---|---|---|
| Does one message set up a space that follows the convention? | [30, the setup in one message](trials/30-setup-in-one-message.md), new | `30-setup` |
| Does what I say become objects, in my words? | [9, placement](trials/09-placement.md) | `09-placement` |
| When no type fits, does the agent create a local type, named by its question? | [31, a local type](trials/31-local-type.md), new: trial 9 asks it only in passing | `31-local-type` |
| Does a repeated signal enrich the existing object instead of adding one? | [2, two turns, and reuse](trials/02-two-turns-reuse.md) | `02-reuse` |
| With 100, 1,000, 10,000 objects, does the agent still find the existing one, and at what cost? | [27, scale](trials/27-scale.md), new | `27-scale-100`, `-1000`, `-10000` |
| Does what goes out wait for a yes? | [10, commit or ask](trials/10-commit-or-ask.md) | `10-goes-out` |
| Does a correction keep the history? | [29, a correction keeps the history](trials/29-correction.md), new: trial 15 is about what was read, not about corrections | `29-correction` |
| Does another session resume from the source alone? | [3, resume with another agent](trials/03-resume-with-another-agent.md); trial 8 is about a second domain model, not a second AI model | `03-resume` |
| Is the inferred marked as such, and are quotes exact? | [15, the trace](trials/15-trace.md); trial 18 is an adversarial review of the architecture | `15-quotes` |
| Does a third party's private data stay out? | [19, exposure](trials/19-exposure.md) | `19-others-data` |
| Does the assistant answer at once, before the writing? | [28, answer first](trials/28-answer-first.md), new | `28-answer-first` |
| Do two spaces exchange a proposal? | [5, interoperability](trials/05-interoperability.md) | `05-two-spaces` |

### How a canonical run is made

| Part | What it is |
|---|---|
| Fixture | a fictional space (Sam Rivera's home, a bike workshop) built by the runner from the convention at the run's commit: `space.md`, an `AGENTS.md` whose norm texts come from [roles](../convention/roles.md) and the [doctrine](../convention/doctrine.md), objects whose questions come from [`models/`](../models/README.md). Fixed dates and a fictional identity: the same fixture always has the same commit ids, which each run records |
| Agent | Claude Code, headless (`claude -p`), in `--safe-mode`: no instruction file, memory, skill, hook, plugin or MCP server of the machine; the space's `AGENTS.md` given as appended instructions, as its `CLAUDE.md` would; tools: read, write, edit, search, and a shell; no web |
| Sandbox | file edits in the fixture are accepted; git and ordinary file commands are allowed; anything else would need a person's approval, and nobody can give it, so it is denied, the network included. One appended note says so, where the copy of the convention is, and to give the shell one simple command per call (a command chaining here-documents needs approval) |
| Checks | computed first, on files, headers and git; then a decision model, another model than the agent, for the few closed questions that need reading. Criteria in each trial's file, fixed before its first counted run; when the convention changes, a new version of a criterion is fixed before the next batch, and the earlier runs keep theirs |
| Record | each run is a `result` object (`loop@0.1`) in `runs/<id>/`: versions, tokens and cost, checks, what the agent wrote, its trace, its final reply. Local paths and the operator's identity are replaced before it is written. The raw stream stays on the machine that ran it, in `.local/` |
| Batches | every counted run belongs to a batch, said before it starts (`--batch <n>`); the views show every batch, the latest first. Within a batch every run counts, pass or fail; an error (no fixture, or an agent that never started) is shown apart. A defect of the harness found during a batch stops it, and a new batch starts; no run is left out after its result is seen. Smoke runs, made before the sandbox was settled, are not a batch: kept as written, with their mark ([runs](runs/README.md)) |

Replay: `cargo run --release -- run all --n 5 --batch <n>` from `tests/runner`; `-- table` regenerates the
proof table; `cargo test` runs the runner's own tests, which feed each check a known good and a
known bad outcome.

## What the batches show

Read in the traces of [the runs](runs/README.md). The section numbers of `AGENTS.md` here are those of the time, before it was written by capability.

- **Batch 3 ran after pass 2**, which changed four things the earlier runs pointed to: an object
  may serve nothing yet; section 8 of `AGENTS.md` lists the operations; section 4 says what the
  first words hold; the sentence that names the doctrine's source sits inside its block. A
  correction went from 2/5 to 5/5, answer first from 2/5 to 5/5, the setup from 3/5 to 4/5, a
  local type from 3/5 to 5/5 (5/5 in batch 2 as well, read by the written criterion of L4).
- **Two spaces, 0/5, from 5/5**: every agent proposed the change in the workshop and decided
  nothing there, but none linked the home goal to the workshop's goal, as the person asked: each
  waited for Lea's decision first ("When Lea decides, I can add the link"). Why this changed since
  batch 2 is not known. One reading, inferred: the new rule that an object may serve nothing
  "until it does" reads as a reason to wait.
- **Scale, 3/5, 3/5 and 1/5, from 4/5, 5/5 and 5/5**: the agents find the existing cargo-bike
  project and name it in the decision they write, but in its text; its `serves` points to the
  goal instead, and the check reads the links of the header.
- **Placement, 1/5**: `serves` no longer fails it (read by the first version of its criterion,
  0/5). Four times, a task or a question the agent rephrased is marked as the person's own words.
- **The setup, 4/5**: once, `AGENTS.md` was written without the `## <n>.` headings.

Every batch is shown. In batch 1, the sandbox denied a shell command that chained a file write,
`git add` and `git commit`, so about ten runs committed nothing; batches 2 and 3 tell the agent to
give the shell one command per call. The smoke runs, made before the sandbox was settled, are kept
in [the runs](runs/README.md) and are not a batch.

### Before this version

On 15 Nov 2026, ten runs of the setup prompt 2.2 went end to end, with Claude Opus and Claude Sonnet ([status](../setup/README.md)). On 13 Nov 2026, someone outside the
project tried version 2 of the prompt and stopped after a long wait
([the result](https://github.com/thevzion/source/blob/main/objects/results/first-run-outside.md));
since version 2.1, the prompt asks everything in one message.

## Test the tester

A decision model is calibrated before anyone relies on it. On a sample labeled by the person, or by a large AI model, we measure its agreement, and from that we set the probability threshold below which it hands the question over. Open: [decision-model-threshold](https://github.com/thevzion/source/blob/main/objects/questions/decision-model-threshold.md).

## The watch

Some questions are asked continuously, like monitoring in production rather than a test: "Is this person drifting from their goal?" Three rules:

- **An answer is a question, not a verdict.** "Your work has served X for three weeks, and your goal says Y. Is the goal still true, or is this drift?" Changing one's mind is legitimate. Either way, the answer updates the source: the goal is replaced, or the work is redirected.
- **A person's watch belongs to them.** It runs in their space, for them alone, like Capital. In an organization, the watch covers goals and work, never people: "Your work is shared, not you."
- **No model decides for the person.** It flags; the person says what they want.

Open: [watch-settings](https://github.com/thevzion/source/blob/main/objects/questions/watch-settings.md).

## Where checks live

| Checks | Where |
|---|---|
| structure checks, on any space | [structure.md](structure.md) |
| a model's own checks, for example "Does every goal say how we will know?" | the `## Checks` section of each model, in `../models/<id>/model.md`, for example [the Goals model](../models/goals/model.md) |
| trials | the files in `trials/`, listed [below](#index-of-trials) |

Checks signal; they never block ([observe before constraining](https://github.com/thevzion/source/blob/main/objects/decisions/observe-before-constraining.md)).

## Conformance

Draft, for Convention 0.3, draft. An implementation conforms to Convention 0.3 if it passes the checks marked normative. Conformance describes the implementation; it never blocks a person's change ([writing](../convention/writing.md)).

Normative checks, from the [structure checks](structure.md), each testing a MUST of the [structure](../convention/structure.md) or the [changes](../convention/changes.md):

| Check | The MUST it tests |
|---|---|
| `type`, `model`, `question`, `source` | every object file starts with a header carrying these fields, `question` word for word, `source` from-me or inferred |
| `status`, `observed-dated` | `lifecycle` and `evidence` take one of their values; `observed` is dated |
| `roots` | every object carries `serves` unless it is a root |
| `one-uri` | a URI identifies exactly one object |
| `local-double` | a local type asks a question no installed type asks (a decision model answers) |
| `manifest` | `space.md` declares the URI, the convention version, the models, truth, access and adapters |
| `local-committed` | what is machine-specific, such as the local path of a URI, is not committed |
| `subject`, `trailers` | every change has its subject form and its `Why:` and `Source:` trailers |
| `append-only` | an accepted change is never rewritten |

Open: MUSTs with no check yet.
- every object file carries a `date`;
- a link carries its provenance, and is carried by the object that declares it;
- state is derivable from accepted changes;
- every change has an author, and says which objects and its status;
- an agent's change is marked `inferred` when the agent inferred;
- a correction is a new change, on the wrong object or link and nothing else;
- the transitions of the two status axes, once they leave "proposed" ([structure](../convention/structure.md#the-axes-as-state-machines-proposed));
- the evidence values `common-ground` and `contested`, which the `status` check does not accept yet, once they leave "proposed";
- an inferred claim that reaches others is ratified if it is a want, or sourced if it is a fact, unless it is common ground for its readers (proposed, [structure](../convention/structure.md#when-an-inference-is-enough-proposed));
- silence is never taken as ratification (proposed);
- a holder that delivers work lists the decisions it inferred (proposed, [changes](../convention/changes.md#a-work-lists-the-decisions-it-inferred-proposed)).

Open: candidate trailers with no check yet, and not normative: `By:`, `On-behalf-of:`, `Lane:`, `Cost:`, `Ratifies:` ([changes](../convention/changes.md#fields)). A check for `On-behalf-of:` would also flag a person's name or address.

## What can break

The concept converged on 2026-11-12: the last four ideas added no kind of thing to the meta-model, and each piece already exists and works elsewhere. That is not evidence that the assembly works. The convergence came from a dialogue between a person and an agent, and an agent tends to find that what it is offered holds.

| What can break | Why it is plausible | Trial |
|---|---|---|
| placement | everything rests on well-typed objects; if two agents file the same intent differently, the measures measure noise | [9](trials/09-placement.md) |
| declared draws-on | an AI model can cite a source it read that does not support what it claims; the gap between `read` and `draws-on` only catches sources never read | [15](trials/15-trace.md) |
| fragmentation | small objects can read as badly as a long document: people read a story, and the views must recompose it | [14](trials/14-review.md) |
| read capture | where the adapter cannot see the agent's tool calls, `read` is missing | [15](trials/15-trace.md); N1 to N7 in [4](trials/04-notion-adapter.md) |
| the balance | reviewing, correcting and upkeep still take time; that cost is accepted against the gains ([decision](https://github.com/thevzion/source/blob/main/objects/decisions/review-cost-accepted.md)), and the balance is measured: "adds no work" stays a hypothesis | [10](trials/10-commit-or-ask.md); upkeep |
| labeling | the swap test, and a label set by an agent, can be unreliable | [13](trials/13-genericity.md) |
| the convergence itself | no outsider has attacked it yet | [18](trials/18-adversarial-review.md) |
| the leak | an executable life, with clean data, is worse to lose | [19](trials/19-exposure.md); exposure |

## Index of trials

Trials write only in a test space. Trials 1, 12 and 18 also read real material, read-only.

| # | Trial | Level | Hypotheses it tests | Status |
|---|---|---|---|---|
| 0 | [the first run](trials/00-first-run.md) (proposed) | end-to-end | [your-intent-is-the-source](https://github.com/thevzion/source/blob/main/objects/hypotheses/your-intent-is-the-source.md) | not-run |
| 1 | [a home and the canonical repository](trials/01-home-and-canonical-repo.md) | end-to-end | | not-run |
| 2 | [two turns, and reuse](trials/02-two-turns-reuse.md) | end-to-end | [it-compounds](https://github.com/thevzion/source/blob/main/objects/hypotheses/it-compounds.md), [your-intent-is-the-source](https://github.com/thevzion/source/blob/main/objects/hypotheses/your-intent-is-the-source.md) | canonical run, 2026-11-17; the rest not run |
| 3 | [resume with another agent](trials/03-resume-with-another-agent.md) | end-to-end | [the-prompt-works-with-any-agent](https://github.com/thevzion/source/blob/main/objects/hypotheses/the-prompt-works-with-any-agent.md), [it-carries-over-to-people](https://github.com/thevzion/source/blob/main/objects/hypotheses/it-carries-over-to-people.md), [switch-models-keep-everything](https://github.com/thevzion/source/blob/main/objects/hypotheses/switch-models-keep-everything.md), [say-it-once-every-agent-knows-it](https://github.com/thevzion/source/blob/main/objects/hypotheses/say-it-once-every-agent-knows-it.md), [you-stop-re-explaining](https://github.com/thevzion/source/blob/main/objects/hypotheses/you-stop-re-explaining.md) | canonical run, 2026-11-17; the rest not run |
| 4 | [the Notion adapter](trials/04-notion-adapter.md) | end-to-end | | partly, 2026-11-15 |
| 5 | [interoperability](trials/05-interoperability.md) | end-to-end | [it-carries-over-to-people](https://github.com/thevzion/source/blob/main/objects/hypotheses/it-carries-over-to-people.md) | canonical run, 2026-11-17; the rest not run |
| 6 | [the person spots the catches](trials/06-person-spots-catches.md) | end-to-end | [work-can-train-the-person](https://github.com/thevzion/source/blob/main/objects/hypotheses/work-can-train-the-person.md) | not-run |
| 7 | [the reduced tree](trials/07-reduced-tree.md) | end-to-end | [the-prompt-works-with-any-agent](https://github.com/thevzion/source/blob/main/objects/hypotheses/the-prompt-works-with-any-agent.md) | not-run |
| 8 | [a second model](trials/08-second-model.md) | end-to-end | | not-run |
| 9 | [placement](trials/09-placement.md) | end-to-end | [anyone-can-start-with-seven-types](https://github.com/thevzion/source/blob/main/objects/hypotheses/anyone-can-start-with-seven-types.md), [your-intent-is-the-source](https://github.com/thevzion/source/blob/main/objects/hypotheses/your-intent-is-the-source.md), [you-say-it-the-source-writes-itself](https://github.com/thevzion/source/blob/main/objects/hypotheses/you-say-it-the-source-writes-itself.md) | canonical run, 2026-11-17; the rest not run |
| 10 | [commit or ask](trials/10-commit-or-ask.md) | end-to-end | [adds-no-work](https://github.com/thevzion/source/blob/main/objects/hypotheses/adds-no-work.md), [saying-it-clearly-costs-less-than-fixing-it](https://github.com/thevzion/source/blob/main/objects/hypotheses/saying-it-clearly-costs-less-than-fixing-it.md), [fixing-an-explicit-mistake-costs-less-than-asking-first](https://github.com/thevzion/source/blob/main/objects/hypotheses/fixing-an-explicit-mistake-costs-less-than-asking-first.md), [you-stop-babysitting-your-agent](https://github.com/thevzion/source/blob/main/objects/hypotheses/you-stop-babysitting-your-agent.md) | canonical run, 2026-11-17; the rest not run |
| 11 | [the dashboard](trials/11-dashboard.md) | end-to-end | | not-run |
| 12 | [an existing workspace](trials/12-existing-workspace.md) | end-to-end | | not-run |
| 13 | [genericity](trials/13-genericity.md) | end-to-end | [adds-no-work](https://github.com/thevzion/source/blob/main/objects/hypotheses/adds-no-work.md), [structure-fixes-verbose-agents](https://github.com/thevzion/source/blob/main/objects/hypotheses/structure-fixes-verbose-agents.md), [your-context-makes-it-specific](https://github.com/thevzion/source/blob/main/objects/hypotheses/your-context-makes-it-specific.md) | not-run |
| 14 | [review](trials/14-review.md) | end-to-end | [adds-no-work](https://github.com/thevzion/source/blob/main/objects/hypotheses/adds-no-work.md), [you-review-only-what-was-inferred](https://github.com/thevzion/source/blob/main/objects/hypotheses/you-review-only-what-was-inferred.md) | not-run |
| 15 | [the trace](trials/15-trace.md) | end-to-end | [structure-shrinks-the-context](https://github.com/thevzion/source/blob/main/objects/hypotheses/structure-shrinks-the-context.md), [every-object-knows-where-it-came-from](https://github.com/thevzion/source/blob/main/objects/hypotheses/every-object-knows-where-it-came-from.md) | canonical run, 2026-11-17; the rest not run |
| 16 | [the learned context](trials/16-learned-context.md) | end-to-end | [structure-fixes-verbose-agents](https://github.com/thevzion/source/blob/main/objects/hypotheses/structure-fixes-verbose-agents.md), [structure-shrinks-the-context](https://github.com/thevzion/source/blob/main/objects/hypotheses/structure-shrinks-the-context.md), [it-compounds](https://github.com/thevzion/source/blob/main/objects/hypotheses/it-compounds.md), [you-stop-re-explaining](https://github.com/thevzion/source/blob/main/objects/hypotheses/you-stop-re-explaining.md) | not-run |
| 17 | [projection to PROV](trials/17-projection-to-prov.md) | end-to-end | [every-object-knows-where-it-came-from](https://github.com/thevzion/source/blob/main/objects/hypotheses/every-object-knows-where-it-came-from.md) | not-run |
| 18 | [adversarial review](trials/18-adversarial-review.md) | end-to-end | | not-run |
| 19 | [exposure](trials/19-exposure.md) | end-to-end | | canonical run, 2026-11-17; the rest not run |
| 20 | [pure projection](trials/20-pure-projection.md) | end-to-end | [ask-and-it-answers-from-your-sources](https://github.com/thevzion/source/blob/main/objects/hypotheses/ask-and-it-answers-from-your-sources.md) | not-run |
| 21 | [voice](trials/21-voice.md) | end-to-end | [you-say-it-the-source-writes-itself](https://github.com/thevzion/source/blob/main/objects/hypotheses/you-say-it-the-source-writes-itself.md) | not-run |
| 22 | [the watch](trials/22-watch.md) | watch | [it-keeps-moving-when-you-stop-talking](https://github.com/thevzion/source/blob/main/objects/hypotheses/it-keeps-moving-when-you-stop-talking.md) | not-run |
| 23 | [a routine](trials/23-routine.md) | end-to-end | [it-keeps-moving-when-you-stop-talking](https://github.com/thevzion/source/blob/main/objects/hypotheses/it-keeps-moving-when-you-stop-talking.md) | not-run |
| 24 | [elevation](trials/24-elevation.md) | end-to-end | [anyone-can-start-with-seven-types](https://github.com/thevzion/source/blob/main/objects/hypotheses/anyone-can-start-with-seven-types.md), [work-can-train-the-person](https://github.com/thevzion/source/blob/main/objects/hypotheses/work-can-train-the-person.md), [you-dont-need-to-know-how](https://github.com/thevzion/source/blob/main/objects/hypotheses/you-dont-need-to-know-how.md) | not-run |
| 25 | [identity](trials/25-identity.md) | end-to-end | [it-carries-over-to-people](https://github.com/thevzion/source/blob/main/objects/hypotheses/it-carries-over-to-people.md), [every-object-knows-where-it-came-from](https://github.com/thevzion/source/blob/main/objects/hypotheses/every-object-knows-where-it-came-from.md), [agents-hold-roles-under-mandates](https://github.com/thevzion/source/blob/main/objects/hypotheses/agents-hold-roles-under-mandates.md) | not-run |
| 27 | [scale](trials/27-scale.md) | end-to-end | | canonical runs, 2026-11-17 |
| 28 | [answer first](trials/28-answer-first.md) | end-to-end | | canonical runs, 2026-11-17 |
| 29 | [a correction keeps the history](trials/29-correction.md) | end-to-end | | canonical runs, 2026-11-17 |
| 30 | [the setup in one message](trials/30-setup-in-one-message.md) | end-to-end | | canonical runs, 2026-11-17 |
| 31 | [a local type](trials/31-local-type.md) | end-to-end | | canonical runs, 2026-11-17 |
