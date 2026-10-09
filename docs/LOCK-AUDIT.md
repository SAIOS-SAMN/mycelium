# The kernel-lock audit (2026-10-09)

The operator's direction (relayed by the coordinator): find the disease, not
the next symptom. Read-only audit of every path that takes the court's one
kernel lock, at 686d43ed (deployed 5751c804). No code changed by it.

**Finding: no path that takes the kernel lock is metered end to end.** A
`Budget` exists on some paths and gates a handful of reads; the loops around
it walk whole carriers without spending.

## Defects found on the way
1. **A spent budget slashes a good word** — `court/verdict/body.rs:535-541`:
   `versor_between`'s `NoPose::Unbounded` (stroke spent) becomes
   `REFUSE_EDGE`; `court_stroke` (`court/replay.rs:254`) treats any refusal as
   a slash. A stroke is ~10 reads on this host and the docket cursor spends one
   per key before any replay (`walks.rs:144`). Likely live, not yet observed.
2. **The step marker goes stale** — `observe::step` is one process-wide value,
   never reset by `held::lock`; the offer hang's "step offers" was the court
   loop's leftover.
3. **Kernel locks invisible to the lock book** — `server/synthesis.rs` (11
   sites) and `server/backfill.rs` (10, incl. `admit_backfill`) take the raw
   `.lock()`; uncounted, and `holding()` reads none while they run.
4. **Cited-form speech has no cycle guard** — `speak.rs:324` recursion; mutual
   citation is allowed (`replay.rs:393-400`). Inferred stack risk.
   (`synthesis::consolidate` is dead code.)

## The paths, worst first

