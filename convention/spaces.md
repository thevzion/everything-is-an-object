# Spaces

Convention 0.3 · draft. Explanatory, not normative.

A space is a source with its own authority: a folder or a page, its `space.md`, its objects and their history. A person's home is a space; so is an organization, and a project that has its own why. Spaces link by URI, propose to each other, and each one decides for itself.

## A space, and its links to others

- **Its address.** `space.md` declares the space's URI, `<handle>/<name>`, unique and stable ([structure](structure.md#layout-one-object-one-file-one-page)). An object's URI is the space's URI and the object's path: `mira-bikes/workshop:objects/goals/1.md`.
- **Links by URI.** Inside a space, a link is a path; to another space, it is that space's URI and the path, or a URL. Where that space lives on this machine is kept in `.local/`, never committed.
- **A site** is a link to another space: one file in `objects/sites/`, which says the space's URI and the goal it serves. A child space is the space's own, in `spaces/`; a site is someone else's.
- **Propose, then decide.** To change what another space holds is to propose: a branch or a pull request in its repository, a proposed row in its Notion, an issue. Nothing becomes true there until someone who decides there says yes ([truth](truth.md)). Your own space stays yours: a goal of yours can serve a goal of theirs without asking, because the link is carried by your object.

## Access

Three verbs: **read, propose, decide.** Access is a rule of the organization, not of the tool.

| Layer | What it is | Example |
|---|---|---|
| **the written agreement** | a mandate, or a commitment on both sides | objects of the [Agreements model](../models/agreements/model.md) |
| **its application by the tool** | the rights the adapter enforces | GitHub permissions, Notion sharing |

If the two layers do not match, `org check` flags it as a smell.

**Private by default, shared by pointer.** In a company, aligned goals, deliverables and decisions are shared; the person's home, vision and capital stay with the person. "Your work is shared, not you."

**Others' data stays out** (proposed). What belongs to another person, their health, money, messages or private life, is not written into a space without their consent, even when the person who wants mentions it to explain why. The object keeps what it means for this space, and leaves their details out. This is the same rule as "Your work is shared, not you", seen from the other side.

## An executable life: the danger, and what protects (candidate)

A whole life can now be executable. A leak can then be serious, for a person who relies on it as much as for an organization. An organization already has a knowledge base; this is one step further.

- **Why it is a step further.** A knowledge base keeps pages. A space keeps what a person wants, decides and knows: typed, linked, dated, with the why and the provenance. What makes the data useful also makes it exploitable: the same cleanliness serves whoever steals it.
- **What the cleanliness also allows: precise protection.** A loose base is protected as a block. Here, each object has a type and a state, so a visibility: you can say exactly what is private, shared, projected or erased, and check it.

What already protects:
- private by default, shared by pointer; read, propose, decide; the written agreement and its application, compared by `org check`;
- the Capital model, private, in the person's home;
- `redact`, to erase a secret or personal data ([changes](changes.md#where-the-convention-departs-from-git));
- the [trace](trace.md) keeps addresses, not content; a projection says where it comes from.

Leak paths, to observe:

| Path | What happens | What the structure gives |
|---|---|---|
| **the AI model provider** | everything the agent reads is sent to it | `read` says exactly what, and to whom; learned context reduces what leaves, since only what serves is sent |
| **what comes from outside** | a page read can carry instructions for the agent | the trace marks it as sourced |
| **the adapters** | Notion, GitHub or a clone have their own access | the written agreement must cover them |
| **the projections** | a site or a public export | takes only what is marked public |

**The gates come first here.** The doctrine observes before constraining, except where a mistake cannot be corrected precisely or reaches someone else ([gates](doctrine.md#gates)). A leak is exactly that case. Exposure is measured ([measures](views.md), [trial 19](../tests/trials/19-exposure.md)); the boundary is open ([question](https://github.com/thevzion/source/blob/main/objects/questions/privacy-boundary.md)).

## Federation and identity

- **One authority per space**, no central authority, like email, the web and git. TheVZion is the authority only on the structure. A directory is possible, never an authority over others.
- **The convention is a protocol**: it is to organizations what HTTP is to servers. To interoperate is to speak the same protocol, in a compatible version ([trial 05](../tests/trials/05-interoperability.md)).

Three questions of authority:

| Question | Answer |
|---|---|
| **identity** | the space publishes its `space.md`, and its members' public keys, at its URI |
| **access** | the remote organization decides |
| **agreement** | a commitment, signed and committed on both sides, each citing the other's commit by its fingerprint |

Connecting works like git remotes:

| Command | What it does |
|---|---|
| `org connect` | reads the remote manifest, checks version compatibility (convention and models), adds a site |
| `org fetch` | brings public objects back as a projection, with their freshness |
| `org serve` | aligns a goal with a remote goal, by URI |
| `org propose` | opens a proposal in the remote inbox |
| `org request` | asks for wider access |
| `org verify` | checks signatures; key fingerprints live in `known_orgs`, like `known_hosts` in SSH |

The command name `org` is open ([question](https://github.com/thevzion/source/blob/main/objects/questions/command-name.md)). Reads across spaces are also open ([question](https://github.com/thevzion/source/blob/main/objects/questions/cross-space-reads.md)).

- **No blockchain.** What it offers that is useful here (an unforgeable history, signatures) is already in git and SSH. Its downsides would get in the way: everything is permanent and public, it costs, and its image contradicts "nothing to buy".
- **The repository as notary.** What must be verifiable in Notion (a decision, a commitment), the toolchain projects into git with its fingerprint.

Vocabulary: "local" and "remote" say where your own objects live; a "site" is another organization; "connect" is the gesture that creates a site.
