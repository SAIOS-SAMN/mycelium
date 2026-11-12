### An idle court parks until the next motion (#234)

As ruled on 2026-11-11 (78.182 item 6, option (a)), an idle court no longer cycles about once per scheduler floor.

- **Its next due event is always a motion.** A peer is due on the court's K, its roots or the final point; the tick is signed where it moved; the reserve regenerates at an orbit's close; and a parent is fetched on an ask.
- **So it parks with no bound** until an act is heard at the door, an act is judged, or a final point advances (`TShelf::park_until_moved`).
- **A judgement now wakes it too** (`TShelf::judged`, called after each judged act), so a seal is read at once rather than at the next arrival.
- **A court still booting** keeps the one-floor park: its genesis is not yet written, or not yet adopted from a peer, and its own cycles do that work.
- **The bounded back-off retires** (`physics::idle_backoff`, #264): the unbounded park replaces it.
