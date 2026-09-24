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

1.4 **Sensor boundary.** The only files that may read or write a site's
coordinates are named `sensor.rs` (or are `family::{extract_rule_with,
speak_rule, orbit}` until S9 lands), and the functions are
`element::sensor::{site_of, carry_center, carry_lamina}`,
`element::letters::carriage`, `blade::carry::{carry, rotor_between,
half_rotor}`, `blade::body::centroid`, `alphabet::letters::rotor_pair`,
`cga::base` (law 11). Nothing else touches a site's integers, a centre's
rationals, or a grid. This list is the review's, not a gate's (§4.4).

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

1.7 **No list of exceptions to the bound** (ruled 2026-09-12). A file
over the bound fails the build and is split before anything else lands
in it. `AWAITING_SPLIT` is retired with the gates (§4).

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
neither is ever a digest. A comparison of two polytonic objects — two μ⃗,
two `Rank`, two `ClassVector`, two reserves — returns the reading, never a
verdict: per-lane moves (`Fell`/`Held`/`Rose`), a shortfall per lane, a
`RankMove`. A `bool` over four lanes is the maximal flattening: it keeps
neither order nor lane, and every caller that needs to act recomputes what
the bit threw away (`fits_book` → `log_room_refused` recomputing the room
is the recorded instance). Ruled 2026-09-07, Stage 9.

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

2.8 **Headers and comments.** A file's header says what the file is, in
the document's words, in at most three lines: the object it carries and
the chapter that binds it. It carries no history — what was deleted, when,
and why is in git and on the backtrack ledger — and no claim about the
tree beyond the file. A header that calls the coefficient field "the
scalar the engine counts in" states the collapse law 5 forbids: ℚ is the
field the coordinates of a multivector are written over; the engine
counts in μ⃗ per lane, in versors and blades, in ℤ-ranks.

The same rule binds every comment in a body (ruled 2026-09-12). A comment
states what the code now does, in the document's words, and the chapter
that says why. It carries no measurement, no commit id, no fleet reading,
no "this was", no story of how the line was found, and no permission
("rule 1 allows this"): a comment cannot allow anything, and the one that
claimed to allow a pair loop was read by no tool and no reviewer while
the loop took the fleet down. What a line used to do is in git; what it
cost is on the ledger. The tree's prevailing inline style was the defect,
not a limit on this section's reach: an agent that touches a file removes
such comments from the functions it touches, in the same commit.

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
class vectors, decoded keys, spoken laminae, refusal strings, work counters
at N versus 4N. It never asserts on tape bytes, letters, `to_bytes()`
equality, counts standing in for properties, or `description()[i]` beyond
the version byte.

3.3 **Layout.** Tests live in `<module>/tests/<function>.rs`, one file
per function of the organ they prove, named for what the tests in it claim
in the document's vocabulary (`sentence/tests/pivot.rs`,
`sentence/tests/tape.rs`, `cognition/tests/ledger_clock.rs`,
`admission/tests/receipts.rs`); the file's first line names the chapter
the claims come from. A file is never named for a chapter number: a number
says where the claim was read, not what it is, and the tree renames when
the document renumbers (ruled 2026-09-08; f799557 and f179bdd carried the
renaming). `tests/common.rs` is the one shared file per module and holds
fixture constructors only (`grid`, `word`, `form_with_two_words`), named
for what they build. A gate recognises a test file by the owning module's
`#[cfg(test)] mod tests;` declaration — in its `mod.rs`, or in `main.rs`
when the module is a binary (§1.5) — never by the file's name or
directory; a file merely named `tests.rs`, or one under `tests/` that no
module declares, is production code to every gate.

3.4 **Fixtures are built, never faked.** Addresses come from
`Element::address()` of a constructed form, keys from `key()`, forms from
`perceive`/`from_residual`/`extract_rule_with`. A hand-built `[1u8; 32]`
address, a zero versor, or an identity gauge standing in for a real one is
a costume and is returned; the only exception is a wire identifier
(`vk`) where the identity is routing, never content.

