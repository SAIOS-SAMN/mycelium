// The one scalar: exact rational arithmetic, and nothing above it.
//
//! Every number in this engine is a `BigRational`. There is no float, no
//! fixed point, and no approximation with an error term: a value is exact or
//! it is refused (rule 11). `Q` is the only thing `core` exports to the rest
//! of the tree — the matrix carriers that used to sit beside it (`QMatrix`,
//! the antisymmetric `Delta`, the conjugating `Operator`) were the squared
//! shadow law 11 refuses, and they are gone.

pub type Q = num_rational::BigRational;
