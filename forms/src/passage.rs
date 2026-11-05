// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE PASSAGE AT A CROSSING (LAW-PASSAGE v3, proposal 78.172; netstratum
//! docs/117 §10 and §12; ROADMAP track 24 B3), whole: the pole swap, the
//! riding state's pass and the flux, exact in the algebra over ℚ, and the
//! refusals beside them.
//!
//! THE FRAME AND ITS POLES. A plane is held by its two units, `e₊`
//! (squaring to +1) and `e₋` (squaring to −1), each one generator of a value
//! of `Cl(p, q)` whose signature is read from the data ([`Poles`]). Its two
//! poles are the null directions `n_Z = e₊ + e₋` (closed) and
//! `n_Q = e₊ − e₋` (open). A frame state is `H·n_Z + R·n_Q`, the
//! enclosure's hold `H` and the body's reach `R` (the code's `(A, Q)`,
//! netstratum docs/117 §12.6), which is `(H + R)·e₊ + (H − R)·e₋`: the
//! whole on `e₊` and the span `H − R` on `e₋` ([`Frame`]). The span is
//! stored signed on `S`, and its sign is the sheet.
//!
//! A PASSAGE IS THE POLE SWAP, the reflection in the throat's unit `e₊`:
//! - the frame state goes by the sandwich `x ↦ e₊·x·e₊` ([`Poles::swap`]):
//!   `e₊` is kept and `e₋ ↦ −e₋`, so `n_Z ↔ n_Q` and `H − R ↦ R − H`, with
//!   `|H − R|` kept. The roles exchange: a body at reach `R` about hold `H`
//!   stands, past the throat, at reach `H` about hold `R`. The whole
//!   `H + R` is kept and the face inverts, `δ = H/(H + R) ↦ 1 − δ`
//!   ([`Frame::share`], [`pass_share`]); no reach is mapped into the old
//!   frame. Within one enclosure the passage is that face exchange alone;
//!   across a throat it is the re-origin (`FORMS-COORDINATE`, not here);
//! - the riding state `q` goes one-sided, `q ↦ e₊·q` ([`Poles::ride`]),
//!   which flips its parity. Never the sandwich for `q`;
//! - two passes are the identity on both objects, because `e₊·e₊ = 1`:
//!   no square is read as a quantity.
//!
//! THE FLUX. One passage writes `T ↦ T − 2·(H − R)`, with `H − R` the span
//! before the pass: signed, first degree ([`Poles::cross`]). A pass and its
//! immediate return cancel on `T`. At the throat (`H = R`) the span is zero
//! and nothing is written. At a pole (`H = 0` or `R = 0`, a null frame
//! state) the swap carries the state to the other pole, and the passage is
//! one-way there: it never returns to the interior ([`Place`]).
//!
//! WHAT ELSE STANDS HERE. A plane's direction `v_k` is no longer the
//! passage operator; it stays the plane's direction in the advance. A
//! crossing that states a reach moved within the old frame is refused
//! [`PassageRefusal::NotLawful`], naming the claim against the reach carried
//! in: the throat sphere's inversion `H²/R` squares `H` (MU-RECURSION law
//! 11) and names the carried reach only at the throat. Every refusal names
//! what it read: a pole that is not one generator at coefficient one
//! (`NotUnit`), a pole of the other sign (`WrongSign`), a frame state off
//! the plane (`NotFrame`), a riding state of both parities
//! (`MixedParity`), a share of no whole (`NoWhole`) or outside `[0, 1]`
//! (`NotShare`), and two signatures that disagree (`Mismatched`). Every
//! reach, hold and flux is a [`Q`], so a float is unwritable.
//!
//! ```compile_fail
//! // A float reach is not a reach: the crossing reads exact rationals only.
//! let throat = forms::passage::Throat::at(forms::Q::from_integer(1.into()));
//! let _ = throat.verify(&0.5_f64, &0.5_f64);
//! ```

use crate::Q;
use algebra::{Clifford, SignatureRefusal};
use num_traits::{One, Signed, Zero};

/// THE SHELL'S LAW ID, as the shared-law table names it (MU-EQUATIONS,
/// *Shared laws*): the form of LAW-PASSAGE.
pub const LAW_ID: &str = "FORMS-PASSAGE";

