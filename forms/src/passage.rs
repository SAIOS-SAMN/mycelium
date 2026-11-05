// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE PASSAGE AT A CROSSING (LAW-PASSAGE v2, proposal 78.172; ROADMAP
//! track 24 B3), whole: the product, exact in the algebra over ℚ, and the
//! refusals beside it.
//!
//! THE LAW IS A PRODUCT IN THE ALGEBRA, NOT A NUMBER. A body's state is an
//! even multivector `q`; plane `k`'s direction `v_k` is a degree-1 element
//! of the same algebra; one crossing in plane `k` is the one-sided product
//! `q ↦ v_k·q`, never a sandwich, taking even degrees to odd ones. The
//! form states no basis for the plane: the generators `v_k` holds, and the
//! square of each, are carried by the value (`algebra::Clifford`, its
//! signature read from the data). [`pass`] is that product. It refuses, by
//! name and with the degrees it read, a direction that is not degree 1
//! ([`PassageRefusal::NotDirection`]) and a state that is not even
//! ([`PassageRefusal::NotEven`]), and refuses two values whose signatures
//! disagree ([`PassageRefusal::Mismatched`]). The four joint-read fixtures
//! of 78.172 §5 are claims in this crate's `tests/crossing.rs`.
//!
//! WHAT ELSE STANDS HERE. A crossing moves no reach: the reaches stand in
//! `v_k` as the coefficients they are, `v_k·q` moves none of them, and
//! nothing is squared. So a crossing that states a moved reach is refused
//! [`PassageRefusal::NotLawful`], naming the claimed reach against the reach
//! carried in. The throat sphere's inversion `H²/R` is such a claim: it
//! squares `H` (MU-RECURSION law 11), and it names the carried reach only at
//! the throat, where `R = ±H`. A share's reading from a plane's other
//! axis, `δ ↦ 1 − δ`, is [`pass_share`]. A reach held as anything but an
//! exact rational (LAW-FIELD's `NotExact`) has no entry here: every reach
//! and throat is a [`Q`], so a float is unwritable.
//!
//! ```compile_fail
//! // A float reach is not a reach: the crossing reads exact rationals only.
//! let throat = forms::passage::Throat::at(forms::Q::from_integer(1.into()));
//! let _ = throat.verify(&0.5_f64, &0.5_f64);
//! ```

use crate::Q;
use algebra::{Clifford, SignatureRefusal};
use num_traits::{One, Signed};

/// THE SHELL'S LAW ID, as the shared-law table names it (MU-EQUATIONS,
/// *Shared laws*): the form of LAW-PASSAGE.
pub const LAW_ID: &str = "FORMS-PASSAGE";

/// The law's version: an amendment raises it.
/// FORMAT: the shell's version, the form of LAW-PASSAGE v2 (proposal
/// 78.172) with the product `q ↦ v_k·q` held here.
pub const LAW_VERSION: u16 = 3;

/// WHY A CROSSING, ITS STATED REACH, OR A SHARE IS REFUSED, by name and
/// with the reading that caused it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PassageRefusal {
    /// The crossing states a moved reach: the breach, the claim against the
    /// reach carried in.
    NotLawful(Box<Breach>),
    /// A share outside `[0, 1]`: the value as presented.
    NotShare(Box<Q>),
    /// A direction that is not degree 1: the degrees it holds, ascending,
    /// each once (none for the zero, which holds no degree).
    NotDirection { degrees: Vec<u32> },
    /// A state that is not even: the degrees it holds, ascending, each once.
    NotEven { degrees: Vec<u32> },
    /// A direction and a state whose signatures disagree on a generator
    /// both hold: the algebra's refusal, naming those generators.
    Mismatched(SignatureRefusal),
}

/// THE DEGREES A VALUE HOLDS TERMS ON, ascending, each once: read over the
/// degrees its algebra spans, `0` to the count of its generators.
fn degrees_of(x: &Clifford) -> Vec<u32> {
    (0..=u32::from(x.n()) + 2).filter(|g| x.terms().any(|(m, _)| m.count_ones() == *g)).collect()
}

/// ONE CROSSING, `q ↦ v·q` (LAW-PASSAGE v2, 78.172 §1): the one-sided
/// geometric product of the direction `v` on the state `q`, exact, under
/// the one signature the two values carry. Never the sandwich: the even
/// state is carried to odd degrees, its other sheet.
///
/// Refused by name, in this order: `v` not degree 1, `NotDirection` with
/// the degrees it holds; `q` holding an odd degree, `NotEven` with the
/// degrees it holds; the two signatures disagreeing on a generator both
/// hold, `Mismatched` with the algebra's refusal.
pub fn pass(v: &Clifford, q: &Clifford) -> Result<Clifford, PassageRefusal> {
    let direction = degrees_of(v);
    if direction != [1] {
        return Err(PassageRefusal::NotDirection { degrees: direction });
    }
    let state = degrees_of(q);
    if state.iter().any(|g| g % 2 == 1) {
        return Err(PassageRefusal::NotEven { degrees: state });
    }
    v.product(q).map_err(PassageRefusal::Mismatched)
}

/// A BREACH OF THE PASSAGE: the reach a crossing states, and the reach it
/// carried in, which the passage leaves unmoved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Breach {
    pub claimed: Q,
    pub reconstructed: Q,
}

/// A THROAT: the reach `H` a crossing passes through, the gauge of one reach
/// line, signed and exact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Throat {
    reach: Q,
}

impl Throat {
    /// The throat at reach `h`.
    pub fn at(h: Q) -> Throat {
        Throat { reach: h }
    }

    /// The throat's reach `H`.
    pub fn reach(&self) -> &Q {
        &self.reach
    }

    /// A CROSSING READ UNDER THE LAW: the reach `r` it carries in and the
    /// reach `claimed` it states on the far side. The passage moves no
    /// reach, so the claim is admitted only where it is `r`; a moved reach
    /// (the inversion `H²/R` among them) is refused `NotLawful`, naming the
    /// claim against `r`.
    pub fn verify(&self, r: &Q, claimed: &Q) -> Result<Q, PassageRefusal> {
        match claimed == r {
            true => Ok(r.clone()),
            false => Err(PassageRefusal::NotLawful(Box::new(Breach { claimed: claimed.clone(), reconstructed: r.clone() }))),
        }
    }
}

/// A SHARE READ FROM A PLANE'S OTHER AXIS (netstratum docs/117 §8.2): a
/// plane's share `δ ∈ [0, 1]` read from its open axis is `1 − δ` read from
/// its closed axis, and the reverse. This is a reading of one state from two
/// faces, never the passage, which is the product `v_k·q` (78.172 §1). A
/// value outside `[0, 1]` is no share, and is refused `NotShare` with the
/// value.
pub fn pass_share(delta: &Q) -> Result<Q, PassageRefusal> {
    match delta.is_negative() || delta > &Q::one() {
        true => Err(PassageRefusal::NotShare(Box::new(delta.clone()))),
        false => Ok(Q::one() - delta),
    }
}
