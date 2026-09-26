// 21 step 2 — THE INCIDENCE OF A SCENE: the faces two supports name, and
// the hue carrier a speech indexes by.
//
//! A face exists because two bodies' supports meet or sit at ring distance
//! one, read off a keyed occupancy of the sites (rule 1); nothing here
//! pairs one body with another, and nothing ranks.

use super::*;

thread_local! {
    static FACE_WORK: Cell<u64> = const { Cell::new(0) };
}

#[expect(clippy::disallowed_methods, reason = "rule 11 — one per event in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
pub(crate) fn note_face_work() {
    FACE_WORK.with(|c| c.set(c.get().saturating_add(1)));
}

#[cfg(test)]
pub(crate) fn reset_face_work() {
    FACE_WORK.with(|c| c.set(0));
}

#[cfg(test)]
pub(crate) fn face_work() -> u64 {
    FACE_WORK.with(|c| c.get())
}

/// BQ5a — WHAT EACH CONTACT THE TOPOLOGY NAMED CARRIED, counted where its
/// edge is read (§8.1, 72.2).
///
/// This was the condensation FIELD's tally: a pair loop over every two
/// members, a dialled band `1 < δ ≤ 2` deciding whether a face was written,
/// then the pose. The loop and the band are gone (SQ1 item 2, 75): whether
/// two bodies touch is topological (A.14, `contact_cells`), and a contact is
/// read only on the pairs that pass names.
///
/// `offered` is every contact read. `unlifted` is one whose tether did not
/// read back. The regime of the tether is CONTENT — which class of motion
/// relates the two nuclei — counted beside the rest and never a gate. The
/// pose arms then say why an edge was not formed, and `condensed` is every
/// contact that carries its refraction `g = (θ_bθ_a⁻¹)·V` into `C¹`.
#[derive(Default, Clone, Copy, Debug)]
pub struct ContactTally {
    pub offered: u64,
    pub unlifted: u64,
    /// The tether's regime, `Contact::of(V)`: apart, tangent from outside,
    /// intersecting, tangent from inside, contained.
    pub apart: u64,
    pub tangent_external: u64,
    pub intersecting: u64,
    pub tangent_internal: u64,
    pub contained: u64,
    /// THE WAYS NO POSE IS DETERMINED, each at its own name (66, V1): a
    /// refusal that does not say which refusal it is answers nothing.
    pub bodiless: u64,
    pub isotropy: u64,
    pub unoriented: u64,
    /// V1 — THE ONLY ARM A DEMAND CAN RETIRE. Everything counted here is
    /// a proper rotation the alphabet cannot yet spell, with the
    /// reflections now drawn off into `mirrored`. Before the split this
    /// counter mixed the mutable with the permanent, and a rising
    /// `oblique` could not be read as "the vocabulary is short".
    pub oblique: u64,
    /// Proved improper: a reflection carries the pair. Permanent, and no
    /// generator will ever retire it.
    pub mirrored: u64,
    /// Proved unlike: no motion carries the pair at all. Also permanent, and
    /// kept apart from `mirrored` because a corpus of handed pairs and a
    /// corpus of unlike ones want different answers — and split by content
    /// (78.38(2), (C)): where neither similarity carries the pair, by each
    /// tower's refusal (`orders[region][sites]`); in chirality alone; on bases
    /// of different `n`; on the tube.
    pub incongruent: Incongruents,
    /// FR1 — a pair on the tube whose scale `ρ²` is not a rational square:
    /// no versor over ℚ relates them, and nothing is claimed about whether
    /// they are alike.
    pub irrational: u64,
    /// 78.47 — a turn read whose constructors met a null step.
    pub null_step: u64,
    /// 65.5 — the stroke was spent before every candidate was read: the
    /// stroke's typed cut, named, never a claim about the pair.
    pub unbounded: u64,
    pub open_180: u64,
    /// A relative turn the peel does not spell: 37.7's candidate, the
    /// demand typed and counted, never an edge invented for it.
    pub unspelled: u64,
    /// A relative turn the peel reads nothing from.
    pub unread: u64,
    /// 78.6(2) — A CONTACT WHOSE MEMBER STANDS AT ONE FRAME: no transition
    /// to read its pose from, so its edge is the tether with the refraction
    /// unread by name — `static`, an edge formed, never a refusal.
    pub statics: u64,
    /// 78.6(1) — A CONTACT WHOSE POSE IS A HELD STABILIZER: its edge is the
    /// coset of candidates, formed, never a representative.
    pub cosets: u64,
    pub condensed: u64,
    /// FR1 — THE CONTACTS WHOSE POSE WAS READ ON THE TUBE. The regime, not
    /// an outcome: each also lands in one arm above or in `condensed`.
    pub tube: u64,
    /// KB1 — bodies standing at a contact complex's witness whose piece did
    /// not read: their standing set mints no cell, counted by name.
    pub cells_unkeyed: u64,
    /// KB1 R1 — witnesses whose bodies are one by their pieces: no cell.
    pub cells_singular: u64,
}

/// THE INCONGRUENT POSES BY CONTENT (78.38(2), (C)).
#[derive(Default, Clone, Copy, Debug)]
pub struct Incongruents {
    /// Neither similarity carries the pair: by the region tower's refusal
    /// (row) and the site tower's (column), each at order 2, 3, 4 or
    /// unplaced — the rows and columns of `TowerRefusal::ALL`.
    pub orders: [[u64; 4]; 4],
    pub chirality: u64,
    pub base: u64,
    pub tube: u64,
}

/// A tower's refusal as its row or column of the tally, read off its content.
fn refusal_ix(r: blade::TowerRefusal) -> usize {
    use blade::{Harmonic, TowerRefusal};
    match r {
        TowerRefusal::At(Harmonic::Two) => 0,
        TowerRefusal::At(Harmonic::Three) => 1,
        TowerRefusal::At(Harmonic::Four) => 2,
        TowerRefusal::Unplaced => 3,
    }
}

impl Incongruents {
    pub fn total(&self) -> u64 {
        self.orders.iter().flatten().sum::<u64>() + self.chirality + self.base + self.tube
    }

