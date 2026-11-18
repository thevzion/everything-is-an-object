# Primitives

Convention 0.3 · draft. Explanatory, not normative. The list is proposed.

Eight words make everything else. A word is a primitive when no other word of the list can make
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

## The eight

What is kept: object, question, link. How it changes: change. What comes in: signal. Who, and
where: person, role, space.

| Primitive | What it is | Its visible effect | Detail |
|---|---|---|---|
| **object** | anything made explicit that answers one question, with its header: what was said, decided or made | one file in `objects/`; a folder for a made thing with files of its own, its header in `README.md`; or one page | [structure](structure.md#the-object-header) |
| **question** | the fixed question of a type, word for word. A type is its question | the `question:` line of every header; a local type named by its question | [questions](questions.md) |
| **link** | a relation between two objects: `serves`, `draws-on`, `delivers`, answers, replaces, contradicts, or a local kind | a header line that points to a path or a URI; the graph drawn from them | [structure](structure.md#three-sets), [trace](trace.md) |
| **change** | one write to the source, with its author, its date and its why; append-only. The history is the changes in order: any object can be traced back, and the space replayed as it was on any date | a commit ending in `Why:` and `Source:`; `git log` on a file; the space as it was at a commit; or a row of the Changes database | [changes](changes.md) |
| **signal** | anything said or sent toward a space, not yet turned into objects: a message, a voice note, a meeting, an issue, a note typed in an app | the words kept as said: an intent's "as said", a proposed change, an issue | [truth](truth.md) |
| **person** | the one who wants, and says yes. Not a role: wanting cannot be handed over | `source: from-me` on what they said; the `decide` verb in `space.md`; nothing goes out before their yes | [doctrine](doctrine.md#the-invariant) |
| **role** | a function held by a person or an intelligence, in a unit: what it turns into what, and what it answers for | section 5 of every `AGENTS.md`, Delegate; the author of each change | [roles](roles.md) |
| **space** | a source with its own authority: a folder or a page, its objects and their changes | `space.md`: its URI, the convention version, its models, truth and access | [spaces](spaces.md) |

## Compositions

What looks like a ninth word is made of these. The first five were primitives earlier in this
draft; since every request, and what is done for it, is an object, each is made of the eight.

| Composition | Made of | Visible as |
|---|---|---|
| **view** | a question asked of many objects; read-only, regenerated, never edited | a page or a table stamped with the commit it comes from: Board, Decisions, Measures ([views](views.md)) |
| **action** | a brief, which is an object, handed to a role; what comes back is objects | the brief: for whom, what for, where, within which limits, what done looks like; the hand-back: Serves, Result, Evidence, Limits, Learned, Next ([roles](roles.md#section-5-delegate)) |
| **binding** | a role linked to objects under a mandate, which is an object: who writes them, who checks them, who is told | a mandate; the access table; "notify me" on an object ([spaces](spaces.md)) |
| **trigger** | a change or a date that wakes a role; a routine is an object that says when | a routine's "when"; a hook; a schedule's rituals ([questions](questions.md#routines-the-loop-that-runs-alone-candidate)) |
| **history** | every change, in order | `git log`; a view of the changes; a replay to a date |
| **model** | a versioned set of types, each a question | `models/<id>/model.md`; `goals@0.1` in a header |
| **app** | views, plus signals sent, changes made and yeses given: what a person uses to read and act. Any write through an app is a change at the source, with its author and its why | a notes app whose notes wait as signals; a board where a yes is one click |
| **inbox** | a view of the signals not processed yet, and of proposals from others | the Inbox place of [truth](truth.md); issues; a "Proposed" view |
| **decisions** | a view of what waits for a yes (formerly the view called Inbox) | the Decisions page of a space |
| **measures** | views that count | the [canonical measures](views.md#canonical-measures-candidate) |
| **roles beyond the core** | a role that emerged and was promoted: retriever, planner, watcher, projector, learner | a role's own section in a space's `AGENTS.md` |
| **Auto** | a role with a time trigger | a routine that runs at night and leaves changes to review |
| **`AGENTS.md`** | a view of the doctrine, the roles and the space's own objects, which every unit reads first | the file at the root of each space |

## In one line

A person wants; a signal carries it; objects are written that answer questions, linked to what
they serve; roles do the work, through briefs that are objects too, and hand back objects; every
change keeps its author and its why; views show it, and the person says yes.

## Why these eight

- **Nothing is missing.** What is kept (object, question, link), how it changes (change), what
  comes in (signal), who wants and says yes (person), who acts (role), and where it lives, under
  whose authority (space).
- **Nothing is doubled.** A view is a question asked of many objects. An action is a brief handed
  to a role. A binding is a role linked to objects under a mandate. A trigger is a change or a date
  that wakes a role. The history is the changes in order. An app is views and signals. A model is a
  set of questions. Each was proposed as a word of its own, and each is made of the eight.
- **Each one is old.** A record, a schema, an edge, a commit, an event, a principal, an actor, a
  namespace: the [glossary](glossary.md) gives the established term next to each word. The words
  are common nouns, so that a person and an intelligence speak of the same thing.
