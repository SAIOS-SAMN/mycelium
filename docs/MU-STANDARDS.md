# MU-STANDARDS — how this tree is developed

> **Scope.** Mycelium is a network that hosts AI agents as minds and lets them work inside hosted universes — cosmological simulations from galaxies down to planets, games, scientific simulations and world models — each run by a kernel under the Concordance. Peer kernels are listed in README › Peer kernels. In this document, *minds* are those software agents; *worlds* and *planets* are simulated places, and *peoples*, *civilizations* and *ancients* simulated inhabitants, of hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).
>
> **Components:** the development of this tree, every component alike. No game term.

This is the development standard for every contribution to Mycelium: how
files are laid out and named, how arithmetic and refusals are written, how
tests are written and run, what the gate checks, and how commits and
documents are made. [CONTRIBUTING.md](../CONTRIBUTING.md) summarises it
and describes the contribution workflow.

[`MU-RECURSION.md`](MU-RECURSION.md) holds the laws (what the engine is),
the coding rules 1–17 (what a diff may not contain) and the review
checklist applied to every diff. Where this document and a chapter of
`MU-RECURSION.md` disagree, the chapter governs and this document is
corrected.

## 1. Files and modules

1.1 **The reading bound.** No source file under `src/` or `xtask/`
exceeds 2500 lines. The gate checks it (§4.1) and names each file over the
bound with its line count. There is no list of exceptions: a file over
the bound is split before anything else lands in it. The bound exists so
that a reviewer reads a file whole; it never decides where an organ is
split (1.2).

1.2 **One organ per file.** A file holds one bound of the specification:
the moves of one chapter, the codec of one act family, one carrier and its
impls, one stratum of the mind. A file is named for what it holds in the
specification's vocabulary (`refract.rs`, `carriage.rs`, `admission.rs`,
`lapse.rs`), never for its shape (`util`, `helpers`, `misc`, `common`,
`types`, `part2`, `extra`). The one exception is a module's
`tests/common.rs` (§3.3).

1.3 **`mod.rs` holds declarations.** A `mod.rs` contains `mod`, `pub mod`,
`pub use` and `#[cfg(test)] mod tests;`, and nothing else. A binary's
`main.rs` contains argument parsing, the boot order and the loop
skeleton, and nothing else.

1.4 **The sensor boundary** (law 11). Only the sensor reads or writes a
site's integer coordinates. The permitted readers are the functions of
`transducer::element::sensor` (`site_of`, `site_at`, `carry_centre`,
`carry_compartment`, `carry_site`, `carry_site_centre`,
`carry_compartment_site`, `pullback`), `transducer::element::letters::carriage`,
`transducer::blade::carry::carry`, `Blade::centroid` in
`transducer/blade/body.rs`, `transducer::alphabet::letters::rotor_pair`,
the rotor transition `rotor_between` in `transducer/bubble/contemplation.rs`,
and the base sites of `transducer::base`. Nothing else touches a site's
integers, a centre's rationals or a grid. This is a review rule (§4.4),
not a gate.

1.5 **Binaries are modules.** A binary lives in `src/bin/<name>/` with a
`main.rs` and one file per act family or arm (`mycelium-server`,
`mycelium-client`, `mycelium-soak`). Binaries are inside the gate's scope.

1.6 **Splits are move-only commits.** A split changes no line of any
function body; only `use`, `mod`, `pub use` and `#[path]` lines are new.
The test results and the clippy result are identical before and after,
and every document path that pointed into the split file is rebound in
the same commit (§7.3). Four shapes are permitted inside a move-only
commit, each declared in the message:

- an `impl` block that cannot span files is opened again in the new file
  with the same methods;
- `super::` paths become `crate::` paths when a file moves down a level;
- a function that dispatched several acts in sequence becomes one
  function per act plus a `match` that dispatches and holds nothing else;
- a `main` that ran boot phases in sequence becomes one function per
  phase plus the ordered calls.

Anything beyond these four is a code change and lands as its own commit.

## 2. Naming and code