    fn note(&mut self, why: blade::Incongruence) {
        match why {
            blade::Incongruence::Order { region, sites } => {
                self.orders[refusal_ix(region)][refusal_ix(sites)] += 1;
            }
            blade::Incongruence::Chirality => self.chirality += 1,
            blade::Incongruence::Base => self.base += 1,
            blade::Incongruence::Tube => self.tube += 1,
        }
    }
}

thread_local! {
    static CONTACTS: Cell<ContactTally> = const { Cell::new(ContactTally {
        offered: 0, unlifted: 0, apart: 0, tangent_external: 0, intersecting: 0,
        tangent_internal: 0, contained: 0, bodiless: 0, isotropy: 0, unoriented: 0,
        oblique: 0, mirrored: 0,
        incongruent: Incongruents { orders: [[0; 4]; 4], chirality: 0, base: 0, tube: 0 },
        irrational: 0, null_step: 0, unbounded: 0, open_180: 0,
        unspelled: 0, unread: 0, statics: 0, cosets: 0, condensed: 0, tube: 0, cells_unkeyed: 0, cells_singular: 0,
    }) };
}

pub(crate) fn note_contact(f: impl Fn(&mut ContactTally)) {
    CONTACTS.with(|c| {
        let mut v = c.get();
        f(&mut v);
        c.set(v);
    });
}

/// Read the contacts' tally and clear it, as the cover's is taken.
pub fn take_contact_tally() -> ContactTally {
    CONTACTS.with(|c| c.replace(ContactTally::default()))
}

/// THE CONTACTS WITHOUT CLEARING THEM, for a reader that prints per form
/// while a whole-run total is still accumulating. Two readers of one
/// counter, and only one of them may take it (law 12: the take is the
/// clear).
pub fn contact_tally() -> ContactTally {
    CONTACTS.with(|c| c.get())
}

impl ContactTally {
    /// THE CONTACT LINE — what the contacts were, then why an edge did not
    /// form, then what condensed. The regime is read, never gated on.
    pub fn line(&self) -> String {
        format!(
            "offered {} unlifted {} | tether: apart {} tangent-out {} intersecting {} \
             tangent-in {} contained {} | pose: tube {} | bodiless {} isotropy {} \
             unoriented {} oblique {} mirrored {} incongruent {} (orders region×sites {:?} chirality {} base {} \
             tube {}) irrational {} null step {} open-180° {} | unspelled {} unread {} | FORMED: CONDENSED {} coset {} \
             static {} | pose refusals by kind: P {} D {} I {} | cells unkeyed {} singular {}",
            self.offered,
            self.unlifted,
            self.apart,
            self.tangent_external,
            self.intersecting,
            self.tangent_internal,
            self.contained,
            self.tube,
            self.bodiless,
            self.isotropy,
            self.unoriented,
            self.oblique,
            self.mirrored,
            self.incongruent.total(),
            self.incongruent.orders,
            self.incongruent.chirality,
            self.incongruent.base,
            self.incongruent.tube,
            self.irrational,
            self.null_step,
            self.open_180,
            self.unspelled,
            self.unread,
            self.condensed,
            self.cosets,
            self.statics,
            self.kinds()[0],
            self.kinds()[1],
            self.kinds()[3],
            self.cells_unkeyed,
            self.cells_singular,
        )
    }

    /// CV1 — THE POSE REFUSALS BY KIND (78.9), each arm through
    /// `NoPose::obstruction_kind`, the one table: P, D, C, I, N,
    /// unclassified. An unoriented pair is a held stabilizer and counts in
    /// none.
    pub fn kinds(&self) -> [u64; 6] {
        use crate::transducer::blade::{Incongruence, NoPose};
        let mut out = [0u64; 6];
        let i = &self.incongruent;
        // Each incongruent arm by its own content, every cell of the orders.
        let orders = blade::TowerRefusal::ALL.into_iter().flat_map(|region| {
            blade::TowerRefusal::ALL.into_iter().map(move |sites| {
                (
                    NoPose::Incongruent(Incongruence::Order { region, sites }),
                    i.orders[refusal_ix(region)][refusal_ix(sites)],
                )
            })
        });
        for (arm, n) in [
            (NoPose::Bodiless, self.bodiless),
            (NoPose::Isotropy, self.isotropy),
            (NoPose::Unoriented, self.unoriented),
            (NoPose::Oblique, self.oblique),
            (NoPose::Mirrored, self.mirrored),
            (NoPose::Incongruent(Incongruence::Chirality), i.chirality),
            (NoPose::Incongruent(Incongruence::Base), i.base),
            (NoPose::Incongruent(Incongruence::Tube), i.tube),
            (NoPose::Irrational, self.irrational),
            (NoPose::NullStep, self.null_step),
        ]
        .into_iter()
        .chain(orders)
        {
            if let Some(k) = arm.obstruction_kind() {
                out[k.index()] += n;
            }
        }
        out
    }

    /// WHAT A DEMAND COULD RETIRE, against what it never will. `oblique`
    /// is the mutable half — a proper turn the alphabet is merely short of
    /// — and the two permanent arms are the geometry's own answer.
    pub fn mutable_over_permanent(&self) -> (u64, u64) {
        (self.oblique, self.mirrored + self.incongruent.total())
    }
}

/// WHICH INCIDENCE NAMED A PAIR (70.4): the Frame's star, a site two
/// members share, or a corner two bodies meet at. Carried with the pair so
/// the face built from it is counted where it came from; a pair two
/// incidences name is one face carrying both (N1). The condensation field
/// that was a fourth is gone with its band (BQ5a, SQ1 item 2): contact is
/// the topology's to name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Incidence {
    Star,
    Site,
    Contact,
}

/// A SITE OF THE OCCUPANCY: its own coordinates, and the members standing
/// on it in the order they were offered.
struct Occupied {
    site: Site,
    members: Vec<Vec<u8>>,
}

