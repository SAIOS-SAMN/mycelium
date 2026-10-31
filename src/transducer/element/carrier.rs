// 18.3 — THE CARRIER: the element's four strata and the fibers it opens.
//
//! `Action` is the four-lane action (S a versor, G a hue permutation, T an
//! extent in receipts, H a shell coordinate) plus the germinated axes;
//! `Element` is tape, action, chain, magnitude, fibre, anchor and connection.

use super::*;

/// Stratum tags on torsion and transitions (not matrix fibre indices).
pub const FIBER_SPATIAL: u16 = 0;
pub const FIBER_GAUGE: u16 = 1;
pub const FIBER_TEMPORAL: u16 = 2;
pub const FIBER_SHELL: u16 = 3;
/// First index of a germinated axis (a versor, never a matrix).
pub const FIBER_GERMINATED: u16 = 4;

/// Hue states on the gauge (ARC palette 0..9; 0 = absent).
pub const PALETTE: usize = 10;

/// WHAT A TORSION CYCLE TURNS BY — its generator, which the period is READ
/// from at every writer, reader and replay (PD1, 78.43(2)): a stored period
/// is a claim, never a bound, and a derivable quantity re-derives at every
/// reader (law 12).
#[derive(Debug, Clone)]
pub enum Generator {
    /// A hue permutation σ, one step round the cycle: its period on a base
    /// is [`orbit_period`], the lcm of its cycle lengths through the base's
    /// hues.
    Hue(Vec<(u8, u8)>),
    /// A holonomy: the rotor's ray, whose finite order is the period — the
    /// Congruent fill's reading (`Rotor::order`, algebra-bounded). For a
    /// cycle carrying RM1's remainder this IS the deficit ray `g`.
    Holonomy(Box<crate::transducer::alphabet::Rotor>),
}

/// One generator and another are one where they turn alike: the same σ, or
/// one ray — a ray and its scalar multiples are one rotor (A1).
impl PartialEq for Generator {
    fn eq(&self, other: &Generator) -> bool {
        match (self, other) {
            (Generator::Hue(a), Generator::Hue(b)) => a == b,
            (Generator::Holonomy(a), Generator::Holonomy(b)) => a.projective_eq(b),
            _ => false,
        }
    }
}

impl Eq for Generator {}

impl Generator {
    /// THE GENERATOR'S OWN ORDER, read with no base: σ's (the lcm of its
    /// cycle lengths, where σ permutes its hues within the palette), or the
    /// ray's finite order. `None` where it has none.
    pub fn order(&self) -> Option<u64> {
        match self {
            Generator::Hue(sigma) => {
                if !gauge_is_perm(sigma) {
                    return None;
                }
                let mut order = 1u64;
                for c in crate::transducer::sentence::hue_cycles(sigma) {
                    let len = u64::try_from(c.len()).ok()?;
                    let g = num_integer::Integer::gcd(&order, &len);
                    order = (order / g).checked_mul(len)?;
                }
                Some(order)
            }
            Generator::Holonomy(ray) => match ray.order()? {
                crate::transducer::alphabet::Order::Finite(l @ 1..) => Some(u64::from(l)),
                _ => None,
            },
        }
    }

    /// THE PERIOD READ OFF THE GENERATOR ON A BASE: a hue cycle's is the
    /// base's orbit under σ ([`orbit_period`]); a holonomy's is the ray's
    /// order, which no base moves.
    pub fn period_on(&self, base: &[(u8, u8)]) -> Option<u64> {
        match self {
            Generator::Hue(sigma) => orbit_period(sigma, base).ok(),
            Generator::Holonomy(_) => self.order(),
        }
    }

    /// σ, where the generator is a hue permutation.
    pub fn hue(&self) -> Option<&[(u8, u8)]> {
        match self {
            Generator::Hue(sigma) => Some(sigma),
            Generator::Holonomy(_) => None,
        }
    }

    /// The ray, where the generator is a holonomy.
    pub fn ray(&self) -> Option<&crate::transducer::alphabet::Rotor> {
        match self {
            Generator::Hue(_) => None,
            Generator::Holonomy(ray) => Some(ray),
        }
    }

    /// The generator written out, its kind first: `0` and σ's pairs behind
    /// the pairs' count (by its own coordinate), or `1` and the ray's canonical
    /// coordinates. One writer for the record, the Torse tape and the
    /// canonical order two cycles are compared in.
    pub fn put(&self, out: &mut Vec<u8>) {
        match self {
            Generator::Hue(sigma) => {
                out.push(0);
                crate::transducer::address::put_key_count(out, sigma.len());
                for &(f, t) in sigma {
                    out.push(f);
                    out.push(t);
                }
            }
            Generator::Holonomy(ray) => {
                out.push(1);
                ray.put_canonical(out);
            }
        }
    }

    /// The inverse of [`Generator::put`]; any other kind refuses.
    pub fn take(b: &[u8], p: &mut usize) -> Option<Generator> {
        let kind = *b.get(*p)?;
        *p += 1;
        match kind {
            0 => {
                let n = crate::transducer::address::take_key_count(b, p)?;
                if n > b.len().checked_sub(*p)? / 2 {
                    return None;
                }
                let mut sigma = Vec::with_capacity(n);
                for _ in 0..n {
                    sigma.push((*b.get(*p)?, *b.get(*p + 1)?));
                    *p += 2;
                }
                Some(Generator::Hue(sigma))
            }
            1 => Some(Generator::Holonomy(Box::new(crate::transducer::alphabet::Rotor::from_bytes(b, p)?))),
            _ => None,
        }
    }

    /// The generator's own coordinates as one key, for a canonical order.
    pub fn key(&self) -> Vec<u8> {
        let mut k = Vec::new();
        self.put(&mut k);
        k
    }
}

