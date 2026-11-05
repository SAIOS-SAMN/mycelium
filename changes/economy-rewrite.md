### MU-ECONOMY specifies the economy as motions, v0.5.0's headline (docs only)
- `docs/MU-ECONOMY.md` is rewritten onto the model ruled 2026-11-05.
  There is no flat ledger: receipts are per-signer lines in a causal
  partial order, never totalised, with TUniversal height the only shared
  extent. A holding is a coordinate `origin / h : S:G:T:H : K` whose lanes
  are vectors of exact modes (`economy::modes::Modes`). A payment is a
  motion's advance, an event citing its causes; a balance and the supply
  are readings. The four proofs (POUW, POWV, POWC, POWR) are relations
  between motions. A payout is proven by re-traversal, and one closure is
  one point network-wide, so a second mint of it is burned. Between courts
  the Note settles; between universes μ crosses only at the universal core
  tube.
- §9 becomes "Built today, and what v0.5.0 changes": `economy::Mu`,
  `Treasury`, `Entry` and its eleven kinds, `Arm`, the escrow, pay,
  settlement, fold and verification paths, each marked with what replaces
  it. §12 lists nine points the model leaves to a ruling. §10.4 announces
  the shared economic law form as proposal 78.174, not drafted.
- The document goes from 936 lines to about 770: restated rulings are
  cited instead (Concordance 5.8 and 5.9 for gas and the map, 78.149 for
  discharge, 47.4 for equivocation), the v0.3.0 defect note and the
  self-correcting paragraphs are struck. Section numbers cited from code
  (§3.1, §3.2, §5.1, §5.2, §7.1) are kept.
- ROADMAP › Now gains 1b, "v0.5.0: the economy as motions".
- **Network contract:** none in this change. v0.5.0 itself changes the
  wire (μ as modes) and the persisted layout.
