### A wait reads the hold it waits behind (78.182 item 5, #221)

- **Each held court carries a `StrokeMeter`** (`physics::meter`): the hold's stroke and the reads granted on the holding thread since the take, in atomics, read by a waiter without a lock.
- **A waiter behind a hold spent past its stroke is not blocked.** The take is answered `Refused::Deferred { holder, spent, stroke, hold }` at once, and the holder is flagged: its next metered read is refused, so its work ends by its own named refusal.
- **A reflex act deferred this way** is answered with the door's deferral (`PENDING_CODE`) when its judge wrote nothing, so it is never judged on a partial reading and is safe to send again.
- **The take's refusal is typed** (`court_lock::Refused`: `Poisoned` or `Deferred`), replacing `Err(())`.
