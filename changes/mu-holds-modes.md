### Every amount of μ holds its modes (MU-ECONOMY §1, §2.5, §9; #169 step 3)

**Network contract.** `economy::Mu` now holds `economy::modes::Modes`: each lane's vector by its components, keyed by its coordinates. It no longer holds one rational per lane.
- **Founding modes.** What no motion made (`Mu::counts`, `Mu::of`, genesis, a fee, a subsidy) is held at each lane's founding mode. A mint whose proof cites a motion can carry that motion's modes (`Mu::of_modes`).
- **Readings.** A lane's whole (`Mu::lane`, `Mu::lanes`) is a reading. So is every comparison against an ask, a room or a cap.
- **Draws.** A draw takes each lane's amount from that lane's modes in proportion, exactly, never from another lane. `Treasury::debit` returns the modes drawn, so they travel with the transfer: a payment, a recycle into the pool, a hold, a stake, the POWC transfer and a burned fee.
- **The codec is the `Modes` codec.** This changes the entries, every wire field carrying μ, the snapshot (`MYT4`; `MYT3` is refused by name) and the keeper record (v3).
- **Recut.** Main's unreleased `0x0300` carries the change.
- **Unchanged for now.** Every mint still writes founding modes, so no holding's lane wholes change. The modes of a closure's motion enter with the next step.
