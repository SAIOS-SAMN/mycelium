### The observer's call test owns its observer (#78)
- `observe::live` holds its state in a `Live` value: the process's mind
  is observed through `live::process()`, which every free function reads,
  and a reader may hold its own. `Live::wait` waits on a call already
  left. `live_call_waits_while_carrier_works_inside_law` raced the minds
  of other tests, which park, enter laws and take calls on the
  process-wide observer; both observer tests now read only an observer
  they own. No wire change.
