// 25 — THE IDEA: one bubble, its remainder, and the volume it holds.
//
//! A `Bubble` is a thought under contemplation: the form it carries, the
//! remainder cells it has not accounted for, the words it glues and is
//! glued by, and the class it was seeded from.

use super::*;

/// One class the mesh or a mind holds on a chart: key, order, representative.
#[derive(Clone, Debug)]
pub struct ClassRecord {
    pub key: crate::transducer::address::Address,
    pub order: Order,
    pub representative: Rotor,
}

pub(crate) type RemCell = (i64, i64, u8, u8);

pub(crate) fn rem_cell_key(r: i64, c: i64) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_CELL];
    buf.extend_from_slice(&r.to_le_bytes());
    buf.extend_from_slice(&c.to_le_bytes());
    crate::transducer::address::Address::new(buf)
}

pub(crate) fn remainder_addr(remainder: &[RemCell]) -> crate::transducer::address::Address {
    element::from_residual(&remainder.to_vec()).address()
}

/// The idea's key: the form's vertex key. Remainder is cargo, not the name.
pub(crate) fn idea_key(f: &Form) -> crate::transducer::address::Address {
    sentence::form_vertex_key(f)
}

pub(crate) fn form_mag(f: &Form) -> [u64; 4] {
    let mut m = element::measure_lanes(&f.remainder);
    for v in &f.vertices {
        if let Vertex::Glued { magnitude, .. } = v {
            for i in 0..4 {
                m[i] = m[i].saturating_add(magnitude[i]);
            }
        }
    }
    m
}

pub(crate) fn rem_cells_of(b: &Bubble) -> Vec<RemCell> {
    let mut out = b.remainder.clone();
    for (_, r) in b.remainders.fiber(ROOT) {
        out.extend_from_slice(r);
    }
    out
}

pub(crate) fn cover_of(inv: &Invariant) -> Keyed<()> {
    let mut c = Keyed::new();
    for p in inv.prefixes() {
        c.insert(p.key(), ());
    }
    c
}

pub(crate) fn volume_of(rems: &Keyed<Vec<RemCell>>, verts: &[impl AsRef<[u8]>]) -> [u64; 4] {
    if verts.is_empty() {
        return [0; 4];
    }
    let Some(first) = rems.get(verts[0].as_ref()) else {
        return [0; 4];
    };
    let mut acc = first.clone();
    for v in &verts[1..] {
        let Some(r) = rems.get(v.as_ref()) else {
            return [0; 4];
        };
        acc = intersect(&acc, r);
    }
    element::measure_lanes(&acc)
}

pub(crate) fn union_rem(a: &[RemCell], b: &[RemCell]) -> Vec<RemCell> {
    let mut out = a.to_vec();
    let mut have = Keyed::new();
    for &(r, c, _, _) in a {
        have.insert(rem_cell_key(r, c), ());
    }
    for cell in b {
        let k = rem_cell_key(cell.0, cell.1);
        if !have.contains(&k) {
            have.insert(k, ());
            out.push(*cell);
        }
    }
    out.sort_unstable();
    out
}

/// A held remainder with the complex it has so far and the chart prefixes
/// it sits on. Keyed by the remainder's address. Persisted; residuals are
/// never lost on restart.
#[derive(Clone, Debug, Default)]
pub struct Bubble {
    pub remainder: Vec<RemCell>,
    pub nerve: Complex,
    pub cover: Keyed<()>,
    /// Classes this idea has met, keyed by [`Rotor::class_key`]. Standing is
    /// the coordinate, never a boolean.
    pub horizon: Keyed<(Rotor, Order, Standing)>,
    pub potential: [u64; 4],
    pub volume: [u64; 4],
    pub dirty: bool,
    /// Last chart key visited by expand; the next stroke resumes here.
    pub cursor: Vec<u8>,
    /// TUniversal when this bubble's charts last moved.
    pub moved_k: u64,
    /// Addresses this idea glued (crystallized forms it rests on).
    pub glues: Keyed<()>,
    /// Ideas that glued this bubble; a non-empty set is not shed.
    pub glued_by: Keyed<()>,
    /// Filed torsion `(σ, ℓ, chord)` keyed by class, written from `pivot_loop`.
    pub torsion: Keyed<crate::transducer::sentence::FiledLoop>,
    /// Certified discharges this idea's crystallization rests on (A20).
    pub certificates: Keyed<PivotCertificate>,
    /// Incomparable discharges held as cells (25.1) until a glue or ruling.
    /// Outer key is the after class; inner is the discharging word's M1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// The form this idea is (chapter 30 step 6).
    pub form: Option<Form>,
    /// Pairs the idea germinated from; trial speaks over them for μ⃗.
    pub bundle: Option<Vec<crate::transducer::family::TrainPair>>,
    /// Per-pair remainder chains, keyed by pair index. `remainder` is their union.
    pub remainders: Keyed<Vec<RemCell>>,
}

impl Bubble {
    pub fn key(&self) -> crate::transducer::address::Address {
        match &self.form {
            Some(f) => idea_key(f),
            None => remainder_addr(&self.remainder),
        }
    }

    pub(crate) fn rotor(&self) -> Option<Rotor> {
        self.form.as_ref().and_then(|f| {
            f.vertices.iter().find_map(|v| match v {
                Vertex::Word { derivation, .. } => Some(derivation.word().rotor()),
                _ => None,
            })
        })
    }