/// ONE TORSION CYCLE of a punctured axis (18.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TorsionCycle {
    /// What the cycle turns by: a hue permutation, or a holonomy's ray.
    pub generator: Generator,
    /// The PERIOD it closes at — the order of that generator, READ off it
    /// (78.43(2)) and verified against it at every writer into `torsion`,
    /// and the extent the word owes on the T lane. `u64` by the type's
    /// domain: a permutation of the u8 hues has order up to Landau's g(256)
    /// ≈ 2⁵⁴, past `u32` (the sensor's PALETTE is site 4's constant, not
    /// this width's provenance).
    pub period: u64,
    /// The fibre it lives on: `FIBER_SPATIAL`, or a germinated axis.
    pub fibre: u16,
    /// RM1, 78.37(4) — THE REMAINDER the cycle closed up to, where a
    /// Congruent fill measured one: `None` is not measured (a cycle an
    /// orbit condensed, a form's filed orbit), never a zero.
    pub remainder: Option<Remainder>,
}

/// RM1, 78.37(4) — A SENTENCE'S REMAINDER, the torsion record completed: the
/// class `κ` it closed up to, inline by its own coordinates; and `at`, the
/// chart chain the residue was perceived on (the thoughts' shared chain on a
/// mind, the complex's chart on a court), by its keys. Evidence on the
/// record: the address never carries it.
///
/// THE DEFICIT RAY `g` IS THE CYCLE'S GENERATOR (PD1, law 12): a holonomy
/// cycle turns by `g`, so the remainder reads its ray off its cycle
/// ([`Element::remainder_rays`]) and carries no second copy of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remainder {
    pub class: Box<crate::transducer::sentence::ClassVector>,
    pub at: Vec<Vec<u8>>,
    /// RM1 (c) — ANSWERED: the stamp of the re-fill act that closed it, a
    /// citation of that act by its coordinates; `None` is open. The
    /// remainder stays on the record as evidence either way — the history is
    /// the record's.
    pub answered: Option<crate::chronometry::Stamp>,
}

impl TorsionCycle {
    /// RM1 (c) — THE PRESSURE a remainder puts on the word while open: the
    /// Torse's own yield, `ℓ − 1` on the shell lane (`torse_yield`, one
    /// formula), and zero once answered. `None` where the cycle carries no
    /// remainder (not measured). A READING: charging it to the room while
    /// the word is held resident is the flux door's, after the recut.
    pub fn pressure(&self) -> Option<[u64; 4]> {
        let r = self.remainder.as_ref()?;
        Some(match r.answered {
            Some(_) => [0; 4],
            None => crate::transducer::rewrite::torse_yield(std::slice::from_ref(self)),
        })
    }
}

impl Remainder {
    /// One remainder and another are the same perception where class and
    /// chain agree — the ray is the generator of the cycle each rides, and
    /// `record_cycle` compares only remainders of cycles whose generators
    /// are one; whether it has been answered is its history.
    pub fn same(&self, other: &Remainder) -> bool {
        self.class == other.class && self.at == other.at
    }
    /// The index key the receptor reads by, CLASS FIRST (a reading depends
    /// on fewer coordinates first): the class key, then the residue's chart
    /// chain, its keys written in order — each length-delimited, so one
    /// prefix read under a class returns every chart it was perceived on.
    pub fn index_key(&self) -> Vec<u8> {
        let mut k = Self::class_prefix(&self.class);
        let mut chain = Vec::new();
        for c in &self.at {
            crate::transducer::address::put_key_bytes(&mut chain, c);
        }
        crate::transducer::address::put_key_bytes(&mut k, &chain);
        k
    }

    /// The prefix every remainder of `class` is indexed under.
    pub fn class_prefix(class: &crate::transducer::sentence::ClassVector) -> Vec<u8> {
        let mut k = Vec::new();
        crate::transducer::address::put_key_bytes(&mut k, class.class_key().as_bytes());
        k
    }

    /// The chart chain an index key names, read back.
    pub fn chain_of_key(key: &[u8]) -> Option<Vec<Vec<u8>>> {
        let mut p = 0usize;
        let _class = crate::transducer::address::take_key_bytes(key, &mut p)?;
        let chain = crate::transducer::address::take_key_bytes(key, &mut p)?;
        if p != key.len() {
            return None;
        }
        let mut q = 0usize;
        let mut out = Vec::new();
        while q < chain.len() {
            out.push(crate::transducer::address::take_key_bytes(&chain, &mut q)?);
        }
        Some(out)
    }

    /// `None` where the chain's count or a chart's length passes its field
    /// (STD1 3c, LD0): the record is refused, never written short.
    #[must_use]
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        self.class.put_canonical(out);
        crate::transducer::address::put_u32_count(out, self.at.len())?;
        for c in &self.at {
            crate::transducer::address::put_bytes(out, c)?;
        }
        match &self.answered {
            Some(s) => {
                out.push(1);
                s.put_canonical(out);
            }
            None => out.push(0),
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Remainder> {
        let class = Box::new(crate::transducer::sentence::ClassVector::take_canonical(b, p)?);
        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
        *p += 4;
        // Each chart is at least its length; a count the body cannot hold
        // refuses before anything is reserved.
        if n > b.len().checked_sub(*p)? / 4 {
            return None;
        }
        let mut at = Vec::with_capacity(n);
        for _ in 0..n {
            at.push(crate::transducer::address::take_bytes(b, p)?);
        }
        let answered = match *b.get(*p)? {
            0 => {
                *p += 1;
                None
            }
            1 => {
                *p += 1;
                Some(crate::chronometry::Stamp::take_canonical(b, p)?)
            }
            _ => return None,
        };
        Some(Remainder { class, at, answered })
    }
}

/// What filing a cycle came to: new, a remainder gained by a cycle filed
/// without one, already standing, refused — a differing remainder for one
/// `(σ, fibre)` — or no cycle at all: a period of zero closes nothing (A17).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filed {
    New,
    Gained,
    Standing,
    Refused,
    NoPeriod,
    /// STD1 — a new cycle the holonomic lane has no room for
    /// ([`holonomic_fits`]).
    PastCarrier,
    /// PD1, 78.43(2) — A STORED PERIOD IS A CLAIM, NEVER A BOUND: a cycle
    /// whose period is not the one read off its generator and the base is
    /// refused by name, so every later walk is bounded by a READ period.
    Misspoken,
}

/// DOES A CYCLE'S PERIOD STAND ON THIS BASE — the one test both writers into
/// `torsion` make (`record_cycle` and the record decoder): the period the
/// cycle claims is the one read off its generator ([`orbit_period`]).
pub(crate) fn period_stands(cycle: &TorsionCycle, base: &[(u8, u8)]) -> bool {
    cycle.generator.period_on(base) == Some(cycle.period)
}

