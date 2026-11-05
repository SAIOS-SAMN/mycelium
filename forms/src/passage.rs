// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE PASSAGE AT A CROSSING (LAW-PASSAGE v2, proposal 78.172; ROADMAP
//! track 24 B3): what this shell keeps of it, over the exact rationals.
//!
//! THE LAW IS A PRODUCT IN THE ALGEBRA, NOT A NUMBER. A body's state is an
//! even multivector `q`; plane `k`'s direction is `v_k = H_k·e_Zk + R_k·e_Qk`
//! with `e_Zk² = +1` and `e_Qk² = −1`; one crossing in plane `k` is the
//! one-sided product `q ↦ v_k·q`, never a sandwich, taking even degrees to
//! odd ones. That product needs the multivector algebra, which this crate
//! does not reach (it depends on the exact rationals alone), so the product
//! and its fixtures stand with the multivector algebra, the library's
//! `transducer::cga` (78.172 §5).
//!
//! WHAT STANDS HERE. A crossing moves no reach: the reach it carries in and
//! the throat's reach both stand in `v_k·q`, in exchanged places, and
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
use num_traits::{One, Signed};

/// THE SHELL'S LAW ID, as the shared-law table names it (MU-EQUATIONS,
/// *Shared laws*): the form of LAW-PASSAGE.
pub const LAW_ID: &str = "FORMS-PASSAGE";

/// The law's version: an amendment raises it.
/// FORMAT: the law version of LAW-PASSAGE v2 (proposal 78.172).
pub const LAW_VERSION: u16 = 2;

/// WHY A CROSSING'S STATED REACH, OR A SHARE, IS REFUSED, by name and with
/// the reading that caused it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PassageRefusal {
    /// The crossing states a moved reach: the breach, the claim against the
    /// reach carried in.
    NotLawful(Box<Breach>),
    /// A share outside `[0, 1]`: the value as presented.
    NotShare(Box<Q>),
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
