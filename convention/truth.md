# Truth

Convention 0.3 · draft

Each place plays one role for each type of object.

| Role | What it is | Written there? |
|---|---|---|
| **Source** | where the object is written and changed | yes, and only there |
| **Projection** | a copy generated from the source (a site, an export, a cache); it says where it comes from, which version, when | never: a correction goes back to the source |
| **Pointer** | an address, without content | no |
| **Inbox** | where proposals and signals from others arrive; in the space, the changes with status proposed | yes, to propose; nothing becomes true there without a decision |

**Rule**: write at the source, read anywhere, and a proposal becomes true only through a decision.

- A projection can also be the answer of a view, written by a pure agent from linked sources ([views](views.md#the-pure-projection)).
- When a space lives in a folder and in Notion, for each type one side is the Source and the other a Projection ([layout](structure.md#layout-one-object-one-file-one-page)).
- A git clone is a working copy of the source, which git brings back to it: a repository and its clones count as one source.

## Example: TheVZion

| Place | Role |
|---|---|
| [thevzion/source](https://github.com/thevzion/source): TheVZion's space | Source |
| [thevzion/everything-is-an-object](https://github.com/thevzion/everything-is-an-object), this repository: the convention and the models | Source |
| the collective's workspace, for operations (members only) | Source |
| thevzion.com, `llms.txt`, `structure.json`, the public Notion template | Projections |
| `objects/sites/`, the pointer files | Pointers |
| GitHub issues and discussions, Discord | Inbox |
