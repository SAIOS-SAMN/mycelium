use crate::anastome::dialogue::seat::Arg;
use crate::transducer::cga::{dilator, is_versor, translator_at, weight_of, FlowPoint, Mv};
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, Zero};

/// A QUANTITY'S EXACT VALUE: a Q, a BigQ or a Count; none for a key, a flag,
/// a multivector, or a denominator of zero.
pub fn quantity(a: &Arg) -> Option<Q> {
    match a {
        Arg::Q { num, den } => (*den != 0).then(|| Q::new(BigInt::from(*num), BigInt::from(*den))),
        Arg::BigQ { num, den } => (!den.is_zero()).then(|| Q::new(num.clone(), BigInt::from(den.clone()))),
        Arg::Count(n) => Some(Q::from_integer(BigInt::from(*n))),
        // A key, a flag, a multivector — and any argument a later wire tag
        // adds — is no quantity.
        _ => None,
    }
}

/// THE SLIDE'S PLANE on a quantity's line: the translator by one, read off
/// the algebra (`translator_at`), so a slide by `Δ` stands at `t = Δ`.
pub fn slide_plane() -> Mv {
    translator_at(&[Q::one()]).grade(2)
}

/// THE SCALE'S PLANE: the dilator by two, its orientation settled against
/// the embedding (`dilator`), so a scale by `ρ` stands at `(ρ − 1)/(ρ + 1)`.
pub fn scale_plane() -> Mv {
    dilator(&Q::from_integer(BigInt::from(2))).grade(2)
}

/// ONE OBSERVED CHANGE OF A QUANTITY between two beats, as the motions of
/// its line that carry one value onto the other.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    /// The slide by the difference, on its null flow.
    pub slide: FlowPoint,
    /// The scale by the ratio, on its boost flow, where both values stand on
    /// one side of zero; none across or at zero (no dilation carries them).
    pub scale: Option<FlowPoint>,
}

impl Change {
    /// The change from `from` to `to`; `None` only where the algebra reads no
    /// flow point (a plane that is not simple — never on these planes).
    pub fn between(from: &Q, to: &Q) -> Option<Change> {
        let d = to - from;
        let slide = match d.is_zero() {
            true => FlowPoint::origin(&slide_plane())?,
            false => FlowPoint::of_motion(&translator_at(std::slice::from_ref(&d)))?.against(&slide_plane())?,
        };
        let scale = match from.is_zero() || to.is_zero() || from.is_negative() != to.is_negative() {
            true => None,
            false => {
                let rho = to / from;
                match rho.is_one() {
                    true => FlowPoint::origin(&scale_plane()),
                    false => FlowPoint::of_motion(&dilator(&rho)).and_then(|p| p.against(&scale_plane())),
                }
            }
        };
        Some(Change { slide, scale })
    }

    /// The exact difference the slide carries.
    pub fn delta(&self) -> Option<Q> {
        self.slide.parameter()
    }
}

/// 78.148 — THE VERSOR CARRYING `from` ONTO `to`: `to · from̃ / ⟨from from̃⟩₀`,
/// so that `carrier · from = to` exactly (checked, coefficient for
/// coefficient). Refused — `None` — where either is no versor (even or odd),
/// where `from`'s weight is zero, or where the product does not carry it
/// exactly: a refusal, never a zero change (rule 11).
pub fn carrier(from: &Mv, to: &Mv) -> Option<Mv> {
    let w = weight_of(from)?;
    if w.is_zero() || !is_versor(to) {
        return None;
    }
    let c = to.geo(&from.reverse()).scale(&(Q::one() / w));
    c.geo(from).sub(to).is_zero().then_some(c)
}

/// Two carriers are one where they agree coefficient for coefficient.
pub(super) fn same_carrier(a: &Mv, b: &Mv) -> bool {
    a.sub(b).is_zero()
}