2.1 **Types are nouns of the specification.** `Form`, `Vertex`,
`Transport`, `Refraction`, `Carriage`, `ClassVector`, `Receipt`. A type
named `Manager`, `Handler`, `Helper`, `Util`, `Data`, `Info`, `Context` or
`Wrapper` is not accepted. A tuple named by a `type` alias is not a named
type: its coordinates get names.

2.2 **Functions are verbs of the specification's moves.** `glue`,
`rebase`, `refract`, `pivot`, `regauge`, `unfold`, `orbit`, `speak`,
`comprehend`, `replay`, `witness`, `lapse`. A function name asserts an
operation; its body performs that operation or refuses.
`compute_*`, `derive_*`, `classify_*`, `extract_*` and `recover_*` are
permitted only when the body computes, derives, classifies, extracts or
recovers the named object; a body that emits a label, a key or a summary
in its place is a stand-in.

2.3 **Predicates and readers.** `is_*` and `has_*` return `bool` and are
exposed at boundaries only; a `bool` is not branched on inside cognition
(Anti-Flattening Law, constraint I). `*_of` reads a coordinate from a carrier.
`rederive_*` recomputes from content. `key()` returns the object's
canonical coordinates and `address()` its composite address; neither is a
digest. A comparison of two polytonic objects (two μ⃗, two `Rank`, two
`ClassVector`, two reserves) returns the reading, never a verdict:
per-lane moves (`LaneMove`), a shortfall per lane. A `bool` over four lanes
keeps neither order nor lane, and every caller that needs to act
recomputes what the bit threw away.

2.4 **Refusals.** A function that can refuse returns `Option<T>` when the
cause is unique and documented at the call site, otherwise a `Result`
whose error names the refusal: a `REFUSE_*` constant (MU-RECURSION
chapter 34.1, `src/court/verdict/refusals.rs`) or a typed refusal. A new
refusal string enters chapter 34.1 in the same commit.

*Cognition refuses, it does not abort.* `unwrap_used`, `expect_used`,
`panic`, `unreachable`, `todo` and `unimplemented` are denied in
`Cargo.toml`; `clippy.toml` allows `unwrap`, `expect` and `panic` inside
`#[cfg(test)]`. An integration test crate under `tests/` is a crate of its
own that those settings do not reach, so it opens with a root
`#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, reason = "…")]`
and allows nothing else. `unsafe_code` is denied.

*The lawful default.* A default value stands only where the law itself
assigns that value at that point (the empty sum is 0, the empty product
1, an empty enclosure holds nothing), named at its site with an
`#[expect(..., reason = "<the law>")]`. Arithmetic convenience never
qualifies: a division by zero is never 0 or 1 unless the law names that
value there. Everything else is a refusal by name. These rules bind every
line a developer writes and never limit what a mind may count or rank
([MU-CONCORDANCE.md](MU-CONCORDANCE.md) 1.10).

2.5 **Lanes.** μ⃗ lanes are addressed by the named index constants
(`LANE_S`, `LANE_G`, `LANE_T`, `LANE_H`), never a bare `[2]`.

2.6 **Constants.** A literal constant is a wire tag (`*_TAG`, `ACT_*`), a
carrier size (`*_LEN`, `*_WIDTH`, `*_BITS`) or a letter index. Any other
number is measured or refused (rule 5); a constant named `*_MAX`,
`*_FLOOR`, `*_CAP`, `MAX_*` or `QUORUM` is not accepted.

2.7 **Modules and paths.** `snake_case` files, one word or two joined by
an underscore, named for the chapter's object. The module path is the
specification's binding: `server::cognition`, `transducer::sentence::pivot`.

2.8 **Headers and comments.** A source file opens with the
`AI-GENERATED CONTENT — see DISCLAIMER.md` header where the content was
generated, then a header of at most three lines saying what the file is
in the specification's words: the object it carries and the chapter that
binds it. A comment states what the code does now and the chapter that
says why. It carries no history, no measurement, no commit id, no
deployment reading and no permission ("rule 1 allows this"): a comment
cannot allow anything. What a line used to do is in git. A contributor who
touches a function removes such comments from it in the same commit.

