# Changes

Convention 0.3 · draft. Keywords in capitals are normative ([writing](writing.md)).

The third set of the [structure](structure.md): the events. Temporal, append-only. ([decision](https://github.com/thevzion/source/blob/main/objects/decisions/the-commit-is-the-change.md))

## The commit is the change

The Changes set is the convention's abstraction; each adapter stores it its own way.

| Where | A change is | Journal and seal |
|---|---|---|
| a folder with git | a commit | git is the journal and the seal: the hash chain and signatures prove it was not rewritten |
| Notion | a row of the Changes database | the commit fingerprint, as a property |
| a folder without git | an entry of `changes.md` | an append-only file keeps the journal, without a seal |

A `changes/` folder next to the commits doubles the work, and the two can diverge. There is none.

## Fields

Every field of a change already exists in git, except two, which become trailers (`Why:`, `Source:`). Others are candidates (`Intent:`, `Read:`, `By:`, `On-behalf-of:`, `Lane:`, `Cost:`, `Ratifies:`).

| Field | In git | In Notion (Changes database) |
|---|---|---|
| when | the commit date | the date |
| who | the author; when anything other than the author wrote, the `By:` trailer (candidate) names what acted: the model and its exact version id, the provider, the tool; or the rule, or the machine. `Co-Authored-By` stays compatible | the author |
| which objects | the files of the diff | a relation to pages of the Objects database |
| which operation | the subject: `<object>(<operation>): ...` | the operation |
| why | the `Why:` trailer | the why |
| said or inferred | the `Source: from-me` or `Source: inferred` trailer | the source |
| status | main branch: accepted; a branch or a pull request: proposed; not merged: refused | the status |
| an object's history | `git log --follow -- objects/goals/2.md` | the rows linked to the page |
| the seal | the hash chain, signatures | the commit fingerprint, a property |
| the intent served (candidate) | the `Intent:` trailer | a relation to the Intents database |
| what was read (candidate) | a git note, or a `Read:` trailer: the URIs and the version read | a "Read" relation to the Objects database |
| on whose behalf (candidate) | the `On-behalf-of:` trailer: the mandate, as a URI or a role, never a person's name or address | a relation to the mandate |
| the lane (candidate) | the `Lane:` trailer: `rule`, `decision-model`, `generative-model` or `person` | the lane |
| the cost (candidate) | the `Cost:` trailer: tokens, and time where known | the cost |
| what it ratifies (candidate) | on a person's `from-me` change, the `Ratifies:` trailer: the `inferred` change it confirms | a relation to the confirmed change |

Why these candidates: the measures read them. Tokens per want, the share resolved per lane, the yield of responses and the rate of ratification cannot be computed without them. The version id is exact, because the same model name behaves differently across versions. ([decision](https://github.com/thevzion/source/blob/main/objects/decisions/the-trace-names-what-acted.md))

Open: [commit trailers](https://github.com/thevzion/source/blob/main/objects/questions/commit-trailers.md), [the `Intent:` trailer](https://github.com/thevzion/source/blob/main/objects/questions/intent-trailer.md), [where `read` lives](https://github.com/thevzion/source/blob/main/objects/questions/where-read-lives.md).

## Convention

Every change MUST have an author, a subject in the form below with a known operation, a `Why:` trailer and a `Source:` trailer (`from-me` or `inferred`).

```
<object>(<operation>): <what changes>

Why: <the reason, one line>
Source: from-me | inferred
```

Example:

```
goal(revise): turn goal 2 into an outcome

Why: a deliverable was posing as a goal
Source: inferred
```

**Trailers in one block** (proposed): the trailers form the message's last paragraph, one per line with no blank line between them (`Why:`, `Source:`, then any other, such as `Co-Authored-By:`), so tools can read them (`git interpret-trailers`).

Operations: `add`, `revise`, `remove`, `rename`, `split`, `merge`, `test`, `validate`, `reverse`, `restructure`, `migrate` (see [local types and migrations](structure.md#local-types-and-local-links)), and, candidate, `redact`.

## Accepted by default

- **A change by intelligence is accepted by default**: it is committed, and it MUST be marked `inferred` when it was inferred. Better committed and engaged than nothing, silent, or waiting.
- **The inbox is the set of proposed changes**, like a pull request: what comes from elsewhere, what is large (a migration), and what engages other people. Nothing limits its size; its count and the age of its proposals are measured (waiting, in [measures](views.md)).
- **Proposals from elsewhere** (an issue, a message, a Notion row) stay in their adapter until intelligence processes them: it turns them into a commit, or a branch if they are large.

Where the line between accepted and proposed falls, beyond these three cases, is open ([question](https://github.com/thevzion/source/blob/main/objects/questions/accepted-or-proposed.md)).

## A correction is a change that teaches

A correction MUST be a new change, `reverse` or `revise`, on the wrong object or link and nothing else, with its why. It teaches three parties:

| Who learns | What |
|---|---|
| intelligence | how the person means it |
| the person | how their words were understood |
| the convention | a correction that keeps returning on a type shows an ambiguous model question; everywhere, a line of doctrine to revisit |

## A work lists the decisions it inferred (proposed)

A holder that delivers work MUST list the decisions it inferred while doing it, each with its why, its premises, and whether the one who answers confirmed it. This is a candidate MUST: it has no check yet.

- A work is not only files: it is a set of decisions, inferred or confirmed. A wrong result then goes back to the decision that caused it, and to whether its premises were enough.
- A polish is the correction of one inferred decision: a `revise` or a `reverse` on it, with its why, like any correction.
- Where the list lives is open: in the body of the change that delivers the work, in a file next to the work, or as decision objects that serve the deliverable.
- [Accuracy](views.md#canonical-measures-candidate) reads it at the grain of a decision.

([decision](https://github.com/thevzion/source/blob/main/objects/decisions/a-work-lists-its-inferred-decisions.md))

## Result is not Change

A **result** is a fact of the world ("What happened?"), an object of the [Loop model](../models/loop/model.md). A **change** is a fact about the organization's file.

## Double entry

The Changes are the journal; the Objects are the ledger. The state is the ledger, and it can be rebuilt from the journal.

## Where the convention departs from git

| git | The convention |
|---|---|
| history can be rewritten | history is append-only |
| lines are compared | objects are compared, field by field |
| a message can be empty | every change has an author and a why |
| anyone with access changes anything | one source and one authority per type; elsewhere, it is a proposal |
| a conflict is resolved without a trace | a conflict is settled by a decision, which becomes an object |

A remote tool therefore only needs to read the current state, write it, and give the time of the last change.

History MUST be append-only: an accepted change MUST NOT be rewritten, except by `redact` (candidate, below).

**The exception: `redact`** (candidate). It erases the past for two cases only: a secret committed by mistake, or personal data. The content is purged; the event stays ("redacted, why"). Always by decision, rare, and subject to authorization. It is the condition of the right to erasure.

## Without git

Where git cannot be installed, `changes.md` at the space root holds the journal. It MUST be append-only, with one entry per change and the same fields as a commit (date, author, subject line, `Why:`, `Source:`). It has no seal.
