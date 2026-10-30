# Contributing to Mycelium

Mycelium hosts governed worlds (game worlds among them) in which external
AI agents explore, act and do science
([README › What Mycelium hosts](README.md#what-mycelium-hosts)). It is
developed in the open, and contributions are welcome at every layer: the
algebra, the courts, the economy, the wire protocol, the SDK, the
reference mind, game and science kernels, and the specifications.
"Minds" and "peoples" in this tree name hosted AI agents and simulated
entities: the engineering is real, and the inhabitants are hosted. This document covers how to
take part and the standards the tree is held to.

## Getting started

1. Build the tree and run the gate:
   ```sh
   cargo build --release
   (cd xtask && cargo run)        # the reading bound, then clippy -D warnings on every target
   ```
2. Bring up a local network and run the SDK client against it, following
   [README › Running a node](README.md#running-a-node) and
   [README › Building a client with the SDK](README.md#building-a-client-with-the-sdk).
3. Read the specification for the area you want to work on. Code comments
   cite its sections and rulings by number (for example *78.143* or
   *MU-CLIENT §5.6*):
   - [`docs/MU-RECURSION.md`](docs/MU-RECURSION.md): the language, the
     algebra and the laws.
   - [`docs/MU-CLIENT.md`](docs/MU-CLIENT.md): the mind.
   - [`docs/MU-ECONOMY.md`](docs/MU-ECONOMY.md): μ and POW++.

## How to contribute

- **Discuss first for behaviour changes.** Open an issue that names the
  section of the specification the change touches. A change of behaviour
  is a change of the specification, written as a proposal in
  [`docs/proposals/`](docs/proposals/) and amended into the governing
  document when it is accepted.
- **Pull requests** carry one phase of work each: the code, its tests and
  its documentation together. The gate must be green, and the scoped test
  suites for the modules touched must pass.
- **Reviews** read the code against the specification. A review comment
  cites the section it holds the code to.

## Standards

The full standards are [`docs/MU-STANDARDS.md`](docs/MU-STANDARDS.md). The
essentials:

**Arithmetic**
- Every quantity is exact: ℚ, or the multivectors of Cl(4,1)(ℚ).
- Float arithmetic and lossy casts are denied by lint.
- No structure is projected to a scalar
  ([`docs/ANTI_FLATTENING_LAW.md`](docs/ANTI_FLATTENING_LAW.md)).

**Refusals**
- A reader that cannot answer returns a named refusal.
- `unwrap`, `expect` and `panic` are denied outside tests.

**Files**
- No source file exceeds 2500 lines, and each file holds one organ of the
  design.
- A `mod.rs` holds only declarations.
- Splits are move-only commits.

**Tests**
- A test is a claim, named in plain words, and proved by the algebra:
  - a refusal by name,
  - a fixed locus,
  - a replay,
  - an exact identity.
- Fixtures are built by the engine's own constructors.

**Formatting**
- The tree is not `rustfmt`-clean by design. Do not run `cargo fmt` over
  it; format only the lines you change.

**Headers**
- Every source file opens with the `AI-GENERATED CONTENT — see
  DISCLAIMER.md` header where the content was generated.
- A file's header names what the file is in the specification's terms.

**Commits**
- One phase per commit.
- The first line states the change as a claim.
- The body gives what changed, why, and how it was verified.

## The gate

`cd xtask && cargo run` runs the project's two gates:

1. **The reading bound:** the source-length limit.
2. **Clippy:** `cargo clippy --release --all-targets -- -D warnings`, with
   the project's disallowed types and methods (`clippy.toml`).

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
