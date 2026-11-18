# Changelog

What changed, version by version. Every change is also in `git log`, with its `Why:` and `Source:`.

## Proposed after the first public commit

- **An object is anything made explicit**: what someone said, decided or wants, and what was made
  for it, a plan, a page, code. What a person said keeps their words. The README gives a second
  example, a page made as an object ([primitives](convention/primitives.md#an-object)).
- **What is made is an object**: a text is one file; a made thing with files of its own is a folder,
  `objects/deliverables/<name>/`, its header in `README.md` ([structure](convention/structure.md#layout-one-object-one-file-one-page)).
  `delivers` joins the links.
- **Doctrine line 9**, the founder's rule: "Every request becomes an object, and so does what is
  done for it; the answer says what was written." In the doctrine, `roles.md`, the setup prompt
  and `AGENTS.md`.
- **Eight primitives, from twelve**: object, question, link, change, signal, person, role,
  space. Since every request, and what is done for it, is an object, view, action, binding and
  trigger are compositions; the history is the changes in order, and its atom, change, is the
  primitive ([primitives](convention/primitives.md)).
- **The template of every `AGENTS.md`**, generated from `roles.md`:
  [`setup/AGENTS.template.md`](setup/AGENTS.template.md), with the norm texts in place and braces
  for what comes from the space; the setup prompt points to it, and a self-test keeps it in sync.
- **A rule can be drawn**: every rule and norm text is written in the primitives, so it can be
  drawn and tested; one that cannot is rewritten ([writing](convention/writing.md#a-rule-can-be-drawn-proposed)).
- **`AGENTS.md` in ten sections, one per capability**, each opened by its reason: make it
  explicit, answer and propose, delegate, build on the objects, keep every change. A maker joins
  the fixed core; every unit reads the same file, the assistant gives the roles, and a role starts
  another unit of itself when the load grows ([roles](convention/roles.md)).
- **Setup prompt 3.1**, draft: line 9, what is made kept as an object, the ten sections, and the
  core roles given to the tool as agent definitions. No run yet.
- **The README** opens on a hero image and says why objects, what the convention fixes and what
  stays open, what it aims to solve as hypotheses, why it may work, and what it makes possible,
  from what is tested to what is not built yet. The detail moves one link away: the reading of the
  batches to `tests/`, what came before to `convention/README.md`. The word "Linked" leaves the
  line under the headline: its trial, two spaces, gave 0/5 in batch 3.

## Convention 0.3, draft (2026-11-18, the first version of this public history)

Everything is an object: the README opens on the idea, and presents the convention as one way to
write it down. The history written before this version, drafts 0.1 and 0.2 under an earlier name,
is kept by the founder; the public history starts here.

- **Names.** The text is in `convention/`, with `roles.md`, `views.md`, `spaces.md` and
  `writing.md`, formerly `agents.md`, `measures.md`, `access.md` and `conventions.md`. A space
  declares `convention: 0.3` in its `space.md`.
- **Primitives**, proposed: twelve words, each with its visible effect, and what is made of them
  ([primitives](convention/primitives.md)). The glossary defines an object.
- **Roles**, proposed: a fixed core beside the person, the assistant, the materializer (formerly
  the note-taker) and the checker (which absorbs the verifier); the other roles emerge and are
  promoted. The assistant answers at once and writes nothing itself; a tool's memory holds only
  pointers to objects ([roles](convention/roles.md)). Section 4 of every `AGENTS.md` changes.
- **Others' data stays out**, proposed: a bullet in section 5 of every `AGENTS.md`, and
  [spaces](convention/spaces.md).
- **Doctrine**: its eight lines, unchanged. A line 9 drafted in this pass narrowed the founder's
  rule without saying so, and was withdrawn; the habit stays in the roles, proposed.
- **Views**: the seven default views open [views](convention/views.md); the view "Inbox" (What
  waits for my yes?) is renamed Decisions. The pure projection moves there from questions.
- **Spaces**: the space, its URI, links by URI, propose and decide, before access.
- **Models**: promotion, named: how a local type becomes a seed, then a type of a model.
- **Setup prompt 3.0**, draft: the Convention named, `convention:` in `space.md`, the view
  Decisions, answer first, every request for work leaves an object, others' data stays out.
- **Proof**: canonical trials with real agents ([tests](tests/README.md)), a runner in Rust
  ([`tests/runner/`](tests/runner/)), every run kept as an object ([`tests/runs/`](tests/runs/README.md)),
  and the README's proof table generated from them.
- **After the runs** (pass 2), proposed: an object may serve nothing in the space yet, and the
  measure gaps and tensions counts it ([structure](convention/structure.md#the-object-header));
  section 8 of every `AGENTS.md` lists the operations; section 4 says what the assistant's first
  words hold; the sentence that names the doctrine's source moves inside the doctrine's block in
  section 3 ([roles](convention/roles.md)).
- **Proof, pass 2**: every batch shown, batch 1 included; the trial of a local type renumbered 31;
  L4 checks what its criterion says; the header criteria get a version 2, from batch 3, where an
  object may serve nothing yet; text kept in LF on every platform (`.gitattributes`).

Spaces written with draft 0.2 keep working: a `space.md` that declares it under the older key,
`standard: 0.2`, declares the version before 0.3. Their `AGENTS.md` is regenerated when they take 0.3.

## 0.2, draft, and before

The rules in normative form, the status axes as state machines, a glossary, a first conformance
section; setup prompt 2.2, run end to end by agents on 2026-11-15. In the history kept by the founder.
