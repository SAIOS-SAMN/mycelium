### Retire `from_residue_framed` and `from_canonical_framed` in favor of `reperiod` (#239)

- Renamed `from_residue_framed` to `from_residue_reperiod` and `from_canonical_framed` to `from_canonical_reperiod` across `src/transducer/element/residue.rs` and its callers (`src/server/route.rs`, `src/server/cognition/market.rs`, `src/server/cognition/tests/book.rs`, `src/bin/mycelium-soak/charting.rs`, `src/transducer/element/tests/vertex_and_address.rs`), matching `Reperiod` per MU-ENGINEERING §9.3.
