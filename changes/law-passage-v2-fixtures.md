### LAW-PASSAGE v2's fixtures are claims in the algebra (2026-11-05)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no law version moves.
- The four joint-read fixtures of proposal 78.172 §5 are claims on
  `Clifford::product` in `src/transducer/cga/tests/passage.rs`, each
  frame plane's closed pole squaring to +1 and its open pole to −1: the
  null direction annuls twice (`v·v·q = 0`); one crossing takes an even
  state to degrees 1 and 3 only; `[e_Z1Q1, e_Z1Q2] = −2·e_Q1Q2`;
  `(e_Z1Z2 + e_Q1Z2)² = 0`.
- Beside them, over every frame plane the algebra holds: every horizon
  is one-way, and passages in different planes anticommute.
- `forms::passage` does not hold the product: `forms` cannot reach the
  algebra without a dependency cycle, and 78.172 says where the
  fixtures stand.