/// STD1 (rule 11) — THE HOLONOMIC LANE'S INVARIANT: a word's `mag[3]` plus
/// its free sections plus its torsion cycles stays inside `u64`, so
/// `Element::magnitude` reads total.
///
/// Held by every writer of the three: `set_magnitude`, `from_connection`
/// and the record decode refuse a lane that would break it; `puncture` —
/// and `attach` and `merge_fibre`, which join through it — and
/// `record_cycle` refuse a section or a cycle past it; `torse` only
/// condenses sections into one cycle. A word built fresh holds no section
/// and no cycle, so the lane it is given always fits.
pub(crate) fn holonomic_fits(mag3: u64, sections: usize, torsion: usize) -> bool {
    match (u64::try_from(sections), u64::try_from(torsion)) {
        (Ok(s), Ok(t)) => s.checked_add(t).and_then(|n| mag3.checked_add(n)).is_some(),
        _ => false,
    }
}

/// ONE TRANSITION of the connection: how a sibling section was seen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    /// The assignment the sibling was observed under.
    pub asg: Vec<(u8, u8)>,
    /// The versor that carried it into this frame.
    pub gauge: Versor,
    /// The fibre it was observed on.
    pub fibre: u16,
}

/// The four strata of a word's action. S is one versor; G a hue permutation;
/// T an extent in receipts; H the shell coordinates. Germinated axes are
/// further versors. Nothing here is a matrix.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Action {
    /// S: `V = carriage · sensor_action_of(word)`. None is the identity
    /// motion.
    pub(crate) spatial: Option<Versor>,
    /// G: hue permutation as pairs. Empty is identity.
    pub(crate) gauge: Vec<(u8, u8)>,
    /// T: extent in receipts. Zero is none.
    pub(crate) temporal: u64,
    /// H: shell coordinate. Zero is none.
    pub(crate) shell: u64,
    /// Axes opened above the four strata, each a versor, tagged ≥ 4.
    pub(crate) germinated: Vec<(u16, Versor)>,
    /// THE ROTOR WHERE NO TAPE SPELLS ONE, in `Cl(4,0)` over all six
    /// bivector planes.
    ///
    /// `spatial` is a `Versor` in `Cl(3,1)`, whose rotation group has ONE
    /// plane, and `sensor_action_of` states that the map from a rotor
    /// cannot be onto. So a composite that kept only `spatial` kept one
    /// plane of six, and `cech::rotor_of` — which reads the tape — answered
    /// nothing at all for it.
    ///
    /// ONE CARRIER PER FACT (law 12): this is written only where the
    /// element has no letters to spell the rotor from, and
    /// `Element::carried_rotor` reads the tape first. An element with a
    /// tape never carries one here.
    pub(crate) rotor: Option<crate::transducer::alphabet::Rotor>,
}

