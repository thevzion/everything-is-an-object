# Everything is an object.

**[Simple](#twelve-words)** · **[Yours](#try-it)** · **[Readable](tests/runs/03-resume/)** · **[Kept](tests/runs/README.md#each-check)** · **[Open](#license)**

An object is anything made explicit that answers one question: what someone said, decided or
wants, and what was made for it, a plan, a page, code. A few header lines say which question it
answers, what it serves, who said or made it, and where it stands. What a person said keeps their
words. Everything else is a link, a view or a change.

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

What is made is an object too. A page, with its files, is a folder whose `README.md` carries the
header:

```
objects/deliverables/landing/
├─ README.md     the header, and what the page is for
└─ index.html    the page itself
```

```markdown
---
type: deliverable
model: work@0.1
question: What comes out, and is it done?
serves: objects/projects/website.md
source: inferred
lifecycle: review
evidence: open
date: 2026-11-18
---

The landing page, in `index.html` beside this file. Done when a visitor can say in one sentence what we do.
```

That is the idea. This repository is **the Convention**: one way to write it down, so that people
and agents read and act on the same things. As "everything is a file" is an idea and POSIX one way
to encode it, "everything is an object" is the idea, and the convention one encoding of it.

Convention 0.3, draft · setup prompt 3.1, draft

## Bring your intelligence. Bring your application.

The convention fixes the middle: `AGENTS.md` and `objects/`, plain files in a folder you own. Both
ends stay open.

- **Your intelligence.** Any person, and any assistant from any provider, that can read and write
  files. `AGENTS.md` is the file assistants already read first; the rest is folders. Change
  assistants, or use several: they read the same objects, so none starts from zero. Only the
  features differ from one tool to another, such as starting other units; the rules are the same.
- **Your application.** Any assistant can build the apps and views you want on the objects: a page,
  a board, a widget, a dashboard. Each reads the objects and writes back as changes to them; the
  files stay the source, so any app can be thrown away and built again.

## A layer below

Agents, agent operating systems, second brains, note apps, frameworks: each keeps its own idea of
what you want, in its own format, and it is lost when you change tools. The convention sits one
layer below them, at the source: plain files in a folder you own, each with a header that says what
it is. A second brain lets each person file in their own way, so only its owner, or its own app,
can read it. Here everything is an object with the same header, so any person, any intelligence and
any app can read any space. None of it is new: files, a header, git, and a convention rather than a configuration. The
foundations are plain on purpose, so that anything can be brought onto them and anyone can read
them: a person, any intelligence, any app. It grows as files and git grow; the trials run at 100,
1,000 and 10,000 objects ([proof](#proof)).

## What it aims to solve

The root cause: what anyone wants is lost every time it is passed on, from a person to a tool,
from one session to the next, from one agent to another, so it is rebuilt again and again
([the problem](https://github.com/thevzion/source/blob/main/objects/problem.md)). Written once as
objects, it is kept, and everyone reads the same thing.

Each line below is a hypothesis, and an ambitious one. A trial shows what a few agents did in a
fictional space; it does not show a problem solved. None of these is solved by a file alone, and
two of them, shared meaning and the commons, cannot be solved by one space: they need many spaces,
and the people who use them.

| Problem | The hypothesis | Tested so far by |
|---|---|---|
| **Continuity**: each new session, tool or person starts from zero | anyone resumes from the folder alone | [another session resumes from the source](tests/runs/03-resume/) |
| **Shared meaning**: each tool keeps its own idea of what you want | a type is a question, word for word, and a repeated want enriches the object already there | [a repeated signal enriches the existing object](tests/runs/02-reuse/) |
| **Authorship**: what you said and what was guessed look the same | every object says `source: from-me` or `source: inferred` | [the inferred is marked, quotes are exact](tests/runs/15-quotes/) |
| **Delegation**: you repeat yourself and watch the agent work | work goes out with a brief that stands alone, comes back as objects, and nothing leaves before your yes | [what goes out waits for a yes](tests/runs/10-goes-out/) |
| **Verification**: you review everything, or nothing | every change keeps its reason, and a correction touches only what was wrong | [a correction keeps the history](tests/runs/29-correction/) |
| **The commons**: what one space learns stays there | a local type returns across spaces, becomes a seed, then a model others install | not tested yet |

## Why it may work

An agent works from what it reads. Here, it reads objects that were already discerned: one
question each, typed, dated, linked to what they serve, with what was said kept apart from what was
inferred. A conversation holds the same facts mixed with repetitions, detours and what was said
then reversed. So the agent should discern better from the objects than from the history, and
spend less to read them. What it writes is discerned in turn, so the next reader, a person or
another agent, starts from a cleaner source than the last one did. If the hypothesis holds, each
pass improves the source.

None of it is new. Builders of agents call the first half context engineering: what a model does
depends on what is put before it, and a long, noisy context degrades it. Long before agents,
knowledge management called the second half making tacit knowledge explicit; a Zettelkasten keeps
one idea per note; and termites, then the blackboard systems of early AI, coordinate many workers
through what they leave in a shared place rather than through messages. The convention puts these
at the source, in files that any intelligence reads, a person included.

**Not tested yet**: the same facts given to two agents, one as objects and one as the conversation
they came from, comparing what each answers and what each spends.

## Twelve words

Everything is made of twelve primitives. Each one has an effect you can point at
([primitives](convention/primitives.md)).

| Primitive | What it is | What you see |
|---|---|---|
| **person** | the one who wants, and says yes; not a role | `source: from-me` on what you said; nothing goes out before your yes |
| **signal** | anything said or sent, not yet turned into objects | your words, kept as you said them |
| **object** | anything made explicit that answers one question: said, decided or made | one file in `objects/`, a folder for a made thing with files of its own, or one page |
| **link** | `serves`, `draws-on`, `delivers`, answers, replaces, contradicts | a header line that points to a path or a URI |
| **question** | the fixed question of a type, word for word | the `question:` line of every header |
| **space** | a source with its own authority | `space.md`: its URI, its models, who decides |
| **role** | a function: what it turns into what, and what it answers for | section 5 of `AGENTS.md`, Delegate |
| **binding** | a role held on an object, under a mandate | a mandate; "notify me" on an object |
| **trigger** | what wakes a role: a change, or a time | a routine's "when" |
| **action** | work handed over with a brief that stands alone | the brief, and what comes back: result, evidence, limits |
| **view** | a question asked of objects, read-only | Board, Decisions, Measures, stamped with their commit |
| **history** | every change, with its author and its why | `git log`, each commit ending in `Why:` and `Source:` |

What looks like a thirteenth word is made of these: a model is a set of questions, an app is views
plus signals and yeses, Decisions is a view of what waits for a yes, Auto is a role with a time
trigger.

Four roles are fixed, beside the person: the **assistant**, who answers at once and says what is
pending; the **materializer**, who turns what was said into objects; a **maker**, who makes what a
brief asks; a **checker**, who checks what others made. Every other role emerges when the work
needs it ([roles](convention/roles.md)). What a role makes, a plan, a page, code, is written as an
object where the work is done.

## Where it goes

What a common format makes possible, and what we want to build on it. None of it is built yet.

- **An app you download.** A desktop app that opens any space: point it at your folder, and it
  shows your objects, the views, and what waits for your yes, with no account and no copy of your
  files. One app can read every space, because every space has the same shape; an app made for one
  second brain reads only that one.
- **Interoperability.** Every tool that reads the format reads your space and writes back to the
  same objects: an assistant, an app, Notion, a script. Change tools, and nothing is converted or
  lost.
- **A network of spaces.** A space points to another's objects by their URI, proposes a change
  there, and receives the answer as an object ([two spaces](tests/runs/05-two-spaces/)). A type
  that returns across many spaces becomes a model anyone installs: the commons.

The convention is the part that has to come first. The apps, the adapters and the network can come
from anyone, and they all read the same spaces.

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

**Not tested yet**: a person, rather than an agent, setting up a space; that what an agent makes is
kept as an object, and line 9 of the doctrine (both proposed in this version, no trial yet);
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
| an app or a view your assistant builds on the objects | reading and saying yes without the chat |

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
