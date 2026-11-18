# Primitives

Convention 0.3 · draft. Explanatory, not normative. The list is proposed.

Twelve words make everything else. A word is a primitive when no other word of the list can make
it; everything else is a composition of them. Each primitive has a visible effect: something you
can point at in the folder, the page or the history.

## An object

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

## The twelve

| Primitive | What it is | Its visible effect | Detail |
|---|---|---|---|
| **person** | the one who wants, and says yes. Not a role: wanting cannot be handed over | `source: from-me` on what they said; the `decide` verb in `space.md`; nothing goes out before their yes | [doctrine](doctrine.md#the-invariant) |
| **signal** | anything said or sent toward a space, not yet turned into objects: a message, a voice note, an issue, a note typed in an app | the words kept as said: an intent's "as said", a proposed change, an issue | [truth](truth.md) |
| **object** | anything made explicit that answers one question, with its header: what was said, decided or made | one file in `objects/`; a folder for a made thing with files of its own, its header in `README.md`; or one page | [structure](structure.md#the-object-header) |
| **link** | a relation between two objects: `serves`, `draws-on`, `delivers`, answers, replaces, contradicts, or a local kind | a header line that points to a path or a URI; the graph drawn from them | [structure](structure.md#three-sets), [trace](trace.md) |
| **question** | the fixed question of a type, word for word. A type is its question | the `question:` line of every header; a local type named by its question | [questions](questions.md) |
| **space** | a source with its own authority: a folder or a page, its objects and their history | `space.md`: its URI, the convention version, its models, truth and access | [spaces](spaces.md) |
| **role** | a function held by a person or an intelligence: what it turns into what, and what it answers for | section 5 of every `AGENTS.md`, Delegate; the author of each change | [roles](roles.md) |
| **binding** | a role held on an object or a set of objects, under a mandate: who writes it, who checks it, who is told | a mandate; the access table; "notify me" on an object | [roles](roles.md), [spaces](spaces.md) |
| **trigger** | what wakes a role: a change (a header takes a value, a file changes) or a time | a routine's "when"; a hook; a schedule's rituals | [questions](questions.md#routines-the-loop-that-runs-alone-candidate) |
| **action** | work handed over with a brief that stands alone: for whom, what for, where, within which limits, what done looks like | the brief; the hand-back: Serves, Result, Evidence, Limits, Learned, Next | [roles](roles.md#section-5-delegate) |
| **view** | a question asked of objects, read-only, regenerated and never edited | a page or a table stamped with the commit it comes from: Board, Decisions, Measures | [views](views.md) |
| **history** | every change, with its author and its why, append-only | `git log`, each commit ending in `Why:` and `Source:`; or the Changes database | [changes](changes.md) |

## Compositions

What looks like a thirteenth word is made of these.

| Composition | Made of | Visible as |
|---|---|---|
| **model** | a versioned set of types, each a question | `models/<id>/model.md`; `goals@0.1` in a header |
| **app** | views, plus signals sent, changes made and yeses given: what a person uses to read and act. Any write through an app is a change at the source, with its author and its why | a notes app whose notes wait as signals; a board where a yes is one click |
| **inbox** | a view of the signals not processed yet, and of proposals from others | the Inbox place of [truth](truth.md); issues; a "Proposed" view |
| **decisions** | a view of what waits for a yes (formerly the view called Inbox) | the Decisions page of a space |
| **measures** | views that count | the [canonical measures](views.md#canonical-measures-candidate) |
| **roles beyond the core** | a role that emerged and was promoted: retriever, planner, maker, watcher, projector, learner | a role's own section in a space's `AGENTS.md` |
| **Auto** | a role with a time trigger | a routine that runs at night and leaves changes to review |
| **`AGENTS.md`** | a view of the doctrine, the roles and the space's own objects, which every role reads first | the file at the root of each space |

## In one line

A person wants; a signal carries it; the materializer writes objects that answer questions, linked
to what they serve; a trigger wakes roles, which act through briefs and hand back what they made,
as objects; every change goes into the history; views and apps show it; the person says yes.

## Why these twelve

- **Nothing is missing.** Wanting and deciding (person), what comes in (signal), what is kept
  (object, link, question), where it lives (space), who acts (role, binding), when and how
  (trigger, action), what is shown (view), and what happened (history).
- **Nothing is doubled.** An inbox is a view of signals. An app is views and signals. Auto is a role
  with a time trigger. A model is a set of questions. Each was proposed as a word of its own, and each
  is made of the twelve.
- **Each one is old.** A row and its schema, a foreign key, a commit, an event, a read model, a
  delegation: the [glossary](glossary.md) gives the established term next to each word. The words
  are common nouns, so that a person and an intelligence speak of the same thing.
