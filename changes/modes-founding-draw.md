### μ as modes: the founding mode and the per-lane draw (MU-ECONOMY §2.5, #169)

This builds the first parts of `Modes` replacing `Mu` (v0.5.0), on 78.180 ruling 3:
- **The founding mode.** `Mode::Founding { lane }` is a lane's identity motion. A genesis allocation, a Note's mode-0 receipt and a mint whose proof cites no motion are held at it, never with empty modes. It is written as a leading byte `0x10` plus the lane, apart from every lane byte. It is refused past the four base lanes (`Mode::founding`, `Modes::founding`).
- **The per-lane draw.** `Modes::draw_lane` takes an amount on one lane from that lane's modes in proportion to their amounts, exactly in ℚ, and never from another lane. The modes drawn are returned so they travel with the transfer. Where the lane holds less, the draw is refused whole, naming the lane's first mode with the lane's whole.
- **No per-lane total is exposed.** Room, prices and asks stay readings per lane and per mode (§12 items 5, 8).
