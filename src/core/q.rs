//! `Q` — the exact coefficient field: every coefficient in the tree is a
//! `BigRational`, exact or refused (rule 11); no float, no fixed point, no
//! error term.

pub type Q = num_rational::BigRational;
