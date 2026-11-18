---
trial: 4
status: partly
last-run: 2026-11-15
level: end-to-end
tests: []
---

# Trial 4: the Notion adapter

This trial runs N1 to N7 from the unpublished feasibility study dated 2026-11-12. The criteria below were recovered before execution. Two questions frame them.

## Questions

1. Does Notion hold a space as the convention lays it out? The page is the space; an Objects database holds one page per object (Type, Question, Serves, Source, Lifecycle, Evidence, Date); a Changes database holds one row per change, and its "Proposed" view is the inbox; a child space is a child page of the same shape.
2. Where the Notion adapter cannot see the agent's tool calls, what is missing from `read`?

## Setting

A private, fictional bakery test space, with the Organization model 0.1
([organization](../../models/organization/model.md)) and the local-first adapter
described in [adapters](../../convention/adapters.md). The source is a disposable
local Git repository; Notion is its projection and property-edit inbox.

Why: "Good bread for the street, every morning." Wants: "my bakery to throw no
bread away." and "every customer answered the same day." No real people or
personal data are fixture content. Seven core types are pushed first; a second
goal is then added for the second want. Inferred content is marked as such.

Run source: draft 0.2 at `62edcd41a72879fc04ccafcb109ae35ce0b56514`, a commit of the
history kept by the founder (branch `0.2-normative`); setup prompt 2.1, written for draft 0.1.
This run follows the checked-out 0.2 draft where they differ. Codex uses its
existing Notion connector; no packaged sync CLI is assumed.

## Fixed before the trial

Source: *Notion comme adapter : faisabilite concrete*, 2026-11-12, section 5.
The study was found locally; Notion search was also performed. Source SHA-256:
`c3fcac62263853126ee80d2274df4072a8a7af38a08444825811c8b92c2b87bd`.
Its private location and workspace links are withheld from this public repository.
The original criteria are copied verbatim below, not defined by this run.

| # | Essai | Reussi si |
|---|---|---|
| N1 | créer la page d'entrée, la base Objects et la base Log | les trois existent, la base Objects a les champs Type, Serves, Source, Lifecycle, Evidence, Date |
| N2 | pousser les sept objets du cœur minimal d'un dossier local | sept pages, propriétés justes, corps lisible |
| N3 | aller-retour : relire chaque page en Markdown et comparer au fichier local | empreintes identiques une fois normalisées, sinon la liste précise des écarts |
| N4 | modifier un statut dans Notion, puis pull | la modification arrive comme proposition, puis un commit local avec son pourquoi |
| N5 | modifier le même objet des deux côtés, puis synchroniser | le conflit est détecté, rien n'est écrasé |
| N6 | projeter cinq commits dans la base Log | cinq lignes, chacune avec empreinte, message, pourquoi et objets touchés |
| N7 | créer une page avec `markdown` dans une data source | confirme ou infirme le point **à vérifier** |

The current convention calls Log **Changes**. This run uses that name without
changing N1 or N6. N7 is attempted through the available connector; its result
will distinguish connector Markdown support from direct REST endpoint evidence.

Additional checks fixed by this run before any fixture creation:

- Objects also has Question and Model; Serves is a self-relation. Changes has
  fingerprint, message, Why, Source, date, author, operation, status and Objects
  relation. A Proposed view exposes incoming changes. All commits, beyond N6's
  first five, must be represented, including unmerged incoming proposals.
- Bodies go local to Notion only. Compare all seven baseline bodies using only
  line-ending, trailing-space and outer-whitespace normalization. Record exact
  remaining differences; do not change normalization to obtain a pass.
- Pull a Lifecycle edit onto `notion/main` from the last pushed commit, inspect
  the proposal, then merge it locally with its Why and Source. Reread properties.
- Deliberately diverge one field on both sides. Stop at the conflict and preserve
  both histories. Leave the conflict unresolved as trial evidence.
- Deliberately edit a Notion body. Check whether property-only pull ignores it
  and whether a guarded push detects drift. Do not silently overwrite it.
