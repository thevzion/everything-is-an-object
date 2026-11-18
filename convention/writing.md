# Writing

Convention 0.3 · draft

How to read the convention, and how its rules are written. This file was called `conventions.md`; it is renamed so that "convention" names only the whole.

## Normative keywords

The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be read as described in RFC 2119 and RFC 8174, when, and only when, they appear in capitals.

| Keyword | Meaning |
|---|---|
| **MUST**, **MUST NOT** | an absolute requirement, or an absolute prohibition |
| **SHOULD**, **SHOULD NOT** | the default; there may be valid reasons to depart from it, and they are understood and weighed |
| **MAY** | truly optional |

## Normative and explanatory

- A sentence with one of these keywords, in capitals, is **normative**: it defines what conforms.
- Everything else is **explanatory**: notes, examples, comparisons, reasons.
- What is marked **candidate** or **open** is not normative yet, even when it uses one of these words.

## A rule can be drawn (proposed)

A rule of the convention, and every norm text of `AGENTS.md`, is written in the
[primitives](primitives.md): who acts (a person, a role), what comes in (a signal), what it becomes
(objects, each answering its question, with their links), and what is kept (a change). The same
eight words describe the objects and the instructions that act on them, so the instructions are a
model too.

So every rule can be drawn, as the README draws sections 3 and 5 of `AGENTS.md`, and every rule can
be put to a real agent and checked ([trials](../tests/README.md)). If a rule cannot be drawn, it is
not clear enough to follow or to test: rewrite it until it can. Close to it: an executable
specification, where a rule is also its test.

## Conformance does not block

A MUST says what an implementation does to conform: a tool, an adapter, the behaviour of intelligence, a space's files. It is not a gate on the person. Checks signal; they never block a change ([observe before constraining](doctrine.md#observe-before-constraining)). Conformance is tested by the checks marked normative ([tests](../tests/README.md#conformance)).
