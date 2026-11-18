# Structure

Convention 0.3 · draft. Keywords in capitals are normative ([writing](writing.md)).

Level 1 of the convention: independent of any domain. Four words, on three sets.

| Word | What it is |
|---|---|
| **Object** | an explicit thing, of one type, that answers a question |
| **URI** | its stable address |
| **Truth** | where it lives: local or remote, once, with a role ([truth](truth.md)) |
| **Access** | who reads it, who proposes, who decides ([access](spaces.md)) |

A space is the unit: a folder or a page, with its `space.md`. An organization is a space that follows the Organization model, the most common case. Structure rules written for the organization hold for any space.

## The closest parallel: the web

| Web | Here |
|---|---|
| a resource | an object |
| a URI | a URI |
| a representation (HTML, JSON) | a projection (a file, a Notion page, a site) |
| each server is the authority on its resources | each organization is the authority on its objects |
| links | links: `serves`, replaces, sites |
| a shared vocabulary (schema.org) | a model |
| GET, POST | read, propose, decide |
| the browser | intelligence |
| the W3C | TheVZion, which holds the protocol, not the network |

What it adds to the web: **meaning** (types that each ask a question), **time** (a loop, and changes that carry a why) and **the human** (who wants, who answers, and what stays with the person).

## Three sets

| Set | What it is | Rule |
|---|---|---|
| **Objects** | the state: what is true now | a URI MUST identify exactly one object |
| **Links** | relations between objects: `serves` (what justifies it), answers, replaces, points to another space | a link MUST be carried by the object that declares it |
| **Changes** | the events: what happened to objects | temporal and append-only; each change MUST say when, who, which objects, which operation, why, and its status: proposed, accepted, refused |

- State MUST be derivable from accepted changes: replaying the history gives the current state.
- A change MUST NOT be rewritten. A correction MUST be another change, which reverses or revises it, with its why.
- Measures are computed from objects and changes; the doctrine tells intelligence how to write them.

Changes in detail: [changes.md](changes.md).

## The meta-model: six kinds

Everything else is an instance of one of them: Organization and Game are models, TheVZion is a space, "goal 2" is an object.

| Kind | What it is | Its relations |
|---|---|---|
| **Space** | `space.md`, a URI, an authority, its members and its adapters | declares models with their version; contains child spaces; points to other spaces (sites); holds objects; defines its local types |
| **Model** | a set of types, with their views, checks and a version | includes other models; defines types |
| **Type** | a question, a horizon, fields | defined by a model, or locally by a space |
| **Object** | an explicit thing with a URI; a file or a page | typed by a type; carries its provenance (model and version, or local; said or inferred) |
| **Link** | a relation of a given kind: `serves`, answers, replaces, `delivers`, contradicts, `draws on` (candidate), or a local kind | joins two objects, including across spaces |
| **Change** | an event: when, who, why, source, status; a commit or a row of the Changes database | touches objects and links; serves an intent; read objects at a version (`read`, candidate) |

**Organization and Game are models, not levels.** The Organization model includes others; the Game model describes the thing made. Neither owns the other: a game can be designed without an organization, and an organization can run without a game.