- Verify every Serves target against the local path-to-page mapping. Mapping and
  raw connector receipts stay private, outside this public repository.
- Count created pages, rows, calls, durations and observable failures. Record
  whether rate/block ceilings were hit; do not stress or exhaust the workspace.
  Distinguish documented REST limits from this connector's observed limits.
- For question 2, perform an object read and a local-file read before a change,
  then inspect the change's Read field. Do not fill it from recollection and
  call that automatic observation.
- Record structure checks and their scope. One operator-run trial, without an
  independent judge or repetitions, cannot establish general conformance.

## Expected answer

None. We record what happens.

## Measures

From [measures](../../convention/views.md); and the [structure checks](../structure.md), run on the Notion space.

## What could break

Read capture: where the adapter cannot see the agent's tool calls, `read` is missing. [Trial 15](15-trace.md) measures the same gap in a folder.

## Run: 2026-11-15

**Result: partly.** One live, operator-run trial through the existing Notion
connector. This is not a reusable sync implementation, a repeated test distribution,
or general conformance to the convention.

Criteria were committed before fixture creation in `be03196`, a commit of the history kept by the founder. Core live window:
16:21:37.895 to 16:31:59.325 UTC (17:21 to 17:31 Europe/Paris). Support views,
recovery attachment and this record followed. Codex ran the trial; the exact model
identifier, token usage and provider cost were not exposed. The session included
global instructions, repository instructions, the two required personal Notion
contracts, model and setup files, the feasibility study, current connector docs,
and official Notion limits documentation. A memory registry search returned no
relevant trial material. This was not a clean, isolated prompt evaluation.

### Results against the frozen criteria

| Criterion | Result | Evidence |
|---|---|---|
| N1 | held | One private space page, Objects and Changes databases. Objects has Type, Question, Model, Serves (self-relation), Source, Lifecycle, Evidence and Date. Changes has fingerprint, message, Why, Source, date, author, operation, status, Objects and Read. Proposed view and seven type views were created and read back. |
| N2 | held | Seven pages, one per core type, were pushed from committed local Markdown. Required properties matched on all seven native rereads. Bodies were readable through the connector; N3 records their serialization differences. The second want then became an eighth page, a second goal. |
| N3 | partly | Comparison completed for all seven. Four normalized body hashes matched; three differed, as listed below. Stable body round-trip failed under the normalization fixed before the run. |
| N4 | held | The operator simulated a Notion-side Lifecycle edit, proposed to review. Pull committed it on `notion/main` as `dc07127`, visible in Proposed before merge. `997f6d8` merged it locally. A property push and native reread both returned review. This was a connector edit, not an observed human UI edit. |
| N5 | held | From shared base `997f6d8`, local `86313dc` set active and incoming `d3fb261` set dropped. Git reported a conflict on the same header line, exit 1. Merge was aborted to leave a clean checkout; both branch tips and the unresolved conflict remain. Remote dropped and the Notion-only body sentence were reread unchanged. |
| N6 | held | The first five commits produced exactly five rows. All thirteen commits then had distinct fingerprint rows, matching message, Why, Source, author and affected-object relations. The manifest commit correctly has no Objects relation. Twelve rows are accepted and one remains proposed. |
| N7 | partly | Eight page bodies were created with Markdown content under a data-source parent through the connector. The direct REST `markdown` parameter and API version were not exercised, so the study's REST uncertainty is only partly answered. |

The live identifiers and link are in the private delivery and test-page recovery
attachment, not this public repository. Final fixture: **24 pages** (root, Agents,
manifest, eight Objects and thirteen Changes), **two databases**, six Serves
edges, one Proposed view, seven type views, and the default database views.
No child space or site was needed or tested.

### Markdown and property differences

The fixed normalization only converts line endings, trims trailing line
whitespace and trims the outside. It was not expanded after seeing the results.

