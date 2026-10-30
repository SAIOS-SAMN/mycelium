// 78.78 — CONFLUENCE, READ (P1, an instrument, deciding nothing): the black
// hole's polarity. An output body built where several input bodies stand is
// read as their gathering, exactly, and the generator that would carry them
// onto it is read off the two gathered spheres.
//
//! THE CONFLUENCE SPHERE. The points `Pₛ` of a set of compartments, summed,
//! `C = Σ Pₛ`, is a sphere — imaginary, `C² ≤ 0` — centred on their
//! barycentre, its square their spread: `C² = −Σ_{s<t} d²ₛₜ`, the moment
//! of inertia, read by the geometric product and never rooted. It is the
//! opposing polarity of the fibre's switch `mⱼPᵢ − mᵢPⱼ` (real, `S² > 0`):
//! a white hole emits, a black hole gathers.
//!
//! PER OUTPUT BODY holding an unsaid site (78.76's remainder):
//! - its INPUTS: the input bodies standing on any of its sites; none is a
//!   creation, one a single carriage, several a CONFLUENCE;
//! - MASS: whether the output holds as many compartments as its inputs;
//! - CENTRE: whether its barycentre is the inputs', exact over ℚ;
//! - THE INVERSE GENERATOR: the fascicle of the inputs' confluence sphere
//!   and the output's, `V = C_B · C_in`, its regime read in the algebra
//!   (`pair_regime`: a turn, a kiss, a boost); where the centres agree the
//!   spheres are concentric and `V` dilates by the exact ratio of their
//!   normalised squares;
//! - REPRODUCES: whether `V` carries the inputs' compartments onto the
//!   output's, point for point;
//! - CONTAINS: whether the inputs' compartments all stand in the output;
//! - CLOSURE: the inputs' and the output's nuclei read as a reflection group
//!   (GR2): finite (a point symmetry), affine (a lattice — crystal), or
//!   infinite (fluid).

use super::*;
use crate::transducer::cga::{self, Mv};
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// THE CLASS A CONSTRUCTOR SET CLOSES IN.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Closes {
    Finite,
    Affine,
    Fluid,
    Unbounded,
    Unread,
}

/// ONE OUTPUT BODY'S CONFLUENCE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Confluence {
    pub inputs: u64,
    pub mass_in: u64,
    pub mass_out: u64,
    pub centre_kept: bool,
    /// The inverse generator's regime, by name.
    pub regime: &'static str,
    /// Where the centres agree: the dilation's exact ratio of the normalised
    /// squares, output over input.
    pub ratio: Option<Q>,
    pub reproduces: bool,
    pub contains: bool,
    pub closes: Closes,
    pub unsaid: u64,
}

/// WHAT ONE PAIR'S UNSAID BODIES READ AS.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PairConfluence {
    pub unframed: bool,
    /// Unsaid output bodies standing on no input body, on one, on several.
    pub creations: u64,
    pub singles: u64,
    pub confluences: Vec<Confluence>,
    /// The unsaid sites in each class.
    pub creation_sites: u64,
    pub single_sites: u64,
    pub confluence_sites: u64,
}

fn point(r: i64, c: i64) -> Mv {
    cga::point_at(&[Q::from_integer(r.into()), Q::from_integer(c.into())])
}

/// The confluence sphere of a set of compartments, and its mass.
pub(crate) fn gathered(sites: &[(i64, i64)]) -> (Mv, u64) {
    (sites.iter().fold(Mv::zero(), |s, &(r, c)| s.add(&point(r, c))), sites.len() as u64)
}

fn centre(sites: &[(i64, i64)]) -> Option<(Q, Q)> {
    let n = Q::from_integer(i64::try_from(sites.len()).ok()?.into());
    if n.is_zero() {
        return None;
    }
    let (sr, sc) = sites.iter().fold((Q::zero(), Q::zero()), |(a, b), &(r, c)| (a + Q::from_integer(r.into()), b + Q::from_integer(c.into())));
    Some((sr / &n, sc / n))
}

pub(crate) fn regime_name(r: cga::PairRegime) -> &'static str {
    match r {
        cga::PairRegime::Finite(_) => "turn of finite order",
        cga::PairRegime::Rosette => "rosette",
        cga::PairRegime::Parabolic => "kiss (light)",
        cga::PairRegime::Boost => "apart (dilation)",
        cga::PairRegime::Null => "null",
    }
}

