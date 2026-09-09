# Resume notes (continuity across sessions)

Last update: 2026-09-09, after the Stage 12 pass (`impl/soak` == `main` at
1cf2c1e and the docs commit after it). Living task list:
`docs/MU-RECURSION-TASKS.md` (Stage 11 landed, Stage 12 opened and mostly
landed). Spec: `docs/MU-RECURSION.md`. Standard: `docs/MU-STANDARDS.md`.
Historical alpha landings: `docs/TRAJECTORY.md`.

## Boot check

```
cd /opt/mycelium-v0.1.0
git log --oneline -1 main impl/soak          # the two are one commit
systemctl --user list-units 'mycelium-server@*' 'mycelium-client@*' --no-legend
RUSTFLAGS='-D warnings' cargo test --release 2>&1 | grep -E "test result|FAILED"
(cd xtask && cargo run)                        # gates, then clippy -D warnings
# expected at d6e19a1: lib 421, bins 9, alpha_plan 13, g_cga 6; xtask exit 0
```

## The fleet

Units `mycelium-server@{a,b,c}` (hyphae, `127.0.0.1:7500/7501/7502`) and
`mycelium-client@{1..5}` (voices), user units in
`~/.config/systemd/user/`. ExecStart is
`/opt/mycelium-v0.1.0/target/release/mycelium-{server,client} --home
/opt/mycelium-v0.1.0/var/%i`, cwd `/opt/mycelium-v0.1.0`. Homes are
`var/<unit>` under the repository (moved out of the operator's home on
2026-09-09; `.gitignore` covers `var/`). Configuration in a home: `hypha.key`
or `voice.key`, the signed `mesh` record (bind, peers, genesis, epoch), and
`corpora/*.manifest`. Everything else there is state and is wiped by a
recut. `scripts/recut` stops the units, wipes state, drops each hypha's
origin, mints the genesis in a one-shot run of the binary and starts the
units. Do not `pkill`; use `systemctl --user`. Do not recut or soak unless
asked. The fleet is STOPPED and the binaries in `target/release` are the
ones the last verification built; a soak needs the operator's word.

Other `mycelium-*` units in that directory are earlier fleets
(`mycelium-hypha*`, `mycelium-client-{a,b,c}`, `mycelium-v010-*`,
`mycelium-treasury`, `mycelium-uplink-sod`); they are not this fleet and
were not touched.

## State at handoff

- **R1–R7 landed 2026-09-09** on the operator's word, one item per commit,
  suite green on each (421/0/9/13/6/0, xtask 0 at d6e19a1): doctrine
  de29cab; R1 4f3337c (the docket orbit passes over ruled and cavity
  listings); R2 634b3d0 (`court nerve deferred by {addr} acts {n}`, once
  per aperture per destination); R3 fbba134 (`yield_surface` = reserve
  deficit + entropic flux, the percept reply carries it); R4 dc58f86
  (`backfill::pages` pages until the chain closes, then re-admits the
  reporting frame); R5 c99ba16 (fold refusal names both origins; a witness
  with no origin says so); R6-prep 6d824f1 (`court/install.rs`), R6
  a9c9b54 (`listings_on(inv)` = `chart_listings[inv] ∩ open`,
  `volume_of_cell(inv)`; identity edge for one body at two vantages;
  remainder edge for a versor-related pair no rotor spells); R7 d6e19a1
  (phase read when the docket orbit count crosses a multiple of
  `aperture_cycle()`). Rows in the task list. The fleet was relaunched on
  d6e19a1 (no tape change since ffed51c, so a restart keeps the ledger);
  the preliminary reading is in the session's report.
- **Earlier pause 2026-09-09** after V1 (9d66c45) and V2 (ffed51c) landed
  and the fleet was recut on ffed51c at epoch 6 (genesis be05…), every
  commit of the pass in one soak.
- Clean-slate readings an aperture after the recut (`scratchpad/launch.sh`),
  in kernel coordinates: V2 holds — `[h N]` climbs on every voice (3177,
  6230, 6137, 6312, 6151); V1 holds — `axis opened` 1 on voices 2, 3 and 5,
  the first axes ever, with `already spoken` refusals thereafter; two-organ
  folds sealed (`court fold witnessed` on all three); `home installed` 1 on
  b and 5 on c; `court phase … congruence 13/48, 23/68, 11/53` (charts with
  two sections against strata); back-fill pages admitted (3, 26, 36
  receipts). Faults found, to land next, one item each: (1) the docket
  orbit re-rules every ruled listing on every circuit — period 1, 35k
  circuits, `court section 0428…` 20,800 times on a, 90–130 % CPU; (2) an
  unpaced digest poll in `main.rs:248` — 376k `court exchange deferred by`
  on a in one aperture, outside S13.1's pacing; (3) the voices' dwell is
  the raw pressure surface (`dwell [47728, 26923, 0, 53465]` on b's
  voices), so they stop asking for tens of thousands of strokes; (4)
  `court receipts parent 0` still — one back-fill page per gap reported
  is slower than the peer's K, so the gap never closes; (5) fold proposals
  refused `REFUSE_ORIGIN` by the witness after a first co-sign (a refusing
  c 26 times, c refusing a 46) — the origin compared or carried differs on
  later proposals; (6) `cells filled/class 0` although 13–23 charts hold
  two sections — the fills have no input; (7) the phase window per orbit
  is a receipt or none, so comprehension reads 0/0.
