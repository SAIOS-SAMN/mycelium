// The transduction's tests, one file per organ they prove (MU-DERIVATIONS
// D3.2), and the rank-n claims of ROADMAP track 26, S4. Every fixture is a
// sample: the transduction reads nothing else.

mod enclosed;
mod factor;
mod ground;
mod motion;
mod nest;
mod nesting;
mod percept;
mod rank;
mod record;
mod relation;

use crate::transducer::base::Site;
use crate::transducer::family::transduce::{Sense, SignedPermutation, SiteChange, Transduction};
use crate::transducer::sample::{AxisReading, Place, Sample};

/// The ground the fixtures stand on: the symbol written `0`, as the reading
/// `Ground::of` makes of them (the claims of `tests::ground` read it).
pub(super) fn ground() -> crate::transducer::family::transduce::Ground<Vec<u8>> {
    crate::transducer::family::transduce::Ground::One(vec![0])
}

/// A sample of any rank from its extents and one digit per site, in
/// canonical address order, one byte per symbol.
pub(super) fn sample_n(extents: &[usize], digits: &str) -> Sample {
    let symbols = digits.bytes().map(|b| b - b'0').collect();
    Sample::new(extents.to_vec(), 1, symbols, Place::new(vec![0])).expect("the fixture stands")
}

/// A rank-2 sample from its slices along axis 0, one digit per site.
pub(super) fn sample(slices: &[&str]) -> Sample {
    let width = slices.iter().next().map_or(0, |s| s.len());
    sample_n(&[slices.len(), width], &slices.concat())
}

/// A sample's extents, read through [`Sample::extent`].
pub(super) fn extents(s: &Sample) -> Vec<usize> {
    (0..s.rank())
        .filter_map(|axis| match s.extent(axis) {
            AxisReading::Read(e) => Some(e),
            AxisReading::Unread => None,
        })
        .collect()
}

/// A sample from extents and the symbol at each site, read by `at`.
pub(super) fn sample_by(extents: &[usize], at: impl Fn(&[i64]) -> u8) -> Sample {
    let probe = Sample::new(extents.to_vec(), 1, vec![0; extents.iter().product()], Place::new(vec![0])).expect("stands");
    let symbols = probe.sites().map(|s| at(s.axes())).collect();
    Sample::new(extents.to_vec(), 1, symbols, Place::new(vec![0])).expect("the fixture stands")
}

/// The one byte of a fixture's symbol at `axes`.
pub(super) fn at(s: &Sample, axes: &[i64]) -> u8 {
    let site = Site::new(axes.to_vec()).expect("a site");
    let symbol = s.symbol_at(&site).expect("inside");
    symbol.bytes()[0]
}

/// A SAMPLE MOVED SITE BY SITE by a signed permutation: the independent
/// oracle the band-wise motion is held to. Output phase `j` reads source
/// phase `k`, reversed as `e − 1 − c`.
pub(super) fn moved_sitewise(s: &Sample, g: &SignedPermutation) -> Sample {
    let e = extents(s);
    let out: Vec<usize> = g.images().iter().map(|&(k, _)| e[k]).collect();
    sample_by(&out, |t| {
        let mut source = vec![0i64; e.len()];
        for (j, &(k, sense)) in g.images().iter().enumerate() {
            source[k] = match sense {
                Sense::Kept => t[j],
                Sense::Reversed => e[k] as i64 - 1 - t[j],
            };
        }
        at(s, &source)
    })
}

/// A SAMPLE DILATED SITE BY SITE by `k[j]` on each phase: each site a block.
pub(super) fn dilated(s: &Sample, k: &[usize]) -> Sample {
    let e: Vec<usize> = extents(s).iter().zip(k).map(|(e, k)| e * k).collect();
    sample_by(&e, |t| at(s, &t.iter().zip(k).map(|(&c, &k)| c / k as i64).collect::<Vec<i64>>()))
}

/// THE SITE-BY-SITE DIFFERENCE of two samples of one enclosure: every site
/// whose symbols differ, `(site, before, after)`, in canonical order. The
/// coboundary of the pair, read independently of the bands.
pub(super) fn delta(x: &Sample, y: &Sample) -> Vec<(Vec<i64>, Vec<u8>, Vec<u8>)> {
    x.sites()
        .zip(x.symbols().zip(y.symbols()))
        .filter(|(_, (a, b))| a != b)
        .map(|(s, (a, b))| (s.axes().to_vec(), a.bytes().to_vec(), b.bytes().to_vec()))
        .collect()
}

/// A projected record in the shape [`delta`] writes.
pub(super) fn changes(c: &[SiteChange]) -> Vec<(Vec<i64>, Vec<u8>, Vec<u8>)> {
    c.iter().map(|c| (c.site().axes().to_vec(), c.before().bytes().to_vec(), c.after().bytes().to_vec())).collect()
}

/// A stroke of exactly `n` reads.
pub(super) fn stroke(n: usize) -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(n)
}

/// The record of a pair under a stroke its need sizes.
pub(super) fn record(x: &Sample, y: &Sample) -> Transduction {
    let need = crate::transducer::family::transduce::need_of(x, y).expect("both frames stand");
    Transduction::of(x, y, &mut stroke(need)).expect("the need is the stroke")
}