/// The law's version: an amendment raises it.
/// FORMAT: the shell's version, the form of LAW-PASSAGE v3 (proposal
/// 78.172): the pole swap, the riding state's pass and the flux.
pub const LAW_VERSION: u16 = 4;

/// WHY A CROSSING, ITS STATED REACH, OR A SHARE IS REFUSED, by name and
/// with the reading that caused it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PassageRefusal {
    /// The crossing states a moved reach: the breach, the claim against the
    /// reach carried in.
    NotLawful(Box<Breach>),
    /// A share outside `[0, 1]`: the value as presented.
    NotShare(Box<Q>),
    /// A frame state whose whole `H + R` is zero holds no share: its span.
    NoWhole { span: Q },
    /// A pole that is not one generator at coefficient one: the terms it
    /// holds, each blade mask with its coefficient.
    NotUnit { terms: Vec<(usize, Q)> },
    /// A pole whose generator squares to the other sign, read from the
    /// negative mask: `e₊` named negative, or `e₋` named positive.
    WrongSign { generator: usize },
    /// A frame state that is not on the plane: the blade masks it holds off
    /// the plane's two units (none for the zero, which holds no state).
    NotFrame { blades: Vec<usize> },
    /// A riding state holding both parities: the degrees it holds,
    /// ascending, each once.
    MixedParity { degrees: Vec<u32> },
    /// Two values whose signatures disagree on a generator both hold: the
    /// algebra's refusal, naming those generators.
    Mismatched(SignatureRefusal),
}

/// THE DEGREES A VALUE HOLDS TERMS ON, ascending, each once: read over the
/// degrees its algebra spans, `0` to the count of its generators.
fn degrees_of(x: &Clifford) -> Vec<u32> {
    (0..=u32::from(x.n()) + 2).filter(|g| x.terms().any(|(m, _)| m.count_ones() == *g)).collect()
}

/// The coefficient `x` holds on blade `mask`, zero where it holds none.
fn coefficient(x: &Clifford, mask: usize) -> Q {
    x.terms().find(|(m, _)| *m == mask).map_or_else(Q::zero, |(_, c)| c.clone())
}

/// THE GENERATOR A UNIT IS: one term, on one generator, at coefficient
/// one; refused `NotUnit` with the terms it holds otherwise.
fn generator_of(unit: &Clifford) -> Result<usize, PassageRefusal> {
    let terms: Vec<(usize, Q)> = unit.terms().map(|(m, c)| (m, c.clone())).collect();
    match terms.as_slice() {
        [(m, c)] if m.count_ones() == 1 && c.is_one() => Ok(*m),
        _ => Err(PassageRefusal::NotUnit { terms }),
    }
}

/// A FRAME STATE READ: the enclosure's hold `H` (the closed count `A`) and
/// the body's reach `R` (the open count `Q`), the state `H·n_Z + R·n_Q`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub hold: Q,
    pub reach: Q,
}

/// WHERE A FRAME STATE STANDS ON ITS PLANE, read from the state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    /// `H = R`: the span is zero, the fixed sphere of the swap. A passage
    /// here writes nothing on `T`. The zero frame, which holds no state,
    /// reads here.
    Throat,
    /// `H = 0` or `R = 0`: a null state, wholly at one pole. The swap
    /// carries it to the other pole and never back into the interior.
    Pole(Pole),
    /// Neither: a state with both counts held and a span of either sign.
    Interior,
}

/// A PLANE'S TWO POLES, its null directions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pole {
    /// `n_Z = e₊ + e₋`: wholly closed, `R = 0`.
    Closed,
    /// `n_Q = e₊ − e₋`: wholly open, `H = 0`.
    Open,
}

impl Frame {
    /// THE WHOLE `H + R`, the coefficient on `e₊`; a passage keeps it.
    pub fn whole(&self) -> Q {
        &self.hold + &self.reach
    }

    /// THE SPAN `H − R`, the coefficient on `e₋`, stored signed on `S`: its
    /// sign is the sheet, and a passage reverses it.
    pub fn span(&self) -> Q {
        &self.hold - &self.reach
    }