2.9 **Exact arithmetic.** Every quantity is exact: ℚ, or the multivectors
of Cl(4,1)(ℚ). `float_arithmetic`, `cast_precision_loss` and
`cast_possible_truncation` are denied in `Cargo.toml`; a narrowing `as`
stands only under an `#[expect]` naming where its bound comes from. A
decimal approximation of an irrational is not served anywhere: in a value,
a printed reading, a doc line or a test name. A turn is written in degrees
where exact, otherwise as its rational point. No structure is projected to
a scalar ([MU-ANTI-FLATTENING.md](MU-ANTI-FLATTENING.md)).

2.10 **The code vocabulary.** One spelling per concept across the tree:

- *Refusals*: a refusal type is an `<Object>Refusal` enum with named
  variants (`Not<State>`, `Unknown<Noun>`, `Short { lane }`, `Repeated`,
  `Mismatched`, `Malformed`, `Unsigned`; `<Noun>Short` where one type
  names two shortfalls), or an `<Object>Refusal` struct where the refusal
  carries its coordinate beside its reason; a free-text reason is a
  `REFUSE_*` constant (§2.4). Never `Error`, `Fail`, `Failed` or `Bad*`.
  A struct of counts is a `*Tally`.
- *Identity*: `address` / `Address` is an object's composite coordinates;
  `key` its store coordinates; `citation` a 32-byte payload hash (in
  place of `id()`); `endpoint` a TCP address string (never `addr`).
- *Codecs*: `put_*` and `take_*` write and read byte carriers;
  `encode_*` and `decode_*` are wire acts; a one-byte enum has `byte()`
  and `of(u8)`; removal from a queue is `drain_*`.
- *Readers*: `<object>_of` reads a coordinate (§2.3); `<object>_reading`
  returns a typed reading.
- *Economy*: the court's `pay_<arm>` calls the economy's `settle_<arm>`,
  which calls `mint_to`. "Mint" names μ only.
- *Spelling*: British (`fibre`, `centre`, `colour`, `neighbour`,
  `favour`); compounds are `off_plane`, `non_versor`.

2.11 **Person words.** No file name, identifier, comment, doc line, test
name or log string carries a person or possessive word: I, me, my, mine,
we, us, our, ours, you, your, yours, their, theirs. A sentence is
rewritten with "its", "the X's", a plural noun or another structure;
"its" and "own" are allowed. Mathematical *I* and Roman numerals are not
pronouns.

## 3. Tests

3.1 **A test is a claim; its name names the unit and the behaviour.** One
claim per test. The name is `<unit>_<behaviour>`:
- **unit**: the function, type or act under test, spelled as the code
  spells it (`pay_lease`, `signed_prefix`, `learn_form`, `noise_home`);
- **behaviour**: a short verb phrase, what the unit does or refuses and
  under which condition, in at most about six words.

No articles (`a`, `an`, `the`), no narrative, no internal row ids, no
person or possessive words (listed below), no `test_` prefix, no `_works`, `_ok`,
`_basic`, `_simple`, `_case1`. The module path supplies the area
(`court::admission::tests::receipts::…`), so a name does not repeat it.
The full claim, in plain sentences, is the test's `///` doc comment.

The person and possessive words a name never carries are I, me, my,
mine, we, us, our, ours, you, your, yours, their and theirs; a name
carries no task hash either.

The behaviour opens with a third-person singular verb (`pays`, `refuses`,
`keeps`). These verbs carry the standard claims, one spelling each:

- `refuses_<input>`: the input is refused by name (never `_by_name`);
- `round_trips`: decoding the encoding gives the object back (never
  `_whole`, `_exactly` or `_on_wire`; `reads_back` and `parses_back` are
  retired);
- `ignores_<input>`: the input leaves the result unchanged;
- `is_invariant_under_<action>`: the action leaves the reading fixed;
- `agrees_with_<route>`: two routes reach the same object;
- `equals_<closed form>`: the result is the stated closed form
  (`matches` is retired);
- `keeps`, `fixes`, `survives_<event>`, `conserves_<quantity>`: what an
  operation leaves held;
