// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE LINEAR PASSAGE AT A CROSSING (LAW-PASSAGE v1, proposal 78.172;
//! ROADMAP track 24 B3). A crossing moves a reach `R` through a throat of
//! reach `H` by
//!
//! ```text
//!     R  ↦  2H − R
//! ```
//!
//! the reflection of the reach about `H` on one line: first degree and exact
//! over ℚ, never a square (MU-RECURSION law 11). It is an involution, so the
//! reverse crossing is the same law, and `H` is its one fixed point. On a
//! share the same passage is the reflection about ½, `δ ↦ 1 − δ`.
//!
//! THE READING. The reach is the gauge of a one-dimensional reading, the
//! reach along one line (MU-SENSING §3.7), never a scalar standing for a
//! multivector. It is SIGNED and never clamped: past `2H` the passage is
//! negative, and the involution returns `R` only because the sign survives.
//!
//! WHAT IT REFUSES. A crossing that states a motion other than the passage it
//! claims is refused [`PassageRefusal::NotLawful`], naming the claimed reach
//! against the reconstructed `2H − R`, never admitted on the law's name. The
//! throat sphere's inversion `H²/R` is such a motion: it squares `H`, and it
//! agrees with the passage only at the throat itself, where both fix `H`.
//! A reach held as anything but an exact rational (LAW-FIELD's `NotExact`)
//! has no entry here: every reach and throat is a [`Q`], so a float is
//! unwritable.
//!
//! ```compile_fail
//! // A float reach is not a reach: the passage takes exact rationals only.
//! let throat = forms::passage::Throat::at(forms::Q::from_integer(1.into()));
//! let _ = throat.pass(&0.5_f64);
//! ```

use crate::Q;
use num_traits::{One, Signed};

/// THE SHELL'S LAW ID, as the shared-law table names it (MU-EQUATIONS,
/// *Shared laws*): the form of LAW-PASSAGE.
pub const LAW_ID: &str = "FORMS-PASSAGE";

/// The law's version: an amendment raises it.
pub const LAW_VERSION: u16 = 1;

/// WHY A CROSSING'S PASSAGE, OR A SHARE, IS REFUSED, by name and with the
/// reading that caused it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PassageRefusal {
    /// The crossing's stated reach is not the linear passage: the breach,
    /// the claim against the reconstruction.
    NotLawful(Box<Breach>),
    /// A share outside `[0, 1]`: the value as presented.
    NotShare(Box<Q>),
}

/// A BREACH OF THE PASSAGE: the reach a crossing states, and the reach
/// `2H − R` reconstructed under the law from what it carried in.
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

    /// THE LINEAR PASSAGE of reach `r` through this throat: `2H − r`, exact
    /// and signed.
    pub fn pass(&self, r: &Q) -> Q {
        let two = Q::from_integer(2.into());
        two * &self.reach - r
    }

    /// A CROSSING READ UNDER THE LAW: the reach `r` it carries in and the
    /// reach `claimed` it states on the far side. The passage is
    /// reconstructed from `r` and the throat, and the claim is admitted only
    /// on that exact object; any other motion is refused `NotLawful`, naming
    /// the claim against the reconstruction.
    pub fn verify(&self, r: &Q, claimed: &Q) -> Result<Q, PassageRefusal> {
        let reconstructed = self.pass(r);
        match claimed == &reconstructed {
            true => Ok(reconstructed),
            false => Err(PassageRefusal::NotLawful(Box::new(Breach { claimed: claimed.clone(), reconstructed }))),
        }
    }
}

/// THE PASSAGE OF A SHARE `δ ∈ [0, 1]`: the linear passage about the throat
/// at ½, `1 − δ`. A value outside `[0, 1]` is no share, and is refused
/// `NotShare` with the value.
pub fn pass_share(delta: &Q) -> Result<Q, PassageRefusal> {
    match delta.is_negative() || delta > &Q::one() {
        true => Err(PassageRefusal::NotShare(Box::new(delta.clone()))),
        false => Ok(Throat::at(Q::one() / Q::from_integer(2.into())).pass(delta)),
    }
}
