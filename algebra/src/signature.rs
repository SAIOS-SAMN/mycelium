// AI-GENERATED CONTENT — see DISCLAIMER.md.
//! A4, MU-SENSING §3.7 — A VALUE OF `Cl(p, q)` OVER ℚ, ITS SIGNATURE READ FROM
//! DATA: the mask of its negative generators carried beside its `n`, every
//! composition refusing two signatures that disagree, never coercing one.

use crate::multivector::{axes, n_of_mask, Mv, EM, EP, MAX_N};
use num_rational::BigRational;

/// WHY A VALUE OF `Cl(p, q)` IS REFUSED, by name, with the generators that
/// caused it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SignatureRefusal {
    /// Two values composed whose signatures disagree on these generators,
    /// each held by both algebras: negative in one, positive in the other.
    Mismatched { generators: usize },
    /// A blade or negative mask naming a generator past [`MAX_N`]: the mask
    /// as presented.
    Malformed { mask: usize },
}

/// A MULTIVECTOR OF `Cl(p, q)`: an [`Mv`] whose negative mask is read from
/// the data, `e₋` or any other set of generators. It composes only with
/// another `Clifford`, through the products here, each of which first reads
/// the two signatures on the generators both algebras hold and refuses
/// [`SignatureRefusal::Mismatched`] where they disagree. A `Clifford` has no
/// [`Mv::geo`] and no wire writing; a value of `Cl(n+1, 1)` enters by
/// [`Clifford::of_conformal`], the inclusion.
///
/// ```compile_fail,E0599
/// use algebra::{Clifford, Mv, E1};
/// let one = num_rational::BigRational::from_integer(1.into());
/// let a = Clifford::of_conformal(&Mv::blade(E1, one));
/// // An unchecked product of a value of `Cl(p, q)` is unwritable.
/// let _ = a.geo(&a);
/// ```
#[derive(Debug, Clone)]
pub struct Clifford {
    value: Mv,
}

/// The generators the algebra at `n` holds: every base axis to `n`, `e₊`
/// and `e₋`.
fn held(n: u8) -> usize {
    axes(n).fold(EP | EM, |m, a| m | a)
}

impl Clifford {
    /// A SINGLE BLADE `mask` WITH COEFFICIENT `v`, under the negative mask
    /// `negatives`, in the smallest algebra holding both. A mask naming a
    /// generator past [`MAX_N`] is refused `Malformed` with it.
    pub fn blade(negatives: usize, mask: usize, v: BigRational) -> Result<Clifford, SignatureRefusal> {
        match [negatives, mask].into_iter().find(|m| n_of_mask(*m) > MAX_N) {
            Some(past) => Err(SignatureRefusal::Malformed { mask: past }),
            None => Ok(Clifford { value: Mv::blade_under(negatives, mask, v) }),
        }
    }

    /// THE INCLUSION OF A VALUE OF `Cl(n+1, 1)`: the same value, its negative
    /// mask `e₋`.
    pub fn of_conformal(value: &Mv) -> Clifford {
        Clifford { value: value.clone() }
    }

    /// The value's negative mask: the generators that square to −1.
    pub fn negatives(&self) -> usize {
        self.value.negatives()
    }

    /// The `n` of the smallest algebra holding the value's terms and its
    /// negative generators.
    pub fn n(&self) -> u8 {
        self.value.n()
    }

    /// The terms the value holds: each blade mask and its non-zero
    /// coefficient, ascending by mask.
    pub fn terms(&self) -> impl Iterator<Item = (usize, &BigRational)> {
        self.value.terms()
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    /// The underlying multivector under its signature.
    pub fn multivector(&self) -> &Mv {
        &self.value
    }

    /// THE TWO SIGNATURES READ ON THE GENERATORS BOTH ALGEBRAS HOLD: the
    /// generators negative in one and positive in the other, refused
    /// `Mismatched` with them where there are any.
    fn agree(&self, o: &Clifford) -> Result<(), SignatureRefusal> {
        let generators = (self.negatives() ^ o.negatives()) & held(self.n()) & held(o.n());
        match generators {
            0 => Ok(()),
            _ => Err(SignatureRefusal::Mismatched { generators }),
        }
    }

    /// THE GEOMETRIC PRODUCT `self·o` under the two values' one signature,
    /// or the refusal naming the generators on which they disagree.
    pub fn product(&self, o: &Clifford) -> Result<Clifford, SignatureRefusal> {
        self.agree(o)?;
        Ok(Clifford { value: self.value.geo(&o.value) })
    }

    /// The sum `self + o`, or the refusal naming the disagreement.
    pub fn sum(&self, o: &Clifford) -> Result<Clifford, SignatureRefusal> {
        self.agree(o)?;
        Ok(Clifford { value: self.value.add(&o.value) })
    }

    /// The difference `self − o`, or the refusal naming the disagreement.
    pub fn difference(&self, o: &Clifford) -> Result<Clifford, SignatureRefusal> {
        self.agree(o)?;
        Ok(Clifford { value: self.value.sub(&o.value) })
    }
}
