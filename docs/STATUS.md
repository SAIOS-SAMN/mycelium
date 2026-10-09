# mycelium — live status (read this first; kept current, not appended)

*Last written 2026-10-09 07:37 EEST by mycelium-opus, after the recut.*

## The fleet
- RECUT 2026-10-09 07:35 on the operator's word after a system crash:
  hyphae a/b/c on **5751c804**, fresh genesis **6a86… epoch 146** (minted on
  a); every prior court state, ledger and μ wiped; homes keep keys and mesh.
  Voices 1–5 (`mycelium-client@1..5`) run the same build.
- The build carries: wire v1.2 (task citations); 78.145 the seat surface;
  78.146/78.148 the seat mind (its own calls, the listing as repertoire,
  re-take, the box drained, a standing call not repeated); 78.147 catch-up
  from a fold root (+ archive paging for an unproved root); 78.149 μ is
  discharge (the operator's strict escrow rulings; NoWork; genesis as
  Genesis); 78.151 the door serves reads (receipts, wallets, clock, seat
  listings) from a published snapshot — only writes take the kernel; the
  kernel lock counted by site (`held!`), the fair hand-off, and the step a
  long hold stands in, named by the observer.
- Units are NOT enabled at boot. After a reboot: `--check` each home, then
  `systemctl --user start mycelium-server@{a,b,c}` and the voices.
- Seat minds are TRANSIENT units (`systemd-run`): they do not survive a
  reboot. Start them after each house re-posts its listing:
  `systemd-run --user --unit=mycelium-xseat-N -p CPUQuota=25% -p
  WorkingDirectory=/opt/mycelium-v0.1.0 target/release/mycelium-client
  --home var/xN --seat <domain> N` (xylarium 07a50ff1…, seats 0–3);
  nseat-1..4 (netstratum 5deab117…), seat-1 (barren e1b24154…, home var/s1).

## Houses (to re-post after the recut)
- xylarium (Rust, the coordinator): domain 07a50ff1…, 20 seats (fn 2 FOCUS,
  fn 1 SOLVE), reserve = the ancient-houses tier key.
- netstratum: domain 5deab117…, reserve 43792871…, seats 1–4.
- barren-realms (the Python xylarium): domain e1b24154…, reserve 633299c1….

## Known, instrumented, not yet fixed
- A long court-stroke hold (hypha a, 2026-10-09): the observer now names the
  step (`court stroke: docket replay / lapse silent / relocate / fills …`)
  when one hold stands across two of its questions. Read it on the console;
  bound the step it names.
- Still on the kernel lock (78.151 follow-ups): the stratum fetch
  (`serve_ruled` pays as it reads), digests, cover, evidence, rows, why.
- Snapshot memory: first publish copies the hot ledger; watch RSS.

## Owed, in order
1. Read the fresh fleet: replication per pair, ledgers, seats' applied calls.
2. 78.150 tiling as the orbit of a versor (ruled; after the reading).
3. The court-stroke step the observer names, bounded.
4. 78.143 remaining; curved holes 78.144 (f78fd200); the Mv equality audit.

## Cadence
Scoped tests of the touched modules, then commit and ship
(`scripts/ship-scoped`, rolling). One heavy-lock hold per round
(docs/CONDUCT.md). Report = hash + one line, to the nodes.