| Object | Exact difference returned by the connector |
|---|---|
| `objects/vision.md` | Leading `# A bakery the street can rely on` and its following blank line were absent. The page title carries the same words, but the body hash differs. |
| `objects/tasks/count-loaves.md` | Code fence language `text` became `txt`. Code content was unchanged. |
| `objects/decisions/missing-is-unknown.md` | `**Not accepted by a real person.**` became `<b>Not accepted by a real person.</b>`. |

Why, problem, no-waste goal and project bodies matched. The sample covered prose,
headings, a quote, a table, checkboxes, code and bold. It did not cover every
common Markdown construct.

The additional URI property appeared with automatic Markdown links in rich-text
readback; a SQL read returned the exact eight original URI strings. This was a
representation difference, not evidence that the URI text was corrupted.

All thirteen Changes Date values read back at minute precision, losing the Git
author-date seconds. For example, `2026-11-15T17:25:27+01:00` became
`2026-11-15 16:25:00Z`. The complete Git bundle preserves the exact dates and
history. This connector projection alone does not preserve timestamp precision.

A Notion-only sentence was appended to the task body. Property-only pull left it
out of Git. Comparing the current remote body with its last pushed readback
detected drift, and the operator stopped before a body replacement. The remote
sentence remains. An unconditional local-body replacement would discard it;
this destructive overwrite was not performed. No body merge was proven.

### History and replay evidence

| Commit | Change |
|---|---|
| `705bbd2be5a551cefc5d7377b092906320344898` | Manifest, doctrine projection and ignore rule |
| `f356d2184366d4421701e9d830736e4a33bbfa9f` | Why |
| `e4bbe623fe0cea4a95645ae1514f08646da07bb7` | Problem |
| `742fad229d48b495469cb0291b27c6a37cb5ad65` | Vision |
| `7f3e6966a3ad4dbec5725fec8b376aaa7edfe24d` | No-waste goal; fifth projected commit |
| `54f7c2d6df852d2232c6743118b978e9cbb39d7d` | Tally project |
| `30b68e910775a1bc0ea203659f5cc574f5646940` | Counting task |
| `7cde5c71043028f86ef9c78d6e588251c14b9bed` | Decision; seven-object baseline |
| `2c6502f073dcfe6569383737f31c3261b1e55318` | Same-day answer goal; first sync base |
| `dc0712738d2020ad2f8e0deb2a612ce77eff6bd6` | Incoming review proposal |
| `997f6d8f9c9372d527f94228be85941858fc4a4d` | Merge of review proposal; second sync base |
| `86313dca139419a94d949fe8577020ea6b936828` | Local active conflict side; main tip |
| `d3fb261482612191acdd2fb524663d740f940965` | Incoming dropped proposal; notion/main tip |

Each row also contains the corresponding Git patch. No Git remote was configured
or pushed. A complete bundle with five refs was verified; its SHA-256 is
`e0f1bf245e475e927ba8bd4d663d82976c0dd692316c69614f00bf4f7d83735c`.
A 64,124-byte JSON recovery attachment on the test page holds that bundle as
base64, its hash, the private page mapping and the fictional evidence. Download
returned exactly the uploaded text. Decoding that downloaded copy and cloning
its bundle recovered all thirteen commits and both branch tips; integrity checks
passed. The local fixture and private sync evidence remain in the requested
scratch space.

Public replay inputs are in [the sanitized record](../notion-adapter/2026-11-15.json).
They contain fictional source files, exact baseline readbacks, commit patches,
property comparisons and the deliberate divergence, with private URLs and IDs
removed. [The offline checker](../notion-adapter/check_run.py) recomputes the
body hashes and diffs, checks required header values against the installed model
questions, links, projected properties and commit-row correspondence. It can
also compare the actual local Git fixture:

```sh
python tests/notion-adapter/check_run.py
python tests/notion-adapter/check_run.py --space <local-test-space>
```

