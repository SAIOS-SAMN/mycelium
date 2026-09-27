// 25 — THE IDEA: one bubble, its remainder, and the volume it holds.
//
//! A `Bubble` is a thought under contemplation: the form it carries, the
//! remainder links it has not accounted for, the words it glues and is
//! glued by, and the class it was seeded from.

use super::*;

/// THE KEYED READ OF A HELD WORD'S SECTION BY ADDRESS: the rotation the
/// chart names and the carriage it names it at. Both halves, because a
/// transition is `D(ρ)·T(t)·R` and a reader that answers `R` alone makes
/// every chart read as if it sat at the origin — which is not a fact about
/// the charts but about the reader's type.
pub type SectionAt<'a> = &'a dyn Fn(&[u8]) -> Option<(Rotor, crate::transducer::element::Versor)>;

/// ONE SECTION THIS IDEA HOLDS: the chart it stands on, the rotation it
/// speaks there, and the carriage it speaks it by. The chart is a
/// coordinate, which is what lets the cover be READ rather than searched
/// for.
pub type Held = (Invariant, Rotor, crate::transducer::element::Versor);

/// 78.37 — ONE DEMAND OF CS2's ARM: the undischarged orbit's class, the
/// charts it stands on, and (78.47) its residue, where it read.
pub type Demand = (sentence::ClassVector, Vec<Invariant>, Option<crate::transducer::cga::Mv>);

/// WHAT ONE EXPANSION OF AN IDEA DID: the charts spent, the lane measure of
/// its snaps, 37.7's adjunction candidates (a rotor and its chart), and the
/// demands its pivots left for the docket.
#[derive(Default)]
pub struct Expansion {
    pub spent: usize,
    pub pivoted: [u64; 4],
    pub adjoined: Vec<(Rotor, Invariant)>,
    pub demanded: Vec<Demand>,
    /// R1 — the class keys of the orbits its pivots discharged.
    pub discharged: Vec<crate::transducer::address::Address>,
    /// STD1 — snaps whose measure would pass its carrier, left out of
    /// `pivoted` and counted: the contemplation carries the count.
    pub overflowed: u64,
}

/// KB1 — A SECTION'S COORDINATES, the vertex of an idea's nerve: the idea's
/// address, the chart, and the carriage the section acts by, each framed. A
/// Čech simplex is the overlap of two sections over one chart, so its
/// vertices are sections and never bare addresses.
pub(crate) fn section_vertex(
    address: &[u8],
    chart: &Invariant,
    carriage: &crate::transducer::element::Versor,
) -> Vec<u8> {
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, address);
    crate::transducer::address::put_key_bytes(&mut k, chart.key().as_bytes());
    let mut c = Vec::new();
    carriage.put_canonical(&mut c);
    crate::transducer::address::put_key_bytes(&mut k, &c);
    k
}

/// One class the mesh or a mind holds on a chart: key, order, representative.
#[derive(Clone, Debug)]
pub struct ClassRecord {
    pub key: crate::transducer::address::Address,
    pub order: Order,
    pub representative: Rotor,
}

/// THE KEY A REMAINDER LINK IS FILED UNDER: its site, written out. It opens
/// with the simplex tag, the one byte a site key and a simplex key share.
pub(crate) fn remainder_site_key(r: i64, c: i64) -> crate::transducer::address::Address {
    let mut buf = vec![crate::transducer::address::TAG_SIMPLEX];
    buf.extend_from_slice(&r.to_le_bytes());
    buf.extend_from_slice(&c.to_le_bytes());
    crate::transducer::address::Address::new(buf)
}

/// Receipts since a bubble's charts last moved, on the kernel's own K
/// (law 9). The mark is a K this clock has already passed, so the extent
/// is exact.
///
/// `None` where the mark is ABOVE the clock: a K that went backwards is a
/// broken count, and an extent of zero read off one is a measurement this
/// clock cannot make, not a bubble that moved just now (rule 11).
pub(crate) fn held_since(now: u64, moved_k: u64) -> Option<u64> {
    now.checked_sub(moved_k)
}

/// The address of a chain A5 has accepted. Total: the type is the proof,
/// so there is nothing here to refuse.
pub(crate) fn remainder_addr(
    remainder: &element::CanonicalChain,
) -> crate::transducer::address::Address {
    element::from_canonical(remainder).address()
}

/// The idea's key: the form's vertex key. Remainder is cargo, not the name.
pub(crate) fn idea_key(f: &Form) -> crate::transducer::address::Address {
    sentence::form_vertex_key(f)
}

/// The form's magnitude: its remainder's measure and what its glued vertices
/// carry, per lane; `None` where the sum would pass its carrier (STD1).
pub(crate) fn form_mag(f: &Form) -> Option<[u64; 4]> {
    let glued = f.vertices.scoped().filter_map(|(_, v)| match v {
        Vertex::Glued { magnitude, .. } => Some(*magnitude),
        _ => None,
    });
    crate::core::lanes::lanes_sum(std::iter::once(element::measure_lanes(&f.remainder)).chain(glued))
}