/// WHAT THE INCIDENCE NAMED BETWEEN TWO MEMBERS: every incidence that named
/// the pair (N1), and each contact between them (C2, 70.6) — its body pair
/// with their nuclei, its grade, and its WITNESS, the shared boundary
/// elements. Two different contacts between one pair are two, held apart,
/// in their witnesses' order, and never merged.
#[derive(Clone, Debug, Default)]
pub(crate) struct Named {
    pub(crate) hows: Vec<Incidence>,
    pub(crate) interfaces: Vec<ContactPair>,
}

/// ONE PAIR THE INCIDENCE NAMES: its two members and what named them.
#[derive(Clone, Debug)]
pub(crate) struct NamedFace {
    pub(crate) a: Vec<u8>,
    pub(crate) b: Vec<u8>,
    pub(crate) named: Named,
}

/// A PAIR IS ONE FACE, AND EVERY INCIDENCE THAT NAMES IT RIDES ON IT (N1,
/// rule 15). A second incidence over the same two members is the same face
/// and never another, and it is not dropped either: the diagonal and the
/// field naming one pair are two readings held together, in the enum's own
/// order, so no pass is first. An interface a contact brings is held the
/// same way: once, beside any other the pair already carries.
fn relate(
    related: &mut crate::network::nerve::Keyed<Named>,
    key: Vec<u8>,
    how: Incidence,
    interface: Option<ContactPair>,
) {
    if related.get(&key).is_none() {
        related.insert(key.clone(), Named::default());
    }
    let Some(named) = related.get_mut(&key) else { return };
    if let Err(at) = named.hows.binary_search(&how) {
        named.hows.insert(at, how);
    }
    if let Some(c) = interface {
        if let Err(at) = named.interfaces.binary_search_by(|x| x.witness.cmp(&c.witness)) {
            named.interfaces.insert(at, c);
        }
    }
}

/// THE KEY OF A BODY PAIR (C3, 70.7): the two bodies' addresses — each a
/// `coord::Piece`, its support written out — the lesser first. Each address
/// is self-delimiting, so the key reads back into the two it was written
/// from ([`read_body_pair_key`]) and no position of the base enters it.
pub(crate) fn body_pair_key(lo: &[u8], hi: &[u8]) -> Vec<u8> {
    let mut k = Vec::with_capacity(lo.len() + hi.len());
    k.extend_from_slice(lo);
    k.extend_from_slice(hi);
    k
}