| # | Site / entry | What bounds the work under the lock | R/W | Worst observed hold |
|---|---|---|---|---|
| 1 | `exchange.rs:307` court loop — `court_stroke`: per docket key `court_replay` → `blind_replay_on`, then `rule_own`/`rule_on` | **UNBOUNDED** — only `versor_between` spends inside the replay (see below); after it, `ruled_admit` (whole-form decodes, `winding.rs:676/687` O(T²) links, `awaiting.rs:115` re-hears every waiter), `measured_replay` (whole family speech), slash path deep clones and walks | W | a: hold 3411 (07:54), 20+ min earlier; b, c repeatedly; blocked 2048 |
| 1b | same hold — rest of the stroke | cursor skip walks the docket prefix free (`walks.rs:135`, `fills.rs:41/165`); `relocate_t_exhausted` walks all of `lane_listings[2]`+`provisional` (`basis.rs:140/143`); `read_phase` clones strata; fills rebuild the whole language per section (`replay.rs:479` → `language.rs:72`) | W | (same) |
| 1c | same hold — answer standing, publish, seed, offers | whole-ledger walk (`observe/carriage.rs:75`); `Shelf::publish` walks every signer every stroke (`publish.rs:208-243`); `syllabus_seed` copies every ruled address every stroke (`stratum.rs:117`) | W | (same) |
| 2 | `judge/offer.rs:105` ACT_OFFER (also holds `mesh.cover`) | **UNBOUNDED** — one stroke; `court_replay` as row 1; `rehear_rulings`, `condense_on`, `refine_stratum`, `links()` ×2, `rehear_awaiting`, language rebuild, `measured_replay` | W | a 07:45 (step label stale); blocked 256 |
| 3 | `judge/syllabus.rs:60` ACT_FETCH | **UNBOUNDED** — `serve_ruled` walks a whole fiber / the whole seed (`court/syllabus.rs:320-346`); `class_records(default)` the whole nerve. The relation reading was metered (4783ecb6); these loops were not | W | **c: ≥32 min (02:00–02:32)** |
| 4 | `exchange.rs:803` per fetched word | **UNBOUNDED** — `hold_glued` → whole-language rebuild PER WORD (`glue.rs:409/419`); `book_field` O(T²), rehears, `mark_reached` DFS | W | blocked 16 |
| 5 | `exchange.rs:764` per cover page | **UNBOUNDED** — per chart × word in flight: `carry_chart` → `discharge_of` → `form_deflation_read` (full family speech) | W | blocked 2 |
| 6 | `exchange.rs:849` fetch reply | **UNBOUNDED** — `confirm_peer_holds_addrs`, `home_install` → `rule_own`, `measured_replay_at` per citation | W | — |
| 7 | `exchange.rs:660` peer digest | **UNBOUNDED, nothing spends** — whole ruled cover, language rebuild, `sheaf::digests`, `fetch_have` every stratum | W | blocked 8 |
| 8 | `exchange.rs:935` sealed close (`orbit()`) | **UNBOUNDED** — `line_between(.., usize::MAX)`, `rows_below` clones the journal ×3, `settle_windows` | W | blocked 1024 |
| 9 | `judge/speech.rs:27` ACT_SPEECH | **UNBOUNDED** — strokes minted per sentence, O(n²) contains, `form_deflation_read` per sentence, language rebuild | W | blocked 1024 |
| 10 | `door.rs:592/537/562` intake / `homes_here` | **UNBOUNDED** — `use_credit` walks every discovered word (`room.rs:151`); `antichain_of`, `mark_reached` | W/R/W | blocked 2048 |
| 11 | `judge/fold.rs:81/137` | **UNBOUNDED** — hashes every fetched receipt, `nets_of` every row; era frame + disk append under the kernel | R/W | blocked 1024 |
| 12 | `persist.rs:22` (via `persist_if_due`, `durable`, synth) | **UNBOUNDED, no Budget** — `Kernel::snapshot` serialises the whole mind | W | blocked 256 |
| 13 | `exchange.rs:350` after persist | **UNBOUNDED** — `nerve_census` every complex/cell; `helicity` O(tubes²) in BigRational | R | blocked 128 |
| 14 | `fold.rs:20/40/67/93` fold round | `rows_below` journal clone; `shed_folded` (`usize::MAX`), journal audits ×2, era file written under the lock | W | blocked 256 |
| 15 | `admission.rs:222` `admit_peer_receipt` | **UNBOUNDED** — `recall_cited` reads a whole era per missing parent; Fold receipt → `shed_folded_lines` | W | blocked 2 |
| 16 | `admission.rs:17/96` own cover / ruled root | cover trie walk and clone; `sheaf::digests(&ruled)` | R | blocked 64 |
| 17 | `judge/rows.rs:17` ACT_ROWS | **UNBOUNDED** — `rows_below` whole journal per page (O(N²/page)); the bounded `rows_span` exists, unused | R | blocked 64 |
| 18 | `door.rs:405/421/354` syllabus seed / page / cover | whole stalk trie; whole nerve per page; cover trie clone | R/W/R | blocked 32 |
| 19 | `door.rs:475/492` refine; `door.rs:246` solve | deep `make_mut` clones; `measured_replay` + `mark_reached` 2+N | W | blocked 16 |
| 20 | `judge/{challenge,domain,handoff,uplink,why,equivocation}.rs` | `reverse`, finality × obstructions, toll scans, whole-form Reasons decode; then `persist::durable` re-takes the kernel for a full snapshot | R/W | — |
| 21 | `server/synthesis.rs` (11 raw) — synth thread, reflex | adopt: language rebuild, deep `make_mut`, `volume()` ×2; quote/probe: `market_open` clones, `fiber_under([])` every listing | W/R | **not counted** |
| 22 | `server/backfill.rs` (10 raw) — exchange `hear` | `admit_backfill` page loop does not spend; `recall_cited` per receipt; `adopt_root` → `shed_folded` | W | **not counted** |
| — | bounded (constant per hold) | boot sites (single-threaded), `exchange.rs:476/631`, `door.rs:118/221`, `admission.rs:71`, `seat.rs:65/69`, `quote.rs:58`, `probe.rs:22`, `offer.rs:27-131` | — | — |

