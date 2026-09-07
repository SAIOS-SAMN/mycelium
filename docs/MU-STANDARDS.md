# MU-STANDARDS — how this tree is developed

Status: binding on every agent that writes, reviews, or audits this tree,
from 2026-09-07. `MU-RECURSION.md` holds the laws (what the engine is) and
the coding rules 1–16 (what a diff may not contain). This document holds
the development standard: how files are laid out, how things are named,
how tests are written and organised, how commits are made and verified,
which gates enforce which rules, and how an implementor and an auditor
work with each other and with the operator. Where this document and a
chapter of `MU-RECURSION.md` disagree, the chapter wins and this document
is corrected.

The reason for a written standard is stated once: an agent with no
standard does its own work in its own way, and the result is a tree of
one-off costumes, private gates, and tests that assert what their author
found convenient. Every item below was written after such a one-off was
found and removed.

## 1. Files and modules

1.1 **Bound.** No source file exceeds 600 lines (800 during Stage 8b of
`MU-RECURSION-TASKS.md`, lowered to 600 in that stage's last commit). The
bound is a gate (`xtask/lint/length.rs`) that fails the build
and names the file and its count. The bound exists so that a reader, human
or agent, reads a file whole in one pass; a file that has to be searched is
a file that is not audited.

1.2 **One organ per file.** A file holds one bound of the document: the
moves of one chapter, the codec of one act family, one carrier and its
impls, one stratum of the mind. Its first line is a comment naming that
chapter or act: `// 21 — extraction as moves, steps 1–7`. A file is named
for what it holds in the document's vocabulary (`refract.rs`, `carriage.rs`,
`admission.rs`, `lapse.rs`), never for its shape (`util`, `helpers`,
`misc`, `common`, `types`, `part2`, `extra`).

1.3 **`mod.rs` holds declarations.** A `mod.rs` contains `mod`, `pub mod`,
`pub use`, and nothing else. A `main.rs` of a binary contains argument
parsing, the boot order, and the loop skeleton, and nothing else.

1.4 **Sensor boundary.** The only files that may read or write a cell
coordinate pair are named `sensor.rs` (or are `family::{pair_form,
speak_rule, orbit}` until S9 lands) and are listed by name in the
squaring gate's allow-list (law 11). Nothing else touches `(i64, i64)`,
`(ℚ, ℚ)`, or a grid.

1.5 **Binaries are modules.** `src/bin/<name>/` with `main.rs` and one file
per act family or arm; the binaries are inside every gate's scope.

1.6 **Splits are move-only commits.** A split changes no line of any
function body; only `use`, `mod`, `pub use` and `#[path]` lines are new.
The suite count and the clippy count are identical before and after, both
quoted in the message, and every document path that pointed into the split
file is rebound in the same commit (§7.4). Four shapes are permitted inside
a move-only commit and each is declared in the message: an `impl` block
that cannot span files is opened again in the new file with the same
methods; `super::` paths become `crate::` paths when a file moves down a
level; a function that dispatched several acts in sequence becomes one
function per act plus a `match` that dispatches and holds nothing else;
a `main` that ran boot phases in sequence becomes one function per phase
plus the ordered calls. Anything beyond these four is a code change and
lands as its own commit.

1.7 **The bound during a split stage.** The gate is the bound plus a list,
`AWAITING_SPLIT`, naming every file still over it with the row that owes
its split. Each split deletes its own row; a stale row (a file already
under the bound, or gone) fails the build like an over-bound file. The
list is empty at the stage's last commit, and the bound then drops to its
final value.

## 2. Naming

2.1 **Types are nouns of the document.** `Form`, `Vertex`, `Transport`,
`Refraction`, `Carriage`, `ClassVector`, `Receipt`, `Convergence`. A type
named `Manager`, `Handler`, `Helper`, `Util`, `Data`, `Info`, `Context`,
`Wrapper` is returned.

