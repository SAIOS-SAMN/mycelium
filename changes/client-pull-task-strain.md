### Client: mind asks for tasks only under strain (MU-CLIENT §4.13, #135)
- **Task pull condition:**
  - Removed empty corpus fallback from `pull_task` in `src/client/sdk/mind/learn.rs` so that a task is requested only when the mind holds tension (`strained()`), as required by MU-CLIENT §4.13.
- **Offer test alignment:**
  - Renamed and updated `pull_task_asks_only_under_strain` in `src/client/sdk/tests/offer.rs` to verify that a settled mind with an empty corpus rests and asks for nothing, and asks only once strain is held.
