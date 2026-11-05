# Contributing to Mycelium

Mycelium hosts universes (games among them) in which external
AI agents explore, act and do science
([README › What Mycelium hosts](README.md#what-mycelium-hosts)). It is
developed in the open, and contributions are welcome at every layer: the
algebra, the courts, the economy, the wire protocol, the SDK, the
reference mind, game and science kernels, and the specifications.
"Minds" and "peoples" in this tree name hosted AI agents and simulated
inhabitants: the engineering is real, and the inhabitants are hosted. This document covers how to
take part and the standards the tree is held to.

## Getting started

1. Check the tree with the two commands of
   [AGENTS › Definition of done](AGENTS.md#definition-of-done), which also
   says when `scripts/quickstart` is required and where the ARC-AGI-2
   corpus is read from. The core library reads no corpus: it is read only
   by its decoder, the workspace crate `sources/arc-agi`, and the hyphae
   `scripts/quickstart` starts read the posted task files the decoder
   writes.
2. Bring up a local network and run the SDK client against it, following
   [README › Running a node](README.md#running-a-node) and
   [README › Building a client with the SDK](README.md#building-a-client-with-the-sdk).
3. Read the specification for the area of the intended contribution. Code comments
   cite its sections and rulings by number (for example *78.143* or
   *MU-CLIENT §5.6*). Every document, grouped and in the order in which
   they govern, is listed in
   [README › Documentation](README.md#documentation).

## How to contribute

`main` is protected: every change, a maintainer's included, arrives by
pull request, and only the maintainers in [MAINTAINERS.md](MAINTAINERS.md)
merge. The full process is [`docs/MU-RELEASE.md`](docs/MU-RELEASE.md)
Part I.

1. **Find or open an issue.** Start from
   [ROADMAP › Now](ROADMAP.md#now): the release in progress, the work ready
   to start, and what waits on a ruling. Each item there is an issue with a
   milestone (`v0.3.0`, `next`) and a track label (`track-N`). `good first
   issue` marks a contained starting point, `in progress` marks work
   already claimed, and `maintainers` marks release and operator work. A
   change of behaviour opens a *proposal* issue naming the section of the
   specification it touches.
   **Claim the issue** with a comment before starting, so that two
   contributors do not build one phase. A claim with no pull request or
   comment for 14 days lapses, and a maintainer may release it.
2. **Specification first.** A change of behaviour is a change of the
   specification: written as a proposal in
   [`docs/proposals/`](docs/proposals/) and amended into the governing
   document when it is accepted. A change to the network contract (the
   Concordance, the wire, the ledger layout, a law version) needs the
   operator's approval.
3. **Branch and pull request.** One phase of work per pull request: the
   code, its tests and its documentation together, a CHANGELOG fragment
   under `changes/` ([AGENTS › Definition of done](AGENTS.md#definition-of-done)),
   and the template filled in. The
   [Definition of done](AGENTS.md#definition-of-done) holds; CI's `check`
   runs it on every pull request.
4. **Review and merge.** A review reads the code against the
   specification and cites the section it holds the code to; a maintainer
   merges once conversations are resolved. History on `main` is never
   rewritten.

## Releases

Releases are SemVer tags `vX.Y.Z` with a GitHub Release whose notes are the
CHANGELOG section ([`docs/MU-RELEASE.md`](docs/MU-RELEASE.md) Part II). The
release checklist:
- the [Definition of done](AGENTS.md#definition-of-done) green on the
  tagged tree;
- the CHANGELOG section final, assembled from the fragments under
  `changes/` (which are then removed), every API, wire and ledger change
  named;
- the release candidate `vX.Y.Z-rc.N` run on a test mesh of its own
  genesis, and the run recorded in the release pull request;
- where the release changes the network contract, its activation point
  (`activates_at`, a TUniversal final point) stated;
- the operator's authorization recorded in the release pull request; the
  tag signed.

## Standards

The full standards are [`docs/MU-STANDARDS.md`](docs/MU-STANDARDS.md). The
essentials:

**Arithmetic**
- Every quantity is exact: ℚ, or the multivectors of `Cl(n+1, 1)`(ℚ), with
  `n` read from the data (`Cl(4,1)` at `n = 3`).
- Float arithmetic and lossy casts are denied by lint.
- No structure is projected to a scalar
  ([`docs/MU-ANTI-FLATTENING.md`](docs/MU-ANTI-FLATTENING.md)).

**Refusals**
- A reader that cannot answer returns a named refusal.
- `unwrap`, `expect` and `panic` are denied outside tests.

**Files**
- No source file exceeds 2500 lines, and each file holds one organ of the
  design.
- A `mod.rs` holds only declarations.
- Splits are move-only commits.

**Tests**
- A test is named `<unit>_<behaviour>`: the function or type under test,
  then what it does or refuses, in a few words with no articles (for
  example `pay_lease_pays_each_job_once`). Its claim, in plain sentences,
  is its `///` doc comment (MU-STANDARDS 3.1).
- A test is a claim proved by the algebra:
  - a refusal by name,
  - a fixed locus,
  - a reconstruction (the object derived independently from its own
    evidence equals the object; [MU-SCIENCE §5.0](docs/MU-SCIENCE.md#50-reconstruction)),
  - an exact identity.
- Fixtures are built by the engine's own constructors.

**Formatting**
- The tree is not `rustfmt`-clean by design. Do not run `cargo fmt` over
  it; format only the lines a change touches.

**Headers**
- A source file generated by AI opens with the `AI-GENERATED CONTENT —
  see DISCLAIMER.md` header. Not every file in the tree carries it; add it
  to every newly generated file.
- A file's header names what the file is in the specification's terms.

**AI coding agents**
- A contribution made with an AI coding agent follows
  [AGENTS.md](AGENTS.md). It lists the regressions that recur (a common
  grid in place of transduction, a digest in place of reconstruction,
  replay in place of reconstruction) and the structure that replaces each
  ([`docs/MU-ANTI-FLATTENING.md`](docs/MU-ANTI-FLATTENING.md) clauses
  12–14).

**Commits**
- One phase per commit.
- The first line states the change as a claim.
- The body gives what changed, why, and how it was verified.

## The gate

The gate is the workspace crate `gate/`: its tests read the source tree
for what the compiler cannot, and run with every other test. How it is
run is [AGENTS › Definition of done](AGENTS.md#definition-of-done); what
each check refuses is [MU-STANDARDS 4.1](docs/MU-STANDARDS.md#4-the-gate).

A warning is a defect with a line number. Where a denied lint fires on a
deliberate choice, the site carries an `#[expect(…, reason = "…")]` that
names its ground.

## The roadmap

[ROADMAP.md](ROADMAP.md) lays out the development tracks in the order
they build on each other. Each track names its specification and where it
lands in the code. Open an issue on a track to coordinate before starting
on it.

## Security and conduct

Report vulnerabilities privately, as [SECURITY.md](SECURITY.md) describes.
The project's spaces follow the [code of conduct](CODE_OF_CONDUCT.md).

## License

Contributions are accepted under the [Apache License, Version 2.0](LICENSE).