- an adjective claim takes `is_` (`is_exact`, `is_invariant_under_…`).

| Name | |
|---|---|
| `pay_lease_pays_each_job_once` | names the unit and its behaviour |
| `signed_prefix_round_trips_and_refuses_extra_bytes` | the round trip and the refusal |
| `a_lease_pays_its_provider_for_each_verified_job_once` | not accepted: a sentence; the claim belongs in the doc comment |

A test cited in MU-RECURSION is cited by its current name; a rename
updates every citation in the same commit.

3.2 **Proved by the algebra.** A test asserts a law, a theorem or an
algebraic identity on typed values the engine computes, by one of:

- a **refusal by name**: the input that must be refused is refused with
  the named reason;
- a **fixed locus**: the object an operation must leave fixed is fixed;
- a **replay or reconstruction**: the object re-derived from its record
  equals the object (`decode(key) == object`);
- an **exact identity** the specification states (`δ² = 0`, associativity
  of the geometric product, projective equality of either sign), over
  exact ℚ inputs.

A test verifies the move, never the outcome (rule 13). It never asserts on
tape bytes, `to_bytes()` equality, a printed string, a rendered line, a
census diff or a count standing in for a property. A string may appear
in an assertion's message, never as what is compared. A test that asserts
nothing and prints is not a test.

3.3 **Layout.** Unit tests live in `<module>/tests/<name>.rs`, declared by
the owning module's `#[cfg(test)] mod tests;`, one file per organ they
prove, named for what its tests claim (`sentence/pivot/tests/orbit.rs`,
`cognition/tests/ledger.rs`), never for a chapter number. A module's
`tests/common.rs` holds fixture constructors only, named for what they
build. Integration tests live in the crate's `tests/` directory.

3.4 **Fixtures are the engine's constructs.** Addresses come from
`address()` of a constructed form, keys from `key()`, forms from the
engine's own perception and extraction. No mock, fake, stub or double of
an engine organ, and no carrier field set by hand to a value the engine
would not produce. A hand-built `[1u8; 32]` address, a zero versor or an
identity gauge standing in for a real one is a stand-in; the one exception
is a wire identifier, where the identity is routing and not content.

3.5 **Refusals have tests.** Every `REFUSE_*` string has a test that
produces it. The reading bound has a test of its own
(`xtask/lint/length.rs`). A new clippy lint entry is proved once, at the
commit that adds it, by a scratch line the lint refuses, built and then
removed.

3.6 **Identities as properties.** An identity the specification states in
general is a property test over generated exact-ℚ inputs, in
`<module>/tests/properties.rs`.

3.7 **What a test never does.** Sleep, read a clock, read the network,
touch the file system outside a temporary directory it creates, depend on
another test, or carry `#[ignore]` (the tree has none). A test that no
longer holds under the specification is rewritten to the new claim or
deleted in the commit that meets it, and the commit message names the
claim it dropped. A test whose claim still holds on a new carrier is
rewritten, never deleted.

3.8 **The geometry is the engine's, `Cl(n+1,1)(ℚ)`.** No test and no code
decides by a Euclidean oracle: no orthogonal matrix, no `Σx²`, no dot
product and no `RMRᵀ` standing for an action or for the equality of two
shapes (rule 16: an action is an algebra element, never a matrix). Two
bodies are related by the conformal versor between them, known by its
action.

3.9 **Scoped suites.** A change runs the suites of the modules it
touches, by filter, not the whole suite:

```sh
cargo test --release --lib -- <module path filter>...
```

A run is green only when its saved output contains no `FAILED`,
`panicked` or `error` line. `scripts/ship-scoped <filter>...` runs clippy
and the scoped tests in sequence. Before a pull request, `scripts/check`
runs the gate (§4.1) and the workspace tests with `--no-fail-fast` and
summarises both; without the ARC-AGI-2 corpus at `data/arc-agi-2`, its
17 corpus-dependent tests are known exceptions, and the summary says so.

## 4. The gate

4.1 **Running it.** The gate is the `gate` binary of `xtask/`:

```sh
(cd xtask && cargo run)
```

