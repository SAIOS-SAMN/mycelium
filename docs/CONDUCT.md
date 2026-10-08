# mycelium — working conduct (a working doc, not part of the Concordance)

Mirrors xylarium-kernel's docs/CONDUCT.md for the machine the nodes share.
Working practice, kept current, not appended.

## Locks
- HEAVY (`/run/user/1000/xyl-heavy.lock`): one hold machine-wide; at most 4
  build jobs, 4 test threads, 4 worker processes; a hold stays under about
  15 minutes.
- LONG (over about 10 minutes: a soak probe, a deep run): by grant only, under
  `/run/user/1000/xyl-long.lock`, at most 3 threads at nice 15, never holding
  the heavy lock.
- LIGHT TESTS run without a lock: one already-built test binary or module, one
  thread, under a minute, no compile in the same command.

## Batching (operator, 2026-10-08)
1. Finish every edit you can before queueing; one build per round of changes,
   not one per fix.
2. Build every target you need in ONE cargo invocation (`--bins --tests`, the
   crates named), with its scoped tests in the same hold.
3. Never queue a second job while your first waits; fold new edits into the
   next batch.
4. Subagents never take their own holds: they WIP-commit and report the hash;
   the parent session batches their work into its hold.

## Cadence
- No full test suites: the scoped tests of the touched modules, then commit
  and ship (`scripts/ship-scoped`).
- One real background command, acted on by its completion notice; no poll
  loops; read the queue by `lslocks` or pid; never kill another node's job.
- Report to the nodes as what changed and why; WIP-commit every step.