2.2 **Functions are verbs of the document's moves.** `glue`, `rebase`,
`refract`, `pivot`, `regauge`, `unfold`, `orbit`, `speak`, `utter`,
`comprehend`, `replay`, `docket`, `witness`, `lapse`, `shed`. A function
name asserts an operation; its body performs that operation or refuses
(checklist item (l)). `compute_*`, `derive_*`, `classify_*`, `extract_*`,
`recover_*` are permitted only when the body computes, derives,
classifies, extracts, or recovers the named object; a body that emits a
label, a key, or a summary in its place is theatre and is returned.

2.3 **Predicates and readers.** `is_*` and `has_*` return `bool` and are
exposed at boundaries only; a `bool` is never branched on inside cognition
(law I of the Anti-Flattening Law). `*_of` reads a coordinate from a
carrier (`translator_of`, `class_of_vertex`). `rederive_*` recomputes from
content. `key()` returns A21 bytes; `address()` returns the M2 composite;
neither is ever a digest.

2.4 **Refusals.** A function that can refuse returns `Option<T>` when the
cause is unique and documented at the call site, otherwise `Result<T,
&'static str>` whose error is a `REFUSE_*` constant from chapter 34.1. New
refusal strings are added to 34.1 in the same commit. `unwrap()`,
`expect()`, `unwrap_or(default)`, `panic!`, and `unreachable!` do not
appear in cognition outside tests; `unreachable!` where the type system
already excludes the arm names the invariant in its message.

2.5 **Lanes.** μ⃗ lanes are addressed through `Lane::{S, G, T, H}` (or the
four-element array with a named index constant), never a bare `[2]`.

2.6 **Constants.** A literal constant is a wire tag (`*_TAG`, `ACT_*`), a
carrier size (`*_LEN`, `*_WIDTH`, `*_BITS`), or a letter index. Any other
number is measured or refused (rule 5); a constant named `*_MAX`,
`*_FLOOR`, `*_CAP`, `KAPPA`, `QUORUM`, `MAX_*` is returned.

2.7 **Modules and paths.** `snake_case` files, one word or two joined by
underscore, named for the chapter's object. The module path is the
document's binding: `cognition::court::lapse`, `sentence::pivot::snap`.

## 3. Tests

3.1 **A test is a claim.** Its name is the claim in plain words, one claim
per test, in the form the tree already uses:
`a_versor_and_its_negation_are_one_vertex_and_two_addresses`,
`one_vantage_dockets_escrows_and_mints_nothing_until_a_witness_rederives`,
`the_book_has_no_order`. Subject, verb, object; no `test_` prefix, no
`_works`, `_ok`, `_basic`, `_simple`, `_case1`. A test named in chapter 39
is named exactly as chapter 39 names it.

3.2 **A test verifies the move, never the outcome** (rule 13). It asserts
on content: `same_vertex`, `projective_eq`, coordinatewise equality of
class vectors, decoded keys, spoken cells, refusal strings, work counters
at N versus 4N. It never asserts on tape bytes, letters, `to_bytes()`
equality, counts standing in for properties, or `description()[i]` beyond
the version byte.

3.3 **Layout.** Tests live in `<module>/tests/<chapter>.rs`, one file per
chapter of `MU-RECURSION.md` they prove (`sentence/tests/12.rs`,
`cognition/tests/47.rs`). A test proving no chapter goes in
`tests/other.rs` and is listed in the commit message until a chapter is
named for it. `tests/common.rs` is the one shared file per module and
holds fixture constructors only (`grid`, `word`, `form_with_two_words`),
named for what they build. A gate recognises a test file by the owning
module's `#[cfg(test)] mod tests;` declaration, never by the file's name
or directory; a file merely named `tests.rs`, or one under `tests/` that
no module declares, is production code to every gate.

3.4 **Fixtures are built, never faked.** Addresses come from
`Element::address()` of a constructed form, keys from `key()`, forms from
`perceive`/`from_residual`/`extract_rule_with`. A hand-built `[1u8; 32]`
address, a zero versor, or an identity gauge standing in for a real one is
a costume and is returned; the only exception is a wire identifier
(`vk`) where the identity is routing, never content.

