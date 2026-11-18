# Glossary

Convention 0.3 · draft. Explanatory, not normative.

Each word of the convention, and the established term closest to it. The established term helps an implementer; where the two differ, the convention's definition wins.

## An object

An object is anything made explicit that answers one question: what someone said, decided or wants, and what was made for it, a plan, a page, code. A few header lines say which question it answers, what it serves, who said or made it, and where it stands. What a person said keeps their words. Everything else is a link, a view or a change.

The established term nearest to it: a record, or a row, whose schema is a question; in the web's words, a resource with a URI. What it adds: the question its type asks, the words of the one who said it, and its provenance. The eight words it is made with are in [primitives](primitives.md).

## The words

| Here | Established term | Where |
|---|---|---|
| an object | a record whose schema is a question; a resource, on the web | [above](#an-object), [structure](structure.md#the-object-header) |
| the person | the principal, in agency: the one an agent acts for, who wants and answers for it; not a role | [primitives](primitives.md), [roles](roles.md#the-person-and-a-fixed-core) |
| a link | a typed relation; an edge of a graph; a link relation (`rel`), on the web | [primitives](primitives.md), [structure](structure.md) |
| a question | a schema, stated as the one question its records answer | [primitives](primitives.md), [structure](structure.md#the-object-header) |
| a space | a repository or a workspace: a source with its own authority, and a namespace, its URI | [spaces](spaces.md) |
| a role | a role, as in role-based access control: a function that a person or an intelligence can hold | [roles](roles.md) |
| a primitive | a primitive: a word no other word of the list can make | [primitives](primitives.md) |
| a signal | an event not yet processed; a message in a queue | [primitives](primitives.md), [truth](truth.md) |
| a binding | a role assignment on a resource; a subscription | [primitives](primitives.md), [roles](roles.md) |
| a trigger | an event trigger, or a schedule (cron) | [questions](questions.md#routines-the-loop-that-runs-alone-candidate) |
| an action, a brief | a delegated task with its acceptance criteria | [roles](roles.md) |
| a view | a read model (CQRS); a database view | [views](views.md) |
| an app | a client: it reads views and sends commands (CQRS) | [primitives](primitives.md#compositions) |
| Decisions, the view | a review queue, an approvals inbox | [views](views.md#the-default-views-proposed) |
| the materializer (proposed) | a parser that writes; it turns what was said into records. It was called the note-taker | [roles](roles.md) |
| the checker (proposed) | a reviewer, a verifier, a fact-checker: it checks a result, or a claim about the world against a source, and never ratifies a want. It absorbs what was called the verifier | [roles](roles.md), [structure](structure.md#when-an-inference-is-enough-proposed) |
| answer first, then write (proposed) | eventual consistency: the answer is right for what is known now, and the source catches up; asynchronous writes | [roles](roles.md#section-4-answer-and-propose) |
| a role that emerges, promotion | desire paths, paved; a local type promoted to a model | [roles](roles.md#roles-that-emerge), [models](../models/README.md#local-types-and-links) |
| a convention | a convention, as in "convention over configuration": an agreed way of doing, adopted by use rather than imposed | [README](README.md) |
| a want, with how we'll know | a desired state, with acceptance criteria | [structure](structure.md), the Goals model |
| a result | an observed state | the [Loop model](../models/loop/model.md) |
| the loop | a reconciliation loop: desired state, observed state, and the work that closes the gap | [questions](questions.md) |
| lifecycle, evidence | finite state machines: states, transitions, guards | [structure](structure.md#the-axes-as-state-machines-proposed) |
| a change, the history | a commit, an event; the history is an event log, append-only (event sourcing) | [primitives](primitives.md), [changes](changes.md) |
| a projection | a read model (CQRS) | [truth](truth.md), [questions](questions.md) |
| propose, decide | commands (CQRS) | [spaces](spaces.md) |
| `source: from-me`, `source: inferred` | attribution, in W3C PROV: to a person, or to a software agent | [structure](structure.md#the-object-header), [trial 17](../tests/trials/17-projection-to-prov.md) |
| the trace (`read`, `draws on`) | provenance, in W3C PROV: what an activity used, what an entity was derived from | [trace](trace.md) |
| read, propose, decide | capabilities | [spaces](spaces.md) |
| a mandate | delegated authority | [spaces](spaces.md), the Agreements model |
| handing a question over | escalation, under a confidence threshold | [tests](../tests/README.md#test-the-tester) |
| a URI, a pointer | a URI (RFC 3986) | [structure](structure.md), [truth](truth.md) |
| a site | a federated link to another authority | [spaces](spaces.md#federation-and-identity) |
| checks and trials | conformance tests | [tests](../tests/README.md#conformance) |
| a MUST | a normative requirement (RFC 2119, RFC 8174) | [writing](writing.md) |
| a response | the second part of an adjacency pair, in conversation analysis | [structure](structure.md#the-response-proposed) |
| a turn: said, responded, ratified | a turn at talk; a closed feedback loop | [doctrine](doctrine.md) |
| a holder | a role occupant, an assignee: what holds a role, a person or an intelligence | [README](README.md#words), [roles](roles.md) |
| the `By:` trailer | the software agent an activity was associated with, in W3C PROV (`wasAssociatedWith`) | [changes](changes.md#fields), [trace](trace.md) |
| the `On-behalf-of:` trailer | delegation, in W3C PROV (`actedOnBehalfOf`) | [changes](changes.md#fields), [spaces](spaces.md) |
| inferred (proposed) | the output of an inference from the premises in its trace: a conclusion, in logic; the output of running a trained model, in machine learning. Generic when no premise comes from the space | [structure](structure.md#when-an-inference-is-enough-proposed) |
| a premise (proposed) | a premise, in logic: what an inference starts from (the person's words, a space object, a source); in the trace, a `draws on` link, `wasDerivedFrom` in W3C PROV | [trace](trace.md) |
| common ground (proposed) | common ground, in pragmatics: what the readers already share, accepted without a source; it depends on the readers | [structure](structure.md#two-status-axes) |
| a projection's stamp (proposed) | the version a read model was built from: the commit of the source it comes from | [truth](truth.md), [views](views.md#the-dashboard) |
