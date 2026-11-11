### 78.152, Law 9 — Idle court exchange park backoff and client session polled pacing (#234, #221)

- Backs off idle park durations in the server exchange loop on consecutive idle cycles without motion up to the aperture bound (`physics::idle_backoff`), reducing CPU utilization from 35% to <1% and eliminating lock contention against arriving reflex acts (#234).
- Resets consecutive idle park to one scheduler floor immediately upon motion on the door shelf or peer activity, preserving zero latency on incoming client work.
- Dials client SDK sessions with `Pace::Polled(budget)` bounded by aperture stroke limits instead of unbounded `Pace::Blocking`, preventing client hangs on slow or lock-held hyphae and properly invoking RTT backoff on timeouts (Law 9, #221).