/// A germ as tape, action, chain, and magnitude.
#[derive(Debug, Clone)]
pub struct Element {
    /// Coordinate-exact ANASTOME description (empty for a pure residual chain).
    ///
    /// IT IS NOT CALLED `word`, and that is a ruling applied rather than a
    /// preference. `word` is the document's noun for a path in the Cayley
    /// graph whose meaning is a VERTEX — `alphabet::Word`, the object
    /// `Spelling` and every rotor reader carry. This field holds serialised
    /// bytes. Giving the encoding the vocabulary's most load-bearing noun is
    /// §2.1 at the worst place for it, and the accessor beside it has always
    /// been `description`, so one field carried two names and one of them was
    /// wrong.
    pub(crate) description: Vec<u8>,
    pub(crate) action: Action,
    /// Per-lane support, exact from the description.
    pub(crate) mag: [u64; 4],
    /// The CENTERED COBOUNDARY CHAIN — the residual's substance with geometry
    /// preserved up to translation: `(row, col, before, after)` over the changed
    /// sites, min-corner framed to the origin, sorted. Empty for a closed word.
    ///
    /// A5'S PROOF IS THE TYPE. As a bare `Vec` this field could hold a chain
    /// that says two things at one site — a chain with no canonical form and
    /// so no address (law 2) — and every such element fell to one key. The
    /// field is the accepted chain itself, so that element cannot be built:
    /// `canonical_chain` is the only constructor, and the tape goes through
    /// it.
    pub(crate) sub: CanonicalChain,
    /// THE PUNCTURED AXIS — sections along the germinated fiber pierced through
    /// this thought: alternative per-site transition ASSIGNMENTS over the SAME
    /// base positions. A recurring family (same geometry, different transitions)
    /// collapses into sections of this one bundle instead of sibling cards —
    /// growth as +axis (volume), not +card (area). The base `sub`'s own
    /// assignment is the implicit zeroth section. Vec-of-relations; empty until
    /// the axis is pierced. (The FREE part of the fiber; cycles condense into
    /// `torsion` below.)
    pub(crate) sections: Vec<Vec<(u8, u8)>>,
    /// THE TORSION OF THE AXIS — CE's law reaching the germinated fiber ("colors
    /// are torsion fields, not lookup tables"; the torsion_order is the period of
    /// the generating fold). When sections form the ORBIT of one hue-map
    /// generator σ (derived from the witnessed family link by link, exactly as peel
    /// derives offsets — never an enumerated plane), the fiber condenses from a
    /// free list to a TORSION CYCLE: `(σ relations from↦to, period ℓ)`. The
    /// orbit {base, σ·base, σ²·base, …} of length ℓ is spoken in O(1) storage —
    /// and a CLOSED cycle (σ^ℓ = id on its support) speaks orbit members never
    /// individually witnessed: capacity earned by exact structure. The fiber is
    /// thus ℤ^r ⊕ ⊕ᵢ ℤ/tᵢ — free sections ⊕ torsion cycles.
    pub(crate) torsion: Vec<TorsionCycle>,
    /// THE SPEAKER'S ATTESTATION ABOUT THIS WORD'S CYCLES (54.5, as ruled
    /// 2026-09-20): class ↦ `(σ, ℓ, chord)`, the orbits a mind's own pivot
    /// stroke filed before it spoke.
    ///
    /// It lived on `Form.torsion`, and a form is GEOMETRY — the same
    /// complex whatever mind holds it. A filing is not: `pivot_orbit` takes
    /// the mesh horizon, the mind's crystal and a budget, and files only in
    /// its `NoneDischarged` branch, so two minds holding one form at
    /// different budgets file differently. The map does not factor through
    /// the form, which is exactly why `from_address` cannot rebuild it and
    /// why keying it would make a pivot RENAME its own word.
    ///
    /// Its type is `Address × Signer → FiledOrbit`, and an element is a word
    /// as one mind minted it — the smallest carrier that has both. The
    /// court reads it here as the speaker's CLAIM and checks it against the
    /// geometry it re-derives itself, instead of asking a form to vouch for
    /// its own filing.
    ///
    /// `TorsionCycle` above is NOT this and the two are never converted:
    /// its third coordinate is a fibre index of the element's own axis,
    /// this one's is a chord of the form's faces, and carrying either
    /// across as the other invents a coordinate the source never held.
    pub(crate) filing: crate::network::nerve::Keyed<
        crate::transducer::sentence::FiledOrbit,
        crate::network::nerve::coord::Holonomy,
    >,
    /// THE RECEIPTS ENVELOPE (54.5 Road (d)): the graded nerve incidence
    /// this word stands in, and the evidence a court replays it with —
    /// citations with the citations' affine transitions, derivations, and pivot
    /// certificates.
    ///
    /// It is here for the same reason `filing` is here. A form is GEOMETRY,
    /// the same complex whatever mind holds it, and none of this factors
    /// through it: a derivation is a PRESENTATION and two minds reaching one
    /// rotor by different ones must reach one address (§1.5); a certificate
    /// is what one mind's stroke did against one budget. `Address × Signer`
    /// is the index, and an element is the smallest carrier that has both.
    ///
    /// All three rode `description` — the tape — which is precisely why the
    /// tape could not be retired. See [`crate::transducer::element::receipts`].
    pub(crate) receipts: crate::transducer::element::ReceiptsEnvelope,
    /// THE ANCHOR FIBER (Step 7) — γ = (π_anchor, Δ_substance): the recursive
    /// selection term restricting WHERE this word's substance applies on a
    /// scene. None = ⊤, the unconstrained carrier (pure topological actions,
    /// and every word born before selection entered the language).
    pub(crate) anchor: Option<crate::transducer::anchor::Sel>,
    /// THE CONNECTION — for every assignment spoken on this fibre (the base's
    /// own, a section, a torsion-orbit member), the versors V ∈ Spin(4,1)(ℚ)
    /// under which a sibling was observed and carried into this frame:
    /// `(assignment, V)`. The parallel-transport record of the bundle; the
    /// set of V reached for one assignment is its holonomy. Vec-of-relations;
    /// excluded from the address (a connection deepens the thought, it does
    /// not rename it).
    pub(crate) transitions: Vec<Transition>,
    /// The memoized address — a measurement of the tape and the action,
    /// taken once and cleared wherever the base mutates. Never serialized.
    pub(crate) address: std::sync::OnceLock<crate::transducer::address::Address>,
    /// THE GAUGE AXIS KEYED BY PROFILE: every assignment this word speaks
    /// (its own, its sections, its torsion orbits) reached by the key of
    /// its gauge charge — one read names the 0 / 1 / many assignments a
    /// body could be spoken with. Memoized; cleared wherever the fibre
    /// mutates. Never serialized. It holds the ASSIGNMENTS under each one's
    /// charge, never positions into `assignments()` (78.43(1)).
    pub(crate) profiles: std::sync::OnceLock<Vec<super::gauge::ChargeRow>>,
}

impl Element {
    /// The identity element — empty tape, identity action, no chain.
    pub fn identity() -> Self {
        Element {
            description: Vec::new(),
            action: Action::default(),
            mag: [0; 4],
            sub: CanonicalChain::default(),
            sections: Vec::new(),
            torsion: Vec::new(),
            filing: crate::network::nerve::Keyed::new(),
            receipts: crate::transducer::element::ReceiptsEnvelope::default(),
            transitions: Vec::new(),
            address: std::sync::OnceLock::new(),
            profiles: std::sync::OnceLock::new(),
            anchor: None,
        }
    }

    /// The action's spatial versor: composed from the tape when letters
    /// exist, otherwise the stored S stratum.
    pub fn versor(&self) -> Option<Versor> {
        if let Some(letters) = self.letters() {
            return conformal_of(&letters.spelling.word, &letters.carriage);
        }
        self.action.spatial.clone()
    }