    /// THE FACE `δ = H/(H + R)`, the share of the whole the hold holds: a
    /// passage takes it to `1 − δ` ([`pass_share`]). Refused `NoWhole` with
    /// the span where the whole is zero, and `NotShare` with the value
    /// where it falls outside `[0, 1]`.
    pub fn share(&self) -> Result<Q, PassageRefusal> {
        let whole = self.whole();
        match whole.is_zero() {
            true => Err(PassageRefusal::NoWhole { span: self.span() }),
            false => {
                let delta = &self.hold / whole;
                match delta.is_negative() || delta > Q::one() {
                    true => Err(PassageRefusal::NotShare(Box::new(delta))),
                    false => Ok(delta),
                }
            }
        }
    }

    /// WHERE THE STATE STANDS: at the throat where `H = R`, at a pole where
    /// one count alone is zero, in the interior otherwise. Read from the
    /// counts, never from a square: the null test `(H·n_Z + R·n_Q)² = 4HR`
    /// is not computed.
    pub fn place(&self) -> Place {
        match (self.hold == self.reach, self.hold.is_zero(), self.reach.is_zero()) {
            (true, _, _) => Place::Throat,
            (false, false, true) => Place::Pole(Pole::Closed),
            (false, true, false) => Place::Pole(Pole::Open),
            _ => Place::Interior,
        }
    }
}

/// ONE CROSSING'S RESULT: the frame state past the swap, its span (`S`,
/// the sheet's sign reversed) and the flux written on `T`.
#[derive(Clone, Debug)]
pub struct Crossing {
    pub frame: Clifford,
    pub span: Q,
    pub flux: Q,
}

/// A PLANE, HELD BY ITS TWO UNITS: the generator of `e₊`, the throat's
/// unit, the generator of `e₋`, and the one signature both are read under.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poles {
    positive: usize,
    negative: usize,
    negatives: usize,
}

impl Poles {
    /// THE PLANE OF THE UNITS `positive` (`e₊`) AND `negative` (`e₋`).
    /// Refused, in this order: a unit that is not one generator at
    /// coefficient one, `NotUnit`; two signatures that disagree,
    /// `Mismatched`; `e₊` negative or `e₋` positive under the one
    /// signature, `WrongSign` naming the generator.
    pub fn of(positive: &Clifford, negative: &Clifford) -> Result<Poles, PassageRefusal> {
        let (plus, minus) = (generator_of(positive)?, generator_of(negative)?);
        positive.product(negative).map_err(PassageRefusal::Mismatched)?;
        let negatives = positive.negatives() | negative.negatives();
        match (plus & negatives, minus & negatives) {
            (0, m) if m == minus => Ok(Poles { positive: plus, negative: minus, negatives }),
            (0, _) => Err(PassageRefusal::WrongSign { generator: minus }),
            _ => Err(PassageRefusal::WrongSign { generator: plus }),
        }
    }

    /// `c` on the generator `mask`, under the plane's signature.
    fn blade(&self, mask: usize, c: Q) -> Result<Clifford, PassageRefusal> {
        Clifford::blade(self.negatives, mask, c).map_err(PassageRefusal::Mismatched)
    }

    /// THE THROAT'S UNIT `e₊`, under the plane's signature.
    fn unit(&self) -> Result<Clifford, PassageRefusal> {
        self.blade(self.positive, Q::one())
    }

    /// THE FRAME STATE `H·n_Z + R·n_Q = (H + R)·e₊ + (H − R)·e₋`.
    pub fn state(&self, frame: &Frame) -> Result<Clifford, PassageRefusal> {
        self.blade(self.positive, frame.whole())?.sum(&self.blade(self.negative, frame.span())?).map_err(PassageRefusal::Mismatched)
    }

    /// THE FRAME STATE `x` READ: its hold and reach, from the whole on `e₊`
    /// and the span on `e₋`. Refused `Mismatched` where `x`'s signature
    /// disagrees with the plane's, and `NotFrame` with the blades it holds
    /// off the plane (none for the zero).
    pub fn read(&self, x: &Clifford) -> Result<Frame, PassageRefusal> {
        self.blade(0, Q::zero())?.sum(x).map_err(PassageRefusal::Mismatched)?;
        let blades: Vec<usize> = x.terms().map(|(m, _)| m).filter(|m| *m != self.positive && *m != self.negative).collect();
        match (blades.is_empty(), x.is_zero()) {
            (true, false) => {
                let (whole, span) = (coefficient(x, self.positive), coefficient(x, self.negative));
                let half = algebra::multivector::half();
                Ok(Frame { hold: (&whole + &span) * &half, reach: (whole - span) * half })
            }
            _ => Err(PassageRefusal::NotFrame { blades }),
        }
    }

