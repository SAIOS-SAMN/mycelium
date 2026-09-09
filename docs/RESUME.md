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
# expected at 1cf2c1e: lib 402, bins 7, alpha_plan 13, g_cga 6; xtask exit 0
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

On the operator's word: rebuild is done, recut and soak the fleet on
1cf2c1e, then read: `cites > 0` on attests (bodies as words, S12.3a),
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