/// BQ5a — THE CONTACT EDGE IS REFRACTION, NEVER RIGIDITY (72.1, 72.1b, 72.2,
/// A.16).
///
/// Two bodies the topology names as touching are related by
///
/// ```text
///     g_ab = (θ_bθ_a⁻¹) · V_ab,     V_ab = s_b s_a
/// ```
///
/// where `θ_x` is body `x`'s OWN pose across the transition — its union
/// before against its union after, FR1's reading on its tube — and `V` the
/// tether the two bodies' nuclei make before it, carried WHOLE in the
/// carriage. `θ` and `V` do not commute, so a circuit of such edges keeps the
/// curvature a product of tethers alone telescopes away (72.1b); the word is
/// the peel of `θ_bθ_a⁻¹`, a reading of the turn, and the carriage holds `g`
/// entire so no product splits it by lane.
///
/// EACH BODY IS COMPARED WITH ITSELF, NEVER WITH THE OTHER (as `meet_edge`
/// reads a meet's pieces, 56.5). A first cut read `relative_pose(a, b)`
/// between the two touching bodies, and on the corpus every contact refused
/// — four `Isotropy`, one `Irrational`, none condensed — because touching
/// pieces are functional complements and almost never congruent: the same
/// famine the retired field's harmonic certificate made. `θ_bθ_a⁻¹` is how
/// `b` turned against how `a` turned, which two unlike bodies both have.
///
/// It replaces `edge_between`'s test on the two bodies' UNION for a contact:
/// whether the pair moved rigidly asks whether two touching puzzle pieces are
/// one piece, and is not what their contact is.
///
/// TWO ARMS FORM WHAT ONE TRANSPORT CANNOT (78.6). A body whose pose is a
/// HELD STABILIZER (`Open180`) rides the edge as its COSET: every candidate
/// `θ_b θ_a⁻¹` over the two bodies' candidates, each with `V`, distinct
/// rays kept, never a representative (checklist (r)). A body AT ONE FRAME
/// has no transition to read a pose across: the edge is STATIC — `V` carried
/// and the refraction unread by name, never the union's rigidity and never
/// the identity.
///
/// Every refusal is counted at its own name on the contact line: an unread
/// tether, each `NoPose` arm of either body's pose, a turn the peel does not
/// spell (37.7's candidate) or reads nothing from. [`ContactEdge::Refused`]
/// then; the face still stands, because whether is topology.
///
/// 78.18(c) — THE TETHER IS BETWEEN THE FACE'S TWO BODIES, `V = s_b s_a` of
/// their nuclei (BD2's pieces), at every arm — never a member's union, whose
/// support can fall into several bodies (checklist (m)); each member's POSE is
/// its own, read across its own transition. 78.18(a) — a static edge keeps
/// each end's pose as far as it was read, the transited end's by name and the
/// one at one frame named so.
pub(crate) fn contact_edge(
    (s_a, s_b): (&crate::transducer::cga::Mv, &crate::transducer::cga::Mv),
    a: Option<(&blade::Blade, &blade::Blade)>,
    b: Option<(&blade::Blade, &blade::Blade)>,
    budget: &mut crate::physics::Budget,
) -> ContactEdge {
    use crate::transducer::sentence::{HeldPose, Refraction, Transport};
    note_contact(|t| t.offered += 1);
    let v = s_b.geo(s_a);
    match blade::Contact::of(&v) {
        Some(blade::Contact::Apart) => note_contact(|t| t.apart += 1),
        Some(blade::Contact::TangentExternal) => note_contact(|t| t.tangent_external += 1),
        Some(blade::Contact::Intersecting) => note_contact(|t| t.intersecting += 1),
        Some(blade::Contact::TangentInternal) => note_contact(|t| t.tangent_internal += 1),
        Some(blade::Contact::Contained) => note_contact(|t| t.contained += 1),
        None => {
            note_contact(|t| t.unlifted += 1);
            return ContactEdge::Refused { poses: [None, None] };
        }
    }
    let (Some((a, a_after)), Some((b, b_after))) = (a, b) else {
        let held = |m: Option<(&blade::Blade, &blade::Blade)>, budget: &mut crate::physics::Budget| match m {
            None => HeldPose::OneFrame,
            Some((before, after)) => match blade::relative_pose(before, after, budget) {
                Ok(blade::Pose::Turn(r)) => HeldPose::Read(vec![r]),
                Ok(blade::Pose::Open180(candidates)) => HeldPose::Read(candidates),
                Err(why) => HeldPose::Refused(why),
            },
        };
        note_contact(|t| t.statics += 1);
        return ContactEdge::Held(Refraction::Static { tether: v, poses: [held(a, budget), held(b, budget)] });
    };
    if blade::on_tube(a, a_after) || blade::on_tube(b, b_after) {
        note_contact(|t| t.tube += 1);
    }
    let pose = |before: &blade::Blade, after: &blade::Blade, budget: &mut crate::physics::Budget| match blade::relative_pose(before, after, budget) {
        Ok(blade::Pose::Turn(r)) => Ok(vec![r]),
        Ok(blade::Pose::Open180(candidates)) => {
            note_contact(|t| t.open_180 += 1);
            Ok(candidates)
        }
        Err(why) => {
            note_contact(|t| match why {
                blade::NoPose::Bodiless => t.bodiless += 1,
                blade::NoPose::Isotropy => t.isotropy += 1,
                blade::NoPose::Unoriented => t.unoriented += 1,
                blade::NoPose::Oblique => t.oblique += 1,
                blade::NoPose::Mirrored => t.mirrored += 1,
                blade::NoPose::Incongruent(why) => t.incongruent.note(why),
                blade::NoPose::Irrational => t.irrational += 1,
                blade::NoPose::NullStep => t.null_step += 1,
                blade::NoPose::Unbounded => t.unbounded += 1,
            });
            Err(why)
        }
    };
    // Each member's pose, or its refusal by name: the refusal rides the
    // edge to the caller, which reads the one arm a demand retires.
    let (thetas_a, thetas_b) = match (pose(a, a_after, budget), pose(b, b_after, budget)) {
        (Ok(x), Ok(y)) => (x, y),
        (x, y) => return ContactEdge::Refused { poses: [x.err(), y.err()] },
    };
    // Every candidate refraction, each ray once.
    let mut thetas: Vec<crate::transducer::cga::Mv> = Vec::new();
    for theta_a in &thetas_a {
        // `θ_b θ̃_a`, a ray (78.32(1)).
        if !crate::transducer::cga::is_versor(theta_a) {
            note_contact(|t| t.unread += 1);
            return ContactEdge::Refused { poses: [None, None] };
        }
        let inv = theta_a.reverse();
        for theta_b in &thetas_b {
            let theta = theta_b.geo(&inv);
            if !thetas.iter().any(|t| t.projective_eq(&theta)) {
                thetas.push(theta);
            }
        }
    }
    let mut edges: Vec<Transport> = Vec::with_capacity(thetas.len());
    for theta in thetas {
        let word = match super::derivation::peel_versor(&theta) {
            Some(Ok(w)) => w,
            Some(Err(_)) => {
                note_contact(|t| t.unspelled += 1);
                return ContactEdge::Refused { poses: [None, None] };
            }
            None => {
                note_contact(|t| t.unread += 1);
                return ContactEdge::Refused { poses: [None, None] };
            }
        };
        edges.push(Transport { word, carriage: theta.geo(&v), gauge: Vec::new() });
    }
    match <[Transport; 1]>::try_from(edges) {
        Ok([edge]) => {
            note_contact(|t| t.condensed += 1);
            ContactEdge::Formed(Box::new(edge))
        }
        Err(coset) => {
            note_contact(|t| t.cosets += 1);
            ContactEdge::Held(Refraction::Coset(coset))
        }
    }
}

/// BQ5a — THE CONTACT'S REFRACTION BETWEEN TWO MEMBERS, each read against
/// its own transition: its union before and after. A member at one frame
/// has none, and is read where it stands — its edge is the static tether
/// (78.6(2)). The edge where one formed, or the refraction the face's place
/// carries instead; neither where it was refused by name.
///
/// CV1b F3 — AND THE DEMAND AN OBLIQUE POSE LEAVES. Where a member's own
/// transition reads `NoPose::Oblique`, the relation is a rational block the
/// court can supply: the member's before and after are read through D3's
/// three gates (`oblique_demanded`, one statement with the word path's) and
/// the `RotorPrime` with its witness is answered beside the edge. The
/// refusal itself is never written onto the face: it is re-derived from the
/// two transitions at every percept.
pub(crate) fn members_contact_edge(
    unions: &crate::network::nerve::Keyed<(super::perceive::Body, super::perceive::Body, crate::transducer::element::Versor)>,
    nuclei: Option<(&crate::transducer::cga::Mv, &crate::transducer::cga::Mv)>,
    a: Vec<u8>,
    b: Vec<u8>,
    budget: &mut crate::physics::Budget,
) -> (
    Option<crate::transducer::sentence::Transport>,
    Option<crate::transducer::sentence::Refraction>,
    Vec<super::perceive::Demanded>,
) {
    // Each member's own transition, where it has one: its union before and
    // after. A member at one frame has none.
    let transit = |key: &[u8]| -> Option<(blade::Blade, blade::Blade)> {
        unions.get(key).map(|(before, after, _)| {
            (
                blade::shape_of(&super::perceive::support_of(before)).blade,
                blade::shape_of(&super::perceive::support_of(after)).blade,
            )
        })
    };
    let Some(nuclei) = nuclei else {
        // The face's body pair carries no nucleus to tether: nothing lifts.
        note_contact(|t| t.unlifted += 1);
        return (None, None, Vec::new());
    };
    let (ta, tb) = (transit(&a), transit(&b));
    match contact_edge(nuclei, ta.as_ref().map(|(x, y)| (x, y)), tb.as_ref().map(|(x, y)| (x, y)), budget) {
        ContactEdge::Formed(t) => (Some(*t), None, Vec::new()),
        ContactEdge::Held(r) => (None, Some(r), Vec::new()),
        ContactEdge::Refused { poses } => {
            // Each member whose own transition read oblique, through D3's
            // gates; its demand stands on the chart its union reaches.
            let mut demands = Vec::new();
            for (key, (pose, blades)) in [(&a, (poses[0], &ta)), (&b, (poses[1], &tb))] {
                let (Some(blade::NoPose::Oblique), Some((before_blade, after_blade)), Some((before, after, _))) =
                    (pose, blades, unions.get(key))
                else {
                    continue;
                };
                if let Some(d) = super::perceive::oblique_demanded(before, after, (before_blade, after_blade), &after.moduli, budget) {
                    demands.push(d);
                }
            }
            (None, None, demands)
        }
    }
}

