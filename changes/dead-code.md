### Dead code removed (2026-11-05)
- The retired acts' codecs are removed: the docket request and reply
  (`encode_docket_request`, `decode_docket_request`, `encode_docket_reply`
  and its `_with` and `_paged` forms, `decode_docket_reply`,
  `decode_settlements`, `decode_cavities`, and the `Docket` record), the
  teach and settle codecs, and `encode_outcome_reply`. The bytes stand in
  `Action` and the door still refuses them by kind; one byte fixture per
  retired act keeps that refusal tested.