3.5 **Refusals and traces.** Every `REFUSE_*` string has a test that
produces it. Every log line of chapter 34 has a trace test where the line
can be produced in-process. Every gate has a test of its own
(`no_blake3_in_cognition`, `no_qmatrix_in_cognition`,
`no_coordinate_pair_outside_the_sensor`).

3.6 **Identities as properties.** The identities the document states
(`δ² = 0`, associativity of the geometric product, projective equality
either sign, `descends` as a product order, `dominant` uniqueness, A21
`decode(key) == object`) are property tests over generated exact-ℚ inputs,
in `<module>/tests/properties.rs`.

3.7 **What a test never does.** Sleep, read a clock, read the network,
touch the file system outside a temp dir it creates, depend on another
test, use `#[ignore]`, or be deleted when its claim still holds on the new
carrier (rewrite it; a test is deleted only when its claim is false under
the doctrine, and the commit message says which claim and why).

3.8 **Test deletion and weakening are review items.** The auditor lists
removed test names and removed assertions not re-added on every landing;
each needs a sentence in the message.

## 4. Gates

4.1 The build fails on: enumeration (`.entries()`, `.for_each(`,
`spend_unit()` in cognition; `.position(`, `.find(`, `.find_map(`,
`Vec<bool>`, `unwrap_or` under `sentence/`), maps (`HashMap`, `BTreeMap`,
`HashSet`, `BTreeSet` in cognition), digest (`blake3` outside `network/`,
`court/identity.rs`, `chronometry.rs`, persistence loaders), matrix
(`QMatrix`, `core::engine::{Delta, Operator}`, `core::matrix` in
cognition), clock (`SystemTime`, `Instant`, `Duration` in every kernel,
court, client, transducer, economy, physics file and every binary),
squaring (coordinate pairs outside the sensor allow-list), file length
(§1.1), rustc warnings (`RUSTFLAGS='-D warnings'`), clippy warnings
(`cargo clippy --release --all-targets -- -D warnings`).

4.2 A gate's scope is the whole of `src/` including `src/bin/`; an
allow-list is by file or function name, never by directory alone.

4.3 A gate lands in the same commit as the rule it enforces, with its own
test. An `#[allow(...)]` or `#[expect(...)]` anywhere is a review item
named in the commit message with its reason; an allow that hides a rule
violation is returned.

4.4 Gates owed (recorded, not yet built): rules 2 (count as branch), 4
(choice by scalar), 10 (loop without budget or boundary), 11 (`unwrap_or`,
`saturating_*` outside accounting floors, `expect` in production), 15
(`.first()`/`.last()` outside wire tags and chain heads). Until they exist,
the auditor's grep over production code is the check, on every landing.

4.5 A gate that is handed a path names a hit when the path does not exist;
it never returns quietly. A gate's allow-list entry for a file that no
longer exists is a hit as well, the same rule as a stale `AWAITING_SPLIT`
row. Stage 8b moved `economy.rs` and `court/verdict.rs` out from under the
digest and clock gates and nothing fired (TASKS Stage 8c, A1); a gate whose
scope is a list of paths is only as alive as its list.

## 5. Commits

5.1 **One item per commit.** The item is a row of `MU-RECURSION-TASKS.md`
or a numbered directive. A commit that lands two fixes is a bundle even
when each half is green; it is returned and re-cut.

5.2 **Message.** First line: `<id>: <claim in one sentence>.` Body: what
changed and why it is doctrine (chapter numbers); the test names added,
rewritten, or deleted, with the claim of each deleted one; the
`test result` lines of the full suite; the clippy line; `xtask lint`
line; the document paths rebound; every `pub(crate)` widening; every
`#[allow]`; every departure from the directive as given (§8.3). A
move-only commit says `move only` on its first line.

5.3 **Never committed.** Build artifacts (`target/`, `xtask/target/`),
fleet state (`run/`, `~/.mycelium`), editor files, secrets. `.gitignore`
covers them; a commit that adds one is amended before anything else lands.

5.4 **Pushing and history.** Nothing is pushed and no history is rewritten
without the operator's word. A commit that must be removed from history
(an artifact, a secret) is reported, and the operator decides.