    /// Walk outward on the cover trie from this bubble's cover, one chart
    /// per stroke, only over overlaps with nonzero volume. Extended grows
    /// the cover; Congruent grows it with a torsion axis; Class writes the
    /// horizon and stops that branch.
    pub fn expand(&mut self, atlas: &dyn AtlasRead, crystal: &dyn Crystal, budget: &mut Budget) -> usize {
        self.expand_with(atlas, &|_| None, 0, crystal, budget).0
    }

    /// `atlas` is the language this thought expands over (the working
    /// atlas, 24.1); `rotor_at` is the keyed read of a held word's rotor by
    /// address, so the Čech arm reads the word the chart names and never a
    /// table built for the stroke.
    pub fn expand_with(
        &mut self,
        atlas: &dyn AtlasRead,
        rotor_at: &dyn Fn(&[u8]) -> Option<Rotor>,
        now: u64,
        crystal: &dyn Crystal,
        budget: &mut Budget,
    ) -> (usize, [u64; 4]) {
        // THE POTENTIAL IS A READING, NEVER A GATE (L7b, law 9): the lane
        // measure of everything this idea holds open — its form's remainder
        // and its per-pair remainders, the same cells `admit_incidence`
        // keys — with T the receipts since its charts last moved, zero when
        // zero. It was floored at one on T so that the gate below would
        // pass for a form with no remainder, which paced every such idea by
        // a clock that admitted nothing (Cut B+1 C4). The pivot loop runs
        // on every dirty idea under the budget (25.1); the budget is the
        // only thing that stops it.
        self.potential = element::measure_lanes(&rem_cells_of(self));
        self.potential[2] = now.saturating_sub(self.moved_k);
        if budget.remaining() == 0 {
            return (0, [0; 4]);
        }
        let inv = stalk::invariant(&self.remainder);
        let peel = inv.peel();
        let cursor = self.cursor.clone();
        let after = if cursor.is_empty() {
            None
        } else {
            Some(cursor.as_slice())
        };
        let mut spent = 0usize;
        // index: budgeted sibling keys of one trie node (peel)
        let last = atlas.children(&peel, after, budget, &mut |child| {
            let ck = child.key();
            if self.cover.contains(&ck) {
                return;
            }
            spent += 1;
            // index: keyed fiber at this sibling chart
            let members = atlas.members(&child);
            let Some(head) = members.first() else {
                return;
            };
            let vol = self.potential;
            let Some(gamma_b) = rotor_at(head.as_slice()) else {
                return;
            };
            let cell = CechCell {
                vertices: {
                    let mut v = vec![self.key().into_vec(), head.clone()];
                    v.sort_unstable();
                    v
                },
                coefficient: gamma_b.clone(),
                volume: vol,
                fill: Fill::Open,
            };
            let Some(gamma) = self.rotor() else {
                return;
            };
            let a = Section {
                addr: self.key(),
                gamma,
            };
            let b = Section {
                addr: crate::transducer::address::Address::from(head.clone()),
                gamma: gamma_b,
            };
            match cech::extend(&a, &b, &cell) {
                Fill::Filled(_) => {
                    self.cover.insert(ck, ());
                    self.moved_k = now;
                }
                Fill::Congruent(g, ell) => {
                    self.cover.insert(ck, ());
                    self.moved_k = now;
                    self.nerve.file_congruent(self.key(), head, g, ell);
                }
                Fill::Class(r) => {
                    if r.class_key().is_none_or(|ck| !self.horizon.contains(ck.as_bytes())) {
                        horizon_put(&mut self.horizon, r, Standing::Held);
                    }
                }
                Fill::Open | Fill::Already => {}
            }
        });
        if let Some(k) = last {
            self.cursor = k;
        }
        let mut pivoted = [0u64; 4];
        if budget.remaining() > 0 {
            if let Some(mut form) = self.form.clone() {
                let key = self.key();
                let t = now.saturating_sub(self.moved_k);
                let snaps = form.pivot_loop(
                    key,
                    &self.horizon,
                    crystal,
                    budget,
                    self.bundle.as_deref(),
                    t,
                );
                for s in &snaps {
                    let m = s.lanes();
                    for i in 0..4 {
                        pivoted[i] = pivoted[i].saturating_add(m[i]);
                    }
                }
                self.take_form(form);
            }
        }
        self.dirty = spent > 0 && budget.remaining() == 0;
        (spent, pivoted)
    }

    /// A glue or a syllabus class whose key equals one held certificate's
    /// `after` applies the certificate keyed by that rotor's A1 key; the
    /// others of the class are not applied (dialect cells).
    pub(crate) fn collapse_vortex(&mut self, class: &[u8], word: &[u8], by: &'static str) -> bool {
        let Some(mut inner) = self.vortex.remove(class) else {
            return false;
        };
        let Some(win) = inner.remove(word) else {
            if !inner.is_empty() {
                self.vortex.insert(class, inner);
            }
            return false;
        };
        if let Some(mut f) = self.form.clone() {
            f.apply_pivot(&win);
            f.certificates.insert(win.key(), win.clone());
            f.vortex = Keyed::new();
            self.take_form(f);
        } else {
            self.certificates.insert(win.key(), win.clone());
            self.vortex = Keyed::new();
        }
        let axis = win.axis;
        eprintln!(
            "  vortex {:02x}{:02x}… collapsed by {by} to axis {axis:?}",
            self.key()[0],
            self.key()[1]
        );
        true
    }

    pub(crate) fn take_form(&mut self, f: Form) {
        self.remainder = f.remainder.clone();
        self.torsion = f.torsion.clone();
        self.certificates = f.certificates.clone();
        self.vortex = f.vortex.clone();
        self.form = Some(f);
    }
}
