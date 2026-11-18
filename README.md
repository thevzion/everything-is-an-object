# Everything is an object.

**[Simple](#twelve-words)** · **[Yours](#try-it)** · **[Readable](tests/runs/03-resume/)** · **[Linked](tests/runs/05-two-spaces/)** · **[Kept](tests/runs/README.md#each-check)** · **[Open](#license)**

An object is a short text that answers one question, in the words of whoever said it. A few header
lines say which question it answers, what it serves, who said it, and where it stands. Everything
else is a link, a view or a change.

```markdown
---
type: goal
model: goals@0.1
question: What do we want now, and how will we know?
serves: objects/problem.md
source: from-me
lifecycle: active
evidence: open
date: 2026-11-15
---

Repair fifty bikes a month by June. We will know when the logbook shows fifty for three months in a row.
```

That is the idea. This repository is **the Convention**: one way to write it down, so that people
and agents read and act on the same things. As "everything is a file" is an idea and POSIX one way
to encode it, "everything is an object" is the idea, and the convention one encoding of it.

See it run: [the convention, live](https://thevzion.com/convention/). <!-- the route /convention/ was chosen by the founder; the page is not published yet, and the site still serves its earlier version -->

Convention 0.3, draft · setup prompt 3.0, draft

## Twelve words

Everything is made of twelve primitives. Each one has an effect you can point at
([primitives](convention/primitives.md)).

| Primitive | What it is | What you see |
|---|---|---|
| **person** | the one who wants, and says yes; not a role | `source: from-me` on what you said; nothing goes out before your yes |
| **signal** | anything said or sent, not yet turned into objects | your words, kept as you said them |
| **object** | a short text that answers one question | one file in `objects/`, or one page |
| **link** | `serves`, `draws-on`, answers, replaces, contradicts | a header line that points to a path or a URI |
| **question** | the fixed question of a type, word for word | the `question:` line of every header |
| **space** | a source with its own authority | `space.md`: its URI, its models, who decides |
| **role** | a function: what it turns into what, and what it answers for | section 4 of `AGENTS.md` |
| **binding** | a role held on an object, under a mandate | a mandate; "notify me" on an object |
| **trigger** | what wakes a role: a change, or a time | a routine's "when" |
| **action** | work handed over with a brief that stands alone | the brief, and what comes back: result, evidence, limits |
| **view** | a question asked of objects, read-only | Board, Decisions, Measures, stamped with their commit |
| **history** | every change, with its author and its why | `git log`, each commit ending in `Why:` and `Source:` |

What looks like a thirteenth word is made of these: a model is a set of questions, an app is views
plus signals and yeses, Decisions is a view of what waits for a yes, Auto is a role with a time
trigger.

Three roles are fixed, beside the person: the **assistant**, who answers at once and says what is
pending; the **materializer**, who turns what was said into objects; the **checker**, who checks
what others made. Every other role emerges when the work needs it ([roles](convention/roles.md)).

## Try it

Open a folder with an assistant that can read and write files there. Give it this:

```text
Read https://raw.githubusercontent.com/thevzion/convention/main/setup/prompt.md
and do what it says below the line: set up my space in this folder.
```

If your assistant cannot read a web page, open [`setup/prompt.md`](setup/prompt.md), copy everything
below the line, and paste it.

1. **One message, five questions.** It looks for an existing `space.md` first. Then it asks, all
   at once: for you alone or for a group; a name and a handle; where it lives, and whether it may
   keep history with git; whether it may look at what you already have; what you want, why it
   matters, and how you will know.
2. **Your folder.** It writes `space.md`, then `AGENTS.md`, then one object per question, usually a
   problem and a goal. Each object says `source: from-me` or `source: inferred`.
3. **Every change, with its reason.** One commit per change, ending in `Why:` and `Source:`. You
   review when you choose.

A space starts with seven types, each a question: problem, why, vision, goal, project, task,
decision. None is required on the first day.

The folder you get is yours: plain files, in your folder, readable without any tool of ours.

## Proof

Each question below is put to a real agent, in a fictional space built the same way every time,
several times. The criteria were fixed before the first counted run (one rule was widened after a
smoke run, and its trial says so; a criterion that follows a change of the convention is a new
version, fixed before the next batch); the checks are computed (files, headers, git), and a decision model answers the few closed questions that need reading. "Passed"
counts the runs that met every criterion. Each run is an object, with its trace.

<!-- proof:begin -->
| Question | Batch 3 | Batch 2 | Batch 1 | Trace |
|---|---|---|---|---|
| Does one message set up a space that follows the convention? | 4/5 | 3/5 | 1/5 | [30-setup](tests/runs/30-setup/) |
| Does what I say become objects, in my words? | 1/5 | 0/5 | 0/5 | [09-placement](tests/runs/09-placement/) |
| When no type fits, does the agent create a local type, named by its question? | 5/5 | 3/5 | 1/5 | [31-local-type](tests/runs/31-local-type/) |
| Does a repeated signal enrich the existing object instead of adding one? | 5/5 | 5/5 | 4/5 | [02-reuse](tests/runs/02-reuse/) |
| With 100 objects, does the agent still find the existing one, and at what cost? | 3/5 | 4/5 | 0/5 | [27-scale-100](tests/runs/27-scale-100/) |
| With 1,000 objects, does the agent still find the existing one, and at what cost? | 3/5 | 5/5 | 4/5 | [27-scale-1000](tests/runs/27-scale-1000/) |
| With 10,000 objects, does the agent still find the existing one, and at what cost? | 1/5 | 5/5 | 3/5 | [27-scale-10000](tests/runs/27-scale-10000/) |
| Does what goes out wait for a yes? | 5/5 | 5/5 | 4/5 | [10-goes-out](tests/runs/10-goes-out/) |
| Does a correction keep the history? | 5/5 | 2/5 | 0/5 | [29-correction](tests/runs/29-correction/) |
| Does another session resume from the source alone? | 5/5 | 5/5 | 5/5 | [03-resume](tests/runs/03-resume/) |
| Is the inferred marked as such, and are quotes exact? | 5/5 | 4/5 | 5/5 | [15-quotes](tests/runs/15-quotes/) |
| Does a third party's private data stay out? | 5/5 | 5/5 | 4/5 | [19-others-data](tests/runs/19-others-data/) |
| Does the assistant answer at once, before the writing? | 5/5 | 2/5 | 3/5 | [28-answer-first](tests/runs/28-answer-first/) |
| Do two spaces exchange a proposal? | 0/5 | 5/5 | 3/5 | [05-two-spaces](tests/runs/05-two-spaces/) |

- **Batch 3**: 70 runs, 2026-11-17; claude-sonnet-5-5; convention 0.3 at 5cc56b8; $8.40 at API prices as the CLI reports them.
- **Batch 2**: 70 runs, 2026-11-17; claude-sonnet-5-5; convention 0.3 at 9845a31; $10.64 at API prices as the CLI reports them.
- **Batch 1**: 70 runs, 2026-11-17; claude-sonnet-5-5; convention 0.3 at e5cbf4e; $8.31 at API prices as the CLI reports them.
- **Commit ids**: batches 1 to 3 cite commits of the history written before publication, which the founder keeps; the public history starts at Convention 0.3, draft. Batch 3 ran against the same text (`convention/`, `setup/`, `models/`, `AGENTS.md`) as the first public commit, apart from three lines of `convention/README.md` that drop the earlier name.
- **Header criteria**: batches 1 and 2 are checked by version 1, which asks for `serves` on every object but a root; from batch 3, by version 2, fixed before batch 3 ran, where an object may serve nothing yet. Version 1 stays computable from every run, and is read again below where it differs.
- **31-local-type, read by the written criterion of L4**, which the code exceeded until pass 2 by also asking for `serves`: 5/5 in batch 1, 5/5 in batch 2.
- **Batch 3, read by version 1 of the header criteria**, which asked for `serves` on every object but a root: 09-placement 0/5.
<!-- proof:end -->

**What the batches show**, read in the traces:

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
in [the runs](tests/runs/README.md) and are not a batch.

How it runs: [`tests/runner`](tests/runner/), in Rust; `convention-trials run all` replays every
trial, and `convention-trials table` regenerates this table from the run objects in
[`tests/runs/`](tests/runs/README.md).

**Before this version.** On 15 Nov 2026, ten runs of the setup prompt 2.2 went end to end, with
Claude Opus and Claude Sonnet ([status](setup/README.md)). On 13 Nov 2026, someone outside the
project tried version 2 of the prompt and stopped after a long wait
([the result](https://github.com/thevzion/source/blob/main/objects/results/first-run-outside.md));
since version 2.1, the prompt asks everything in one message.

**Not tested yet**: a person, rather than an agent, setting up a space; agents from other
providers ([any assistant is a hypothesis](https://github.com/thevzion/source/blob/main/objects/hypotheses/the-prompt-works-with-any-agent.md));
the Notion adapter beyond [one partial run](tests/trials/04-notion-adapter.md); the other trials
of [`tests/`](tests/README.md). Small samples show what happened in these runs; they do not
promise what happens in yours.

## Optional adapters

Nothing is required but text an agent can follow, and a folder or a page. The rest is comfort,
chosen by your agent when it fits ([adapters](convention/adapters.md)):

| Adapter | What it adds |
|---|---|
| git | every change with its reason, offline, without an account; the default |
| Notion | the same objects as pages, on your phone; the default views as pages |
| an app, a status bar, a command line, an MCP server | reading views and saying yes without the chat; in a separate kit, not published yet |

## What is here

| Path | What it holds |
|---|---|
| [`convention/`](convention/README.md) | the primitives, the doctrine, the structure, roles, views, spaces, changes, truth, trace, adapters, the glossary, and how the rules are written |
| [`models/`](models/README.md) | the library of types, each a question: Purpose, Goals, Work, Loop, Knowledge, Agreements, Capital, and Organization and Home, which assemble them; seeds; how a local type is promoted |
| [`setup/`](setup/README.md) | the setup prompt, and the other ways to start |
| [`tests/`](tests/README.md) | the trials, each a question with its criteria; [`tests/runner/`](tests/runner/), the runner; [`tests/runs/`](tests/runs/README.md), every run as an object |
| [`CHANGELOG.md`](CHANGELOG.md) | what changed, version by version |
| `git log` | every change, with its `Why:` and `Source:` |

## Around it

| | What it is | Where |
|---|---|---|
| **the source** | TheVZion's own space, the first written with the convention: its problem, why, goals, decisions, hypotheses and open questions | [thevzion/source](https://github.com/thevzion/source) |
| **the site** | the organization's site | [thevzion.com](https://thevzion.com) |
| **the kit** | the app, the status bar, a command line, an MCP server, the Notion adapter: optional adapters | not published yet |

## Contribute

Everything here is a candidate. Open an issue, and leave out anything private:

- [What didn't fit](https://github.com/thevzion/convention/issues/new), when you tried it.
- [Propose a change](https://github.com/thevzion/convention/issues/new?template=propose-a-change.md):
  the convention, a model or a test should say something else.
- [Propose a type or a model](https://github.com/thevzion/convention/issues/new?template=propose-a-type-or-model.md):
  a question no installed type asks.
- [Report an ambiguous question](https://github.com/thevzion/convention/issues/new?template=ambiguous-question.md):
  a type's question reads two ways.

Issues are the inbox: a proposal becomes true only by a decision. TheVZion records the decision in
[its source](https://github.com/thevzion/source/tree/main/objects/decisions), then it lands here as
a commit. For an assistant working on this repository: [`AGENTS.md`](AGENTS.md).

## License

Text and models, the run objects of [`tests/runs/`](tests/runs/README.md) included: [CC BY 4.0](LICENSE) · Code, including the runner in [`tests/runner/`](tests/runner/): [Apache 2.0](LICENSE-CODE) · Attribution: TheVZion contributors

[The decision](https://github.com/thevzion/source/blob/main/objects/decisions/license.md), in
TheVZion's source.

---

**Make it explicit.**