/// BQ5a's guard (77, 78.4 E1) — THE CIRCUITS THAT CROSS A CONTACT FACE, a
/// face the topology named, carrying its interface, each by its arm through
/// the one circuit (78.6): CURVED where a member path's whole transport reads
/// and is not the identity ray — of them, those whose gauge half is static;
/// through a held coset, by how it closes; UNREAD where no member path read,
/// so the circuit survives `orbits_of` for being unread (rule 11), never for
/// carrying anything. In that order: curved, gauge static, coset closed,
/// modulo held, open, unread.
pub(crate) fn contact_circuits(form: &crate::transducer::sentence::Form, budget: &mut crate::physics::Budget) -> [usize; 6] {
    use crate::transducer::sentence::circuit::{paths_carried, CosetClosure};
    let identity = crate::transducer::element::identity_versor();
    let mut t = [0usize; 6];
    for o in &form.orbits {
        if !o.steps.iter().any(|s| form.faces.at(&s.overlap).is_some_and(|f| f.interface.is_some())) {
            continue;
        }
        let Some(carried) = paths_carried(&form.faces, &form.vertices, o) else {
            t[5] += 1;
            continue;
        };
        let curved = |c: &&crate::transducer::sentence::circuit::PathCarried| {
            c.composite().is_some_and(|m| !crate::transducer::element::same_versor(&m, &identity))
        };
        let paths: Vec<_> = carried.scoped().map(|(_, c)| c.clone()).collect();
        let closures: Vec<CosetClosure> = paths.iter().filter_map(|c| c.coset_closure(budget)).collect();
        match (paths.iter().filter(curved).count(), closures.as_slice()) {
            (0, []) if paths.iter().all(|c| c.composite().is_none()) => t[5] += 1,
            (0, []) => {}
            (0, cs) if cs.contains(&CosetClosure::Open) => t[4] += 1,
            (0, cs) if cs.contains(&CosetClosure::ClosedModuloHeld) => t[3] += 1,
            (0, cs) if cs.contains(&CosetClosure::Closed) => t[2] += 1,
            // The stroke spent before a coset's candidates were formed: unread
            // by name (rule 10).
            (0, _) => t[5] += 1,
            _ => {
                t[0] += 1;
                if paths.iter().filter(curved).any(|c| c.gauge_static()) {
                    t[1] += 1;
                }
            }
        }
    }
    t
}

/// WHAT A CONTACT'S EDGE IS (78.6): one transport, the held coset or the
/// static tether, or refused by the name its tally counted — with each
/// member's pose refusal where that was the refusal (CV1b F3), so the arm a
/// demand retires reaches the caller.
pub(crate) enum ContactEdge {
    Formed(Box<crate::transducer::sentence::Transport>),
    Held(crate::transducer::sentence::Refraction),
    Refused { poses: [Option<blade::NoPose>; 2] },
}

/// The two bodies a pair key names, read back as their addresses (A21).
/// `None` for bytes that are not two addresses exactly. The A21 property's
/// reader: the builder needs no read-back, because the pair's two sides ride
/// the gathered contact itself.
#[cfg(test)]
pub(crate) fn read_body_pair_key(k: &[u8]) -> Option<(coord::Piece, coord::Piece)> {
    // The first address's extent is read off its own layout by the piece's
    // one reader, never searched for and never restated here: the second
    // address starts where the first ends, and nothing follows it.
    let mut pos = 0usize;
    let a = coord::Piece::take(k, &mut pos)?;
    let b = coord::Piece::take(k, &mut pos)?;
    (pos == k.len()).then_some((a, b))
}

/// A CONTACT BETWEEN TWO BODIES, AS THE CORNER PASS GATHERS IT: the shared
/// boundary elements, the members standing at each side of them, and the
/// two bodies' pieces, the lesser first (BD2).
#[derive(Default)]
struct Gathered {
    elements: Vec<Site>,
    left: Vec<Vec<u8>>,
    right: Vec<Vec<u8>>,
    bodies: (Vec<u8>, Vec<u8>),
    nuclei: (Option<crate::transducer::cga::Mv>, Option<crate::transducer::cga::Mv>),
    /// The grade of the largest shared cell met so far.
    grade: Option<u8>,
}

/// ONE CONTACT, AS THE FACE BUILT ALONG IT READS IT: the two bodies' pieces,
/// the lesser first, their nuclei in the same order — the tether is theirs,
/// two BODIES' (78.18(c)), never a member's union — the members standing on
/// the lesser's side, so a face built between two members reads which body
/// stands at each of its ends (CX1); the grade of the contact's largest
/// shared cell; and the WITNESS, the shared boundary elements at their
/// doubled centres — WHETHER the bodies touch, read by the topology and
/// entering no address (BQ5b, A.14). The face carries the pair in hand; no
/// key of the witness stands for it.
#[derive(Clone, Debug)]
pub(crate) struct ContactPair {
    pub(crate) bodies: (Vec<u8>, Vec<u8>),
    pub(crate) left: Vec<Vec<u8>>,
    pub(crate) nuclei: (Option<crate::transducer::cga::Mv>, Option<crate::transducer::cga::Mv>),
    pub(crate) grade: u8,
    pub(crate) witness: Vec<Site>,
}

