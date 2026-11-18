# Start

Three doors, one convention: everything is an object. Each ends in the same thing: a space, written as typed objects, where every change keeps its reason.

| Door | When | What to do |
|---|---|---|
| A folder, with an agent | you work with an agent that can read and write files | copy [prompt.md](prompt.md) and give it to the agent |
| Notion | you live in Notion | duplicate the template: not public yet ([public-template-and-counting](https://github.com/thevzion/source/blob/main/objects/questions/public-template-and-counting.md)) |
| Just talk | you want to talk it through first | give any chat agent the instruction below, and say you just want to talk. It asks the questions of the seven starting types together, in one message, keeps your words, and gives you the result as one file. |

## The one instruction

For any agent that can read a web page:

```
Read https://raw.githubusercontent.com/thevzion/convention/main/setup/prompt.md and do what it says below the line: set up my space in this folder.
```

`llms.txt` is meant to become a projection of [prompt.md](prompt.md): generated from it, never edited by hand. Until then the two can differ, and `prompt.md` is the source.

## Required, default, optional

| | What | Why |
|---|---|---|
| Required | the [convention](../convention/README.md), one model ([the Organization model](../models/organization/model.md) by default), the prompt and `llms.txt`, and a folder or a Notion page | it is text that any agent can follow |
| Default | git, installed once with your consent; where it cannot be, an append-only `changes.md`, without a seal | the agent carries it entirely, it is open and universal, and it works without an account or a network |
| Optional | the toolchain (a command line, an MCP server, sync, checks) and the plugins of a client; an MCP server comes before a plugin | comfort |

Status: version 3.1 of the prompt, draft, for Convention 0.3, proposed: it adds line 9 of the doctrine and keeps what is made as an object, and has no run yet. Version 3.0: its runs are counted in [the proof](../README.md#proof). Before it, version 2.2, candidate: on 2026-11-15, ten newcomer runs went end to end: agents (Claude Opus and Claude Sonnet) given only the instruction above, each setting up the home of a person who wants to run 10 km, over two or three turns. The first seven also carried, by a fault of the test harness, the operator's own global instruction file. The eighth ran without it, as a probe checked, with the nine sections of [`AGENTS.md`](../convention/roles.md), and gave the same result: its questions in one message; `space.md`, a problem, a goal, and a why in the person's own words; the nine sections, with the norm texts word for word; and commits whose trailers git reads. The last two, also clean, tested step 7: the tool's own instruction file pointing to `AGENTS.md`, which a new session then read; and the `commit-msg` hook, which Claude Code would only write with the person's permission, and which then refused a malformed message. The criteria were fixed before the runs; the transcripts and the spaces they wrote are kept outside this repository. Not yet tested by a person ([trial 7](../tests/trials/07-reduced-tree.md)). The live `llms.txt` is built from the `main` branch on GitHub: it serves version 2.1 until this version is pushed there and the site is deployed.