Both recorded evidence and the actual source repository were checked. Eight
object headers and six links matched; thirteen commits matched thirteen rows;
the three body differences remain visible. The checker passing means evidence
consistency, not that every trial criterion held. Git bundle verification and
object integrity checks completed; dangling trees from the aborted merge are
not accepted changes.

### Limits met and not measured

- **53 core connector calls**, excluding discovery and contract reads, including
  two schema reads outside the timer log. Of these, 45 have per-call timings:
  median 1,449 ms, minimum 1,167 ms, maximum 4,641 ms. Seven baseline reads ran
  concurrently. No surfaced 429, overload or workspace-block error occurred.
  Hidden connector retries and backend request counts were not exposed.
- Discovery first rejected search `page_size: 100`: this connector's search
  maximum is 50. Retrying at 50 succeeded. This was before the live fixture.
- Creating Agents returned three parser warnings for literal `<object>`,
  `<operation>` and `<summary>` placeholders. They were retained as raw text.
  The ordinary object bodies produced the N3 differences above.
- **Exact native block count, workspace plan and remaining quota were not
  exposed** by the tools used. Twenty-four created pages are not a block count.
  No quota-exhaustion or load test was attempted.
- The official [request limits](https://developers.notion.com/reference/request-limits),
  read on 2026-11-15, distinguish 180 requests/minute for other plans and 600 for
  Business/Enterprise, a shared workspace limit, a roughly 3/second data-source
  query limit, and separate MCP throttling. None of those ceilings was measured.
- The official [workspace block limits](https://developers.notion.com/reference/workspace-block-limits)
  distinguish a 1,000-lifetime-block limit for multi-member Free workspaces under
  specified connection types, with a separate MCP policy. This run cannot establish
  the workspace's ceiling or remaining allowance.
- No access or publication change, deletion outside the fixture, task mutation,
  message to others, or Git push occurred. Private creation used the connector's
  draft mode. Actual ACL inheritance and access from another account were not tested.

### Question 2: what is missing from read?

All **thirteen Read fields are empty**, including the change that followed a
Notion fetch of the vision and a shell read of the local vision file. Those two
reads happened in the assistant's session, but the projection did not observe
or attach them to the resulting change.

Missing are observed input URIs, their exact versions or fingerprints, read
times, and the association with the change that followed. Local files, model and
doctrine reads, external documentation and any reads made outside this connector
are not automatically represented. Current page state and a commit diff describe
outputs, not the complete context that produced them.

An empty Read field means **capture is absent**, not "nothing was read". Filling
it manually would be a declaration, not the observed provenance claimed in
[trace](../../convention/trace.md). The read-versus-draws-on comparison, unused
context measure and version-sensitive stale-source analysis cannot rely on this
trace as complete. No interceptor, hook, replay capture or automatic association
was installed or tested.

### Interpretation and next honest claim

The landing can say:

> Notion: tested with limits. Local-first objects and commit history work through
> the connector; body round-trip and automatic read capture remain unproven.

It cannot yet say "fully supported", "lossless two-way sync", "complete trace"
or that a public template is available. The landing itself was not edited.

Not checked: direct REST Markdown creation, independent judge or repetitions,
a calibrated semantic check, mobile or visual UI rendering, a child space,
cross-space links, all property types, large relations, webhooks, automatic
incremental discovery, concurrent-write races, crash/retry recovery, quota
ceilings, signatures, anonymous access or template distribution. Notion's
last editor identity was not available in these reads; the simulated edits were
made and attributed locally to the trial operator, not a real person.

TheVZion's task tracker was searched read-only. No task names this trial. An
earlier template project has tasks for installation testing, correction,
measurement and publication preparation; they concern a different kit and were
not changed.

The inferred implementation choices were: model placement of the supplied wants,
proposed measures and bakery means, connector-based execution, the extra URI and
link-provenance fields, fixed minimal normalization, and stopping at the injected
conflict. They are trial choices, not accepted changes to the convention. The
fixture deliberately retains its unresolved proposal and remote-only body edit
for inspection.
