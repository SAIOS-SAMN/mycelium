use super::{landed_hue, Body};
use crate::transducer::blade;
use crate::transducer::element::Versor;

/// One body carried onto another: the transport and the gauge map.
#[derive(Clone, Debug)]
pub struct Carriage {
    pub v: Versor,
    pub sigma: Vec<(u8, u8)>,
}

/// Carry `a` onto `b`: the versor from the moments, and the hue map read
/// compartment by compartment on the base `(r, c, k)` — functional, or refused. One
/// reading for a body in one frame and a body across frames: a frame-0
/// body's compartments stand at `k = 0` (51.3), which is the compartments' coordinate and
/// not a second path.
pub fn carriage(a: &Body, b: &Body, budget: &mut crate::physics::Budget) -> Option<Carriage> {
    let v = blade::versor_between(&a.shape, &b.shape, budget).ok()?;
    let mut sigma: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for l in &a.compartments {
        let h2 = landed_hue(&v, &l.site, b)?;
        let key = crate::transducer::element::hue_pair_key(l.hue, 0);
        match sigma.get(&key) {
            Some(t) if *t != h2 => return None,
            Some(_) => {}
            None => {
                sigma.insert(key, h2);
            }
        }
    }
    let mut out: Vec<(u8, u8)> = sigma
        .scoped()
        .filter_map(|(k, t)| crate::transducer::element::hue_pair_of(&k).map(|(h, _)| (h, *t)))
        .collect();
    out.sort_unstable();
    Some(Carriage { v, sigma: out })
}

/// Carry `a` onto `b`: directly via `carriage`, or by factoring a
/// planar projection artifact with `Surgery::factor` to align the homeomorphic core.
/// The continuous versor is preserved; the tape remains an invertible group action.
pub fn carriage_with_core(a: &Body, b: &Body, budget: &mut crate::physics::Budget) -> Option<Carriage> {
    if let Some(car) = carriage(a, b, budget) {
        return Some(car);
    }
    let (surg, v) = crate::transducer::blade::surgery::Surgery::factor(&a.shape, &b.shape, budget)?;
    match surg.kind {
        crate::transducer::blade::surgery::SurgeryKind::FillHole
        | crate::transducer::blade::surgery::SurgeryKind::Puncture
        | crate::transducer::blade::surgery::SurgeryKind::BridgeMerge
        | crate::transducer::blade::surgery::SurgeryKind::SplitCut => {}
    }
    let mut sigma: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for l in &a.compartments {
        let Some(h2) = landed_hue(&v, &l.site, b) else { continue };
        let k = crate::transducer::element::hue_pair_key(l.hue, 0);
        match sigma.get(&k) {
            Some(t) if *t != h2 => return None,
            Some(_) => {}
            None => {
                sigma.insert(k, h2);
            }
        }
    }
    let mut out: Vec<(u8, u8)> = sigma
        .scoped()
        .filter_map(|(k, t)| crate::transducer::element::hue_pair_of(&k).map(|(h, _)| (h, *t)))
        .collect();
    out.sort_unstable();
    Some(Carriage { v, sigma: out })
}