**Inside `blind_replay_on`** only `versor_between` spends (8 reads per
candidate pair). Unmetered: glued vertices × cover edges (`replay.rs:330-356`);
`resolve()` per citation with a whole-form clone each (`listings.rs:777`) and
a cavity word decoded on every read (`identity.rs:76-77`); prime factoring per
demand (`:438`); contact vertex × pair painting (`:684-716`); every pair
perceived (`:757`, `body.rs:289-341`, again at `body.rs:464`); quadratic
`form_covers_compartments` on `FAIL_PART`; `orbits_of`, face loop, coherence,
orbit classes, rank; **`form_deflation_read` per pair with recursion through
cited forms** (`deflation.rs:204`, `speak.rs:301-331`; scene rebuilt per
pair); **`replay_certificates`** (whole-form clone + `orbits_of` + class
vectors + rank per certificate).

## The fix list (to close the cause by construction)

**1. No unbounded loop under the lock, enforced in code.**
- `Held` stops dereferencing to `Kernel`. `held!` mints the hold's one stroke;
  the only way in is `Held::run(|k: &mut Kernel, reads: &mut Budget| …)`.
  Kernel entry points reachable from the binary become methods on a `Locked`
  facade, each taking `&mut Budget`; anything without one stays `pub(crate)`.
  The library's raw locks (synthesis, backfill) move behind the same guard —
  counted by the book.
- Already in our favour: `Budget` is constructible only by `Physics::stroke`
  (field `pub(super)`); `Keyed::cursor(from, &mut Budget)`; `NoPose::Unbounded`.
- Inner loops by lint: whole-carrier walks (`Keyed::scoped`, `fiber_under`,
  `line_between`/`rows_below`, `language_rebuild`, `Kernel::snapshot`,
  `Arc::make_mut` on Language/Stalks, `Form::clone`) added to
  `disallowed_methods`; each remaining use carries an `#[expect(reason =
  "bound: …")]` or moves to a budgeted cursor. Run clippy directly (the xtask
  gate stops at the 1200-line bound).
- The docket cursor seeks to `from` through the trie (`walks.rs:135`).
- Runtime check: `Budget::spend` bumps a thread-local; a hold that spent zero
  reads and blocked ≥N waiters is logged "unmetered hold" on drop.
- A replay returns `#[must_use]` Clean / Refused / **Unread** — Unread is "not
  ruled this stroke", never a slash (fixes defect 1).
- **DECISION NEEDED — stroke size:** a stroke is ~10 reads here; fully
  metered, a large form never finishes in one stroke. Either replays become
  resumable (the docket entry keeps a phase cursor), or the docket's progress
  rule skips an over-stroke word and rewinds starved ones.

**2. Every read onto the 78.151 shelf.** Needed on it beyond lines, eras,
proofs, accounts: own cover charts; own `lane_room`; `ruled_root` and ruled
digests; the full head receipt; ruled addresses in stratum order (a delta);
class records; ledger rows below the root via `rows_span`; open obstructions,
finality, tolls, line heads (challenge reads); mouths, handoffs, tubes;
reserves, bonds, origins, universes (domain reads); stamp→word + Reasons
(why). `judge/fold.rs:81` can leave the lock today (its input is in
`Published.accounts`). `serve_ruled` stays a write (it pays).

**3. Watchdog in TUniversal ticks** (shape per the TU-1..7 ruling, 78.152):
the lock book records the mesh tick at each take and resets the step per hold
(fixes defect 2); the mesh tick register is updated outside the kernel from
verified peer readings and client stamps; a standing hold that spans ≥N tick
advances is named (site, hold, step, reads spent). No dedicated clocked thread:
events wake it (a Condvar notified by tick updates, no timeout). Where every
court is held TUniversal stops too; the existing counts (waiters blocked,
observer questions spanned) cover that. A per-hold cut flag checked in
`Budget::spend` lets the watchdog end metered work with Unread.
