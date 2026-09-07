// ℚ — the coefficient field.
//
//! Every COEFFICIENT of a multivector, a rotor, a class vector and a lane
//! measure is a `BigRational`. There is no float, no fixed point and no
//! approximation with an error term: a coordinate is exact or it is refused
//! (rule 11).
//!
//! `Q` is the field those coordinates are written in, not a quantity the
//! engine reasons over. What it reasons over is versors, blades, μ⃗ per lane
//! and ℤ-ranks; "scalar" is grade 0 of a multivector and never the whole of
//! one (law 5).

pub type Q = num_rational::BigRational;