/// Every remainder link this bubble holds: the form's own and each pair's.
pub(crate) fn remainder_links_of(b: &Bubble) -> Vec<Link> {
    let mut out = b.remainder.to_vec();
    for (_, r) in b.remainders.scoped() {
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

pub(crate) fn volume_of(rems: &Keyed<Vec<Link>>, verts: &[impl AsRef<[u8]>]) -> [u64; 4] {
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

pub(crate) fn union_rem(a: &[Link], b: &[Link]) -> Vec<Link> {
    let mut out = a.to_vec();
    let mut have = Keyed::new();
    for &(r, c, _, _) in a {
        have.insert(remainder_site_key(r, c), ());
    }
    for link in b {
        let k = remainder_site_key(link.0, link.1);
        if !have.contains(&k) {
            have.insert(k, ());
            out.push(*link);
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
    /// A5's chain and the translator that places it in the scene (§2.1):
    /// the chain is the identity this bubble is addressed by, the
    /// translator is where it sits. A bubble holding a chain A5 refuses is
    /// never built, so the address never refuses.
    pub remainder: element::PlacedChain,
    pub nerve: Complex,
    pub cover: Keyed<()>,
    /// Classes this idea has met, keyed by [`Rotor::class_key`]. Standing is
    /// the coordinate, never a boolean.
    pub horizon: Keyed<(Rotor, Order, Standing)>,
    pub potential: [u64; 4],
    pub volume: [u64; 4],
    pub dirty: bool,
    /// N4 (0.7's layers, ruled 2026-09-26) — HELD AT THE PERCEPTION LAYER: a
    /// form the mind perceived. A perception is not a thought: it never
    /// passes the unglued door, and it leaves by the physics — its paid
    /// height passed, or what it cites lapsed. A THOUGHT is formed where the
    /// bodies of two held perceptions resonate (a filament of the hyphae),
    /// and the transition is its glue.
    pub percept: bool,
    /// N4 — A PERCEPTION'S BODIES, by the chart each stands on, at its input
    /// and output frames (`resonance::Spores`): the bodies heard against
    /// another perception's. Empty for every other idea.
    pub spores: super::resonance::Spores,
    /// TUniversal when this bubble's charts last moved.
    pub moved_k: u64,
    /// Z4d — THE ESCROW THIS THOUGHT IS HELD ON: the μ⃗ drawn from the
    /// voice's reserve to keep it, per lane, returned when it lapses or is
    /// promoted. What a voice can still escrow on each lane is its
    /// attention, and the four lanes move apart (law 5): a mind rich on
    /// `𝒽` and spent on `𝒮` holds turns and withholds placements.
    pub escrow: [u64; 4],
    /// THE HEIGHT THIS THOUGHT IS PAID THROUGH, on TUniversal. It stands
    /// while its receipts are ahead of the clock and leaves when the clock
    /// passes it.
    ///
    /// `None` is NOT "forever": it is a thought held before any aperture
    /// was measured, and an unmeasured horizon is held rather than guessed
    /// (rule 15). Such a thought lapses when the aperture is first measured
    /// and it is paid to a height, never against a default.
    pub until: Option<u64>,
    /// FX1 — THE HEIGHT ITS PAID EXTENT BEGAN AT, on TUniversal: written
    /// wherever `until` is, so the extent `[paid_from, until]` is one object
    /// for the flux reading and the door after it. `None` with `until`.
    pub paid_from: Option<u64>,
    /// Addresses this idea glued (crystallized forms it rests on).
    pub glues: Keyed<()>,
    /// Ideas that glued this bubble; a non-empty set is not shed.
    pub glued_by: Keyed<()>,
    /// Filed torsion `(σ, ℓ, chord)` keyed by class, written from `pivot_orbit`.
    pub torsion: Keyed<crate::transducer::sentence::FiledOrbit, coord::Holonomy>,
    /// Certified discharges this idea's crystallization rests on (A20).
    pub certificates: Keyed<PivotCertificate>,
    /// Incomparable discharges held as candidates (25.1) until a glue or
    /// ruling. Outer key is the after class; inner is the discharging
    /// word's M1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// P9 — CANDIDATE TRANSPORTS HELD OVER AN OPEN DEMAND (25, 37.7).
    /// Outer key is the demand's chart; inner is the candidate rotor's own
    /// key.
    ///
    /// NOT `vortex`, which holds `PivotCertificate`s. A certificate asserts
    /// a chord, an axis, a before and after class, a `RankMove` and a μ⃗
    /// measured over a bundle — a closed orbit's discharge. An open demand
    /// has closed nothing and has none of them, so putting a candidate
    /// there means writing those fields with values nothing measured (rule
    /// 8, rule 11). The two carriers hold two different objects: a
    /// certified discharge, and a transport this mind has not yet ruled on.
    ///
    /// Re-derived at hold time from the demand's own bodies and never
    /// stored beside them (law 12): the demand carries the bodies it stands
    /// for (50.5), the two sides of the transition are the links split by
    /// which hue they occupy, and `carriage_with_core` reads the transport
    /// between them. Not persisted — a candidate is this session's reading
    /// of a demand the tape already carries, and the tape rebuilds it.
    pub candidates: Keyed<Keyed<Rotor>>,
    /// The form this idea is (chapter 30 step 6).
    pub form: Option<Form>,
    /// THE ROTOR ITS WORD VERTICES AGREE ON, derived ONCE when the form is
    /// taken and read thereafter (Z4 item 3). It read every vertex of the
    /// form on every call, and `rotor_between` asks two bubbles for it per
    /// neighbour pair.
    ///
    /// Not on the tape: the decoder re-derives it from the form it decoded
    /// (law 12), so there is one construction of it and a tape cannot carry
    /// a rotor the form does not have.
    pub(crate) rotor: Option<Rotor>,
    /// Pairs the idea germinated from; trial speaks over them for μ⃗.
    pub bundle: Option<Vec<crate::transducer::family::TrainPair>>,
    /// Per-pair remainder chains, keyed by pair index. `remainder` is their union.
    pub remainders: Keyed<Vec<Link>>,
    /// CS2 — THE CELLS THIS IDEA'S PIVOT READS AGAINST, memoised by the form's
    /// own address: read once per form per change, re-read where the form
    /// moved, because the algebra depends on the form and not on the orbit.
    /// Volatile, never persisted.
    pub(crate) cells_read: Option<(crate::transducer::address::Address, std::sync::Arc<sentence::PivotCells>)>,
}

/// CS2 — the cells a held form's pivot reads against, from the idea's memo
/// where the form's address stands, read afresh (and memoised) where it moved.
pub(crate) fn cells_for_pivot(
    memo: &mut Option<(crate::transducer::address::Address, std::sync::Arc<sentence::PivotCells>)>,
    form: &Form,
    budget: &mut Budget,
) -> std::sync::Arc<sentence::PivotCells> {
    let at = sentence::form_vertex_key(form);
    if let Some((key, cells)) = memo.as_ref() {
        if *key == at {
            return cells.clone();
        }
    }
    let cells = std::sync::Arc::new(crate::transducer::family::pivot_cells(form, budget));
    *memo = Some((at, cells.clone()));
    cells
}

/// ONE CHART'S AMBIENT CONNECTION: the chart, the transports between the
/// frames it occupies, and the gauge composed around them. All three, and
/// the gauge is not dropped — a transport that recolours is a different
/// transport (52.5 reading 6).
pub(crate) type Connection = (Invariant, Vec<Rotor>, Vec<(u8, u8)>);

/// P10(c), 52.5 readings 6 and 8 — THE AMBIENT CONNECTIONS OF A FAMILY:
/// per chart of the bundle occupying two or more frames, the transports
/// between successive OCCUPIED frames and the gauge composed around them.
///
/// THE BREADTH IS THE BUNDLE'S CARRIED CHARTS, not the demand's own. A
/// `TopologicalDeformation` is a moduli defect: it is raised because the
/// body's harmonics differ between input and image, so its chart CHANGES
/// and no body stands at that chart across two frames. Measured on 40
/// corpus families: 13 of 13 candidate-bearing charts had an empty
/// connection set while the same families carried connections on 18
/// others. The demand's chart is what a candidate must explain; the
/// bundle's carried charts are where the explaining is measured.
///
/// THE FRAME KEY CARRIES THE ORDER. `Keyed` walks its key bytes, so a frame
/// written little-endian sorts by its low byte and 256 reads before 1.
/// Written big-endian with the sign bit flipped, byte order IS order in ℤ,
/// so the succession is the carrier's own walk and nothing is sorted into
/// it (rule 4).
///
/// The whole `Carriage` is kept: the rotor the monodromy conjugates by AND
/// the hue map it carries, composed around the cycle. A transport the
/// seventeen do not spell names no rotor and contributes none; nothing is
/// stood in for a connection that was not read (rule 11).
pub(crate) fn bundle_connections(
    bundle: &[crate::transducer::family::TrainPair],
    budget: &mut crate::physics::Budget,
) -> Vec<Connection> {
    let ins: Vec<crate::transducer::family::Grid> =
        bundle.iter().map(|(a, _)| a.clone()).collect();
    let frame_key = |k: i64| ((k as u64) ^ (1u64 << 63)).to_be_bytes();
    let mut charts: Keyed<Invariant> = Keyed::new();
    let mut by_chart: Keyed<Keyed<Vec<crate::transducer::blade::Compartment>>> = Keyed::new();
    for b in &crate::transducer::family::perceive_frames(&ins) {
        let Some(k) = crate::transducer::family::frame_of_body(b) else {
            continue;
        };
        let ck = b.moduli.key();
        if !charts.contains(&ck) {
            charts.insert(&ck, b.moduli.clone());
        }
        if !by_chart.contains(&ck) {
            by_chart.insert(&ck, Keyed::new());
        }
        let Some(at) = by_chart.get_mut(&ck) else { continue };
        match at.get_mut(&frame_key(k)) {
            Some(l) => l.extend(b.compartments.iter().cloned()),
            None => {
                at.insert(frame_key(k), b.compartments.to_vec());
            }
        }
    }
    let mut out = Vec::new();
    for (ck, at) in by_chart.scoped() {
        let occupied: Vec<Vec<crate::transducer::blade::Compartment>> =
            at.scoped().map(|(_, l)| l.clone()).collect();
        if occupied.len() < 2 {
            continue;
        }
        let mut rotors = Vec::new();
        // THE GAUGE COMPOSED AROUND THE CYCLE, not discarded and not filed
        // empty: each connection carries its own hue map and the orbit's is
        // their composition, which is what a filed orbit's σ IS (52.3).
        #[expect(
            clippy::cast_possible_truncation,
            reason = "STD1 C1 — the sensor's: a hue slot of the [u8; 10] image, below PALETTE = 10 as the sensor delivers it"
        )]
        let mut image: [u8; 10] = std::array::from_fn(|i| i as u8);
        for w in occupied.windows(2) {
            let [la, lb] = w else { continue };
            let (Some(a), Some(b)) = (
                crate::transducer::family::body_of(la.clone()),
                crate::transducer::family::body_of(lb.clone()),
            ) else {
                continue;
            };
            let Some(car) = crate::transducer::family::carriage(&a, &b, budget) else {
                continue;
            };
            let Some(Ok(word)) = crate::transducer::family::peel_versor(&car.v) else {
                continue;
            };
            rotors.push(word.rotor());
            let mut next = image;
            for (f, t) in &car.sigma {
                if let (Some(slot), Some(&from)) =
                    (next.get_mut(*f as usize), image.get(*f as usize))
                {
                    let _ = from;
                    *slot = *t;
                }
            }
            image = next;
        }
        if rotors.is_empty() {
            continue;
        }
        #[expect(
            clippy::cast_possible_truncation,
            reason = "STD1 C1 — the sensor's: a hue slot of the [u8; 10] image, below PALETTE = 10 as the sensor delivers it"
        )]
        let sigma: Vec<(u8, u8)> = image
            .iter()
            .enumerate()
            .filter(|(i, &t)| *i as u8 != t)
            .map(|(i, &t)| (i as u8, t))
            .collect();
        if let Some(chart) = charts.get(&ck) {
            out.push((chart.clone(), rotors, sigma));
        }
    }
    out
}

/// P10(c), §25.12 — THE CANDIDATE'S MONODROMY AROUND THE FAMILY'S FRAMES:
/// `M = ∏ γ⁻¹ V γ`, the product of `V` conjugated by each measured
/// connection.
///
/// `None` where a connection does not invert: the product was not measured,
/// which is not a rupture (rule 11). An EMPTY connection set is the empty
/// product — the identity — and is returned as such, so a family with no
/// measured succession dissolves nothing.
pub(crate) fn monodromy(v: &Rotor, gammas: &[Rotor]) -> Option<Rotor> {
    let mut m = Rotor::identity();
    for g in gammas {
        // `g̃ v g`, a ray (78.32(1)).
        let gi = g.is_versor().then(|| g.reverse())?;
        m = m.geo(&gi.geo(v).geo(g));
    }
    Some(m)
}

impl Bubble {
    /// Read off what the bubble holds on every call, never stored beside
    /// it (rule 6, law 12): the remainder carries A5's proof in its type,
    /// so the address never refuses.
    pub fn key(&self) -> crate::transducer::address::Address {
        match &self.form {
            Some(f) => idea_key(f),
            None => remainder_addr(&self.remainder),
        }
    }

    /// THE ROTOR THIS BUBBLE'S FORM SPEAKS, when it speaks ONE.
    ///
    /// A form with several word vertices has several rotors, and which one
    /// arrives first is an insertion order, so a representative read would
    /// answer a different motion on a node that took the same bodies in
    /// differently (rule 15). It would cost more than a wrong answer
    /// downstream: `rotor_between` composes two of these into `rb ∘ ra⁻¹`
    /// and calls that the transport between two bubbles, so two nodes
    /// would breed different transports from identical mathematics.
    ///
    /// A form whose word vertices all speak one rotor up to projective
    /// equality has that rotor. One that speaks several has no single one,
    /// and says so — absence, not a representative. Nothing here is chosen.
    pub(crate) fn rotor(&self) -> Option<&Rotor> {
        self.rotor.as_ref()
    }

    /// The derivation itself, run once at `take_form` and at every other
    /// site that builds a bubble around a form.
    pub(crate) fn rotor_of_form(f: &Form) -> Option<Rotor> {
        let mut held: Option<Rotor> = None;
        for (_, v) in f.vertices.scoped() {
            let Vertex::Word { derivation, .. } = v else {
                continue;
            };
            let r = derivation.rotor();
            match &held {
                None => held = Some(r),
                Some(h) if h.projective_eq(&r) => {}
                Some(_) => return None,
            }
        }
        held
    }

    /// THE SECTIONS THIS BUBBLE'S FORM CARRIES: one per word vertex per
    /// chart it acts on — the rotation it speaks there and the carriage it
    /// speaks it by. Keyed by both, so two vertices holding one section are
    /// one section and one vertex acting on two charts is two.
    ///
    /// [`Bubble::rotor`] answers ONE rotor or absence, which is right for
    /// `rotor_between` — two bubbles have one transport between them or they
    /// have none. It is wrong for the cover: a form that speaks several
    /// rotors has several sections, and a reader that asks for one and
    /// takes the absence drops exactly the multi-section forms.
    pub(crate) fn sections(&self) -> Keyed<Held> {
        let mut out = Keyed::new();
        let Some(f) = self.form.as_ref() else {
            return out;
        };
        for (_, v) in f.vertices.scoped() {
            let Vertex::Word { derivation, acts, .. } = v else {
                continue;
            };
            let r = derivation.rotor();
            // index: keyed fibre of this vertex's members — its own charts,
            // never a read of the whole form (rule 1).
            for (_, a) in acts.scoped() {
                let mut k = a.chart.key().into_vec();
                r.put_canonical(&mut k);
                // A4 output (17.2): one projective carriage is one section.
                // Raw bytes made a carriage and its negation two.
                a.carriage.put_canonical(&mut k);
                if !out.contains(&k) {
                    out.insert(
                        k,
                        (a.chart.clone(), r.clone(), (*a.carriage).clone()),
                    );
                }
            }
        }
        out
    }

    /// MEET THIS IDEA'S OWN CHARTS. Each of its sections stands on a chart;
    /// the other sections there are that chart's fibre, read at its key. A
    /// meet that fills grows the cover, one that is congruent grows it with
    /// a torsion axis, and one that stands a class writes the horizon and
    /// leaves the chart out of the cover.
    pub fn expand(&mut self, atlas: &dyn AtlasRead, crystal: &dyn Crystal, budget: &mut Budget) -> usize {
        self.expand_with(atlas, &|_| None, 0, crystal, budget, &mut sentence::PivotTally::default()).spent
    }

    /// `atlas` is the language this thought expands over (the working
    /// atlas, 24.1); `section_at` is the keyed read of a held word's
    /// SECTION by address — its rotation and its carriage — so the Čech arm
    /// reads the word the chart names and never a table built for the
    /// stroke, and the word's placement reaches `extend`. `pivots` gathers
    /// CS2's reading of the pivot against the cells its orbits cross.
    pub fn expand_with(
        &mut self,
        atlas: &dyn AtlasRead,
        section_at: SectionAt,
        now: u64,
        crystal: &dyn Crystal,
        budget: &mut Budget,
        pivots: &mut sentence::PivotTally,
    ) -> Expansion {
        // 78.37 — CS2's demand arm's demands, carried out to the docket, and
        // the class keys its discharges answer (R1).
        let mut demanded: Vec<Demand> = Vec::new();
        let mut discharged: Vec<crate::transducer::address::Address> = Vec::new();
        // 37.7's candidates, carried out rather than dropped: the rotors this
        // stroke met that the seventeen cannot spell, each with the chart it
        // was met on. The bubble cannot mint — minting writes a word into the
        // language and the language is the entity's — so it carries them.
        let mut adjoined: Vec<(Rotor, crate::transducer::stalk::Invariant)> = Vec::new();
        // THE POTENTIAL IS A READING, NEVER A GATE (L7b, law 9): the lane
        // measure of everything this idea holds open — its form's remainder
        // and its per-pair remainders, the same links `admit_incidence`
        // keys — with T the receipts since its charts last moved, zero when
        // zero. Nothing is floored so that a gate would pass: the pivot
        // loop runs on every dirty idea under the budget (25.1), and the
        // budget is the only thing that stops it.
        self.potential = element::measure_lanes(&remainder_links_of(self));
        // A clock that went backwards measures no extent, and an idea whose
        // T cannot be read is not expanded against one.
        let Some(t) = held_since(now, self.moved_k) else {
            return Expansion::default();
        };
        self.potential[2] = t;
        if budget.remaining() == 0 {
            return Expansion::default();
        }
        let mut spent = 0usize;
        // THE CHARTS THIS IDEA MEETS ARE ITS OWN SECTIONS' CHARTS, and their
        // members are a keyed read at those coordinates. Nothing is traversed
        // and nothing is discovered.
        //
        // A Čech simplex is the OVERLAP of two sections, and two sections
        // overlap when they are sections over the same chart. A sibling of
        // my chart that none of my vertices acts on is not an overlap; it
        // is a stranger, and `extend` asked about it would classify a
        // transition between bodies that share nothing. The cover is built
        // out of incidence in the sheaf, never out of adjacency in a trie,
        // and needs no cursor to resume: an idea's expansion depends on its
        // coordinates and not on where the last stroke stopped.
        //
        // The charts are already in hand. Every `Vertex::Word` names the
        // charts it acts on and the carriage it acts by at each, so the
        // sections are a reading of the form and the far side of each meet
        // is `members(chart)` — the fibre, keyed, at an address this idea
        // already holds. Both bounds are content: this idea's sections, and
        // one chart's own fibre (rule 10).
        //
        // EVERY MEMBER AGAINST EVERY SECTION, and both sections carry their
        // real carriage — without which `extend`'s `flat` is true by
        // construction and `Fill::Class` is unreachable.
        let mine = self.sections();
        let me = self.key();
        let vol = self.potential;
        for (_, (chart, gamma_a, car_a)) in mine.scoped() {
            let ck = chart.key();
            if self.cover.contains(&ck) {
                continue;
            }
            spent += 1;
            // index: keyed fibre at a chart this idea itself stands on
            let members = atlas.members(chart);
            let mut closed = true;
            let mut met = false;
            for head in &members {
                // A section does not overlap itself.
                if head.as_slice() == me.as_bytes() {
                    continue;
                }
                let Some((gamma_b, car_b)) = section_at(head.as_slice()) else {
                    continue;
                };
                met = true;
                // KB1 — THE SIMPLEX IS THE OVERLAP OF TWO SECTIONS OVER THIS
                // CHART, its vertices the sections' coordinates: an idea
                // meeting another on two charts, or by two sections of one,
                // is that many simplices and never one key.
                let (mine_at, theirs_at) =
                    (section_vertex(me.as_bytes(), chart, car_a), section_vertex(head, chart, &car_b));
                let simplex = Simplex {
                    vertices: {
                        let mut v = vec![mine_at.clone(), theirs_at.clone()];
                        v.sort_unstable();
                        v
                    },
                    coefficient: gamma_b.clone(),
                    // What the simplex expects of the far side, both halves,
                    // exactly as the coefficient above is its rotation.
                    carriage: car_b.clone(),
                    volume: vol,
                    fill: Fill::Open,
                };
                let a = Section {
                    addr: me.clone(),
                    gamma: gamma_a.clone(),
                    carriage: car_a.clone(),
                };
                let b = Section {
                    addr: crate::transducer::address::Address::from(head.clone()),
                    gamma: gamma_b,
                    carriage: car_b,
                };
                match cech::extend(&a, &b, &simplex) {
                    Fill::Filled(..) => {}
                    Fill::Congruent(_, g, ell) => {
                        self.nerve.file_congruent(&mine_at, &theirs_at, *g, ell);
                    }
                    Fill::Class(_, r) => {
                        closed = false;
                        if r.class_key().is_none_or(|k| !self.horizon.contains(k.as_bytes())) {
                            horizon_put(&mut self.horizon, *r, Standing::Held);
                        }
                    }
                    // 37.7 at a simplex: a rotation the seventeen do not
                    // spell seeds no horizon, closes nothing, and is an
                    // adjunction candidate — the rotor the mind measured and
                    // has no letter for — carried out to the entity, which
                    // mints and stands it (`Derivation::adjoined`).
                    Fill::Unspellable(r) => {
                        closed = false;
                        adjoined.push((r.as_ref().clone(), chart.clone()));
                    }
                    // Unmeasured by its reason, or not asked: nothing closes.
                    Fill::Unread(_) | Fill::Open => closed = false,
                }
            }
            // THE CHART ENTERS THE COVER WHEN EVERY SECTION CLOSES ON IT,
            // not when one of them does. A chart one section fills and
            // another stands a class on is not covered — it is the chart the
            // standing class is about.
            if met && closed {
                self.cover.insert(ck, ());
                self.moved_k = now;
            }
        }
        let mut pivoted = [0u64; 4];
        let mut overflowed = 0u64;
        if budget.remaining() > 0 {
            if let Some(mut form) = self.form.clone() {
                let key = self.key();
                // CS2 — the cells this pivot reads against, memoised here.
                let cells = cells_for_pivot(&mut self.cells_read, &form, budget);
                let snaps = form.pivot_orbit_read(
                    key,
                    &self.horizon,
                    crystal,
                    budget,
                    self.bundle.as_deref(),
                    t,
                    Some(&cells),
                    pivots,
                );
                for s in &snaps {
                    // STD1 — a snap whose measure, or the sum it would join,
                    // passes its carrier is left out of `pivoted`, counted.
                    if !s.lanes().is_some_and(|m| crate::core::lanes::lanes_step(&mut pivoted, &m)) {
                        overflowed += 1;
                    }
                    match s {
                        // 37.7 from the orbit's own residue, joining the
                        // cover's own candidates on the way out; the entity
                        // mints it through the one construction.
                        crate::transducer::sentence::Snap::Adjoin { rotor, chart, .. } => {
                            adjoined.push((rotor.as_ref().clone(), chart.clone()));
                        }
                        // A held orbit of infinite order the basis did not
                        // discharge stands on the form as the demand the
                        // peel types its holonomy as — the prime it lacks,
                        // or the rotor it cannot spell, carried as witness
                        // (16.16 step 6, 50.4). A rotor the seventeen spell
                        // is no demand on the language (37.7 adjoins what
                        // the presentation lacks); the form's vertices
                        // carry no links, so the demand declares none.
                        // 78.47 — a rotor the mind's OWN crystal spells is no
                        // demand, whatever the fixed seventeen say; the demand's
                        // kind and witness are still the peel's to type.
                        crate::transducer::sentence::Snap::Demand { rotor, chart, .. } => {
                            let own = crate::transducer::family::letter_reading(crystal, budget, rotor);
                            if own != crate::transducer::family::LetterReading::Derivable {
                                if let Some(Err(kind)) = crate::transducer::alphabet::factor_rotor(rotor) {
                                    form.stand_demand(kind, chart, Vec::new());
                                }
                            }
                        }
                        // 78.37 — the demand arm's demand, carried out to the
                        // contemplation's docket.
                        crate::transducer::sentence::Snap::Demanded { class, charts, residue, .. } => {
                            demanded.push((class.as_ref().clone(), charts.clone(), residue.clone()));
                        }
                        crate::transducer::sentence::Snap::Discharged { certificate, .. } => {
                            discharged.push(certificate.before.clone());
                        }
                        _ => {}
                    }
                }
                self.take_form(form);
            }
        }
        self.dissolve_candidates(budget);
        self.dirty = spent > 0 && budget.remaining() == 0;
        Expansion { spent, pivoted, adjoined, demanded, discharged, overflowed }
    }

    /// P10(c), §25.12 — A CANDIDATE DIES ON AN INFINITE MONODROMY, AND ONLY
    /// ON ONE.
    ///
    /// THE READING IS THE ORDER, NOT AN EQUALITY. This asked whether
    /// `M = ∏ γ⁻¹Vγ` was projectively the identity after ONE circuit, which
    /// calls every torsioned fibre ruptured: a connection with torsion does
    /// not close on the first trip, it closes at its period. `Rotor::order`
    /// answers that exactly over ℚ, and the three readings are three
    /// different facts:
    ///
    /// - `Finite(1)` — flat. The candidate returns on the first circuit.
    /// - `Finite(ℓ ≥ 2)` — TORSION. It returns after `ℓ` circuits and not
    ///   before; the fibre is twisted by `ℓ` and 50.8 case 3 files that as
    ///   torsion at its period. It has not ruptured, and it stands.
    /// - `Infinite` — RUPTURE. No number of circuits returns it. A19 spells
    ///   that period `0`, there is no closure to file, and this alone
    ///   dissolves.
    ///
    /// THE BREADTH IS THE COVER, NOT THE DEMAND'S OWN CHART. A candidate is
    /// carried around every chart this idea stands on, and what comes back
    /// is a period per chart. Read at the demand's chart alone it measured
    /// nothing: a `TopologicalDeformation` is raised because the body's
    /// harmonics differ between input and image, so its chart CHANGES, and
    /// no body stands at that chart across two frames — measured on 40
    /// corpus families, 13 of 13 candidate-bearing charts had an empty
    /// connection set while the families carried connections on 18 others.
    /// The demand's chart is what the candidate must explain; the cover is
    /// where the explaining is measured.
    ///
    /// A CANDIDATE LEAVES ONLY WHEN ITS SUB-COVER IS EMPTY. A chart whose
    /// monodromy is `Infinite` drops out of the candidate's sub-cover; a
    /// candidate standing on no chart at all is the one that dissolves, and
    /// when every candidate over a demand empties, nothing the mind holds
    /// closes anywhere in the family, which is 37.7's condition. A candidate
    /// standing on SOME charts is a section and is held, never decided
    /// (rule 15).
    ///
    /// NOTHING DIES UNMEASURED (rule 11). A chart with fewer than two
    /// occupied frames carries no connection, and the empty product is the
    /// identity by absence rather than by measurement — such a chart is not
    /// counted for or against, and a candidate measured nowhere stands.
    ///
    /// Condensation is the carrier's (1.5): `candidates` is keyed by
    /// `Rotor::key`, the ray's canonical coordinates, so `V` and `−V` are
    /// one candidate before this runs.
    ///
    /// OWED: a `Finite(ℓ ≥ 2)` period is content and wants filing on
    /// `Bubble.torsion`, which is keyed by class and holds a `FiledOrbit`
    /// whose `chord` is a face index — and a demand's form has no faces for
    /// it to point at. The period is read and the candidate stands on it;
    /// where it is filed is the operator's to rule.
    pub(crate) fn dissolve_candidates(&mut self, budget: &mut crate::physics::Budget) {
        let Some(bundle) = self.bundle.clone() else {
            return;
        };
        let connections = bundle_connections(&bundle, budget);
        if connections.is_empty() {
            return;
        }
        let charts: Vec<Vec<u8>> = self.candidates.scoped().map(|(c, _)| c).collect();
        for ck in charts {
            let Some(here) = self.candidates.get(&ck) else {
                continue;
            };
            let mut dying: Vec<(Vec<u8>, Rotor)> = Vec::new();
            let mut filing: Vec<(coord::Holonomy, crate::transducer::sentence::FiledOrbit)> =
                Vec::new();
            for (rk, v) in here.scoped() {
                // THE SUB-COVER: the charts whose monodromy closes at some
                // period. Empty is the dissolution; anything else is a
                // section the mind keeps (rule 15).
                let mut stands = false;
                let mut ruptured_at: Option<Rotor> = None;
                for (_, gammas, sigma) in &connections {
                    let Some(m) = monodromy(v, gammas) else { continue };
                    let Some(order) = m.order() else { continue };
                    match order {
                        crate::transducer::alphabet::Order::Finite(1) => stands = true,
                        crate::transducer::alphabet::Order::Finite(_period) => {
                            stands = true;
                            // TORSION IS CONTENT, so it is filed at the
                            // period it closes at (50.8 case 3). A bundle
                            // connection cycle is transversal: it winds the
                            // fibre's puncture and closes on no overlap of
                            // this form, and the arm says so (AT1, ruled —
                            // the slot era wrote `0`, naming a real face).
                            // σ is the gauge composed around the cycle,
                            // which the connections carried.
                            if let Some(class) = m.holonomy_key() {
                                filing.push((
                                    class,
                                    crate::transducer::sentence::FiledOrbit {
                                        sigma: sigma.clone(),
                                        periods: m.toral_period(),
                                        chord: crate::transducer::sentence::Closes::Fibre,
                                    },
                                ));
                            }
                        }
                        crate::transducer::alphabet::Order::Infinite => {
                            if ruptured_at.is_none() {
                                ruptured_at = Some(m);
                            }
                        }
                    }
                }
                if let (false, Some(m)) = (stands, ruptured_at) {
                    dying.push((rk, m));
                }
            }
            for (class, orbit_at) in filing {
                self.torsion.put(&class, orbit_at.clone());
                if let Some(f) = self.form.as_mut() {
                    f.torsion.put(&class, orbit_at);
                }
            }
            let Some(here) = self.candidates.get_mut(&ck) else {
                continue;
            };
            // A candidate whose monodromy ruptured dissolves.
            for (rk, _) in dying {
                here.remove(&rk);
            }
        }
    }

    /// THE VORTEX HERE STANDING AT A CONJUGACY STRATUM AND HOLDING `word`.
    ///
    /// A vortex is filed under the orbit's own class vector (`vortex_put`),
    /// whose depth-one coordinate is the holonomy rotor's conjugacy class.
    /// A deciding word knows that stratum and nothing finer: not the
    /// carriage the orbit stood at, not its hue. So the boundary is parsed
    /// out of each key through the class schema and compared at depth one
    /// — never a byte count, and never the whole key.
    ///
    /// The read is this bubble's own carrier, which is one idea's boundary
    /// (rule 1). A key that does not parse as a class vector is not a
    /// vortex of this system and is passed over rather than matched.
    fn vortex_at_stratum(&self, stratum: &[u8], word: &[u8]) -> Option<Vec<u8>> {
        let schema = crate::transducer::sentence::ClassVector::class_schema();
        // index: this bubble's own vortices, its boundary
        for (class, candidates) in self.vortex.scoped() {
            if !candidates.contains(word) {
                continue;
            }
            let Ok(end) = schema.boundary(&class, 1) else {
                continue;
            };
            if class.get(..end) == Some(stratum) {
                return Some(class);
            }
        }
        None
    }

    /// A glue or a syllabus class standing at the stratum a held vortex's
    /// orbit stands in applies the certificate keyed by the deciding move;
    /// the others of that vortex are not applied (dialect candidates).
    pub(crate) fn collapse_vortex(
        &mut self,
        stratum: &[u8],
        word: &[u8],
        _by: &'static str,
    ) -> Option<Vec<crate::transducer::alphabet::Rotor>> {
        let class = self.vortex_at_stratum(stratum, word)?;
        let mut inner = self.vortex.remove(&class)?;
        let Some(win) = inner.remove(word) else {
            if !inner.is_empty() {
                self.vortex.insert(&class, inner);
            }
            return None;
        };
        // EXPANSION 4 — THE LOSERS ARE A SYMMETRY, NOT A TIE BROKEN. Two
        // candidates that both discharge one orbit to one class prove the
        // orbit admits `w_a⁻¹·w_b`, an element that stabilises that class
        // because each leaves it where the other does. That is a rotation
        // the mind MEASURED, which is 37.7's object exactly, and a collapse
        // that only cleared the vortex would throw it away — the mind
        // resolving its own superposition by forgetting one half of it.
        //
        // The symmetries are carried out, not minted here, for the reason
        // every other adjunction is: minting writes a word into the language
        // and a bubble does not hold the language.
        let losers: Vec<PivotCertificate> = inner.scoped().map(|(_, c)| c.clone()).collect();
        let symmetries = crate::transducer::sentence::symmetries(&win, &losers);
        if let Some(mut f) = self.form.clone() {
            f.apply_pivot(&win);
            f.certificates.insert(win.key(), win.clone());
            f.vortex = Keyed::new();
            self.take_form(f);
        } else {
            self.certificates.insert(win.key(), win.clone());
            self.vortex = Keyed::new();
        }
        Some(symmetries)
    }

    /// THE CONJUGACY STRATA THIS BUBBLE'S VORTICES STAND AT: the depth-one
    /// coordinate of each outer key of `vortex`. The contemplation files
    /// the bubble under each (`by_vortex`), so a deciding word's own class
    /// key reaches it by a keyed read, and there is one construction of
    /// that coordinate for the writer and the reader (law 12).
    ///
    /// A key that does not parse as a class vector contributes no stratum
    /// rather than contributing its own bytes as one (rule 11).
    pub(crate) fn vortex_strata(&self) -> Vec<Vec<u8>> {
        let schema = crate::transducer::sentence::ClassVector::class_schema();
        let mut out = Vec::new();
        // index: this bubble's own vortices, its boundary
        for (class, _) in self.vortex.scoped() {
            let Ok(end) = schema.boundary(&class, 1) else {
                continue;
            };
            if let Some(s) = class.get(..end) {
                out.push(s.to_vec());
            }
        }
        out
    }

    /// Total: the form's remainder carries A5's proof in its own type, so
    /// there is nothing to ask here (§4.4).
    pub(crate) fn take_form(&mut self, f: Form) {
        self.rotor = Self::rotor_of_form(&f);
        self.remainder = f.remainder.clone();
        self.torsion = f.torsion.clone();
        self.certificates = f.certificates.clone();
        self.vortex = f.vortex.clone();
        self.form = Some(f);
    }
}
