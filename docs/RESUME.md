# Resume notes (continuity across sessions)

Last update: 2026-09-09, after the first soak on Stage 11 (`impl/soak` ==
`main`, 2c67ee2 and after). Living task list: `docs/MU-RECURSION-TASKS.md`
(Stage 11 landed, Stage 12 opened). Spec: `docs/MU-RECURSION.md`.
Standard: `docs/MU-STANDARDS.md`. Historical alpha landings:
`docs/TRAJECTORY.md`.

## Boot check

```
cd /opt/mycelium-v0.1.0
git log --oneline -1 main impl/soak          # the two are one commit
systemctl --user list-units 'mycelium-server@*' 'mycelium-client@*' --no-legend
RUSTFLAGS='-D warnings' cargo test --release 2>&1 | grep -E "test result|FAILED"
(cd xtask && cargo run)                        # gates, then clippy -D warnings
# expected at 2c67ee2: lib 391, bins 6, alpha_plan 13, g_cga 6; xtask exit 0
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
asked.

Other `mycelium-*` units in that directory are earlier fleets
(`mycelium-hypha*`, `mycelium-client-{a,b,c}`, `mycelium-v010-*`,
`mycelium-treasury`, `mycelium-uplink-sod`); they are not this fleet and
were not touched.

## State at handoff

- Fleet stopped 2026-09-09 after a seven-hour soak on a91fcb8; state wiped,
  build artifacts cleaned (`target/`, the `.wt/impl` worktree). The
  readings are in `MU-RECURSION-TASKS.md` Stage 11 (I34, I34b) and the
  directive that follows from them is Stage 12.
- Stage 11 rows I12–I33 landed. Open: I22 (rulings asked), I30–I32, I35–I39
  (faults found on the fleet), 5.2g (receipt back-fill, the live blocker).
- What the soak showed, in one line each: perception discriminates instead
  of recognising (one 13-axis stratum per task, no shared charts); forms
  have no loops (two-vertex stars, no pivot ever fires, no axis opens);
  words are chains, not letters (no held word deflates a demand); the fleet
  polls (131k digest requests per hypha in ten minutes).

## Immediate next actions

Stage 12, in the order of `MU-RECURSION-TASKS.md`: perception as
congruence (coarse-first axes, peel, the harmonic eye decides the pairing),
forms with loops (non-constant faces kept as sections), reduction before
listing, pacing by the ledger (I39, I37, I36), governance by room (I38,
I31, S9.7), footprint (7.3, the treasury fold, a ruling on receipt
checkpoints). Pause and report after; the soak restarts only on the
operator's word.

## Standing operational rules

- Read/Edit/Write for every file; Bash for git, cargo and grep-to-locate.
- Load every file in `docs/` whole after a compaction before doctrine work.
- One item per commit (§5); the document before the code (§5.5); one
  foreground `cargo test --release` then `cd xtask && cargo run` (§6.1).
- Commit locally only; trailer `Co-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>`.
- Nothing outside the repository changes without the operator's word.
