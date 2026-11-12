### MU-CLIENT §4.12, §4.13, §7.1 — Mind idle pacing and initial registry task attention (#246)

- Restores continuous language and syllabus acquisition in `court::Mind::language_inflow` when the mind holds no language (`self.language.atlas_is_empty() || self.language.genome_len() == 0`), resolving the post-#242 latch deadlock where an uncompleted initial sync tick permanently prevented subsequent syllabus pulls.
- Decouples `court::Mind::quote_inflow` syllabus requests from quote readiness `due`, ensuring an unfunded-language mind with funded reserve continually seeks syllabus across quote dwell intervals.
- Resolves the startup task attention deadlock in `learn::Mind::pull_task` for minds without local corpus feeds (`board.corpus.size().0 == 0`), allowing unexhausted initial registry task offers to land and couple into active contemplation.
- Tracks court registry exhaustion per court height (`CourtPace::task_exhausted`), ensuring settled minds rest and emit no task pull requests once the registry is exhausted at current height (#135) until court height advances or tension arises (`Mind::strained`).
