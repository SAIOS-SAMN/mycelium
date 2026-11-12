### The wire's envelope is no longer called a frame (#239, MU-ENGINEERING §9.3)

The 2026-11-11 ruling (#239) retires "frame" in every sense, as a collapse of volume and dimensions. This entry renames the wire envelope's sense across the server, the wire and the documents. There is no wire change.
- **The body budget:** `network::wire::frame_budget()` is removed. Every caller reads `BODY_BUDGET`, the one registered number, whose documentation now holds the wire-MTU ruling's history.
- **Noise refusals:** `NoiseRefusal::FrameOverBudget` and `FrameMalformed` are now `OverBudget` and `Malformed`.
- **Paging and carriage:** `Court::receipts_frame` is now `receipts_bounds`, and the carriage tally's `note_frame` and `of_frames` are now `note_delimited` and `of_delimited`.
- **Observer reading:** the carrier the observer reads as `frame` is now `delimited`.
- **Prose and tests:** the transport's documents and test names say envelope.
- **Still to do on the client's mind:** `Inflow::Frame`, `hear_frame`, `read_frame` and `hold_answer_frame` are listed in §9.3.
