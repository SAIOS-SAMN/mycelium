// 28, 78.27, LD3-b — THE COMPLEX PERSISTED AS KEYED TRIES: its sections,
// its simplices and its remainder edges, each a trie by its own keys, every
// rotor and versor written as its ray, every class vector inline by its own
// coordinates. Nothing is read by its position; a trie that does not read
// whole refuses the complex.

use super::*;
use crate::network::nerve::Keyed;
use crate::snapshot::{leaf, put_trie, take_trie, Leaves};

/// The sections trie: the vertex's key → its address, its rotation as a
/// ray, its placement as a ray.
const SECTIONS: &str = "sections";
const LEAF_ADDR: u8 = 1;
const LEAF_GAMMA: u8 = 2;
const LEAF_CARRIAGE: u8 = 3;

/// The simplices trie: the simplex's key (`by_key`'s) → its degree, its
/// vertices, its coefficient and carriage as rays, its volume and its fill.
const SIMPLICES: &str = "simplices";
const LEAF_DEGREE: u8 = 1;
const LEAF_VERTICES: u8 = 2;
const LEAF_COEFFICIENT: u8 = 3;
const LEAF_SIMPLEX_CARRIAGE: u8 = 4;
const LEAF_VOLUME: u8 = 5;
const LEAF_FILL: u8 = 6;

/// The remainder trie: the key of each remainder edge, and no leaf.
const REMAINDER: &str = "remainder";

/// A leaf read whole by `read`, or refused.
fn whole<T>(v: &[u8], read: impl FnOnce(&[u8], &mut usize) -> Option<T>) -> Option<T> {
    let mut p = 0usize;
    let t = read(v, &mut p)?;
    (p == v.len()).then_some(t)
}

fn ray_rotor(r: &Rotor) -> Vec<u8> {
    let mut v = Vec::new();
    r.put_canonical(&mut v);
    v
}

fn ray_versor(g: &Versor) -> Vec<u8> {
    let mut v = Vec::new();
    g.put_canonical(&mut v);
    v
}

impl Complex {
    /// THE COMPLEX AS ITS TRIES (78.27, LD3-b): sections, simplices and
    /// remainder, each by its own keys.
    pub fn put_tries(&self, out: &mut Vec<u8>) {
        let mut sections: Keyed<Leaves> = Keyed::new();
        crate::snapshot::persist(&self.sections, |k, s: &Section| {
            sections.insert(
                k,
                vec![
                    (LEAF_ADDR, s.addr.as_bytes().to_vec()),
                    (LEAF_GAMMA, ray_rotor(&s.gamma)),
                    (LEAF_CARRIAGE, ray_versor(&s.carriage)),
                ],
            );
        });
        put_trie(out, SECTIONS, &sections);
        let mut simplices: Keyed<Leaves> = Keyed::new();
        crate::snapshot::persist(&self.by_key, |k, d| {
            let Some(c) = self.degrees.get(*d as usize).and_then(|l| l.get(k)) else { return };
            let mut verts = (c.vertices.len() as u64).to_le_bytes().to_vec();
            for v in &c.vertices {
                crate::snapshot::put_key(&mut verts, v);
            }
            let mut volume = Vec::with_capacity(32);
            for x in &c.volume {
                volume.extend_from_slice(&x.to_le_bytes());
            }
            // A CLASS IS WRITTEN AS ITS COORDINATES, INLINE (78.27 (7)):
            // the class vector by its own `put_canonical`, never its key
            // nested as a key.
            let mut fill = Vec::new();
            match &c.fill {
                Fill::Open => fill.push(0),
                Fill::Filled(g, carriage) => {
                    fill.push(1);
                    g.put_canonical(&mut fill);
                    carriage.put_canonical(&mut fill);
                }
                Fill::Class(k, r) => {
                    fill.push(2);
                    k.put_canonical(&mut fill);
                    r.put_canonical(&mut fill);
                }
                Fill::Congruent(kappa, g, ell) => {
                    fill.push(3);
                    kappa.put_canonical(&mut fill);
                    g.put_canonical(&mut fill);
                    fill.extend_from_slice(&ell.to_le_bytes());
                }
                Fill::Unspellable(r) => {
                    fill.push(4);
                    r.put_canonical(&mut fill);
                }
                // 78.26(5) — unmeasured by its reason, the reason's place in
                // its own list of arms.
                Fill::Unread(why) => {
                    fill.push(5);
                    fill.push(FillUnread::ALL.iter().position(|w| w == why).map_or(u8::MAX, |i| i as u8));
                }
            }
            simplices.insert(
                k,
                vec![
                    (LEAF_DEGREE, vec![*d]),
                    (LEAF_VERTICES, verts),
                    (LEAF_COEFFICIENT, ray_rotor(&c.coefficient)),
                    (LEAF_SIMPLEX_CARRIAGE, ray_versor(&c.carriage)),
                    (LEAF_VOLUME, volume),
                    (LEAF_FILL, fill),
                ],
            );
        });
        put_trie(out, SIMPLICES, &simplices);
        let mut remainder: Keyed<Leaves> = Keyed::new();
        crate::snapshot::persist(&self.remainder, |k, _| {
            remainder.insert(k, Vec::new());
        });
        put_trie(out, REMAINDER, &remainder);
    }

