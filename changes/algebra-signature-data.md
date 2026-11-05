### The algebra's signature is data: `Cl(n+1, 1)` within `Cl(p, q)` (2026-11-05)
- **Network contract:** no wire byte, act, encoding or ledger layout
  changes, and no law version moves. Every value the wire carries is in
  `Cl(n+1, 1)`, and `Mv::from_bytes` reads it there; the encoding carries
  no signature. A value of another signature has no wire writing until
  the court reads one, which will be a v0.4.0 break of its own.
- Operator ruling: the court's multivector is generalised from
  `Cl(n+1, 1)` to `Cl(p, q)`. `Mv` carries the mask of its negative
  generators beside its `n` (`Mv::negatives`), and the product's metric
  sign is read from it: −1 for each shared generator the mask names.
- Every value made by a constructor, a product or the wire carries `e₋`
  alone, so `Cl(n+1, 1)` is unchanged: the product over every blade pair
  at `n ≤ 4` equals the conformal metric's, and the oracle golden
  (`tests/oracle.golden`) reproduces line for line.
- `transducer::cga::Clifford` holds a value of any other signature. It
  composes only through `product`, `sum` and `difference`, each of which
  refuses two signatures that disagree on a generator both algebras hold
  `SignatureRefusal::Mismatched`, naming those generators; a mask past
  `MAX_N` is refused `Malformed`. It has no unchecked product (a
  `compile_fail` doc test).
