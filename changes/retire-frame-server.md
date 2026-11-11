### The server's court, chain and observer name each "frame" by its function (#239)

"Frame" is retired across `src/server`, `src/chronometry`, `src/observe` and `src/economy`, with each sense renamed for what it is. There is no wire change.

| Sense | New name | Where |
|---|---|---|
| What an act or orbit took | **measure** | the receipt's `S:G:T:H`, the orbit close and `ChainView::collides`, ACT1's acts sealed without a measure, the escrow's draw |
| μ as a share of a pair's extent | **enclosure** (78.170 O1) | |
| A signer's signature over its line at a place | **signed prefix** | `reach.rs`, `founding.rs`, admission |
| The line file's length-prefixed batches | **appends** | |
| The era file's entry | **record** | `Era::frame()` is now `Era::record()` |
| A heard head | **head** | the back-fill, and the observer's replication labels |
| A throat's or mouth's two ends | **poses** | `MapRecord::LocalThroat.poses`, a field name only |
| The re-period two scenes measure | `cognition::reperiod_of` | was `frame_of` |
| The paging bound | **budget** | `Learner::Budget`; the observer reads `ended: budget` |
| Transport | **envelope** and **body budget** | |

The observer's body-fault axes now read `before the enclosure` and `past the enclosure`.
