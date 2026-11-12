### 78.152, Law 9 — Client session polled pacing (#221)

- Dials client SDK sessions with `Pace::Polled(budget)` bounded by aperture stroke limits instead of unbounded `Pace::Blocking`, preventing client hangs on slow or lock-held hyphae and properly invoking RTT backoff on timeouts (Law 9, #221).