fn sites_of(b: &Body) -> Vec<(i64, i64)> {
    b.compartments.iter().filter_map(|l| Some((l.site.axis(0)?, l.site.axis(1)?))).collect()
}

/// READ ONE PAIR'S UNSAID OUTPUT BODIES as creations, single carriages and
/// confluences.
pub fn read_pair(x: &Grid, y: &Grid, remainder: &[(i64, i64, u8, u8)], budget: &mut crate::physics::Budget) -> PairConfluence {
    let mut read = PairConfluence::default();
    let same_frame = x.len() == y.len() && x.iter().zip(y).all(|(a, b)| a.len() == b.len());
    if !same_frame {
        read.unframed = true;
        return read;
    }
    let unsaid: Vec<(i64, i64)> = remainder.iter().filter(|l| l.3 != 0).map(|l| (l.0, l.1)).collect();
    let ins: Vec<Vec<(i64, i64)>> = perceive(x).iter().map(sites_of).collect();
    for out in perceive(y) {
        let out_sites = sites_of(&out);
        let here = out_sites.iter().filter(|s| unsaid.contains(s)).count() as u64;
        if here == 0 {
            continue;
        }
        let inputs: Vec<&Vec<(i64, i64)>> = ins.iter().filter(|i| i.iter().any(|s| out_sites.contains(s))).collect();
        match inputs.len() {
            0 => {
                read.creations += 1;
                read.creation_sites += here;
                continue;
            }
            1 => {
                read.singles += 1;
                read.single_sites += here;
                continue;
            }
            _ => {}
        }
        read.confluence_sites += here;
        let in_sites: Vec<(i64, i64)> = inputs.iter().flat_map(|i| i.iter().copied()).collect();
        let (c_in, m_in) = gathered(&in_sites);
        let (c_out, m_out) = gathered(&out_sites);
        let centre_kept = centre(&in_sites) == centre(&out_sites);
        let regime = cga::pair_regime(&[c_in.clone(), c_out.clone()], 0, 1);
        // Concentric: the dilation's ratio, each square normalised by its
        // weight squared (the weight of a sum of points is its mass).
        let ratio = centre_kept
            .then(|| {
                let (sq_in, sq_out) = (c_in.geo(&c_in).scalar_part(), c_out.geo(&c_out).scalar_part());
                let (mi, mo) = (Q::from_integer(m_in.into()), Q::from_integer(m_out.into()));
                (!sq_in.is_zero()).then(|| (sq_out / (&mo * &mo)) / (sq_in / (&mi * &mi)))
            })
            .flatten();
        // The inverse generator carries the inputs' compartments onto the
        // output's, point for point, or it does not.
        let v = c_out.geo(&c_in);
        let reproduces = m_in == m_out
            && cga::is_versor(&v)
            && in_sites.len() <= 400
            && in_sites.iter().all(|&(r, c)| {
                let img = v.sandwich(&point(r, c));
                out_sites.iter().any(|&(rr, cc)| img.projective_eq(&point(rr, cc)))
            });
        let contains = in_sites.iter().all(|s| out_sites.contains(s));
        let nuclei: Option<Vec<Mv>> = inputs
            .iter()
            .map(|i| crate::transducer::blade::shape_of_section(i).blade.nucleus())
            .chain(std::iter::once(crate::transducer::blade::shape_of_section(&out_sites).blade.nucleus()))
            .collect();
        let closes = match nuclei {
            None => Closes::Unread,
            Some(ns) => match crate::transducer::cech::read_coxeter(&ns, &[], budget).closure {
                crate::transducer::cech::Closure::Finite { .. } => Closes::Finite,
                crate::transducer::cech::Closure::Infinite(crate::transducer::cech::Refusal::Affine(_)) => Closes::Affine,
                crate::transducer::cech::Closure::Infinite(_) => Closes::Fluid,
                crate::transducer::cech::Closure::Unbounded { .. } => Closes::Unbounded,
                crate::transducer::cech::Closure::Unread => Closes::Unread,
            },
        };
        read.confluences.push(Confluence {
            inputs: inputs.len() as u64,
            mass_in: m_in,
            mass_out: m_out,
            centre_kept,
            regime: regime_name(regime),
            ratio,
            reproduces,
            contains,
            closes,
            unsaid: here,
        });
    }
    read
}