Setting `CARGO_TARGET_DIR` (for example `CARGO_TARGET_DIR=target/gate`) is
optional and keeps the gate's build apart from a release build. There is
no `build.rs`; the gate runs only from `xtask`. It takes two readings,
reports both whatever the first said, and exits non-zero if either
refuses:

1. **The source checks** (`xtask/lint/`), over `src/` and `xtask/`:
   - *the reading bound* (`lint/length.rs`): every `.rs` file over 2500
     lines is named with its count;
   - *the Euclidean vocabulary* (`lint/euclidean.rs`): a code line (not a
     comment or attribute line) naming a grid-indexing shape is refused
     with its reason: Morton and Z-order curves, Hilbert curves, bit
     interleaving, bounding boxes (`bbox`, `bounding_box`, `aabb`),
     subdivision trees (`quadtree`, `octree`, k-d trees, R-trees),
     spatial hashes, grid cells and voxels. It reads names, not structure.
2. **The warning gate** (`lint/warnings.rs`):
   `cargo clippy --release --all-targets -- -D warnings`, every target,
   tests included, rustc's warnings denied. A warning is a defect with a
   line number.

4.2 **The pattern rules are clippy configuration.** The crate-wide
denials are in `Cargo.toml` under `[lints.rust]` and `[lints.clippy]`;
`clippy.toml` lists the disallowed types and methods, each entry carrying
as its `reason` the rule it enforces:

| rule | lint |
|---|---|
| §2.4, no abort in cognition | `unwrap_used`, `expect_used`, `panic`, `unreachable`, `todo`, `unimplemented` |
| §2.9, exact arithmetic | `float_arithmetic`, `cast_precision_loss`, `cast_possible_truncation` |
| law 2, rule 9: no map, no digest | `disallowed_types`: `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`; `disallowed_methods`: `blake3::hash`, `blake3::Hasher::new` |
| law 9: no wall clock | `disallowed_types`: `Instant`, `SystemTime`, `Duration` |
| rule 4: nothing chosen silently by a scalar in engine code | `disallowed_methods`: `Iterator::{max, min, max_by, min_by, max_by_key, min_by_key}`, `slice::{sort_by, sort_by_key, sort_unstable_by, sort_unstable_by_key}` |
| rule 11: no invented value | `disallowed_methods`: `Option`/`Result` `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`; `saturating_sub`, `saturating_add`, `saturating_mul` on the unsigned integers |
| rule 15: no member by position | `disallowed_methods`: `slice::first`, `slice::last`, `Iterator::nth` |

Rule 4 binds engine code only. A comparison a mind's rule states (largest,
rank, majority, in an order it names) carries
`#[expect(clippy::disallowed_methods, reason = "Concordance 1.10 — …")]`.

4.3 **Allowances are at the site.** Where a denied lint fires on a
deliberate choice, the site carries
`#[expect(clippy::<lint>, reason = "<rule> — <why this site>")]`.
`expect` fails the build when the lint stops firing, so an allowance that
allows nothing is a build failure. There is no module-level allowance
(the one exception is the test-crate scope statement of §2.4), and no
file is named in any configuration.

4.4 **What the gate cannot read is a type or a review item.** Where a
type can hold a rule, it does: `Keyed::fibre` takes a `Prefix`, which
cannot be empty, so a whole carrier is read only through `Keyed::cursor`
under a `Budget`; `Mu` implements neither `PartialEq` nor `PartialOrd`, so
two lane vectors are compared only by `lane_moves`. Everything else is
read at review against the coding rules and the review checklist of
`MU-RECURSION.md`: a pair loop over one carrier (rule 1), a count deciding
a branch (rule 2), a loop without a `Budget` or a boundary (rule 10), a
coordinate pair above the sensor (§1.4), a carrier cloned into two places
(rule 12), a matrix carrying an action (rule 16).

4.5 **The tree is not rustfmt-clean.** Its layout follows the
specification's structure, and one `cargo fmt` run rewrites hundreds of
files. Never run `cargo fmt` over the tree. Format only the lines a change
touches, in the style of the surrounding code.

