# AGENTS.md

A projection of the doctrine ([convention/doctrine.md](convention/doctrine.md)) and of this
repository's mandates. When they differ, the source wins: correct it there.

## Read first

1. [convention/README.md](convention/README.md): what the convention is, its levels, words and index.
2. [convention/primitives.md](convention/primitives.md): the twelve words everything is made of.
3. [convention/doctrine.md](convention/doctrine.md): how any intelligence behaves here.
4. [convention/roles.md](convention/roles.md): the roles, and what every `AGENTS.md` holds.
5. The model you change: `models/<id>/model.md`, after [models/README.md](models/README.md).

## The doctrine

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

## Mandates in this repository

| Working for | You do, then say what you wrote | You propose (a branch or a pull request) | You never |
|---|---|---|---|
| a maintainer | revise `tests/` and `setup/`, fix links and wording that change no rule, marked `Source: inferred` | changes to `convention/` or `models/` (every space conforms to them), migrations, anything that comes from elsewhere | decide what the convention says; push, publish or post |
| a contributor | | everything: your fork, then a pull request or an issue | the same |

Decisions about the convention are TheVZion's. They are recorded in its source
([objects/decisions/](https://github.com/thevzion/source/tree/main/objects/decisions)), then
land here as a commit.

The proof is generated, never written by hand: `tests/runs/` holds one object per run, written
by the runner in `tests/runner/`, and the proof table of `README.md` is regenerated from them
(`convention-trials table`). A trial's criteria are fixed before its first run and not changed
after; a run that should not count is marked with its reason, never deleted.

This repository is public. Exposure is where the gates come first: no personal data, no
private names, no links to private workspaces, in files, commits or issues.

## Writing

- Links inside this repository are relative paths. A link to TheVZion's source is a full URL
  (`https://github.com/thevzion/source/...`).
- Revise before you add: change a file or add a link before adding a model or a type.
- One change per commit, where `<object>` names the part that changes (`doctrine`, `primitives`,
  `roles`, `views`, a model id, `trial`, `run`, `runner`, `prompt`, `readme`):

```
<object>(<operation>): <what changes>

Why: <the reason, one line>
Source: from-me | inferred
Co-Authored-By: <the agent that wrote it, if any>
```

- Plain English, short. No em dashes.
