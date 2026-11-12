### Client first motion, clock unreturned thought release, and perspective guards (#246, #288, #290)

- **The Mind's First Motion (Issue #246, Candidate 2):**
  - Marks an initial task pulled by `pull_task` on an empty mind (no contemplation wells, empty comprehending queue, and empty spoken language) as `first_motion` in `Offer`.
  - Admits this first pulled task into perception in `task_inflow`, resolving the sovereign mind bootstrap deadlock while preserving geometric `couples()` law.
- **Chatter Unreturned Thought Release (Issue #288):**
  - Calls `Carrier::release_unmet()` on height advances in `Inflow::Clock`, preventing quiescent minds from monotonically accumulating unacknowledged chatter frames.
- **Perspective Guard Relaxation (Issue #290):**
  - Relaxes Guard 3 in `family.rs` from `train.len() == 2` to `train.len() >= 2` so that tasks with 3, 4, or 5 demonstration pairs couple and send valid solutions to the court.
