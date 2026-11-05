### "Replay" (verification) renamed to "reconstruct" — names and strings left by the prose cleanup (#103)
- **Network contract**, inside v0.4.0's break: the wire-visible refusal
  text changes, the refusal names and codes do not. `REFUSE_PIVOT_INEXACT`
  is "pivot certificate does not reconstruct" (was "...does not
  replay"); `REFUSE_UNREAD` is "the reconstruction holds a unit no
  stroke can read" (was "the replay holds..."); the server's two
  `SolveRefusal` reasons for `SolveVerdict::Unread` and
  `SolveVerdict::Unverifiable` reword "replay" to "reconstruction"/
  "reconstruct" (`bin/mycelium-server/control.rs`). A client that
  matches refusal text (rather than the refusal name or verdict code)
  updates the strings it matches.
- **Kept:** `ReceiptsRequest.replay` (`anastome::dialogue::receipts`) is
  the era-history sense (78.147: re-apply recorded receipts in order to
  read history) and is correctly named; this rename does not touch it.
- Names only, no behaviour change otherwise: the observed reading
  `glue_replay_refused` is `glue_reconstruction_refused`
  (`observe::REPLICATION_AXES`).