    /// Open a germinated axis holding `v`. Returns the axis index.
    ///
    /// The new index is the successor of the largest germinated axis this
    /// word already carries, which the set determines; a word carrying none
    /// opens at the first germinated seat, which is that set's successor
    /// too and not a value invented for it.
    /// AN AXIS ALREADY HOLDING THIS VERSOR IS THIS AXIS, and opening it
    /// again is the same axis answered, never a second seat.
    ///
    /// This pushed unconditionally. `germinated` has no remover anywhere in
    /// the tree, its writer `Basis::germinate_fibre` is ungated and fires
    /// once per stroke per address for as long as a held orbit stands
    /// unclosed on a Twisted chart, and every push re-mints the address and
    /// is copied by `compose`, by `solution`, and by the court on every
    /// ruling. A versor is sixteen exact rationals. So one word could carry
    /// an unbounded list of the SAME opened axis, and nothing would ever
    /// take one out.
    ///
    /// Dormant on the corpus this was found on — `germinated fibre axis`
    /// fires zero times in twenty-four minutes — so this is the carrier
    /// bounded at its writer, not a reading that moved.
    ///
    /// Projective equality, because a versor and its negation are one
    /// transport (1.5) and two seats for them would be the same axis twice.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — the largest index a word carries is a reading of its own axis set, content"
    )]
    pub fn open_axis(&mut self, v: Versor) -> Option<u16> {
        if let Some((held, _)) = self
            .action
            .germinated
            .iter()
            .find(|(_, w)| same_versor(w, &v))
            .map(|(i, w)| (*i, w))
        {
            return Some(held);
        }
        // STD1 — the next seat is one past the largest, and a word whose
        // largest seat is the carrier's last has no seat left: refused,
        // never a second axis pinned onto the last one.
        let i = match self.action.germinated.iter().map(|(i, _)| *i).max() {
            Some(m) => m.checked_add(1)?.max(FIBER_GERMINATED),
            None => FIBER_GERMINATED,
        };
        self.action.germinated.push((i, v));
        self.address.take();
        Some(i)
    }

    pub fn germinated(&self) -> &[(u16, Versor)] {
        &self.action.germinated
    }

    pub fn gauge(&self) -> &[(u8, u8)] {
        &self.action.gauge
    }

    /// Occupied strata: S when the element MOVES, G when the permutation is
    /// not empty, germinated axes by index.
    ///
    /// S IS OCCUPIED BY THE MOTION, WHEREVER THE MOTION IS WRITTEN. This
    /// read `versor()` alone, which is `Cl(3,1)` and holds ONE rotation
    /// plane: a 90° turn on `e₂∧e₃` has the IDENTITY versor, so the
    /// element that turns it reported no occupied stratum at all. That is
    /// not a cosmetic reading — `is_identity` is `axes().is_empty()`, and
    /// `cech::glues`, `ruling`, `learn`, `stalk::chart` and `comprehend`
    /// all decide on this slice or on that predicate. A transition in a
    /// plane the sensor cannot see would have read as no transition, which
    /// is the flattening arriving one organ later.
    pub fn axes(&self) -> Vec<u16> {
        let mut a = Vec::new();
        let moves = self.versor().is_some_and(|v| !same_versor(&v, &identity_versor()))
            || self.carried_rotor().is_some_and(|r| !r.is_identity());
        if moves {
            a.push(FIBER_SPATIAL);
        }
        let g = if let Some(letters) = self.letters() {
            letters.sigma
        } else {
            self.action.gauge.clone()
        };
        if !g.is_empty() {
            a.push(FIBER_GAUGE);
        }
        if self.action.temporal != 0 {
            a.push(FIBER_TEMPORAL);
        }
        if self.action.shell != 0 {
            a.push(FIBER_SHELL);
        }
        for (i, _) in &self.action.germinated {
            a.push(*i);
        }
        a
    }

    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — the largest axis index this word carries is a reading of the axis set, content; a word with no axes has none and refuses"
    )]
    pub fn max_axis(&self) -> Option<u16> {
        self.axes().into_iter().max()
    }

    /// Set the per-lane support this word carries (an opened axis carries
    /// the listing's measure and the charts it joined). Refused — the word
    /// unchanged, `false` — where the holonomic lane would break
    /// [`holonomic_fits`] with the sections and cycles it holds (STD1).
    #[must_use]
    pub fn set_magnitude(&mut self, mag: [u64; 4]) -> bool {
        if !holonomic_fits(mag[3], self.sections.len(), self.torsion.len()) {
            return false;
        }
        self.mag = mag;
        true
    }

    /// Room for `sections` more free sections and `torsion` more cycles
    /// under [`holonomic_fits`].
    fn has_room(&self, sections: usize, torsion: usize) -> bool {
        match (self.sections.len().checked_add(sections), self.torsion.len().checked_add(torsion)) {
            (Some(s), Some(t)) => holonomic_fits(self.mag[3], s, t),
            _ => false,
        }
    }

    pub fn is_identity(&self) -> bool {
        // A residual with a canceling hue tally has identity action but a
        // real chain — the chain decides. A word with a description tape
        // is never the identity.
        //
        // The tag reads come FIRST because they are one byte each and
        // `axes` now asks `carried_rotor`, whose sentence arm decodes the
        // form. Same predicate, and a sentence no longer pays for a parse
        // to be told it is not the identity.
        !self.is_sentence()
            && !self.is_rewrite()
            && !self.is_translation()
            && self.sub.is_empty()
            && self.axes().is_empty()
    }

    /// The centered coboundary chain (empty for pure-algebra elements).
    /// The anchor fiber — None is ⊤ (the unconstrained carrier).
    pub fn anchor(&self) -> Option<&crate::transducer::anchor::Sel> {
        self.anchor.as_ref()
    }

    /// Bind the anchor fiber (germination writes it at birth; the court's
    /// refinement rebinds it).
    pub fn with_anchor(mut self, pi: crate::transducer::anchor::Sel) -> Self {
        self.anchor = Some(pi);
        self
    }

    pub fn sub(&self) -> &[(i64, i64, u8, u8)] {
        &self.sub
    }

    /// The ANASTOME description tape (connection or sentence).
    pub fn description(&self) -> &[u8] {
        &self.description
    }

    /// Replace the description tape and drop the memoized address.
    pub fn set_description(&mut self, description: Vec<u8>) {
        self.description = description;
        self.address.take();
    }

    /// The free sections along the punctured axis (cycles live in `torsion`).
    pub fn sections(&self) -> &[Vec<(u8, u8)>] {
        &self.sections
    }

    /// The torsion cycles of the axis: `(generator σ, period ℓ)` each.
    /// The speaker's filing: class ↦ `(σ, ℓ, chord)`.
    pub fn filing(&self) -> &crate::network::nerve::Keyed<
        crate::transducer::sentence::FiledOrbit,
        crate::network::nerve::coord::Holonomy,
    > {
        &self.filing
    }

    /// THE RECEIPTS ENVELOPE this word carries (54.5 Road (d)): its nerve
    /// incidence and the evidence a court replays it with.
    pub fn receipts(&self) -> &crate::transducer::element::ReceiptsEnvelope {
        &self.receipts
    }

    /// Set the envelope. The address does not move: none of this is a
    /// coordinate of the form (§1.5, `form_vertex_key_ignores_citations_and_certificates`), which is
    /// the whole reason it rides here.
    pub fn set_receipts(&mut self, receipts: crate::transducer::element::ReceiptsEnvelope) {
        self.receipts = receipts;
    }

    pub fn torsion(&self) -> &[TorsionCycle] {
        &self.torsion
    }

    /// The connection: every (assignment, g) under which a sibling was
    /// transported into this frame.
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }

    /// Record a dislocation versor on the fibre: (assignment, V, fibre)
    /// excluded from the address. Duplicate (asg, V, fibre) not stored twice.
    pub fn record_dislocation(&mut self, asg: Vec<(u8, u8)>, g: Versor, fibre: u16) {
        if !self
            .transitions
            .iter()
            .any(|t| t.asg == asg && same_versor(&t.gauge, &g) && t.fibre == fibre)
        {
            self.transitions.push(Transition { asg, gauge: g, fibre });
        }
    }

    /// File a torsion cycle `(σ, ℓ)` on fibre `fibre`. Duplicate generators
    /// on the same fibre are not stored twice.
    pub fn record_torsion(&mut self, sigma: Vec<(u8, u8)>, period: u64, fibre: u16) {
        let _ = self.record_cycle(TorsionCycle { generator: Generator::Hue(sigma), period, fibre, remainder: None });
    }

    /// RM1 — FILE A CYCLE WITH WHAT IT CLOSED UP TO. One cycle per
    /// `(σ, fibre)`: a cycle filed without a remainder GAINS the arriving
    /// one; one filed with a different remainder refuses it, and the caller
    /// counts the refusal. A period of zero is no cycle and is refused by
    /// name (A17). A new cycle past [`holonomic_fits`] is refused by name
    /// (STD1).
    pub fn record_cycle(&mut self, cycle: TorsionCycle) -> Filed {
        if cycle.period == 0 {
            return Filed::NoPeriod;
        }
        if !period_stands(&cycle, &assignment(&self.sub)) {
            return Filed::Misspoken;
        }
        let room = self.has_room(0, 1);
        match self.torsion.iter_mut().find(|t| t.generator == cycle.generator && t.fibre == cycle.fibre) {
            None if !room => Filed::PastCarrier,
            None => {
                self.torsion.push(cycle);
                self.profiles.take();
                Filed::New
            }
            Some(t) => match (&t.remainder, cycle.remainder) {
                (_, None) => Filed::Standing,
                (None, Some(r)) => {
                    t.remainder = Some(r);
                    Filed::Gained
                }
                (Some(have), Some(r)) if have.same(&r) => Filed::Standing,
                (Some(_), Some(_)) => Filed::Refused,
            },
        }
    }

    /// RM1 — the remainders this word's torsion record carries.
    pub fn remainders(&self) -> impl Iterator<Item = &Remainder> {
        self.torsion.iter().filter_map(|t| t.remainder.as_ref())
    }

    /// RM1 — each remainder with its RAY, read off the generator of the cycle
    /// it rides (law 12): a holonomy cycle's ray is the deficit `g`; a hue
    /// cycle's remainder has no ray (`None`).
    pub fn remainder_rays(&self) -> impl Iterator<Item = (&Remainder, Option<&crate::transducer::alphabet::Rotor>)> {
        self.torsion.iter().filter_map(|t| Some((t.remainder.as_ref()?, t.generator.ray())))
    }

    /// RM1 (c) — THE OPEN REMAINDER of `class` perceived at `at`, answered by
    /// the re-fill act at `stamp`, and the pressure it put on the word. `None`
    /// where the record carries no open remainder of that class there.
    pub fn answer_remainder(
        &mut self,
        class: &crate::transducer::sentence::ClassVector,
        at: &[Vec<u8>],
        stamp: crate::chronometry::Stamp,
    ) -> Option<[u64; 4]> {
        let t = self.torsion.iter_mut().find(|t| {
            t.remainder.as_ref().is_some_and(|r| *r.class == *class && r.at == at && r.answered.is_none())
        })?;
        let pressure = t.pressure()?;
        if let Some(r) = t.remainder.as_mut() {
            r.answered = Some(stamp);
        }
        Some(pressure)
    }

    /// The pressure an open remainder of `class` at `at` puts on this word.
    pub fn open_pressure(&self, class: &crate::transducer::sentence::ClassVector, at: &[Vec<u8>]) -> Option<[u64; 4]> {
        self.torsion
            .iter()
            .find(|t| t.remainder.as_ref().is_some_and(|r| *r.class == *class && r.at == at && r.answered.is_none()))?
            .pressure()
    }

    /// The fibre's span ABOVE ITS BASE — free sections plus every torsion
    /// orbit's members past the one it opens on. The volume metric proper, and
    /// a reader that wants it asks for it here: a caller subtracting the base
    /// off `fiber_span` to recover this is decomposing a composite by
    /// arithmetic, which says the composite was the wrong granularity for that
    /// reader, not that the reader owed a subtraction.
    ///
    /// It sums THIS element's own two fibre fields. A sentence carries its
    /// cycles on its form, leaves both of these empty, and is read through
    /// `sentence::form_torsion` (`Kernel::fibre_volume_total`).
    ///
    /// In the period's own carrier, `u64`, exact or refused (78.43(2)): a
    /// period reaches Landau's g(256), and the volume sums them. `None`
    /// where the sum passes its carrier.
    pub fn fibre_volume(&self) -> Option<u64> {
        let mut volume = u64::try_from(self.sections.len()).ok()?;
        for t in &self.torsion {
            // A cycle of period ℓ spans ℓ − 1 beyond its base. A17 files a
            // cycle only at the period it CLOSES at and the tape refuses a
            // zero, so ℓ ≥ 1 here; `checked_sub` says so rather than trusts.
            volume = volume.checked_add(t.period.checked_sub(1)?)?;
        }
        Some(volume)
    }

    /// The fiber's total spoken span — base + the volume above it (cycles
    /// count each cycle's full period). `None` past the carrier.
    pub fn fiber_span(&self) -> Option<u64> {
        self.fibre_volume()?.checked_add(1)
    }

    /// THE TORSE MOVE — extract the torsion of the punctured axis. Candidate
    /// generators are DERIVED from the witnessed family (base → each section,
    /// link by link — the same way peel derives offsets; nothing enumerated); the
    /// first σ whose orbit spans enough of the family condenses those sections
    /// into ONE cycle `(σ, ℓ)`. The period ℓ is READ before the first
    /// revolution — the lcm of σ's cycle lengths through the base's hues
    /// ([`orbit_period`], 78.43(2)) — the FULL cycle, including unwitnessed
    /// members: capacity by exact structure. A section belongs to the cycle
    /// exactly when its hue map is a power of σ on the cycles ([`on_orbit`]),
    /// decided per section; the orbit is never walked, and nothing caps it —
    /// `MAX_ORBIT = 32` was a telescope that stopped before the planet came
    /// back. Returns the number of sections condensed (0 = no torsion found;
    /// the free part stays as it was).
    pub fn torse(&mut self) -> usize {
        if self.sections.is_empty() {
            return 0;
        }
        let base = assignment(&self.sub);
        for k in 0..self.sections.len() {
            let Some(sigma) = derive_hue_map(&base, &self.sections[k]) else { continue };
            // An orbit that leaves its domain, or a map that does not permute
            // its hues, has no period and files nothing (refused by name).
            let Ok(period) = orbit_period(&sigma, &base) else { continue };
            let spanned: Vec<usize> = (0..self.sections.len())
                .filter(|&i| on_orbit(&sigma, &base, &self.sections[i]))
                .collect();
            // Only a CLOSED orbit is a cycle (σ^t = id on its support, the orbit
            // witnessed at the mint): it condenses the sections it spans, even
            // one (a swap). An orbit that leaves its domain is not filed under
            // a period — those sections stay free; the open orbit is the
            // remainder the word carries, never a winding it claims.
            if !spanned.is_empty() {
                for &i in spanned.iter().rev() {
                    self.sections.remove(i);
                }
                self.torsion.push(TorsionCycle { generator: Generator::Hue(sigma), period, fibre: FIBER_SPATIAL, remainder: None });
                self.profiles.take();
                return spanned.len();
            }
        }
        0
    }

    /// PUNCTURE — pierce (or extend) the germinated axis with a new section: a
    /// fresh transition assignment over the same base positions. The Z through
    /// the paper: the thought gains VOLUME; the family member becomes a section,
    /// not a card. Returns false if the assignment is already spoken (or does
    /// not fit the base geometry), or if one more section would break
    /// [`holonomic_fits`] (STD1).
    pub fn puncture(&mut self, asg: Vec<(u8, u8)>) -> bool {
        if !self.has_room(1, 0) {
            return false;
        }
        if asg.len() != self.sub.len()
            || asg == assignment(&self.sub)
            || self.sections.contains(&asg)
            || self.assignments().contains(&asg)
        {
            return false; // already spoken — by base, a section, or a torsion orbit
        }
        self.sections.push(asg);
        self.profiles.take();
        true
    }

    /// ATTACH — connect a congruent sibling to this bundle. Its chain is
    /// transported by g into this frame (g · supp(sibling) = supp(self)), the
    /// transported assignment becomes a section (or is recognised as one the
    /// fibre already speaks), and (assignment, g) joins the connection.
    /// Returns the sections the fibre gained (0 or 1); a g that does not
    /// carry the sibling onto this base changes nothing, and neither does a
    /// join with no room for its section under [`holonomic_fits`] (STD1).
    pub fn attach(&mut self, sibling: &Element, g: &Versor) -> usize {
        if !self.has_room(1, 0) {
            return 0;
        }
        let Some(asg) = pullback(&positions(&self.sub), &sibling.sub, g) else { return 0 };
        let gained = self.puncture(asg.clone()) as usize;
        if !self.transitions.iter().any(|t| t.asg == asg && same_versor(&t.gauge, g)) {
            self.transitions.push(Transition { asg, gauge: g.clone(), fibre: FIBER_SPATIAL });
        }
        gained
    }

    /// Merge another placement's fibre into this bundle (adoption of a word
    /// already held at this address): its sections puncture here and its
    /// connection joins. Returns the count of sections newly spoken. The
    /// join is refused whole — nothing punctures, no transition joins —
    /// where the other's sections, every one of them new, would break
    /// [`holonomic_fits`] (STD1).
    pub fn merge_fibre(&mut self, other: &Element) -> usize {
        if !self.has_room(other.sections.len(), 0) {
            return 0;
        }
        let mut added = 0usize;
        for s in &other.sections {
            if self.puncture(s.clone()) {
                added += 1;
            }
        }
        for Transition { asg: a, gauge: g, fibre: f } in &other.transitions {
            if !self
                .transitions
                .iter()
                .any(|t| &t.asg == a && same_versor(&t.gauge, g) && &t.fibre == f)
            {
                self.transitions.push(Transition {
                    asg: a.clone(),
                    gauge: g.clone(),
                    fibre: *f,
                });
            }
        }
        added
    }

}

