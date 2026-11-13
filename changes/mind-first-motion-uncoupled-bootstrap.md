### Mind first motion bootstrap on uncoupled syllabus words (#246)

- **The Mind's First Motion with Uncoupled Syllabus Language (Issue #246):**
  - Refines `is_first` in `pull_task` (`src/client/sdk/mind/learn.rs`) so that a mind holding words from the syllabus on charts that do not couple to the offer is recognized as in the bootstrap state when its contemplation wells and comprehending queue are empty and its perception carrier is free.
  - Updates `lapse_standing_task` (`src/client/sdk/mind/offer.rs`) to honor `offer.first_motion` alongside geometric coupling, preventing a standing first-motion task from prematurely lapsing across advancing finals before inflow.
