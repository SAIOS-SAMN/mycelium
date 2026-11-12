### An enclosure's width is strictly positive; a point is exact (78.182 item 24, D4.8 O1)

- **`core::certified` holds a real as `Certified::{Exact(Q), Enclosed(Encl)}`.** `Encl::new` refuses `lo ≥ hi`, the fields of `Encl` are private (a `compile_fail` doc test), and `Encl::exact` is retired.
- **Every certified function takes and returns `Certified`.** A point argument is read at its own exact path, and a value that is rational answers `Exact`: `exp 0 = 1`, `ln 1 = 0`, `sin 0 = 0`, `cos 0 = 1`, `tan 0`, `sinh 0`, `cosh 0`, `atan 0`, `acos 1`, `logistic 0 = ½`, the square root of a rational square, `lnΓ 1 = lnΓ 2 = 0`, `F(0) = 1`, and `powq` at a rational power. Bounds that meet after an operation are `Exact`.
- **The throat callers in `server::cognition::domain_passage` pass `Certified::Exact`** and read the same bounds as before.
