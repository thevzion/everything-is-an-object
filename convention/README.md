# The convention

Convention 0.3 · draft

Everything is an object. The convention is one way to write that down: the doctrine and the structure, the same for everything. Models plug into it. Spaces are its instances. Its words are in [primitives](primitives.md): eight, from which everything else is made.

`0.1` and `0.2` were drafts, kept in the history before this public one; `0.2` put the rules in normative form ([writing](writing.md)), the status axes as state machines, a [glossary](glossary.md) and a first [conformance](../tests/README.md#conformance) section. `0.3`, this draft, adds the [primitives](primitives.md), the fixed core of [roles](roles.md), the default [views](views.md), and what the first runs showed; the [doctrine](doctrine.md) gains a ninth line, proposed. The [trials](../tests/README.md) decide what holds; their runs are counted in the [README](../README.md#proof).

## An organization, in one sentence

An organization is an intent made explicit enough that others, people or intelligences, can act on it, and kept alive by what they learn.

The convention writes it down without depending on any tool: tools live in the organization, not the other way round.

## The simple idea

**Your intent is the source. Intelligence runs it.**

| Software | Here |
|---|---|
| source code | what you want, decide and know, made explicit: the objects |
| the language and its types | the convention and its models: types, each one a question |
| compiling | intelligence files what you say into typed objects; checks do the type checking |
| the processor | intelligence, from any provider |
| electricity | compute: tokens |
| the running program | the work done, and what comes back from it |
| tests, assertions | checks (a question and its expected answer) and trials |
| a source map | the trace: what each object draws on |
| default behavior when the source says nothing | the generic |
| a view, a materialized view | a projection: a question asked of the sources, refreshed when they change |
| a pure function | a pure agent: it sees only its sources |
| `make` | regenerating the projections whose sources changed |
| a service, a scheduled job | a routine: a question asked alone, on a trigger, under a mandate |
| git | git |

Where the comparison stops:
- execution need not be deterministic, by choice: compute when it is enough, intelligence when something must be understood ([questions](questions.md));
- the executor fills gaps in the source, which a compiler never does: that is the inferred, and it is marked;
- the person writes the source and is also part of the execution: they decide, and they answer for it;
- the source is never finished: the loop rewrites it with what each turn teaches.

Why now: each time the executor grew more capable, the source moved closer to people (assembly, then higher-level languages, then declarative code such as SQL). Intelligence understands intent, so the source can be the intent itself: what you want, and why. Most organizations do not have this source. It lives in heads, chats and meetings.

## Levels

| Level | What it is | Contents |
|---|---|---|
| **0. Doctrine** (convention) | how intelligence behaves | make explicit, discern, link, commit rather than wait, mark the inferred, correct precisely |
| **1. Structure** (convention) | independent of any domain | objects, links, changes; URI, truth, access; adapters |
| **2. Models** | a pluggable, versioned library of types, each with its question | the library; Organization and Home, which include others; Game and seeds ([models](../models/README.md)) |
| **3. Spaces** | real spaces, each its own authority, each with its models | TheVZion, a home, a company |

- Content depends on the convention and the models, never the reverse.
- Organization is the first model and the default door, not the foundation. The center stays the person.
- A space is an instance of the convention and of the models it declares, as a database is an instance of a schema.

## The frontier

| | What | Why |
|---|---|---|
| **Required** | text any agent can follow: the convention, one model (Organization by default), the setup [prompt](../setup/prompt.md) and `llms.txt`, a folder or a Notion page | nothing else is needed to start |
| **Default** | git | it meets three conditions: the agent carries it fully, it is open and universal, it works without an account or a network. Installed once, with the person's consent. Where it cannot be installed, an append-only `changes.md` keeps the journal, without a seal |
| **Optional** | the toolchain (CLI, MCP server, sync, checks), a client's plugins | comfort. The MCP server comes before the plugin |

Three doors, one convention:
- an agent in a folder: copy the prompt;
- living in Notion: duplicate the template;
- to just talk: "Just talk".

## Files

| File | What it holds |
|---|---|
| [primitives.md](primitives.md) | the eight primitives, each with its visible effect; their compositions; what an object is |
| [writing.md](writing.md) | how to read the convention: normative keywords (RFC 2119, RFC 8174), normative and explanatory text |
| [doctrine.md](doctrine.md) | level 0: the doctrine and its notes, the invariant, the gates |
| [structure.md](structure.md) | level 1: objects, links, changes; the meta-model; layout; header; recursion |
| [changes.md](changes.md) | the commit is the change: fields, operations, inbox, corrections |
| [trace.md](trace.md) | `read` and `draws on`: what an object comes from |
| [truth.md](truth.md) | the four places: Source, Projection, Pointer, Inbox |
| [spaces.md](spaces.md) | the space, its URI, links between spaces, propose and decide; read, propose, decide; exposure; federation and identity |
| [questions.md](questions.md) | everything is a question; the loop; routines; intelligence at several sizes |
| [views.md](views.md) | the seven default views, the pure projection, the dashboard, canonical measures, anchoring, balance and yield |
| [adapters.md](adapters.md) | tools as adapters; Notion local-first |
| [roles.md](roles.md) | the units, the person and the fixed core (assistant, materializer, maker, checker), the roles that emerge; what every `AGENTS.md` holds, and its ten sections |
| [glossary.md](glossary.md) | what an object is; each word of the convention, and the established term closest to it |

## Words

| Word | Meaning |
|---|---|
| **space** | the unit: a folder or a page with its `space.md`, its objects and their history |
| **convention** | the doctrine and the structure, the same for every space |
| **model** | a versioned set of types, with their views and checks (for example, the Goals model) |
| **type** | a question, a horizon, fields; defined by a model, or locally by a space |
| **object** | anything made explicit that answers one question, with its header: what was said, decided or made; one file, one folder or one page, one URI ([glossary](glossary.md#an-object)) |
| **link** | a relation of a given kind between two objects (`serves`, answers, replaces, `draws on`, `delivers`) |
| **change** | an event on objects and links, with when, who, why and status; a commit |
| **trace** | what a change read (`read`) and what an object draws on (`draws on`) |
| **projection** | a copy or an answer generated from sources; regenerated, never edited |
| **site** | a link to another space |
| **organization of one** | an organization whose intent is that of a single human |
| **home** | a person's own space: the root of their tree, and the space of their organization of one |
| **intelligence** | the capacity to understand and to produce: what writes, links, marks and responds, whatever runs it |
| **holder** | what holds a role and acts in it: a person, a model, a rule, a machine ([roles](roles.md)) |
| **agent** | only in its technical sense (an AI agent working in a folder, an agent over MCP) and in the W3C PROV mapping, where an agent includes people (decision in TheVZion's source, not published yet) |

## Prior art

No piece is new: the journal and the ledger come from Pacioli, history from git, events from event sourcing, URIs and federation from the web, objectives from OKRs, `AGENTS.md` from coding agents. The assembly is new, applied to what nobody writes yet: what we want, why, and how we will know, at every size.

Why it may work comes from older ideas too. Builders of agents call it context engineering: what a model does depends on what is put before it, and a long, noisy context degrades it. Long before agents, knowledge management called it making tacit knowledge explicit; a Zettelkasten keeps one idea per note; and termites, then the blackboard systems of early AI, coordinate many workers through what they leave in a shared place rather than through messages. Closest, and recent: the [LLM Wiki](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f) pattern (April 2026) keeps raw sources a model reads but does not change, a wiki the model writes, and a schema file that says how. The convention adds what gives every space the same shape: each object answers one fixed question and carries its header, what a person said keeps their words, and every change keeps its reason.

What would show it: the same facts given to two agents, one as objects and one as the conversation they came from, comparing what each answers and what each spends. Not tested yet.