/// STD1 (rule 11) — MEASURE_BOUND, the one statement every pure-count sum
/// cites under its `#[expect]`.
///
/// Each lane of a chain's measure is a support count: 𝒮 and 𝒢 count links;
/// 𝒽 is a difference of component counts plus a difference of boundary
/// counts of supports of at most that many sites, at most one plus four per
/// site; 𝒯 is 0. So every lane is at most five times the chain's links. A
/// link is 18 bytes and an address space is below 2⁴⁸, so a sum of measures
/// over RESIDENT chains stays below 2⁵⁰ — as do a resident grid's frame
/// dimensions, the dimensions' sum and its cell count — and none can pass a `u64`, or
/// a 64-bit `usize`. A sum whose summands can arrive from outside a resident
/// carrier (a wire magnitude, an escrow, a price) is not covered: it is
/// checked and refused.
pub const MEASURE_BOUND: &str =
    "each lane of a chain's measure is at most five times its links; a sum over resident chains cannot pass u64";

/// The perceptual lane projection of a centered coboundary chain — the
/// per-lane magnitude MEASURED from the chain's geometry, never funneled
/// onto one axis:
///   𝒮 spatial:  occupancy transformation — sites where support appears or
///               vanishes (mass moved, scaled, or reshaped).
///   𝒢 gauge:    chromatic content — hue transitions on sites where the
///               support stands on both sides (recolour, palette orbit).
///   𝒯 temporal: standing extent — TUniversal ticks a listing has stood,
///               or the family's scene-state span at mint. The chain has
///               none; 0 here. `Element::magnitude` and the kernel overlay
///               the live T coordinate.
///   𝒽 holonomic: boundary topology — the connected-component and
///               boundary-cochain deltas between the before-support and the
///               after-support (merges, splits, enclosures). Sections and
///               torsion cycles the word holds are added on this lane by
///               `Element::magnitude`.
/// A recolour word is gauge-lane work; a move is spatial; a hole closing is
/// shell. Without these projections the four-lane manifold degenerates to a
/// scalar on 𝒯 and every gradient downstream flatlines.
///
/// STD1 — the lanes are support counts, and [`MEASURE_BOUND`] is what that
/// buys every sum of them.
pub fn measure_lanes(chain: &[(i64, i64, u8, u8)]) -> [u64; 4] {
    let mut spatial = 0u64;
    let mut gauge = 0u64;
    let void = crate::arc::ground();
    let mut before_pos: Vec<(i64, i64)> = Vec::new();
    let mut after_pos: Vec<(i64, i64)> = Vec::new();
    for &(r, c, b, a) in chain {
        if b != void {
            before_pos.push((r, c));
        }
        if a != void {
            after_pos.push((r, c));
        }
        match (b != void, a != void) {
            (true, true) if b != a => gauge += 1,
            (true, false) | (false, true) => spatial += 1,
            _ => {}
        }
    }
    let sb = crate::transducer::beam::signature(&before_pos, &[]);
    let sa = crate::transducer::beam::signature(&after_pos, &[]);
    let shell = sb.components.abs_diff(sa.components) + sb.boundary.abs_diff(sa.boundary);
    // LANE 2 IS NOT THE CHAIN'S TO REPORT. 𝒯 is temporal extent — the
    // scene-state span in receipts (51.9) — and a chain is a set of links
    // with no clock in it: nothing here can measure how long anything stood.
    // Reporting zero is therefore a reading of this carrier and not a
    // placeholder, and the three lanes beside it ARE the chain's own. A
    // caller that needs 𝒯 asks the mint (`sentence_measure` takes
    // `t_extent`, read by `Contemplation::span_of`) or the bundle
    // (`bundle_pressure`); one that reads a chain gets what a chain knows,
    // which on this lane is nothing.
    [spatial, gauge, 0, shell]
}