/// Faces the bundle names: star from the Frame (when the form carries one),
/// pairs whose supports share a site, and one pair for each two bodies whose
/// cube complexes share an edge or a corner, carrying the interface (70.6).
/// Occupancy is keyed by site; related pairs are a Keyed set.
///
/// COST IS THE SITES, NOT THE BODIES ON THEM. A site with k occupants
/// contributes k−1 faces, not k(k−1)/2 — see the chain below for why the
/// chords are not faces the complex is missing. Every face this writes is
/// counted through `note_face_work`, so the gauge in
/// `face_work_is_linear_in_occupied_sites` reads the whole cost and not the
/// part that was already linear.
///
/// The supports arrive KEYED BY MEMBER — a vertex AND the chart it stands at
/// there (D10) — which is the coordinate a face is actually between: 21 step 2
/// relates two BODIES, and a vertex covering several bodies on several charts is
/// several occupants of the sites they sit on. Keyed by the vertex alone, the
/// pairs this returns would name a relation between two vertices and leave
/// which of their bodies met to be guessed by whatever built the face.
///
/// The pairs are member pairs, so the face is created already carrying which
/// member of `i` relates to which member of `j`. NO PRODUCT OF TWO FIBRES is
/// taken anywhere: the occupancy is keyed by SITE and a site names its
/// occupants, so the chain over `windows(2)` yields incidences the scene has.
///
/// THE MEMBERS ARE READ FROM THE INDEX. `supports` IS the member index, and
/// iterating it is reading it (rule 1); a count of the integers with a keyed
/// read inside finds nothing for a vertex with no support and says nothing
/// about one the index holds and the count does not reach.
#[cfg(test)]
pub(crate) fn named_faces(
    frame: Option<&[u8]>,
    supports: &crate::network::nerve::Keyed<Vec<Site>>,
) -> Vec<NamedFace> {
    named_incidence(frame, supports).0
}

/// BQ5b — THE INTERFACE A CONTACT FACE CARRIES, WHOLE: `V = s_b s_a` between
/// the two bodies' nuclei, in the face's orientation, and the contact's
/// grade (`Interface::between`, the one producer). A contact whose nuclei —
/// or their tether's ray — did not read is a face still built on WHETHER
/// (A.14) with its WHAT unread: `(None, true)`, counted `unnucleated`. A face
/// that is no contact carries none: `(None, false)`.
pub(crate) fn interface_of(
    contact: Option<&ContactPair>,
    nuclei: Option<&(Option<crate::transducer::cga::Mv>, Option<crate::transducer::cga::Mv>)>,
) -> (Option<crate::transducer::sentence::Interface>, bool) {
    let Some(c) = contact else { return (None, false) };
    let interface = match nuclei {
        Some((Some(sa), Some(sb))) => crate::transducer::sentence::Interface::between(sa, sb, c.grade),
        _ => None,
    };
    let unread = interface.is_none();
    (interface, unread)
}

/// WHAT ONE READ OF THE OCCUPANCY NAMES: the faces, each contact on its
/// pair, and the contact complex's certified cells.
pub(crate) type SceneIncidence = (Vec<NamedFace>, crate::network::nerve::Keyed<crate::transducer::sentence::FormCell>);

