### The server's door, judges and speech hearing no longer say frame (#239)

This continues retiring "frame" by function, in the server binary and the court's speech hearing. There is no wire change.
- **Judges:** every judge takes its act as `delimited: &Delimited`, and the prose says envelope and body budget.
- **Court acts:** `Court::hear_frame` and `hear_frame_on` are now `hear_act` and `hear_act_on`, which take a speech act.
- **Senses that were never the envelope**, renamed for what they are:
  - a peer's signed act in admission and exchange;
  - the era file's record;
  - the console's refresh;
  - the measure that `why` prints.
- **Console:** the clock count reads `unmeasured`, matching `c.unmeasured`.