impl Element {
    /// The four lanes this germ carries content on — 𝒮 spatial, 𝒢 gauge, 𝒯
    /// temporal, 𝒽 holonomic — exactly where the coordinate-exact magnitude
    /// is nonzero (never the lossy tally).
    pub fn lanes(&self) -> [bool; 4] {
        let m = self.magnitude();
        std::array::from_fn(|l| m[l] > 0)
    }

    /// The germ's polytonic MAGNITUDE — per-lane support size (‖Δ‖'s |supp|
    /// component), never summed, computed EXACTLY from the description at
    /// germination. A settled demand's residual magnitude is 0⃗; a real germ's is
    /// nonzero — the residual the POW++ gate reads.
    /// Spatial, gauge, and shell from the chain (and what a connection
    /// carries). Lane 2 is temporal extent at mint (scene-state span).
    /// Lane 3 gains the holonomic volume — sections and torsion cycles.
    /// Total under [`holonomic_fits`], which every writer holds.
    pub fn magnitude(&self) -> [u64; 4] {
        let mut m = self.mag;
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 11 — holonomic_fits: mag[3] + sections + torsion stays inside u64, held by set_magnitude, from_connection, the record decode, puncture (attach, merge_fibre) and record_cycle; torse only condenses"
        )]
        let h = m[3].saturating_add((self.sections.len() + self.torsion.len()) as u64);
        m[3] = h;
        m
    }

    /// The lanes the chain itself carries, before the sections and torsion
    /// cycles the word gained after it was spoken are counted. This is what
    /// re-derives from the chain; a word deepened by `attach` or
    /// `merge_fibre` keeps this fixed while `magnitude` grows on lane 3.
    pub fn base_magnitude(&self) -> [u64; 4] {
        self.mag
    }

    /// The base magnitude on the lanes a chain names: 𝒮 spatial, 𝒢 gauge,
    /// 𝒽 shell. Lane 𝒯 is temporal extent, measured on the standing
    /// carrier and never on the chain (`measure_lanes` writes it zero), so
    /// it is quotiented here by the same law. This is the vector the court
    /// compares with `from_residual(chain).base_magnitude()`, as one
    /// equality, never lane by lane.
    pub fn chain_magnitude(&self) -> [u64; 4] {
        let [spatial, gauge, _, shell] = self.mag;
        [spatial, gauge, 0, shell]
    }

    /// THE TRANSPOSITION QUOTIENT — factor out the translation (the ℤ² orbit) so
    /// the element becomes its position-independent SUBSTANCE
    /// (dilation·rotation·gauge·residual). Two transposition-equivalent demands
    /// (same substance, different position) share ONE substance — the object of
    /// confluent reduction, not the coordinate-exact word. (Gauge S₁₀ canonical
    /// relabel is a further quotient, still to land.)
    pub fn substance(&self) -> Element {
        let mut out = self.clone();
        out.description = Vec::new();
        out.address.take();
        if let Some(v) = out.versor() {
            if let Some(t) = translator_of(&v) {
                if !same_versor(&t, &identity_versor()) {
                    // `v t̃`: a translator's reverse is its inverse (`T T̃ =
                    // 1`), so nothing is divided (78.32(1)).
                    if crate::transducer::cga::is_versor(&t) {
                        let d = v.geo(&t.reverse());
                        out.action.spatial = if same_versor(&d, &identity_versor()) {
                            None
                        } else {
                            Some(d)
                        };
                    }
                }
            }
        }
        out.mag = if out.sub.is_empty() {
            out.reduction_magnitude()
        } else {
            measure_lanes(&out.sub)
        };
        out
    }

    /// Reduction order: a non-identity versor or off-plane rotor occupies 𝒮,
    /// a nonempty permutation occupies 𝒢 (Law 12 parity with `axes`).
    pub fn reduction_magnitude(&self) -> [u64; 4] {
        let spatial = u64::from(
            self.versor().is_some_and(|v| !same_versor(&v, &identity_versor()))
                || self.carried_rotor().is_some_and(|r| !r.is_identity()),
        );
        let g_empty = if let Some(letters) = self.letters() {
            letters.sigma.is_empty()
        } else {
            self.action.gauge.is_empty()
        };
        let gauge = u64::from(!g_empty);
        [spatial, gauge, 0, 0]
    }

}