3.5 **Refusals and traces.** Every `REFUSE_*` string has a test that
produces it. Every log line of chapter 34 has a trace test where the line
can be produced in-process. The reading bound has a test of its own; a
lint entry is proved once, at the commit that adds it (§4.2).

3.6 **Identities as properties.** The identities the document states
(`δ² = 0`, associativity of the geometric product, projective equality
either sign, `descends` as a product order, `dominant` uniqueness, A21
`decode(key) == object`) are property tests over generated exact-ℚ inputs,
in `<module>/tests/properties.rs`.

3.7 **What a test never does.** Sleep, read a clock, read the network,
touch the file system outside a temp dir it creates, depend on another
test, use `#[ignore]`, or be deleted when its claim still holds on the new
carrier (rewrite it; a test is deleted only when its claim is false under
the doctrine, and the commit message says which claim and why). A test
that asserts a scene extracts as one form asserts a claim chapter 53
makes false, and is rewritten to the charts' forms, never deleted (ruled
2026-09-19).

3.8 **Test deletion and weakening are review items.** The auditor lists
removed test names and removed assertions not re-added on every landing;
each needs a sentence in the message.

3.8 **No ignored tests, and stale tests leave as they are met** (ruled
2026-09-24, the operator's word). A test marked `#[ignore]` is not kept:
it is a claim nobody checks, and §3.7 refuses it. A test is valid — it
reads what the tree does now under the rulings that stand — or it is
deleted, in the commit that meets it, with the claim it dropped named
in the message. No test audit is opened for this and no development
stops for it: the implementor removes a stale test when a change brings
them to it, and the auditor returns a stale test found in a read.

## 4. Gates

Ruled 2026-09-12 by the operator, replacing this section as written on
2026-09-07 and amended on 2026-09-10. The line-matching gates of
`xtask/lint/` are deleted: eighteen files that read source as text with
every comment erased, six debt mechanisms for one idea, some two hundred
hand-kept rows of file path and hit count, and a clique gate for two
spellings written after the second clique. They passed both pair loops
that took the fleet down, permitted the whole-carrier walk under one name
while refusing it under another, and carried a red test of their own for
a week because the gate command did not run it. A rule is checked where
the compiler reads it, or it is a type, or it is a review item. Nothing
in between exists again.

4.1 **Two gates remain.** The reading bound (`xtask/lint/length.rs`: no
source file over 1200 lines, no list of exceptions; a raise to 2500
landed 2026-09-19 on a word that did not pass through the coordinator's
console and was returned) and the warning gate
(`cargo clippy --release --all-targets -- -D warnings`, rustc's warnings
denied). `cd xtask && cargo run` runs the bound and then clippy and exits
non-zero on either. There is no `build.rs`. Closing a warning by naming a
tuple in a `type` alias is not closing it (§2.1): the coordinates get
names, or the warning stands.

4.2 **The pattern rules are clippy configuration.** Every rule the
compiler can read is a lint, denied crate-wide under `[lints.clippy]` in
`Cargo.toml` or listed in `clippy.toml`, each entry carrying as its
`reason` the rule it enforces:

| rule | lint |
|---|---|
| §2.4, no abort in cognition | `unwrap_used`, `expect_used`, `panic`, `unreachable`, `todo`, `unimplemented` |
| law 2, rule 9: no map, no digest | `disallowed_types` for `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`; `disallowed_methods` for `blake3::hash` and `blake3::Hasher::new` |
| law 9: no clock | `disallowed_types` for `Instant`, `SystemTime`, `Duration` |
| rule 4: nothing chosen by a scalar | `disallowed_methods` for `min_by`, `max_by`, `min_by_key`, `max_by_key`, `sort_by`, `sort_by_key`, `sort_unstable_by`, `sort_unstable_by_key`, `Iterator::max`, `Iterator::min` |
| rule 11: no invented value | `disallowed_methods` for `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`, `saturating_sub`, `saturating_mul` |
| rule 15: no member by position | `disallowed_methods` for `slice::first`, `slice::last`, `core::iter::Iterator::nth` — the slice spellings are the ones clippy 1.95 resolves; `core::slice::<impl [T]>::first` does not, and the build says so in a warning |
| rules 1, 8: no whole-carrier walk in cognition | `disallowed_methods` for every whole-carrier reader `Keyed` exposes (`keyed_persist_walk` and any entries-style walk); `fiber` is a type rule, 4.4 |

A lint fires on the item, in scope, with `#[cfg(test)]` handled by the
compiler and not by a brace counter. Each entry is proved once, at the
commit that adds it: a scratch line the lint refuses, built and then
deleted, named in the message. An entry whose path clippy cannot name is
not approximated by a text match; the rule it would have carried is
listed in 4.4 as a review item, with the reason.

4.3 **Allowances are at the site and expire.** Where a denied lint fires
and the site is right anyway, the site carries
`#[expect(clippy::<lint>, reason = "<rule> — <why this site is the
boundary>")]`. `expect` fails the build when the lint stops firing, so an
allowance that allows nothing is a build failure and not a stale row.
There is no module-level allowance (corrected 2026-09-12, the day it was
written: `disallowed_methods` is one switch carrying law 2, rule 4, rule
11 and rule 15 together, so a root `#![allow]` for "rule 11 only" allows
a digest and a `min_by` in the same module and is a path table by another
name). A codec that reads a tag byte by position carries an `expect` on
that line naming rule 15; a reader that would default a missing field
refuses instead, because a short frame read as zero is the clock-zero
fault of law 9 and not a boundary. The count of `expect`s in a module is
a measurement of that module and stays visible. There is no table of
paths anywhere in the tree, and no file is named in any configuration.
One scope statement, not an allowance: §2.4 binds cognition and never
bound a test, and an integration test under `tests/` is a crate of its
own that `allow-*-in-tests` cannot reach, so each such crate opens with
`#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic,
reason = "§2.4 binds cognition; this crate is a test")]`. `allow`, not
`expect`: a test crate owes no abort. No other lint is allowed there.

4.4 **What the compiler cannot read is a type or a review item.** A pair
loop over one carrier (rule 1), a count deciding a branch (rule 2), a loop
without a budget or a boundary (rule 10), a coordinate pair above the
sensor (law 11, §1.4), a carrier cloned into two places (rule 12), a
matrix carrying an action (rule 16), and a chain composed into a word on
the utterance path (law 8, 50.9) are read at review by the preamble's
checklist on every diff. Where a type can hold the rule, it does, and
the type is the gate: `Keyed::fiber` takes a prefix that cannot be
empty, so a whole carrier is read in cognition only through
`Keyed::cursor` under a `Budget`, and `ROOT` does not exist; μ⃗ is a type
that implements neither `PartialEq` nor `PartialOrd`, so the only
comparison of two lane vectors is `lane_moves` and a `bool` over two of
them cannot be written; `from_residual` returns a listing key or a
measure and never a value a frame can carry.

4.5 **The ratchet is retired.** The `OWED` tables measured the debt of
2026-09-10 by file. That debt is visible where it stands: a site under a
denied lint is fixed, or carries its `expect` with its reason, or sits in
a bytes module under 4.3. Nothing counts hits, nothing shrinks a row, and
a file move moves nothing but the file.

4.6 **No new gate.** §8.5 stands. An agent that wants a check adds a lint
entry under 4.2 with its proof, or a type under 4.4, or proposes a change
to this document. A walker over source text is not written again for any
reason, and a check that cannot be a lint or a type is a sentence in the
checklist, not a file. Where `MU-RECURSION.md` or `MU-ENGINEERING.md`
names "the enumeration gate", "the clock gate", "the blake3 gate", "the
squaring gate", "the order gate" or "the panics gate", the name means the
row of 4.2 or 4.4 that carries that rule.

## 5. Commits

5.1 **One phase per commit** (ruled 2026-09-22 by the operator; before
that date, one item). The unit of a landing is a phase: the rows of
`MU-RECURSION-TASKS.md` that together make one claim the engine can be
audited on, landed in one commit whose message names every row it
carries with that row's own observable, quotes the suite and the gate,
and lists every departure. A phase is not a bundle: a bundle is two
claims in one commit, a phase is one claim across several rows, and a
commit whose rows do not share a claim is returned and re-cut. Move-only
splits stay their own commit (§1.6). A phase is audited as one landing
under §6: every changed file read whole, each row's observable
reproduced, the Owed ledger updated per row; a phase with one row
unreproduced is a phase landed in part, and the next phase's message
says so.

5.2 **Message.** First line: `<id>: <claim in one sentence>.` Body: what
changed and why it is doctrine (chapter numbers); the test names added,
rewritten, or deleted, with the claim of each deleted one; the
`test result` lines of the full suite; the clippy line; `xtask lint`
line; the document paths rebound; every `pub(crate)` widening; every
`#[allow]`; every departure from the directive as given (§8.3). A
move-only commit says `move only` on its first line.

5.3 **Never committed.** Build artifacts (`target/`, `xtask/target/`),
fleet state (`run/`, `var/` — the fleet's homes, `var/<unit>` under the
repository since 2026-09-09; keys, mesh records and corpus manifests live
there too and are configuration, not tracked), editor files, secrets.
`.gitignore` covers them; a commit that adds one is amended before anything
else lands.

5.4 **Pushing and history.** Nothing is pushed and no history is rewritten
without the operator's word. A commit that must be removed from history
(an artifact, a secret) is reported, and the operator decides.

5.5 **Docs commits.** A document change lands in its own commit the same
day as the code it describes; a doctrine change (§9) lands before the code
that implements it.

5.6 **Shared files.** `git commit --only <path>` commits the file as it
stands in the working tree, not the hunk the item wrote. A documents file
two agents edit (`MU-RECURSION-TASKS.md` above all) is named in a commit
only after `git diff -- <path>` shows nothing but the item's own lines;
otherwise the item's rows wait for their own commit and the other agent's
pending edits are left alone. Four bundles on 2026-09-07 had this one
cause.

5.7 **The ledger is kept, not appended** (ruled 2026-09-24, the
operator's word: "it has grown like a weed"). `MU-RECURSION-TASKS.md`
carries ONE live order. A row is an object the engine can be audited
on; a reading is a line of the observer's row (CV5) and never a row of
its own. A ruling that changes an object's statement amends that
object's row; a new row is opened only for a new object. A row a later
ruling supersedes is marked once in the superseded table with its
successor and is not re-sequenced; a row that lands moves to the landed
ledger with its commit and its audit. In `MU-RECURSION.md`, chapters run
in numeric order within their parts, a ruling amends the chapter that
governs its object (a new subsection only for a new object), and Part 0
carries every master's statement, site and state, changed in the same
commit as the ruling that moves it. A relay to the implementor names
rows by the live ledger only.

## 6. Verification of a landing

6.1 The implementor runs, in the foreground and in this order, on the
commit as it will land: `RUSTFLAGS='-D warnings' cargo test --release`
(every target), then `cd xtask && cargo run`, which runs the reading bound
and then `cargo clippy --release --all-targets -- -D warnings` and exits
non-zero on either. Both lines go in the message, the suite's `test
result` lines and the gate's exit. "Lint clean" alone is not green.

6.2 The auditor reads every changed file whole and verifies, on the code:
(a) the wiring — every new reader has a production caller and a producer
feeding it, and the path from the wire or the percept to the reading
runs, so nothing landed is a type, a line or a fixture standing in for a
mechanism; (b) the coding standards — checklist (a) through (t) and the
laws, returned by rule number; (c) the trajectory — the change against
its ruling AND against the carrier it writes into and every ruling that
retired a carrier (§6.6). A landing is called verified only after that
read; a report is never the object of an audit, the code is. THE AUDITOR
DOES NOT RE-RUN THE SUITE, THE GATE AND THE CENSUS ON THE SAME MACHINE,
TOOLCHAIN AND COMMIT the implementor quoted them from (ruled 2026-09-24,
the operator's word): that reproduction confirms only that the report was
not misquoted, which reading the quoted lines already does, and it is
reserved for a commit whose report there is reason to doubt or that lands
from a dirty tree.

6.3 The cold-boot trace of chapter 40 for the item is reproduced before the
next item lands (chapter 41's consolidation discipline). A landing that
changes admission, the wire, or the market is followed by a whole-fleet
recut before its readings are used.

6.6 **A landing is checked against the carrier it writes into** (ruled
2026-09-24, the operator's word; the coordinator's fault at LD0 steps 4
and 5). A change that moves a field onto "a row" is audited against what
the row lives in: LD0's per-word fields were landed onto rows of the
MYC7 snapshot — a linear byte tape read by position in write order —
after the tape had been retired (MU-RECURSION 1.9, K6, 54.5) and after
LD2 had persisted the ledger as the trie, because each step was read
against its own ruling and not against the carrier. From here the audit
names, for every change, the carrier it writes to and reads from, and
returns any write onto a retired carrier or any positional read of a
keyed one; a ruling applies at every site of its shape.

6.5 **A fleet reading is quoted against TUniversal height and `K`, never
a wall clock** (ruled 2026-09-24, MU-RECURSION 78.21). "Up seven hours"
and "written six hours ago" are not anchored; "head 16 739, K 2 928,
ledger persisted at 16 700" is. The console's header places every source
on TUniversal first, and a diagnostic, an audit or a relay that names a
time names a height and a `K`.

6.4 **Verification never writes.** A check reads: `git diff`, `git show`,
`git log`, a build, a test run. `git checkout <rev> -- <path>` is a write
that reverts the file it names, and it reverted a landed fix once
(2026-09-07, F3f) while being used to "verify" it. A probe that must
change source — a `panic!` to prove a test reaches an arm, an offending
line to prove a gate fires — is made only in a detached worktree, never in
the tree an agent is working in, and a worktree build that shares the
target directory is followed by `touch build.rs && cargo build --release`
from the working tree so the build script's cached path is that tree's.


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

8.6 **Cadence (ruled 2026-09-08).** The implementor never waits for the
auditor. It runs the directive queue as given, end to end, one item per
commit as §5 says, and reports at milestones — the end of a stage or
phase, a ruling it needs, a blocker — not after each item. The auditor
audits commit ranges, asynchronously, and files rows; a finding that
returns a landing is corrected forward as the next item, never by
stopping the queue. A ruling the implementor needs is asked in one line
and the queue continues on every item the ruling does not touch. A
report is the commit log (id, first line, the three gate lines) plus
rulings asked plus departures; prose is for findings only. Stopping
after one commit to report it is a departure (§8.3).

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
repository (`scripts/recut`): stop 1–5 then c, b, a; wipe ledgers, genomes,
thoughts, registry, court federation; keep keys, mesh, corpora; drop each
hypha's origin (`--recut`, one run of the binary per hypha, which bumps the
epoch under the signed mesh record); `--mint-genesis` on hypha a once;
start a, b, c then 1–5.
No flag of the recut lives in a unit file or a drop-in; a unit starts the
binary with `--home` and nothing else.

Ruled 2026-09-07: this is a development environment and nothing persisted
survives a recut. A change to any persisted layout (tape, genome, ledger,
snapshot section) is a recut, never a reader for the old layout; no
migration or legacy reader exists in the tree before beta, and one that is
found is deleted. Keys, the peer list and the corpora are configuration,
not data, and stay.
