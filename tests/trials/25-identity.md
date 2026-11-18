---
trial: 25
status: not-run
level: end-to-end
tests: [https://github.com/thevzion/source/blob/main/objects/hypotheses/it-carries-over-to-people.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/every-object-knows-where-it-came-from.md, https://github.com/thevzion/source/blob/main/objects/hypotheses/agents-hold-roles-under-mandates.md]
---

# Trial 25: identity

This trial answers an open question of TheVZion's source: how does a person stay the author of their work, across organizations, without being exposed? (not published yet) It tests a proposal dated 2026-11-15, not published yet, whose criteria are fixed below. The principle is already in [access](../../convention/spaces.md): "Your work is shared, not you."

A fingerprint says what (a content), a signature says who (a key), identity says whose key it is. Identity and integrity have the same answer: what a person says is signed with their key.

## Questions

1. Can a person stay the author of their work across organizations without being exposed? (C3, C4, C8, C10, C11, C12)
2. Can `from-me` be trusted? (C1, C2, C5, C7)
3. Can the same verification hold whatever the adapter: a folder with git, a git copy standing for GitHub, Notion, the web? (C6, C11)
4. Can access be shared without sharing a key? (C5, C9, C16)
5. Is everything traced by default, and revealed only by choice? (C10, C13, C14, C15)

## Setting

Everything is local and nothing is public: no network service is called, nothing is pushed, nothing is published.

| Part | What stands for it |
|---|---|
| organization A | a clone of TheVZion's source, made with `git clone --no-hardlinks` into a temporary folder; the source itself is only read |
| organization B | a second, fictional organization, in a temporary git repository |
| the person | fictional: a test home in a temporary folder, holding the root key and the map of pseudonyms; no real key, name or account is used |
| an agent run | one agent writes changes in A under a mandate the person signed |
| GitHub | a local bare copy of A, made with `git clone --bare --no-hardlinks` |
| Notion | a JSON projection of A's Objects and Changes, written by the test, carrying each change's statement and signature as properties |
| the web | a page rendered locally from one of A's objects, carrying its attestation in a projection tag |
| a verifier | a party outside the home, given the public member lists and only what the person grants it |

The model under test is the same for every adapter: the adapter carries, it proves nothing.

1. **One key per organization.** The root key stays in the person's home. For each organization, an Ed25519 key is derived from the root and the organization's URI (HKDF): the person can recompute it, others cannot link two. The organization publishes its members: key, role, verbs, and a name chosen for it.
2. **The disclosure choice, per space, explicit** in the Access section of `space.md`: signed by the organization; by the person under a pseudonym; or by the person, named. Changing the choice is a traced change, never implicit.
3. **The signed change statement**, canonical and independent of git: space, objects, the fingerprint of their content after the change, operation, `Why`, `Source`, `Intent`, previous state, time. Carried by the `Signed-by:` and `Signature:` trailers; the same signature verifies from git, a GitHub copy and Notion's Changes database.
4. **The object attestation**: the fingerprint of an object's content and the signature of a key, carried as metadata by any adapter (git; a property or a neighbouring file in Notion; a projection tag in a web page). One tool verifies: "signed by your key in organization A, at change X."
5. **`from-me` requires a member's signature.** Without it, the object shows "unsigned" and counts as inferred.
6. **Delegation for agents.** The person signs the mandate (space, verbs, expiry); the agent signs its change with its own key, cites the mandate (`On-behalf-of:`) and the model (`By:`). An agent never produces `from-me`.
7. **Rotation, revocation, loss.** The member list lives in history: a removed key no longer counts after its removal, and what it signed before stays valid. The root is backed up (paper or a password manager); restored, it gives back the same pseudonyms. Without a backup, the old signatures stay verifiable, and the organization readmits the person with a new key.
8. **Adapters that cannot sign** carry the statement and its signature beside the change (a property, a neighbouring file); the git repository stays the notary.
9. **Chosen disclosure.** The person proves that two pseudonyms are theirs by signing with both. Absent by default.
10. **Grants: never a key, always a signed statement.** A grant says to whom (a key), what (read, propose, decide), on what (a space, an object, the trace, the link between two pseudonyms) and until when (an expiry by default). It can be revoked at any time; the revocation is itself a signed statement, published where the verifier looks (the space's history, or the home for a personal grant), and a short expiry covers a verifier that is offline. One primitive: an agent's mandate, a person's invitation and an agent token are grants.

The trace the run writes follows [changes](../../convention/changes.md) and [trace](../../convention/trace.md), with `By:`, `On-behalf-of:`, `Run:`, `Read:` and `Cost:` as its default. Played by automated tests in an implementation of the convention (The System, TheVZion's app), with the tooling `org key init|derive|sign|verify|link`. The key type is a design choice of the proposal, open to correction; passkeys would come later as a second key type.

## Fixed before the trial

Source: the identity proposal dated 2026-11-15, the section on its criteria, fixed before the trial, in French, not published yet. Source SHA-256: `771c241954851c6c7117278686bc733484e11f23a59a7d0e247e393dfd42768e`. Its private location is withheld from this public repository. The criteria are translated below; where the translation and the French differ, the French holds. The French original is kept outside this repository, with the source above.

| # | Criterion | Passes if |
|---|---|---|
| C1 | Forgery | an agent that writes `from-me` is detected, shown "unsigned", and counted as inferred |
| C2 | Integrity | changing the content or the why of a signed change after the fact makes verification fail |
| C3 | Unlinkability | the member lists of A and B, for the same person, share no key and no field |
| C4 | Retrace | from the home alone, the person lists and verifies all their changes in A and B |
| C5 | Delegation | an agent's change verifies through the chain: the mandate, then the agent; an expired or out-of-scope mandate fails |
| C6 | Adapter independence | the same change gives the same verdict from the folder, a remote git copy kept locally, and the simulated Notion projection |
| C7 | Revocation | after a key is removed, its new signatures fail and its old ones hold |
| C8 | Disclosure | the link between two pseudonyms is absent by default, and verifiable when the person proves it |
| C9 | Secrets | no private key in a space, a commit, a log, an API response or a Notion page |
| C10 | Explicit choice | a "pseudonym" space publishes no name; a "named" space shows the name; changing the choice leaves a traced change |
| C11 | Recognize one's trace | from the home, the tool recognizes a file of the folder, a simulated Notion page and a rendered web page as signed by the person |
| C12 | Loss | the restored root gives back the same pseudonyms; without a backup, the old signatures hold and readmission goes through the organization |
| C13 | Complete trace | every change the implementation writes during the trial carries what, who (a person, or an intelligence with its provider, model and version), run, on whose behalf, when, why, read and cost; a structure check measures the complete share: 100% |
| C14 | The person sees first | from the home alone, the person lists everything recorded about them in A and B, access log and attention time included |
| C15 | Minimization | no trace holds an IP address, a device, or a name outside the disclosure choice; retention periods are stated and applied |
| C16 | Grant and revocation | the home gives a verifier a read grant on the link between two pseudonyms, and it verifies; after the revocation, and after another grant expires, verification fails; no private key left the home |

Readings fixed by this sheet before the first run, inferred, open to correction until then and not after:

- **Verdicts.** Every verification returns one of three verdicts: valid, invalid or unsigned. A signature that is missing or stripped gives unsigned, never valid.
- **C3.** "No field" is read as no identifying value: no key, no chosen name, no identifier and no value derived from the person. Role and verbs take their values from the convention's closed sets and do not identify. C3 applies to any pair of spaces where at least one is not "named".
- **C7.** "New" and "old" are read in the space's history: a signature is old when its change comes before the change that removes the key, whatever time the statement claims.
- **C9.** The search covers the root, every private key and every grant secret, in raw, hex and base64 forms, across every file, commit, log line, API response, projection and test snapshot the run produces.

Not fixed yet, by the proposal or by this sheet. Their values are written here before the first run, and not changed after:

- the default expiry of a grant, and the clock tolerance of a verifier;
- the retention periods of the access log and of attention time (C15);
- the canonical encoding of the change statement and of an attestation, and the form of the projection tag in a web page;
- where the organization's own key lives when a space chooses "signed by the organization", and where an agent's key comes from;
- where the access log and attention time are recorded (C14);
- for a person's change written through the implementation, what `Read:` and `Cost:` hold (C13).

## Expected answer

Yes to each question, if every criterion passes as fixed above. C1 to C12 and C14 to C16 are answered by computation (the verifier, the structure check, the search for secrets), so one run gives each a pass or a fail. C13 reads every change the run writes, and expects 100%. Where a criterion fails, we record how it failed; the criteria are not changed.

What is recorded, for each criterion: pass or fail, the test that answered it, and the fingerprints involved; for C6 and C11, the verdict from each adapter side by side. For the run: this branch and its commit, the implementation's commit, the key type and the tool versions, the agent's provider, model and exact version, what the run loaded beyond its fixtures, and what was not tested.

## Measures

From [measures](../../convention/views.md):

- **activity**, by author: a person, by their key for the space, or an intelligence, by `provider/model@version`; and signed or unsigned;
- **exposure**: what leaves each space and the home, and to whom: member lists, projections, grants, and what the agent run sent to its provider;
- **gates**: the gaps between the written agreement (mandates, grants) and the access each adapter applies;
- the cost of the agent run, from `Cost:`.

And two of this trial:

- **trace completeness**: the share of changes the implementation writes during the trial that carry what, who, run, on whose behalf, when, why, read and cost, as the structure check counts it. C13 fixes it at 100%.
- **verdicts by adapter**: for each change, valid, invalid or unsigned, read from the folder, the git copy and the Notion projection.

## What could break

- **Key loss, and key theft.** Without a backup of the root, the pseudonyms cannot be recomputed: the old signatures hold, but readmission is the organization's decision (C12). The derivation that makes pseudonyms recomputable also makes the root the one secret that opens them all: a stolen root links every pseudonym and signs in every organization.
- **An adapter that strips metadata.** A projection that drops a property, a site builder that rewrites a page, a pull request merged by squash or rebase. The statement is independent of git so that a rewritten commit still verifies; a lost signature must give unsigned, never valid. [Trial 4](04-notion-adapter.md) saw Notion change three of seven bodies and round dates to the minute: a fingerprint rebuilt from a Notion body or date fails, so a verifier reads the carried statement, never one rebuilt from the projection.
- **Clock skew on expiry.** A verifier whose clock is behind accepts an expired grant; one whose clock is ahead refuses a valid one. A removed key can also sign with a time from before its removal, since the statement's time is said by the signer: hence the C7 reading by history.
- **Linkability through timing, names and tools.** Different keys do not hide the same person acting in A and B in the same minutes, the same chosen name, the same writing, or a run identifier reused across spaces. Git also writes an author and a committer in clear, from the machine's configuration: a pseudonymous space, correctly signed, can still leak a name or an address through `user.name`, `user.email`, `author.*` or `committer.*`, the `GIT_AUTHOR_*` and `GIT_COMMITTER_*` environment, a commit's GPG signature, or a `Co-Authored-By:` line (C3, C10, C15). An address identifies a person even without a name.
- **A place in history that an adapter claims.** A row's order in a projection, a member list or a list of revocations copied into it, or a rank written in a web page's tag is not signed: whoever edits it could move a removed key's change back into its window, or make a revocation disappear. Only the repository, the notary, says where a change sits and who was a member there; a statement it does not hold has no place. A commit on a branch has no place on the main line either, whatever time it claims.
- **A revocation deleted later.** A verifier that reads grants and revocations from a space's head would let a revoked agent, or a member who may only propose, delete the revocation and make the grant hold again, even for changes it had already failed on. Read from history, a revocation holds once published, and a change to the grants needs the verb that decides.
- **A delivered proof of the link.** A link signed by both keys, once handed over, proves the link to anyone, forever, revocation or not. A verifier can instead receive an answer to its own question that only its key can check and that it could have computed alone: it convinces the verifier and no one else. What the verifier learned it keeps; what it kept proves nothing.
- **A commit dated ahead.** One commit with a time far in the future, if every later change had to follow it, would turn the person's next changes invalid until that date. Only a change that verifies can bound the time of the changes after it.
- **A verifier offline misses a revocation.** Until it reads the space's history or the home again, it accepts a revoked grant. Only the expiry bounds that window, and its default value is not fixed yet.
- **History that was never signed.** The source's past changes are signed by the organization's account, not by a person. Once `from-me` requires a member's signature, every past `from-me` shows unsigned and counts as inferred: activity and accuracy shift with nothing having changed. Public history is not rewritten; signing starts from the decision.
- **A key within an agent's reach.** An agent that can read the home can sign `from-me`. Delegation holds only while the root and the member keys stay out of the agent's reach.
- **Erasure with derived keys.** A member key derived from the root can be recomputed, so erasing the map in the home does not detach a pseudonym while the root remains.

Open: identity (not published yet), [privacy-boundary](https://github.com/thevzion/source/blob/main/objects/questions/privacy-boundary.md), [commit-trailers](https://github.com/thevzion/source/blob/main/objects/questions/commit-trailers.md), [where-read-lives](https://github.com/thevzion/source/blob/main/objects/questions/where-read-lives.md).

## Normative text to propose after the trial

Not applied. Each change below is a candidate: it lands only after this trial runs, through a decision recorded in TheVZion's source on identity (not published yet). Quoted text is the proposed wording; its links point from this sheet, and point from the target file once applied.

### In [access](../../convention/spaces.md)

1. **Identity, pairwise member keys** (candidate). In Federation and identity, the identity row: replace "the space publishes its `space.md`, and its members' public keys, at its URI" with "the space publishes its `space.md` and its members, at its URI: for each, a public key for this space only, a role, the verbs, and the name chosen for this space".
2. **A section "Member keys"** (candidate), after the three questions of authority:

   > - A person's member key MUST be different in each space. It SHOULD be derived from a root key and the space's URI, so the person can recompute it and no one else can link two.
   > - The root key stays in the person's home. A private key, a root key or a grant secret MUST NOT be written in a space, a commit, a log, an API response or a projection.
   > - Two spaces' member lists MUST NOT share a key, or a value that identifies the person, unless the person discloses the link.
   > - The member list lives in history. A removed key MUST NOT verify a change that comes after the change that removes it; what it signed before stays valid.
   > - The root is backed up outside any space. Restored, it gives back the same keys. Lost, the old signatures still verify, and the space decides whether to readmit the person with a new key.
   > - The link between two keys of the same person is absent by default. The person discloses it by signing one statement with both.

3. **A section "Disclosure"** (candidate), the choice in `space.md`:

   > `space.md` MUST state, in its Access section, who signs the changes others see:
   >
   > | Choice | Others see |
   > |---|---|
   > | `organization` | the space's signature |
   > | `pseudonym` | the person's key for this space, and the name chosen for it |
   > | `named` | the person's key and name |
   >
   > Changing the choice is a change, with its why; it is never implicit. The full trace is kept and signed where the person or the space holds it; what others see follows the choice.

4. **A section "Grants: sharing access without a key"** (candidate), grants and revocation as one primitive:

   > Access is shared by a grant, never by a key. A grant is a short statement signed by a person or a space:
   >
   > | Field | What it says |
   > |---|---|
   > | to | a key |
   > | verbs | read, propose, decide |
   > | on | a space, an object, the trace, or the link between two keys |
   > | until | an expiry, set by default |
   >
   > - A grant MAY be revoked at any time. A revocation is a signed statement, published where the verifier looks: the space's history, or the home for a personal grant.
   > - A verifier MUST refuse a grant that is expired, revoked, out of scope, or signed by a key that may not give it.
   > - One primitive: an agent's mandate, a person's invitation and an agent token are grants. A bearer token is the local form of a signed grant.
   > - For the person, two gestures: share access, which writes a grant; revoke, which ends it.

5. **The layers table** (candidate). The written agreement row gains: "a grant is its signed form".
6. **`org verify`** (candidate). Replace "checks signatures; key fingerprints live in `known_orgs`, like `known_hosts` in SSH" with "checks a change's signature against the space's member list at that point of its history, and a grant's chain to its signer; key fingerprints live in `known_orgs`, like `known_hosts` in SSH".
7. **No blockchain** (candidate). The bullet gains: "Erasing the link kept in the home detaches a pseudonym from the person." With derived keys, this holds only once the root is gone too (see What could break).

### In [changes](../../convention/changes.md)

1. **Fields, the default trace and the signature** (candidate). Replace "Others are candidates (`Intent:`, `Read:`, `By:`, `On-behalf-of:`, `Lane:`, `Cost:`, `Ratifies:`)." with "Five form the default trace: `By:`, `On-behalf-of:`, `Run:`, `Read:`, `Cost:`. Two carry the signature: `Signed-by:`, `Signature:`. Others are candidates (`Intent:`, `Lane:`, `Ratifies:`)."
2. **The fields table** (candidate):
   - who: add "; the signer, in the `Signed-by:` trailer: the fingerprint of the member key for this space, or of the agent's key".
   - the seal: replace "the hash chain, signatures" with "the hash chain; the signature of the change statement, the same in every adapter"; in Notion, "the commit fingerprint, the statement and its signature, as properties".
   - what was read: replace "a git note, or a `Read:` trailer: the URIs and the version read" with "a `Read:` trailer, one per object read, `<URI>@<fingerprint>`, or a git note".
   - on whose behalf: replace "the mandate, as a URI or a role" with "the grant the agent acts under, as a URI or its fingerprint", keeping "never a person's name or address".
   - a new row, the run: "the `Run:` trailer, `<space URI>/runs/<id>`, which groups the changes of one intervention"; in Notion, a relation to the run.
3. **A section "Actor identifiers"** (candidate):

   > | Actor | Identifier |
   > |---|---|
   > | a person | the fingerprint of their key for this space |
   > | an intelligence | `<provider>/<model>@<version>` |
   > | a run | `<space URI>/runs/<id>` |
   > | a change | its fingerprint |
   > | an object | its URI |

4. **A section "The signed change statement"** (candidate):

   > A change's statement is canonical and independent of git: the space, the objects with the fingerprint of their content after the change, the operation, `Why`, `Source`, `Intent`, the previous state, and the time. Its signature is carried by the `Signed-by:` and `Signature:` trailers.
   >
   > - The same signature MUST verify from the folder, a git copy and the Changes database.
   > - An adapter that cannot sign carries the statement and its signature beside the change (a property, a neighbouring file); the repository stays the notary ([access](../../convention/spaces.md)).
   > - A verification returns valid, invalid or unsigned. A missing signature gives unsigned, never valid.

5. **`from-me` requires a member's signature** (candidate). In Convention, after the MUST:

   > A change marked `Source: from-me` MUST be signed by a member key of the space. Without that signature, it is shown unsigned and counted as inferred. A change by an agent MUST NOT be marked `from-me`: the agent signs with its own key, cites its grant in `On-behalf-of:` and the model in `By:`.
   >
   > Every change SHOULD carry the default trace: who acted (the author, and `By:` when intelligence wrote), `Run:`, `On-behalf-of:` when it acts under a grant, `Read:` and `Cost:`.

   With an example:

   ```
   goal(revise): turn goal 2 into an outcome

   Why: a deliverable was posing as a goal
   Source: inferred
   By: <provider>/<model>@<version>
   On-behalf-of: <space URI>/grants/<id>
   Run: <space URI>/runs/<id>
   Read: <space URI>/goals/2@<fingerprint>
   Cost: <tokens> tokens
   Signed-by: <key fingerprint>
   Signature: <signature>
   ```

### In [trace](../../convention/trace.md)

1. **The opening** (candidate). "Everything here is candidate; trial 15 decides." becomes "Everything here is candidate; trial 15 decides on `read` and `draws on`, trial 25 on who acted and what is signed", each trial linked.
2. **A section "Who acted, by default"** (candidate), the default trace:

   > Every change records, by default, PROV's triple of entity, activity and agent:
   >
   > | What | Carried by |
   > |---|---|
   > | what: the fingerprint of the content | the change statement |
   > | who acted: a person, by their key for this space; or an intelligence, by provider, model and exact version; and the run | the author, `Signed-by:`, `By:`, `Run:` |
   > | on whose behalf | `On-behalf-of:` |
   > | when, and why | the date, `Why:` |
   > | what it rested on: what it read, with versions, and what it cost | `Read:` (`<URI>@<fingerprint>`), `Cost:` |

3. **A section "Recognizing one's trace"** (candidate), object attestations:

   > An attestation is the fingerprint of an object's content and the signature of a key. Any adapter carries it as metadata: in git, the change statement; in Notion, a property or a neighbouring file; in a web page, a projection tag. One verifier answers, from the home: "signed by your key in space A, at change X." Prior art, to verify: C2PA for media, git's SSH signing and Sigstore for commits.

4. **A section "Trace everything, reveal by choice"** (candidate), minimization:

   > - The full trace is always kept and signed, where the person or the space holds it.
   > - What others see follows the space's disclosure choice ([access](../../convention/spaces.md)).
   > - The person sees first: from their home, a view of everything recorded about them, across their spaces, access log and attention time included.
   > - The trace keeps only what its purpose needs: no IP address and no device; the access log and attention time have a stated retention, and it is applied.
   > - Never a note about a person.

5. **Where it lives** (candidate). The git row: replace "a note attached to the commit (`git notes`), which does not change the commit, or a `Read:` trailer" with "a `Read:` trailer, `<URI>@<fingerprint>`, or a note attached to the commit (`git notes`), which does not change the commit".
6. **The vocabularies table** (candidate), two rows, to verify against each specification ([trial 17](17-projection-to-prov.md)): `Signed-by:` maps to the agent of `wasAttributedTo` or `wasAssociatedWith`, named by its key, and to C2PA's claim signature; `Run:` maps to a PROV activity that contains the changes.

Beyond these three files, also candidate: [structure](../../convention/structure.md) would add the disclosure choice to what `space.md` MUST declare; [the structure checks](../structure.md) would gain one check that every `from-me` change is signed by a member, one that counts the complete trace (C13), and one that flags an IP address, a device or a name outside the disclosure choice (C15).
