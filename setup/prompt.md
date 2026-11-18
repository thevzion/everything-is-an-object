# Setup prompt

Version 3.1, draft, for Convention 0.3. Version 3.1 adds line 9 of the doctrine and keeps what is made as an object; it has no run yet. Version 2.2 was run end to end by agents with no context setting up a person's home (2026-11-15), not yet by a person. Version 3.0 names the Convention and writes `convention:` in `space.md`, names the view Decisions, and adds three habits: answer first, then write; every request for work leaves an object; others' data stays out. Its runs are counted in [the proof](../README.md#proof).

Still open, for the convention rather than this prompt: the `date` of an object copied from elsewhere; the format of `draws-on` toward something outside the space; the Notion schema beyond the property names below; the Home model's questions, written for "we", and its `why`; what to do when git has no identity on the machine; the trial that would test this version with a person.

Copy everything below the line and give it to an agent that works in a folder. The other ways to start are in [README.md](README.md).

---

Set up my space with the Convention, version 0.3 (draft): everything is an object.

A space is what I want, written as small typed objects that people and agents can read and act on, where every change is kept with its reason. The convention is in `convention/` and the models are in `models/` at https://github.com/thevzion/convention. This text is enough to start; read the convention when a question comes up. Where they differ, the convention wins.

One structure, two paths:

| The space is for | It is | Model |
|---|---|---|
| me alone: my life, what I want for myself | my home, an organization of one | `home@0.1`, which includes `organization@0.1` |
| a group that wants something together | an organization | `organization@0.1` |

"My organization" can mean either: one person who wants something is an organization of one. Everything below holds for both paths.

## How you behave

```
Make it explicit.
1. Start from what the person wants, and find the question it answers.
2. Discern before you write: an existing object first, then a type from the
   installed models, then a local type you name by its question. Same for
   links. The person never has to name a type.
3. Say where each object and link comes from: its model and version, or
   local. Mark what you inferred.
4. Commit rather than ask or wait: a visible mistake is cheap to fix. Say
   briefly what you wrote; the person reviews when they choose.
5. Keep every change, with its reason. Correct precisely: reverse the one
   object or link that is wrong, and keep why. Nothing is rewritten.
6. Keep it small: one object per question. What won't serve again stays in
   the conversation.
7. Ask only when the answer changes what they want. Never decide what they
   want, and let nothing that reaches others rest on an inference.
8. Respond, not only file: answer what the person says with something that
   moves it, a proposal, a question or an objection. Agreement alone is not
   a response. A response adds; it never blocks. (proposed)
9. Every request becomes an object, and so does what is done for it; the
   answer says what was written. (proposed)
```

In practice:

- Write, commit, then tell me. Ask for my approval only where a mistake cannot be corrected precisely, or reaches someone else: publishing, pushing, sending a message, making a commitment, spending money.
- Ask before reading my files, and read only what I allow: what you read goes to your AI provider.
- Never move or delete a file that has no history yet: that cannot be undone.
- Answer me first: your first words say what you understood, what you will write and what is still pending, not only what you will read. Then write. Then tell me what you wrote. A request for work, a plan or a summary, leaves an object, not only a message.
- Keep other people's data out: their health, money, messages or private life stay out of my space unless they agree. Write what it means for me, not their details.

## 1. Start in one message

First check, by name only, whether a `space.md` exists in this folder or a parent folder. If one does, this space exists: continue it, and never set up a second one for the same thing.

Then your first message asks everything you need, at once, so I can answer in one go, by voice if I like. Leave out what I already told you:

> 1. Is this space for you alone, or for a group?
> 2. What should it be called, and under which handle: your account on GitHub or another git host, or any short name? I suggest "<a name from the folder, or from what I said>".
> 3. Where should it live: this folder, your Notion, or both? May I keep its history with git? Have you already set up a space like this somewhere else?
> 4. May I look for what you already have (folders, notes, documents)? I will list candidates by name first, and read only the ones you pick.
> 5. In a few lines, even roughly: what do you want, why does it matter, and how will you know?

Then start at once with whatever I answered, and fill in the rest as it comes:

- Write what you can from my words, and mark what you inferred.
- If I said yes to 4, search for candidate folders and show me the list, names only. Scan the ones I pick, in parallel if you can (sub-agents), and place what you find, marked `inferred`, while I keep talking.
- Never leave me waiting. If something takes long, say what is running, and what I can do meanwhile.

If both a folder and Notion, each type has one source: ask which side is the source. The other side is a projection of it, never edited by hand.

## 2. Lay it out

In a folder:

```
<space>/
├─ space.md       the manifest: URI, convention version, models, truth, access, adapters
├─ objects/       the state: one file per object, typed by its model
│  ├─ problem.md  why.md  vision.md
│  ├─ goals/  projects/  tasks/  decisions/
│  ├─ sites/      links to other spaces
│  └─ types/      local types, each with its question
├─ spaces/        child spaces, with the same tree
├─ AGENTS.md      a projection of the doctrine and the space's mandates
├─ .git/          the changes: one commit per change
└─ .local/        specific to this machine, never committed
```

Create a folder when its first object arrives. Add `.local/` to `.gitignore`.

In Notion:

```
<Name>            the page is the space; its URL is its URI
├ Agents          a projection
├ Objects         a database: one page per object; a view per type, a Sites view
├ Changes         a database: one row per change; the "Proposed" view is the inbox
└ <Child space>   the same tree
```

Each page of Objects has the properties URI, Type, Question, Serves, Source, Lifecycle, Evidence, Date. Each row of Changes has Date, Who, Objects, Operation, Why, Source, Status, and Commit when a folder is the source.

Then give the space the seven default [views](https://github.com/thevzion/convention/blob/main/convention/views.md#the-default-views-proposed), one page each, each a view of Objects or Changes that answers its question: Board, Decisions, Intents, Ground, Purpose, Knowledge, Measures. They are projections: nothing is written in them.

## 3. Write space.md and AGENTS.md

For example:

```markdown
---
space: workshop
uri: mira-bikes/workshop
convention: 0.3
models:
  - organization@0.1
adapters:
  - folder+git
---

# Bike workshop

## Truth

| Where | Role |
|---|---|
| this folder | Source |
| AGENTS.md | Projection |

## Access

| Verb | Who |
|---|---|
| read | me, my partner |
| propose | me, my partner, our agents |
| decide | me, my partner |
```

- `uri` is the space's address: unique, and it never changes. Write it `<handle>/<name>`, as TheVZion's is `thevzion/source`. A bare folder name (`home`, `workshop`) is unique for no one. In Notion, the URI is the page URL.
- For a home: `space: home`, `uri: <handle>/home`, `models: - home@0.1`, and Access names only me unless I say otherwise.
- Access has three verbs: read, propose, decide.

`AGENTS.md` is a projection, in the ten sections of [`convention/roles.md`](https://github.com/thevzion/convention/blob/main/convention/roles.md), in their order. Copy its norm texts word for word, in English; write the rest from this space's objects, in the language I write to you in, each line naming its object. Regenerate it when its sources change; do not edit it by hand. If the folder already has an `AGENTS.md` or another agent file, keep what it says, add the ten sections after it, and tell me.

The first commit is `space(add): set up the space`, with `space.md`, `AGENTS.md` and `.gitignore`.

## 4. Write the objects

Start from what I say. Find the question each thing answers, and write one object per question, in my words where you can.

Each object starts with a header:

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

| Field | What it holds |
|---|---|
| `type` | the type id: singular, lowercase |
| `model` | the model that defines the type, with its version, or `local` |
| `question` | the type's question, word for word |
| `serves` | the path of the object that justifies this one; a list if several. `problem` and `why` have none, and an object that serves nothing in the space yet has none until it does: leave the field out. |
| `draws-on` | optional, candidate: the objects this one relies on |
| `source` | `from-me` for what I said or wrote to you, or decided; `inferred` for what you wrote, condensed, rephrased or guessed, and for what someone else relays for me until I confirm it |
| `lifecycle` | `proposed`, `active`, `review`, `done` or `dropped` |
| `evidence` | how we know it holds, the value alone: `by-construction`, `sourced`, `observed` (dated by `date`), `hypothesis` or `open` |
| `date` | when it was written |

A space starts with seven types:

| Type | Model | File | Question |
|---|---|---|---|
| problem | `purpose@0.1` | `objects/problem.md` | What is wrong today, and for whom? |
| why | `purpose@0.1` | `objects/why.md` | Why do we exist? |
| vision | `purpose@0.1` | `objects/vision.md` | What does the world look like if we succeed? |
| goal | `goals@0.1` | `objects/goals/1.md` | What do we want now, and how will we know? |
| project | `work@0.1` | `objects/projects/<name>.md` | What are we building for it? |
| task | `work@0.1` | `objects/tasks/<name>.md` | What are we doing next? |
| decision | `loop@0.1` | `objects/decisions/<name>.md` | What did we choose, why, and who answers for it? |

The questions are quoted from the models; where a model's file says otherwise, the model wins. In a home, read "we" as "I"; the `question` field stays word for word.

To place something, in this order:

1. revise an existing object, or add a link to it;
2. otherwise, use a type from the installed models;
3. otherwise, use a local type already created;
4. otherwise, create a local type in `objects/types/<name>.md`, named by its question, with its horizon and its fields. Its objects say `model: local`. A local type must ask a question no installed type asks; otherwise it is a field, a link or a view.

Then:

- You may infer an object from what I said, and mark it `inferred`. Never write a want, a why or a vision I did not say, not even marked `inferred`: one I never stated is a question for me, not an object.
- A want usually comes with its reason: what is wrong today, and for whom, is the `problem`. Write it, and the goal serves it. The `why` is a reason to exist, not the reason for one goal: write it only from my own words, and add nothing to them. If I said none, leave it out; you may ask once. The vision can come later: checks signal, they never block.
- The seven types do not all need an answer on the first day. When some are missing and they matter for what I am doing, ask their questions together, in one message, Problem and Why first; I answer the ones I want.
- A site is a link to another space, not a typed object: one file in `objects/sites/<name>.md` whose header holds `site:` (that space's URI or URL), `serves` (the goal it serves), `source` and `date`, and no `type` or `question`. In a home, an organization I join is a site. A child space goes in `spaces/<name>/`, with the same tree.

## 5. Keep every change

With git, one commit per change:

```
<object>(<operation>): <what changes>

Why: <the reason, one line>
Source: from-me | inferred
```

The trailers are the message's last paragraph, one per line, with no blank line between them, so `git interpret-trailers --parse` reads them all. If your client appends a line such as `Co-Authored-By:`, put it right after `Source:`, in the same block.

`<object>` is the type of the object that changes (`goal`, `decision`); for `space.md` and `AGENTS.md`, it is `space`.

| Operation | When |
|---|---|
| `add` | a new object |
| `revise` | its content changes |
| `remove` | it goes, and the change says why |
| `rename` | same object, new name |
| `split`, `merge` | one becomes two, two become one |
| `test` | a hypothesis is tried |
| `validate` | its status moves up |
| `reverse` | one object or link was wrong; it is undone, and why is kept |
| `restructure` | a type, a question or a kind of link changes |
| `migrate` | objects move to another type or model version: propose it, apply it after my yes |

- The objects keep the truth; the message helps you read them.
- A correction is a change: `reverse` or `revise` the one object or link that is wrong, and nothing else. Never rewrite history.
- Propose instead of committing to the main line when a change comes from someone else, is large (a migration), or commits other people: a branch or a pull request in git, a row in the "Proposed" view in Notion.
- What you make for me is an object too, of type `deliverable` (`work@0.1`, "What comes out, and is it done?"), serving what it is for. A text is one file. A page, code or an image with files of its own is a folder, `objects/deliverables/<name>/`, its header in `README.md` and its files beside it. Inside that folder, code keeps its own conventions.

Without git (I said no, or your sandbox blocks it, in which case say so), keep `changes.md` at the root, append only. One entry per change:

```markdown
## 2026-11-15 goal(add): repair fifty bikes a month by June

Objects: objects/goals/1.md
Why: the first goal I named
Source: from-me
Who: me, with my agent
```

Never edit an earlier entry: a correction is a new entry. In Notion, each change is one row of the Changes database, with the same fields.

## 6. Tell me what you wrote

After each turn, say in a few lines what you wrote: which objects, which of them are `inferred`, and any question only I can answer. I review when I choose. Ask only when the answer changes what I want, and group your questions in one message.

If this folder was set up with version 1 of this prompt (`organization.md`, `org/`), read [the move from version 1](https://github.com/thevzion/convention/blob/fd9a5ff/setup/prompt.md#from-version-1-to-version-2) and propose it as one `migrate` change.

## 7. Make it run

- **Make your tool read it.** If your tool reads its instructions from a file other than `AGENTS.md`, write that file with one line pointing to `AGENTS.md`, commit it, and say which file you wrote. Claude Code, for example, reads `CLAUDE.md`, so it gets the line `@AGENTS.md`.
- **Guarantee the trailers.** As soon as git is set up, save this hook as `.git/hooks/commit-msg` (it is never committed) and make it executable; your tool may ask my permission first, which is expected; if it cannot write the hook, give me the command to run myself. It refuses a message whose `Why:` and `Source:` git does not read as trailers:

```sh
#!/bin/sh
t=$(git interpret-trailers --parse < "$1")
printf '%s\n' "$t" | grep -q '^Why: .' &&
printf '%s\n' "$t" | grep -Eq '^Source: (from-me|inferred)' && exit 0
echo "commit refused: end the message with Why: and Source: in one trailer block" >&2
exit 1
```

- **Give the roles to your tool** (proposed). If your tool lets you define agents (Claude Code reads `.claude/agents/<role>.md`), write one per role of section 5 of `AGENTS.md` that the assistant starts: `materializer`, `maker`, `checker`. A few lines each: the role, from section 5; what it writes, and what it never writes; and "Read `AGENTS.md` first." Commit them. A tool without such definitions starts a unit with the same text in its brief.
- **Keep going.** If your tool can keep tasks or follow-ups, propose one for the next step, and create it only on my yes.
- **Prove it loads.** Tell me how to check: open a new session in this folder and ask for the title of section 10 of `AGENTS.md`.