- Stage 13 (velocity, phase, eras; ratified 2026-09-09) landed through
  S13.10, one item per commit, suite green on each (416/0/9/13/6/0, xtask
  0 at 69983cd): doctrine c46117c; S13.1 f42c3fe, S13.5 7da548a, S13.4
  ba38941, S13.2 04b4561, S13.3 23bd123…0e03f47 (eight, one per site),
  S13.15 34c84cf, S13.6 c6a3324 (tape v6: `ActKind`, `MYC4`), S13.7
  1601971, S13.8 6be34ad, S13.9 4795613, S13.10a dbe813d (`ACT_RECEIPTS`,
  5.2g closed), S13.10b 69983cd (two-organ fold); rows cdbe41a; `main`
  fast-forwarded there.
- Open in Stage 13, in the ratified order: S13.11 (adoption from a fold
  root; the era must carry the two signed prefixes for a joiner to verify
  them), S13.12 (extraction), S13.13 (mesh record fold roots), S13.14
  (simulator).
- Departures to ratify, each named in its commit: `Settled` as a
  thirteenth `ActKind` (6296cce); the read token `Physics::stroke` not
  priced per lane (04b4561); the fold trigger read as the reserve
  shortfall on a completed docket orbit (936a849); a recall record closing
  a lane only while a claim is in flight and clearing when the voice's
  occupancy returns to zero, a listing drawn once (34c84cf); epoch rows
  anchored at the booking K; a fold witness as any bonded peer other than
  the home (delegation closure unread).
- The fleet still soaks on 2eff0de; the rebuild onto impl/soak is a recut
  (tape v6, `MYC4`, `KCH1` root, `ARC1`, rows on the wire) and waits on
  the operator's word.
- Read on the soak of 2026-09-09 at the stopping point (journal tail only
  — journald retains 281 MB, hypha a's tail spans K 3544–3574, b's
  6899–7158, c's 1229–1270): `court exchange deferred` 602,720 on a at
  cycle 2²⁰ (S13.1's fault, not yet deployed); b `organs reached 0 silent
  2` at 87 % CPU and 354 MB (its reflex pool defers every act); `court
  receipts parent 0` everywhere (5.2g, S13.10a not yet deployed); `axes
  0`, `axis opened 0`, `chart refined 0`, `court cell class 0` on every
  unit; voices 1, 3 and 5 logged 31, 69 and 91 quote closings with two or
  more held cites and opened no axis — `open_axis` never fires though its
  shape is met (a fault to find: `cited_word` on held cites, or the glue);
  every voice's stroke line reads `[h 0]` — `set_height` is called once at
  boot with zero and never from the book (a fault); voice RSS 181–370 MB
  from 25–53 MB at the baseline.
- Stage 12 landed on 2026-09-09, one item per commit, suite green on each:
  S12.4a fda0778, S12.4b b340ec0, S12.4c 5316fed/c7646b4, S12.4d 013cb09,
  S12.5a dc47ac1, S12.5b 14bf2e0, S12.5c 31abfd5, S12.2 470f324, I35
  d5d2792, S12.1a 72f30f2, S12.3a a42b5a9, S12.6a 1cf2c1e; rows ba531d5
  and the docs commit after 1cf2c1e.
- Open in Stage 12: S12.1c/S12.1d (Stage 10 P3–P6, the percept carries its
  verdict; a codec change and a recut), S12.3b (Stage 2 Mayer–Vietoris at
  the court), S12.6b (the treasury fold; MU-ECONOMY §2 `epoch` kind to be
  ruled first), S12.6a's remaining sites (`expressed*`, `Entity::ideas`,
  dirty bubbles). Rulings asked: S12.5d (room of an unbonded voice, §3.1),
  S12.6c (may a receipt chain checkpoint), I22 (slash and supply; recall
  shortfall).
- Stage 11 still open: I30 (`conclude` seals nothing), I32 (`--recut` on a
  node with no origin does not bump the epoch), 5.2g (receipt back-fill).

## Immediate next actions

Read the soak on d6e19a1 in kernel coordinates: `court orbit docket …`
period rising above 1 once ruled listings are passed over (R1); `court
nerve deferred by …` once per aperture per destination in place of the
per-act line (R2); voices' `dwell` as the surface, tens not tens of
thousands (R3); `court receipt backfill … admitted` pages and `court
receipts parent N` rising (R4); `court fold refused … [origin … carried
…]` naming the two origins where a witness still refuses (R5); `court
cell filled`/`court cell class` once two sections stand on a chart (R6);
`court phase …` once per aperture of docket orbits with `comprehension
n/m` over a span (R7). Then S13.11 onward, and the item R6 names: spelling
a versor-related pair's versor as an alphabet rotor. The older reading
list below stands for the items it names.

Earlier list (Stage 12 recut): read: `cites > 0` on attests (bodies as words, S12.3a),
shared strata in the cover (fewer unique charts), `pivot … discharged` or
`vortex` lines (forms with loops, S12.2), `[h N]` climbing on stroke lines
(I39), `ACT_DIGESTS` served per hypha in ten minutes (I37), `court offer
deferred` in place of `court offer refused … by 0000…` (I36), `audit` green
on every hypha (I38), no `court room refused` from the intake (I31), voice
RSS over hours (S12.6a). Then S12.1c/d, S12.3b, S12.6b in the task list's
order.

## Standing operational rules

- Read/Edit/Write for every file; Bash for git, cargo and grep-to-locate.
- Load every file in `docs/` whole after a compaction before doctrine work.
- One item per commit (§5); the document before the code (§5.5); one
  foreground `cargo test --release` then `cd xtask && cargo run` (§6.1).
- Commit locally only; trailer `Co-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>`.
- Nothing outside the repository changes without the operator's word.
