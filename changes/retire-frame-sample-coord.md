### Retire `ChartAt::Frame`, `VertexAt::FRAME`, and `coord::put_frame`/`take_frame` (#239)

- Renamed `coord::ChartAt::Frame` to `coord::ChartAt::Enclosure` (matching `VertexAt::Enclosure`) and `VertexAt::FRAME` to `VertexAt::ENCLOSURE`.
- Renamed `coord::put_frame` / `coord::take_frame` to `coord::put_sample_index` / `coord::take_sample_index` across `src/network/nerve/keyed/coordinate.rs` and its callers (byte encoding unchanged).