    /// The inverse of [`Complex::put_tries`], whole or refused: each simplex
    /// read back re-derives its own key and degree, and a node whose key or
    /// degree is not the simplex's refuses the complex. An unknown fill tag
    /// is a refusal, never an open cell: an open 2-cell is a 1-hole, so a
    /// bad read would invent homology the corpus does not have.
    pub fn take_tries(b: &[u8], pos: &mut usize) -> Option<Complex> {
        let mut c = Complex::new();
        let sections = take_trie(b, pos, SECTIONS, &[LEAF_ADDR, LEAF_GAMMA, LEAF_CARRIAGE])?;
        for (k, leaves) in sections.scoped() {
            let addr = crate::transducer::address::Address::from(leaf(leaves, LEAF_ADDR)?.to_vec());
            let gamma = whole(leaf(leaves, LEAF_GAMMA)?, Rotor::from_bytes)?;
            let carriage = whole(leaf(leaves, LEAF_CARRIAGE)?, Versor::from_bytes)?;
            c.sections.insert(k, Section { addr, gamma, carriage });
        }
        let simplices = take_trie(
            b,
            pos,
            SIMPLICES,
            &[LEAF_DEGREE, LEAF_VERTICES, LEAF_COEFFICIENT, LEAF_SIMPLEX_CARRIAGE, LEAF_VOLUME, LEAF_FILL],
        )?;
        for (k, leaves) in simplices.scoped() {
            let [degree] = leaf(leaves, LEAF_DEGREE)? else { return None };
            let verts = whole(leaf(leaves, LEAF_VERTICES)?, |b, p| {
                let n = usize::try_from(u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?)).ok()?;
                *p += 8;
                if n > b.len().checked_sub(*p)? {
                    return None;
                }
                let mut out = Vec::with_capacity(n);
                for _ in 0..n {
                    out.push(crate::snapshot::take_key(b, p)?);
                }
                Some(out)
            })?;
            let coefficient = whole(leaf(leaves, LEAF_COEFFICIENT)?, Rotor::from_bytes)?;
            let carriage = whole(leaf(leaves, LEAF_SIMPLEX_CARRIAGE)?, Versor::from_bytes)?;
            let volume = whole(leaf(leaves, LEAF_VOLUME)?, |b, p| {
                let mut volume = [0u64; 4];
                for x in &mut volume {
                    *x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
                    *p += 8;
                }
                Some(volume)
            })?;
            let fill = whole(leaf(leaves, LEAF_FILL)?, |b, p| {
                let tag = *b.get(*p)?;
                *p += 1;
                Some(match tag {
                    // NAMED, not reached by falling through: a state the
                    // writer emits is spelled.
                    0 => Fill::Open,
                    1 => Fill::Filled(Box::new(Rotor::from_bytes(b, p)?), Box::new(Versor::from_bytes(b, p)?)),
                    2 => {
                        let kappa = crate::transducer::sentence::ClassVector::take_canonical(b, p)?;
                        Fill::Class(Box::new(kappa), Box::new(Rotor::from_bytes(b, p)?))
                    }
                    3 => {
                        let kappa = crate::transducer::sentence::ClassVector::take_canonical(b, p)?;
                        let g = Rotor::from_bytes(b, p)?;
                        let ell = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
                        *p += 4;
                        // A17, as at every other reader: a congruence is
                        // believed only where the rotation closes at ℓ.
                        if ell == 0 {
                            return None;
                        }
                        Fill::Congruent(Box::new(kappa), Box::new(g), ell)
                    }
                    4 => Fill::Unspellable(Box::new(Rotor::from_bytes(b, p)?)),
                    5 => {
                        let why = *FillUnread::ALL.get(usize::from(*b.get(*p)?))?;
                        *p += 1;
                        Fill::Unread(why)
                    }
                    _ => return None,
                })
            })?;
            let simplex = Simplex { vertices: sorted_verts(verts), coefficient, carriage, volume, fill };
            // The key and the degree are the simplex's own, re-derived.
            if simplex.key().as_bytes() != k.as_slice() || simplex.degree() != usize::from(*degree) {
                return None;
            }
            c.remember(simplex);
        }
        let remainder = take_trie(b, pos, REMAINDER, &[])?;
        for (k, leaves) in remainder.scoped() {
            if !leaves.is_empty() {
                return None;
            }
            c.remainder.insert(k, ());
        }
        Some(c)
    }
}