- **Compose rather than own.** A model includes others, as a package depends on packages, and a space declares the ones it takes. If a domain always wants the pair, a Studio model can include Organization and Game.
- **Links are the glue.** In a game's space, a mechanic (Game) serves a goal (Goals), a playtest (Game) produces a result (Loop), and a decision (Loop) revises the mechanic.
- **Game includes Loop** (candidate): a playtest is an experiment, and its results are Loop results. A model included twice, by Organization and by Game, is installed once ([question](https://github.com/thevzion/source/blob/main/objects/questions/model-included-twice.md)).
- **No inheritance between types**, to start. A type does not extend another; models include each other, and objects link to each other. A measure may later show it is needed ([question](https://github.com/thevzion/source/blob/main/objects/questions/type-inheritance.md)).

## Local types and local links

When no installed type fits, intelligence creates a **local type**, named by its question, without the person naming it. Links follow the same rule: existing kinds first (`serves`, answers, replaces), otherwise a local kind with its meaning (depends on, blocks, contradicts). "Contradicts" is precious: explicit content makes contradictions detectable.

Placement order:
1. revise an existing object, or add a link to it;
2. a type from the installed models;
3. a local type already created;
4. a new local type;
5. if the choice changes what the person wants, one question.

- **A local type is an object**: `objects/types/supplier.md` carries its question, horizon and fields, versioned like the rest.
- **Restraint**: a local type MUST ask a question no installed type asks. Otherwise it is a field, a link or a view. The aim is reuse and clarity, not the number of objects.
- **Desire paths**: let people walk, then pave the paths they made. Local types emerge within a space; the same local type often returns across spaces: it becomes a seed, then a model. Only type definitions are shared, on agreement, never objects.
- **Migrations**: a large change (`migrate`), proposed by intelligence and accepted by the person, for example from `local:supplier` to `work@0.2:vendor`, twelve objects, and the why. History stays whole. Upgrading an installed model follows the same path. A migration can split or merge, and rewrites internal links in the same change. The old URI must stay resolvable ([question](https://github.com/thevzion/source/blob/main/objects/questions/uri-on-type-change.md)).

Detail on models, seeds and local types: [models](../models/README.md), [seeds](../models/seeds.md), [where local types live](https://github.com/thevzion/source/blob/main/objects/questions/where-local-types-live.md).

## Layout: one object, one file, one page

What can change status or be cited alone has its own file and its own page.

| Nature | Folder | Notion |
|---|---|---|
| a single object (the vision) | a file: `vision.md` | a page, a row of type Vision |
| a set (the projects) | a folder, one file per item | a view of the Objects database, one page per item |
| a made thing with files of its own (a page, code, an image) (proposed) | a folder in its set, `objects/deliverables/<name>/`: its header in `README.md`, its files beside it | a page of type Deliverable, its files attached or linked |
| a child space | a folder with the same tree, in `spaces/` | a child page with the same shape |

The reduced tree: a manifest, the two sets, and nothing else ([trial 07](../tests/trials/07-reduced-tree.md)).

```
<space>/
├─ space.md              the manifest: URI, convention version, models, truth, access, adapters
├─ objects/              the state, one file per object, typed by its model
│  ├─ problem.md  why.md  vision.md ...
│  ├─ goals/  projects/  tasks/  decisions/ ...
│  ├─ deliverables/     what was made: a file, or a folder with its header and its files
│  ├─ sites/             links to other spaces
│  └─ types/             local types, each with its question
├─ spaces/               child spaces, same tree
├─ AGENTS.md             a projection of the doctrine and the mandates
├─ .git/                 the changes: one commit per change, sealed
└─ .local/               machine-specific, never committed
```

```
<Name>                   the page is the space (URI, convention, models); its URL is the URI
├ Agents                 a projection
├ Objects                a database: one page per object; one view per type, a Sites view
├ Changes                a database: one row per commit; the "Proposed" view is the inbox
└ <Child space>          same tree
```

- `space.md` maps to the space page; an object file to a page of the Objects database (Type, Question, Serves, Source, Lifecycle, Evidence, Date); a commit to a row of the Changes database. For each type, one side is the Source and the other a Projection.
- The URI is derived from the path, or is the Notion page URL. The folder is the index; nothing is kept by hand.
- `space.md` MUST declare the root URI, the convention version, the models and their versions, truth, access and adapters. It declares the convention version as `convention: <version>`; a `space.md` written before 0.3 says `standard: <version>`, which declares the same thing.
- The local path of a URI MUST live in a machine-specific map, and MUST NOT be committed.

## The object header

Every object file MUST start with a header that carries `type`, `model`, `question`, `source`, `lifecycle`, `evidence` and `date`, and `serves` when it serves something: a root (`problem`, `why`) serves nothing, and another object MAY serve nothing in the space yet, until it does; the measure gaps and tensions counts it ([views](views.md#canonical-measures-candidate)) (proposed). `question` MUST be the type's question, word for word. `source` MUST be `from-me` or `inferred`. `draws-on` MAY be present (candidate). A link MUST carry its provenance.

```yaml
---
type: hypothesis                 # singular type id, lowercase
model: loop@0.1                  # the model and version that define the type, or `local`
question: What do we think is true, and how will we know?   # the type's question, verbatim
serves: objects/goals/1.md       # what justifies it; a list if several
draws-on: [convention/views.md] # optional (candidate): what it rests on
source: inferred                 # from-me | inferred
lifecycle: active                # proposed | active | review | done | dropped
evidence: hypothesis             # by-construction | sourced | observed | hypothesis | open; proposed: common-ground | contested
date: 2026-11-12
---
```

An adapter MAY add its sync identifiers.

## Two status axes

| Axis | Values |
|---|---|
| **lifecycle** | `proposed`, `active`, `review`, `done`, `dropped` |
| **evidence** | `by-construction`, `sourced`, `observed` (dated), `hypothesis`, `open`; proposed: `common-ground`, `contested` |

`lifecycle` MUST take one of the lifecycle values, and `evidence` one of the evidence values. An object in `observed` MUST carry a date.

- `common-ground` (proposed): accepted without a source, because its readers already share it. Common ground depends on the readers: what every architect shares, not everyone does.
- `contested` (proposed): an observation or a source contradicts the claim.

### The axes as state machines (proposed)

Each axis is a finite state machine: its states are the values above; a transition moves an object from one state to another, under a guard. A transition is a change ([changes](changes.md)): it carries its why and its source.

**Lifecycle**

| From | To | Guard | Who may perform it |
|---|---|---|---|
| `proposed` | `active` | the one who answers for the object decides | the one who answers (the decide verb, [access](spaces.md)) |
| `active` | `review` | a result or a change is submitted | anyone who may propose |
| `review` | `done` | the one who answers accepts it, and the checks that apply pass | the one who answers |
| `review` | `active` | changes are requested | the one who answers |
| any state | `dropped` | the one who answers decides | the one who answers |

- A holder MAY perform any transition. A transition a holder performs on an object it does not answer for MUST be marked `inferred`, and the object stays `inferred` until the one who answers confirms it ([changes](changes.md#accepted-by-default)). Validation is the provenance mark, not the lifecycle. (decision in TheVZion's source, not published yet)
- A holder MUST NOT treat silence as ratification: until the one who answers makes a `from-me` change that confirms it, an inferred object stays `inferred`. (proposed) (decision in TheVZion's source, not published yet)
- Invariant: the state tells the truth. At every moment, a reader MUST be able to tell what was said by the one who answers from what was inferred; the state never says more than what happened.
- Invariant: an object in `done` or `dropped` has its transitions in the changes, each with its why.
- Open: who "the one who answers" is in a shared space. Proposed: whoever holds the decide capability for the object, derived from mandates, by default whoever the space's access table names for decide.

**Evidence**

| From | To | Guard | Who may perform it |
|---|---|---|---|
| `open` | `hypothesis` | the claim states how we will know | anyone who may propose |
| `hypothesis` | `observed` | a dated observation is recorded, as a result ([Loop model](../models/loop/model.md)) | anyone who may propose |
| `hypothesis` | `sourced` | an external source is cited, as a Pointer ([truth](truth.md)) | anyone who may propose |
| `hypothesis` | `contested` (proposed) | an observation or a source contradicts it, recorded as a result or cited as a Pointer | anyone who may propose |

- `by-construction` is an initial state, for objects true by definition (a decision, a type). It has no transition into it.
- `common-ground` (proposed) is an initial state too, for a claim its readers already share. It has no transition into it.
- Other transitions are open.
- Invariant: an object in `observed` carries the date of its observation.

### When an inference is enough (proposed)

An inferred claim that reaches others MUST be ratified, if it is a want, or sourced, if it is a fact, unless it is common ground for its readers.

- A want is ratified by the one who answers for it, with a `from-me` change. No other holder can ratify it.
- A fact is verified against the world. Any holder can do it: a checker finds the source and attaches it as a Pointer, marks the claim `common-ground`, or flags it as unsourced. A checker never ratifies a want.
- Inference is enough for what stays in the space and can be corrected precisely ([doctrine](doctrine.md), rule 7 and the gates).
- Inferred is not a fault: it is the output of an inference from the premises in its trace (`draws on`). It is generic when no premise comes from the space, and generic is a fault only where the question was the person's.

(decision in TheVZion's source, not published yet)

### The response (proposed)

An object MAY carry a response: what an intelligence answers to it, so that it moves ([doctrine](doctrine.md), rule 8). A response is one of four kinds: a proposal, a question, an objection, or a next step.

- A response MUST be marked `inferred` and MUST name its kind.
- A response MUST NOT change the object it answers. Acting on it is a separate change: the one who answers ratifies, revises or drops it.
- It lives as a section headed `## Response` in the object's body, or as a separate object that `serves` it.

(decision in TheVZion's source, not published yet)

## Recursion

- **There is always a center: a person**, where someone wants and answers. Their home is their life: an organization of one, the root of their tree.
- **A project becomes a space** when it has its own why and goals, measured differently. It keeps a `serves` link to its parent, and often takes one more model (the Game model for a game). The tree is the chain of `serves`.
- **Containing is not pointing.** What is yours is a child (a folder in `spaces/`, a child page). What is shared is a site: a pointer to a space with its own centers. The graph is a tree only when seen from one person.
- **Same shape at any size**: a person alone, without a legal form; a team or an association; a company and the organizations of its employees. "Same structure, any size. And they connect."

| Seen from | TheVZion is |
|---|---|
| its own `space.md` | the root of its space, with the Organization model |
| the home of a member | a site: a shared organization that serves one of their goals |

### Organization of one

An organization of one is an organization whose intent is that of a single human. The term stays in English, like "organization". ([decision](https://github.com/thevzion/source/blob/main/objects/decisions/organization-of-one.md))

- **"One" counts those who want, not those who work.** An organization of one is not alone: intelligence acts in it as much as needed, but one human wants and answers for it. The [invariant](doctrine.md#the-invariant) defines it.
- **"Human" rather than "person"**: a company is also a person, in law. "Human" removes the ambiguity.
- **You become one**: a person becomes an organization of one when their intent is explicit enough to be acted on. "Everyone can be an organization now", not "everyone is".

The organization of one is the person as an organization; their home is its space (`space.md`, `objects/`, the history).

"One human who wants. As many agents as it takes."

"Company of One" (Paul Jarvis, 2019) and the one-person billion-dollar company talk about the size and revenue of a business. This is not a business: it is a structure, that of what one human wants, and it connects to others.

### The network, by wills

An organization is made of organizations of one, plugged into it by `serves` links: a person's goal serves a goal of the organization. Each will passes through the system and can feed a larger one. Nothing is taken from the person; what is shared is shared by pointer ("Your work is shared, not you."). Each person has more reach alone, so an organization whose members are organizations of one gains as much. Organizations link to each other (sites), and organizations of one link directly to each other too. The same structure, at every size, up to the scale of the planet.
