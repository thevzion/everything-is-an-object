# Adapters

Convention 0.3 · draft. Explanatory, not normative.

Every tool is an adapter. All of them do the same operations:
1. read an object by its URI;
2. write it where its source is;
3. list;
4. give the history;
5. give the last change;
6. apply access.

Each one also says what it cannot do.

| Adapter | What it carries | What it cannot do |
|---|---|---|
| folder + git | objects, history, signatures, offline | fine-grained access control |
| Notion | objects, changes (the inbox included), sharing, phone | sign; keep the why natively; rate limits; 1,000 blocks for a free workspace with several members |
| GitHub | inbox (issues, discussions), access | per-object visibility |
| Discord | inbox | objects |
| Obsidian | a folder, so the same adapter as the first | sharing |

- **Intelligence talks to the organization, not to the tools**: it asks for `thevzion:goals/2`, and the adapter finds it.
- **AI models are adapters too**: the space declares the ones it uses, and the convention names none ([questions](questions.md#intelligence-at-several-sizes-candidate)).
- **The web is one more adapter**; a tag in a page is a projection ([trace](trace.md#beyond-the-organization)).
- Since history is append-only and authority is per type, a remote tool only needs to read the current state, write it, and give the time of the last change ([changes](changes.md#where-the-convention-departs-from-git)).

## The toolchain is optional

The optional tools live in a kit, not published yet: an app and a status bar, to read the views and say yes without the chat; a command line for people; an MCP server for agents; and the Notion adapter. They host the adapters. They add comfort, never a requirement ([frontier](README.md#the-frontier)). See the [toolchain project](https://github.com/thevzion/source/blob/main/objects/projects/toolchain.md).

## Notion, local-first

Intelligence writes local Markdown and commits, then pushes to Notion:
- objects go to the Objects database;
- each commit goes to the Changes database, one row per commit;
- properties flow both ways;
- page bodies flow one way until a round trip is proven;
- local Markdown stays within the common subset.

**Reconcile like git.** Notion is a remote branch:
- the merge base is the last pushed version, kept in git;
- a pull commits Notion's state on `notion/main`, then git merges;
- a conflict stops and asks;
- merge, never rebase.

**What [trial 4](../tests/trials/04-notion-adapter.md) found** (2026-11-15, partly; one operator-run trial):
- held: the space and its two databases, objects and their relations, properties both ways, conflicts (detected, nothing overwritten), one Changes row per commit;
- partial: page bodies, 4 of 7 identical after a round trip; creating a page from Markdown directly through the API was not exercised;
- missing: what the assistant read is not captured, so the Read fields stay empty.

The detailed feasibility notes are not published yet.