/// THE FACES THE BUNDLE NAMES, each carrying its contacts, and the CONTACT
/// COMPLEX'S CERTIFIED CELLS, read on the same pass (NB1): one reading of the
/// occupancy, never two.
pub(crate) fn named_incidence(
    frame: Option<&[u8]>,
    supports: &crate::network::nerve::Keyed<Vec<Site>>,
) -> SceneIncidence {
    let mut related: crate::network::nerve::Keyed<Named> = crate::network::nerve::Keyed::new();
    if let Some(frame) = frame {
        for (mk, _) in supports.scoped() {
            if mk == frame {
                continue;
            }
            relate(&mut related, member_pair_key(frame, &mk), Incidence::Star, None);
        }
    }
    // Site → the site's own coordinates and its occupants. The coordinates
    // ride with the occupants so the corner pass below reads the sites the
    // occupancy already holds, never a second read of the supports.
    let mut occ: crate::network::nerve::Keyed<Occupied> = crate::network::nerve::Keyed::new();
    for (mk, supp) in supports.scoped() {
        if Some(mk.as_slice()) == frame {
            continue;
        }
        for s in supp {
            note_face_work();
            let k = s.key();
            match occ.get_mut(&k) {
                // The members ascend in key order and a member is offered its
                // own support once, so a repeat can only be the one just
                // pushed; a walk of the site's occupants on every offer would
                // be k² comparisons on the same site that the chain below is
                // already paying k² for.
                Some(o) => match o.members.as_slice() {
                    [.., last] if *last == mk => {}
                    _ => o.members.push(mk.clone()),
                },
                None => {
                    occ.insert(k, Occupied { site: s.clone(), members: vec![mk.clone()] });
                }
            }
        }
    }
    // A SITE'S OCCUPANTS ARE A SIMPLEX, NOT A CLIQUE. Every vertex on this
    // site meets every other ON IT, so the subcomplex they span is a simplex
    // — contractible, and every cycle inside it is a boundary. Writing all
    // k(k−1)/2 of those pairs as faces would hand `orbits_of` a cycle basis
    // of the 1-skeleton with nothing to fill k(k−1)/2 − (k−1) orbits that
    // bound by construction, and the pivot drive would clone the whole form
    // once per candidate against each one.
    //
    // The chain is that simplex's spanning tree: k−1 faces, the same
    // connectivity and the same components, and the only cycles that survive
    // are the ones spanning DISTINCT sites — which are the cycles that carry
    // a class. The occupants are read in the form's own vertex coordinate,
    // which is content order, so two minds holding one scene chain it alike.
    for (_k, o) in occ.scoped() {
        for w in o.members.windows(2) {
            let [a, b] = w else { continue };
            note_face_work();
            relate(&mut related, member_pair_key(a, b), Incidence::Site, None);
        }
    }
    // C1, C2, 70.6 — CONTACT IS THE SHARED BOUNDARY OF TWO CUBE COMPLEXES.
    // Two hue-blind bodies never share a face of a cube, since sharing one
    // they would be one body (70.4). Where two distinct bodies touch, their
    // complexes share a cell of lower grade. `contact_cells` reads every
    // such cell off the distinct occupied sites, in its own canonical order
    // (BQ2: the grade is the contact). The frame axis is among them, and
    // the reading is exact and integer, with no extent consulted and no band.
    //
    // THE BODIES ARE THE OCCUPANCY'S OWN COMPONENTS. The shared faces make
    // a body, so the occupied sites fall into bodies under
    // `face_incidence`. A boundary element between two sites of one
    // body is that body's interior and is not contact.
    //
    // ONE FACE PER BODY PAIR (C1). A corner names two sites, and each site
    // names its occupants. Relating every occupant of one to every occupant
    // of the other was the join of two occupancy simplices written as a
    // clique, the shape the site pass above refuses in its own comment. The
    // contact is between the two BODIES. It is gathered whole, and offered
    // once, between the least member standing at each side. That is the
    // form's own vertex coordinate, which is content order, as the chain
    // above reads a site's occupants. A body pair whose two sides have only
    // one member between them is that member's interior, and no face is
    // offered.
    //
    // THE INTERFACE IS A SET (C2). Each shared element is written as its
    // centre doubled, `p + q`, which is exact and integer and names the
    // element itself: two pairs of cubes sharing one edge name it once. The
    // parity says what the element is. An odd third coordinate is an element
    // across two frames. The set rides the face as its coordinates, never
    // as a tally.
    let held: Vec<&Occupied> = occ.scoped().map(|(_, o)| o).collect();
    let positions: Vec<Site> = held.iter().map(|o| o.site.clone()).collect();
    let body = crate::transducer::sentence::components(
        positions.len(),
        &crate::transducer::sentence::face_incidence(&positions),
    );
    // C3, 70.7 — A BODY IS NAMED BY ITS ADDRESS, NEVER BY A SITE. Each body
    // of the occupancy is written out once as its `coord::Piece` — its
    // support, the one producer `piece_at` — and a body pair is keyed by the
    // two addresses in canonical order (rule 6, law 11). It was keyed by the
    // two bodies' ROOT SITES, a position of the base standing for an
    // identity: the root is whichever site the union-find left on top, so
    // the key named a place in the pixel grid rather than the body.
    let mut support_of: Vec<Vec<Site>> = vec![Vec::new(); positions.len()];
    for (at, &root) in body.iter().enumerate() {
        if let (Some(s), Some(sup)) = (positions.get(at), support_of.get_mut(root)) {
            sup.push(s.clone());
        }
    }
    // Each body's nucleus, read once from its support (NB1): the cells below
    // carry the meet of their bodies' nuclei.
    let nucleus_of: Vec<Option<crate::transducer::cga::Mv>> = support_of
        .iter()
        .map(|sup| match sup.is_empty() {
            true => None,
            false => blade::shape_of(sup).blade.nucleus(),
        })
        .collect();
    let address_of: Vec<Vec<u8>> = support_of
        .into_iter()
        .map(|mut sup| {
            sup.sort_unstable();
            super::pieces::piece_at(crate::transducer::peel::ShellKind::Whole, &sup).encode().as_bytes().to_vec()
        })
        .collect();
    // NB1, 78.2 — THE CONTACT COMPLEX'S CELLS, read off the base's elements:
    // each element with the BODIES standing in it (a body is its union-find
    // root here, read once per element), one certified cell per standing
    // set of two or more, every element under exactly those bodies a
    // witness, and the meet of their nuclei as the cell's content. No pair
    // is traversed and no subset minted; the faces of a cell are read off it.
    let standing: Vec<(Site, Vec<usize>)> = crate::transducer::sentence::elements_standing(&positions)
        .into_iter()
        .map(|(element, at)| {
            let mut bodies: Vec<usize> = at.iter().filter_map(|&p| body.get(p).copied()).collect();
            bodies.sort_unstable();
            bodies.dedup();
            (element, bodies)
        })
        .collect();
    // KB1 — each body by its piece, the coordinate a body has: the cells are
    // keyed by their bodies' pieces, framed, in their own order.
    let over = crate::transducer::cech::cells_over(
        standing.iter().map(|(element, bodies)| {
            (crate::network::nerve::coord::Compartment { site: element.clone() }, bodies.as_slice())
        }),
        |m| address_of.get(m).cloned(),
        |members| {
            members
                .iter()
                .map(|&m| nucleus_of.get(m).cloned().flatten())
                .collect::<Option<Vec<crate::transducer::cga::Mv>>>()
                .and_then(|n| crate::transducer::cech::meet_of(&n))
        },
    );
    // 78.14, BD2 — EACH BODY BY ITS PIECE: its support, canonical, the
    // coordinate a body has. A member key names a vertex on a chart whose
    // support can fall into several bodies (measured on the corpus: keyed by
    // member keys alone, a 2-cell and two 1-cells of one form collided and
    // were lost), and a nucleus is the body's order-0 and order-1 reading,
    // a projection that two bodies can share. A piece is small (C3); the
    // cell holds its bodies' pieces, and `cells_over` has already keyed and
    // ordered it by them (KB1), the constructors riding in the same order.
    note_contact(|t| {
        t.cells_unkeyed += over.unkeyed;
        t.cells_singular += over.singular;
    });
    let mut cells: crate::network::nerve::Keyed<crate::transducer::sentence::FormCell> =
        crate::network::nerve::Keyed::new();
    for c in over.cells {
        let mut key: Vec<u8> = Vec::new();
        for piece in &c.coords {
            crate::transducer::address::put_key_bytes(&mut key, piece);
        }
        let nuclei: Option<Vec<crate::transducer::cga::Mv>> =
            c.members.iter().map(|&m| nucleus_of.get(m).cloned().flatten()).collect();
        let witnesses = c.witnesses.into_iter().map(|l| l.site).collect();
        cells.insert(&key, crate::transducer::sentence::FormCell { bodies: c.coords, witnesses, nuclei });
    }
    let mut contacts: crate::network::nerve::Keyed<Gathered> = crate::network::nerve::Keyed::new();
    for crate::transducer::sentence::SharedCell { i, j, grade, .. } in
        crate::transducer::sentence::contact_cells(&positions)
    {
        let (Some(&bi), Some(&bj)) = (body.get(i), body.get(j)) else { continue };
        if bi == bj {
            continue;
        }
        let (Some(ai), Some(aj)) = (address_of.get(bi), address_of.get(bj)) else { continue };
        // The body pair is keyed by its two bodies' addresses, the lesser
        // first, and the element's sides are read in the same orientation.
        let ((x, ax, bx), (y, ay, by)) = match ai <= aj {
            true => ((i, ai, bi), (j, aj, bj)),
            false => ((j, aj, bj), (i, ai, bi)),
        };
        let (Some(px), Some(py)) = (held.get(x), held.get(y)) else { continue };
        note_face_work();
        let k = body_pair_key(ax, ay);
        // The element's centre doubled, `p + q`: exact, integer, and a site
        // of the same base.
        let Some(element) = px.site.offset(py.site.axes()) else { continue };
        if contacts.get(&k).is_none() {
            contacts.insert(
                k.clone(),
                Gathered {
                    bodies: (ax.clone(), ay.clone()),
                    nuclei: (nucleus_of.get(bx).cloned().flatten(), nucleus_of.get(by).cloned().flatten()),
                    ..Gathered::default()
                },
            );
        }
        let Some(g) = contacts.get_mut(&k) else { continue };
        g.elements.push(element);
        g.left.extend(px.members.iter().cloned());
        g.right.extend(py.members.iter().cloned());
        // The contact's grade is its largest shared cell's: a cell of a
        // higher grade met here raises it.
        let grade = u8::try_from(grade).ok();
        if let Some(gr) = grade {
            match g.grade {
                Some(have) if have >= gr => {}
                _ => g.grade = Some(gr),
            }
        }
    }
    for (_k, g) in contacts.scoped() {
        let mut elements = g.elements.clone();
        elements.sort_unstable();
        elements.dedup();
        let (mut left, mut right) = (g.left.clone(), g.right.clone());
        left.sort_unstable();
        left.dedup();
        right.sort_unstable();
        right.dedup();
        // The least member at each side; where it is one member, the next
        // on the other side stands for it. A member meeting only itself is
        // its interior.
        let ends = match (left.as_slice(), right.as_slice()) {
            ([a, ..], [b, ..]) if a != b => Some((a, b)),
            ([a, ..], [_, b, ..]) | ([_, b, ..], [a, ..]) => Some((a, b)),
            _ => None,
        };
        let (Some((a, b)), Some(grade)) = (ends, g.grade) else { continue };
        // The contact rides the pair it names: the face built along it has
        // its body pair in hand (BQ5b — BD2's key of the witness is gone).
        let contact =
            ContactPair { bodies: g.bodies.clone(), left: left.clone(), nuclei: g.nuclei.clone(), grade, witness: elements };
        relate(&mut related, member_pair_key(a, b), Incidence::Contact, Some(contact));
    }
    // THE CONDENSATION FIELD IS GONE — BQ5a, SQ1 item 2 (72.2, 75). A pair
    // loop over every two members stood here, the tether's scalar split
    // gated by a dialled band `1 < δ ≤ 2`, then the pose. Whether two bodies
    // touch is the topology's (the contact pass above, A.14); what they
    // carry is read on exactly those pairs, in `contact_edge`, where the
    // face is built and each member's transition is in hand. No band
    // decides anything, and no pair loop runs.

    // THE RING IS GONE — T1, THE LAST INCIDENCE STENCIL (60.5, ruled
    // 2026-09-22). A third pass stood here: each member's support dilated by
    // `ring_of_support`, a Manhattan 4-neighbourhood on ℤ², and every member
    // standing in the dilation related to it.
    //
    // 58.3 purged the same call from the cover's meet index and the corpus
    // went from 186 meets to 2, from 184 determined frames to 0. It did not
    // move the defect spectrum, because the FORM's faces were built here and
    // this dilation still ran: `b₁`, `orbit_defects`, the spectra, and the
    // `non-simple 3` that 59.7 named as the corpus's only luminal candidates
    // were all readings of a second stencil. A stenciled screw is not chiral
    // torsion between two bodies — it is a raster array inflated until two
    // disjoint regions collide, minting a synthetic 1-cycle that registers
    // as strained precisely because it should not exist.
    //
    // 58.3's own argument, one level up: a complex cannot have its incidence
    // evaluated against two topologies, and a stencil is not one of them.
    // What this pass was reaching for is real — two adjacent bodies DO
    // relate — but contact is not a shared coordinate. It is the conformal
    // strain between their spheres (59.3), condensed where the mind can
    // spell the interface (59.6), and it arrives through the tether and not
    // through a dilation. Until it does, this incidence is ABSENT and the
    // absence is the reading (rule 11).
    //
    // `ring_of_support` has no caller left and is deleted with this pass, so
    // the class cannot silently return.
    let faces = related
        .scoped()
        .filter_map(|(k, named)| {
            read_member_pair_key(&k).map(|(a, b)| NamedFace { a, b, named: named.clone() })
        })
        .collect();
    (faces, cells)
}

/// Physical size of the hue carrier. Index by hue, never search.
pub const PALETTE: usize = 256;
pub type HueMap = [Option<u8>; PALETTE];

pub(crate) fn hue_map_from_pairs(pairs: &[(u8, u8)]) -> HueMap {
    let mut m = [None; PALETTE];
    for &(f, t) in pairs {
        m[f as usize] = Some(t);
    }
    m
}