    /// THE POLE SWAP, `x ↦ e₊·x·e₊`: the reflection in the throat's unit,
    /// `n_Z ↔ n_Q`, the span reversed and the whole kept. Refused as
    /// [`Poles::read`] refuses `x`.
    pub fn swap(&self, x: &Clifford) -> Result<Clifford, PassageRefusal> {
        self.read(x)?;
        let e = self.unit()?;
        e.product(x).and_then(|ex| ex.product(&e)).map_err(PassageRefusal::Mismatched)
    }

    /// ONE CROSSING of the frame state `x` with `flux` on `T`: the swap,
    /// the span past it, and `T − 2·(H − R)` with the span before. Refused
    /// as [`Poles::read`] refuses `x`.
    pub fn cross(&self, x: &Clifford, flux: &Q) -> Result<Crossing, PassageRefusal> {
        let before = self.read(x)?.span();
        let frame = self.swap(x)?;
        let span = self.read(&frame)?.span();
        Ok(Crossing { frame, span, flux: flux - &before - &before })
    }

    /// THE RIDING STATE'S PASS, `q ↦ e₊·q`: one-sided, never the sandwich,
    /// so its parity flips. Refused `MixedParity` with the degrees it holds
    /// where `q` holds both parities, and `Mismatched` where its signature
    /// disagrees with the plane's.
    pub fn ride(&self, q: &Clifford) -> Result<Clifford, PassageRefusal> {
        let degrees = degrees_of(q);
        match (degrees.iter().any(|g| g % 2 == 0), degrees.iter().any(|g| g % 2 == 1)) {
            (true, true) => Err(PassageRefusal::MixedParity { degrees }),
            _ => self.unit()?.product(q).map_err(PassageRefusal::Mismatched),
        }
    }
}

/// A BREACH OF THE PASSAGE: the reach a crossing states, and the reach it
/// carried in, which the passage leaves unmoved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Breach {
    pub claimed: Q,
    pub reconstructed: Q,
}

/// A THROAT: the reach `R = H` at which a crossing's span is zero, the
/// enclosure's hold, signed and exact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Throat {
    reach: Q,
}

impl Throat {
    /// The throat at hold `h`.
    pub fn at(h: Q) -> Throat {
        Throat { reach: h }
    }

    /// The throat's reach, the hold `H`.
    pub fn reach(&self) -> &Q {
        &self.reach
    }

    /// A CROSSING READ IN THE OLD FRAME: the reach `r` it carries in and the
    /// reach `claimed` it states on the far side. The swap keeps `|H − R|`
    /// and maps no reach into the old frame, so the claim is admitted only
    /// where it is `r`; a moved reach (the reflection `2H − R` and the
    /// inversion `H²/R` among them) is refused `NotLawful`, naming the claim
    /// against `r`.
    pub fn verify(&self, r: &Q, claimed: &Q) -> Result<Q, PassageRefusal> {
        match claimed == r {
            true => Ok(r.clone()),
            false => Err(PassageRefusal::NotLawful(Box::new(Breach { claimed: claimed.clone(), reconstructed: r.clone() }))),
        }
    }
}

/// THE FACE READ FROM THE OTHER POLE (netstratum docs/117 §8.2 and
/// §12.6): a frame state's share `δ = H/(H + R) ∈ [0, 1]` read past the
/// swap is `1 − δ`, and the reverse. A reading of one state from two
/// faces, never the swap itself ([`Poles::swap`]). A value outside
/// `[0, 1]` is no share, and is refused `NotShare` with the value.
pub fn pass_share(delta: &Q) -> Result<Q, PassageRefusal> {
    match delta.is_negative() || delta > &Q::one() {
        true => Err(PassageRefusal::NotShare(Box::new(delta.clone()))),
        false => Ok(Q::one() - delta),
    }
}