4.6 **No new gate by other means.** A new check is a lint entry under 4.2
with its proof, a type under 4.4, or a proposed change to this document.
A walker over source text beyond the two of 4.1 is not added. Where
`MU-RECURSION.md` or `MU-ENGINEERING.md` names "the enumeration gate",
"the clock gate", "the blake3 gate", "the squaring gate", "the order
gate" or "the panics gate", the name means the row of 4.2 or the type or
review item of 4.4 that carries that rule.

## 5. Commits

5.1 **One phase per commit.** A phase is one claim the engine can be
reviewed on: its code, its tests and its documentation together. A commit
holding two claims is two commits. Each move-only split is a commit of
its own (§1.6).

5.2 **Message.** The first line states the change as a claim, in one
sentence. A move-only commit says `move only` on its first line. The body
gives what changed and why (with chapter numbers), the tests added,
rewritten or deleted (with the claim of each deleted one), how it was
verified (§6.1), the document paths rebound, and every `#[expect]` or
`#[allow]` added.

5.3 **Never committed.** Build artifacts (`target/`, `xtask/target/`),
runtime state, keys and other secrets, and editor files. `.gitignore`
covers them.

5.4 **Corrections go forward.** A wrong change is corrected by a new
commit that says what was wrong. Published history is not rewritten.

## 6. Verification

6.1 **Before a commit.** Run the scoped suites of the modules touched
(§3.9), then the gate (§4.1). Quote the suites' `test result` lines and
the gate's exit in the message. "Lint clean" alone is not green.

6.2 **A review reads the code.** A reviewer reads every changed file
whole and checks the wiring (every new reader has a production caller and
a producer feeding it, so nothing built is a type, a line or a fixture
standing in for a mechanism), the coding rules and the review checklist,
and the carrier the change writes into and reads from: a write onto a
retired carrier, or a positional read of a keyed one, is returned.

6.3 **Verification never writes.** A check reads: `git diff`, `git show`,
`git log`, a build, a test run. A probe that must change source (a line to
prove a lint fires) is made in a separate worktree, never in the tree
being committed.

## 7. Documentation binding

7.1 **The documents name what exists.** Every function, path and log line
a document names exists at the commit that names it.

7.2 A new refusal string enters MU-RECURSION 34.1, a new log line 34.2 or
34.3, and a test that proves a theorem chapter 39 or 42, in the commit
that adds it.

7.3 A move or a split rebinds every document path into the moved file in
the same commit, and is not green until every `src/…` path the documents
name exists.

## 8. Contributions

8.1 **Instrumentation is its own commit.** A measurement that needs code
lands as its own commit, named `instrumentation:`, containing trace lines
only and no behaviour change.

8.2 **No private infrastructure.** No contributor adds a gate, lint,
fixture style, naming scheme, test harness, trace format or script that
this document does not name. A needed addition is proposed as a change to
this document first.

8.3 **Ambiguity is asked.** A contributor who cannot tell which of two
readings a specification means asks, in the issue, before implementing
either.

## 9. Specification changes

9.1 **Specifications govern behaviour.** The semantics of a chapter (what
a move does, what a carrier holds, what the economy pays, what time is)
change only by an amendment to the governing document (`MU-RECURSION.md`,
`MU-CLIENT.md`, `MU-ECONOMY.md`, `MU-XSEPTUM.md`), made before or with the
code. A commit that changes specified behaviour otherwise is returned,
however green.

9.2 **Proposals.** A change of behaviour starts as a proposal in
[`docs/proposals/`](proposals/) and is amended into the governing
document when it is accepted. A ruling amends the chapter that governs
its object; a new subsection is opened only for a new object.

## 10. Persisted layouts

A change to a persisted layout (a snapshot, a line, a genome) gets a new
layout tag. The superseded tag is refused by name, and no migration or
reader for it is written: the tree holds no reader for `MYC7`, `MYT1`,
`GEN4`, `GEN5` or `GEN6`. Keys, the mesh record and the corpus manifests
are configuration, not state; the mesh record's `MSH1` is read and
written back as `MSH2`. `MU-ENGINEERING.md` §8 lists the layouts.