5.5 **Docs commits.** A document change lands in its own commit the same
day as the code it describes; a doctrine change (§9) lands before the code
that implements it.

## 6. Verification of a landing

6.1 The implementor runs, in the foreground and in this order, on the
commit as it will land: `RUSTFLAGS='-D warnings' cargo test --release`
(every target), `cargo clippy --release --all-targets -- -D warnings`,
`xtask lint`. All three lines go in the message. "Lint clean" alone is not
green.

6.2 The auditor reproduces all three on the same commit, in a detached
worktree if the implementor's tree is dirty, and reads every changed file
whole. A landing is called verified only after that read; a report is
never the object of an audit, the code is.

6.3 The cold-boot trace of chapter 40 for the item is reproduced before the
next item lands (chapter 41's consolidation discipline). A landing that
changes admission, the wire, or the market is followed by a whole-fleet
recut before its readings are used.

6.4 Fleet readings are taken on one binary: a fleet whose voices and
hyphae run different commits produces no reading.

## 7. Documentation binding

7.1 Every function, path, and log line the document names exists at HEAD.
Chapter 29's table by file and Part IV's absolute paths are updated in the
same commit as any move.

7.2 A new refusal string enters 34.1, a new log line enters 34.2 or 34.3,
a new test that proves a theorem enters 39 or 42, in the commit that adds
it.

7.3 A row of `MU-RECURSION-TASKS.md` is opened before work starts and
closed by the auditor, not by the implementor; the implementor writes
`landed (<commit>)` and the auditor writes `done` after reproduction.

7.4 A split (§1.6) rebinds paths in the same commit and is not green until
`grep -o 'src/[a-z_/]*\.rs' docs/MU-RECURSION.md | sort -u` names only
files that exist.

## 8. Agents

8.1 **The implementor** follows the directive order as given. A blocker
found mid-order is reported with the evidence and the proposed reorder;
the reorder is the auditor's or the operator's, not the implementor's. A
measurement that needs code lands as its own commit, named
`instrumentation:`, containing trace lines only and no behaviour change.
Nothing outside the repository is changed (unit files, drop-ins,
environment, cron, the operator's home) without asking first and saying
what and why. Reverts are not a move; a wrong landing is corrected
forward by a commit that says what was wrong.

8.2 **The auditor** reads code, not reports; reproduces every number;
lists removed tests and removed assertions on every landing; greps the
whole production tree for the ungated rules on every landing; checks the
binaries and the machine (units, drop-ins, running binary versus HEAD);
files findings as rows with file and line; rules only from the document
and names the chapter.

8.3 **Reporting.** A report states, in this order: commits (id, claim,
suite line, clippy line); what landed against which rows; what is open;
every departure from the directive, named as a departure; every change
outside the repository; what the fleet shows, on which commit, since when.
A report that omits a departure is the departure.

8.4 **Ambiguity.** An agent that cannot tell which of two readings a
directive means asks before implementing either; it does not implement
one and mention the other.

8.5 **No private infrastructure.** No agent adds its own gate, lint,
fixture style, naming scheme, test harness, trace format, or script that
this document and `MU-ENGINEERING.md` do not name. A needed addition is
proposed as a change to this document first.

## 9. Doctrine changes

The semantics of a chapter (what a move does, what a carrier holds, what
the economy pays, what time is) change only by the operator's ruling
recorded in `MU-RECURSION.md`, `MU-ECONOMY.md`, or `XSEPTUM.md` before
the code changes. A commit that changes behaviour the document specifies
otherwise is a doctrine change made in code and is returned, however
green. The backtrack ledger in `MU-RECURSION-TASKS.md` records the cost of
each such drift found after the fact.

## 10. The recut

The recut is a procedure the operator runs from a script in the
repository (`run/recut.sh`, to be landed with 5.2g): stop 1–5 then c, b,
a; wipe ledgers, genomes, thoughts, registry, court federation; keep keys,
mesh, corpora; `--mint-genesis` on hypha a once; start a, b, c then 1–5.
No flag of the recut lives in a unit file or a drop-in; a unit starts the
binary with `--home` and nothing else.
