### The exact algebra stands in its own crate, and `forms::passage` holds LAW-PASSAGE v2 whole (track 24 B3; Concordance §12)
- **Network contract:** no wire byte, act, ledger layout or reading
  changes; the oracle golden reproduces. The coefficient codec, the
  generator masks and the canonical writing are the same bytes. The
  shell version `FORMS-PASSAGE` steps 2 → 3 (the law is unchanged; the
  shell now holds its product). The Concordance's bytes change with the
  §12 landing note, so `CONCORDANCE_HASH` changes with them.
- **The `algebra` crate** (`algebra/`), below both `forms` and the
  library, depends on the exact rationals alone: `Mv`, the generator
  masks, `Clifford`, `SignatureRefusal`, the product, the wire codec of a
  value of `Cl(n+1, 1)`, `projective_ratio`, the key-component codec, and
  `Address`, `Cited`, `tag_of` and `TAG_VERSOR` (which `Mv::key` writes).
  The library re-exports every item at its old path
  (`transducer::cga::*`, `transducer::address::*`).
- **What stayed in the library:** `weight_of`, `is_versor` and the rest of
  `transducer::cga`; and a value's conjugacy class, which reads the versor
  test and writes the network's coordinates, now the trait
  `transducer::cga::Conjugacy` (`class_invariants`, `holonomy_key`,
  `class_key`), implemented for `Mv` with the same bodies.
- **`forms::passage::pass(v, q)`**, the one-sided product `v·q`, refusing
  `NotDirection` (a `v` not degree 1), `NotEven` (a `q` holding an odd
  degree) and `Mismatched` (two signatures disagreeing), each with its
  reading. The four fixtures of 78.172 §5 move to
  `forms/src/tests/crossing.rs` as claims on `pass`. `forms` re-exports
  the algebra (`forms::algebra`) and still builds alone
  (`cargo build --release -p forms`), so a kernel linking it by path gets
  the algebra with it.
- **SDK surface:** every SDK item keeps its path and type (`Mv` is the
  same type). The SDK does not re-export `Conjugacy`, so `Mv`'s three
  class readers are no longer reachable through the SDK alone; a caller
  imports `mycelium::transducer::cga::Conjugacy`.
- **The gate** reads `algebra/src/` with the other roots (reading bound,
  Euclidean vocabulary, numbers, drift).
