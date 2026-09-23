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

/// T4 — WHAT THE CONDENSATION FIELD DID, COUNTED AT THE STEP THAT DECIDED
/// (§8.1, 66).
///
/// The first census read `0 condensed` over the whole corpus, and a zero is
/// not a reading: "every gate refused" and "the pass never ran" are
/// indistinguishable from the outside, which is the vacuous-truth defect
/// exactly. Each gate counts its own refusals, so the zero says WHICH.
///
/// `offered` is every ordered pair the field evaluated. `unlifted` is a
/// pair whose nuclei did not read back. The three gates then refuse in
/// order, and `condensed` is what reached `C¹`.
#[derive(Default, Clone, Copy, Debug)]
pub struct FieldTally {
    pub offered: u64,
    pub unlifted: u64,
    /// THE METRIC REFUSAL, SPLIT BY POLE. `|δ| ≤ 2` is antipodally
    /// symmetric and a single `off_horizon` counter is NOT: it fused
    /// `Apart` (`δ > 2`, the bodies are too far) with `Contained`
    /// (`δ < −2`, one is nested deep inside the other), which are opposite
    /// diagnoses with opposite repairs. `Contact`'s five arms exist
    /// precisely so the involution's two poles stay apart (65.2), and
    /// fusing them here threw that away at the measurement.
    pub apart: u64,
    pub contained: u64,
    pub sheared: u64,
    /// THE TWO WAYS A FRAME IS NOT A DETERMINED TURN, kept apart because
    /// they are different facts and one count reports them as one.
    ///
    /// `unturned` is `relative_rotor` answering NONE: no rotor exists at
    /// all, because the two bodies' harmonic towers do not correspond and
    /// a turn between them is not defined. `half_open` is `HalfOpen`: a
    /// rotor DOES exist as a projective class and only its parity is
    /// undetermined, which is what `m = 3` would resolve.
    ///
    /// Lumped together they said "the parity gap blocks everything" when
    /// the reading may be "no relative frame exists between dissimilar
    /// bodies at all" — a far larger claim, and a different repair.
    /// THE SIX WAYS NO FRAME IS DETERMINED, each at its own name. The
    /// census read `89 unturned` under one counter and could not say
    /// whether the corpus is full of UNSPELLABLE TURNS — 37.7's condition,
    /// live — or of bodies that do not orient at all.
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
    /// Proved unlike: no element of `D₄` carries the pair at all. Also
    /// permanent, and kept apart from `mirrored` because a corpus of
    /// handed pairs and a corpus of unlike ones want different answers.
    pub incongruent: u64,
    pub half_open: u64,
    pub condensed: u64,
}

thread_local! {
    static FIELD: Cell<FieldTally> = const { Cell::new(FieldTally {
        offered: 0, unlifted: 0, apart: 0, contained: 0, sheared: 0,
        bodiless: 0, isotropy: 0, unoriented: 0, oblique: 0, mirrored: 0,
        incongruent: 0, half_open: 0, condensed: 0,
    }) };
}

pub(crate) fn note_field(f: impl Fn(&mut FieldTally)) {
    FIELD.with(|c| {
        let mut v = c.get();
        f(&mut v);
        c.set(v);
    });
}

/// Read the field's tally and clear it, as the cover's is taken.
pub fn take_field_tally() -> FieldTally {
    FIELD.with(|c| c.replace(FieldTally::default()))
}

/// THE FIELD WITHOUT CLEARING IT, for a reader that prints per form while
/// a whole-run total is still accumulating. Two readers of one counter,
/// and only one of them may take it (law 12: the take is the clear).
pub fn field_tally() -> FieldTally {
    FIELD.with(|c| c.get())
}

impl FieldTally {
    /// THE VALVE LINE — every gate, in the order the field passes them, so
    /// a reader sees WHERE the flow stops rather than only that it did.
    ///
    /// A valve that is starved and a valve that is closed look the same on
    /// a total; they do not here. `offered` is what arrived, each named
    /// count is what one valve turned away, and `condensed` is what came
    /// out the far end. The seven frame arms are the six `NoFrame`
    /// refusals plus the parity class, because a refusal that does not say
    /// which refusal it is answers nothing (rule 11).
    pub fn line(&self) -> String {
        format!(
            "offered {} unlifted {} | horizon: apart {} contained {} \
             | frame: bodiless {} isotropy {} unoriented {} oblique {} \
             mirrored {} incongruent {} half-open {} | CONDENSED {}",
            self.offered,
            self.unlifted,
            self.apart,
            self.contained,
            self.bodiless,
            self.isotropy,
            self.unoriented,
            self.oblique,
            self.mirrored,
            self.incongruent,
            self.half_open,
            self.condensed,
        )
    }

    /// WHAT A DEMAND COULD RETIRE, against what it never will. `oblique`
    /// is the mutable half — a proper turn the alphabet is merely short of
    /// — and the two permanent arms are the geometry's own answer. The
    /// ratio is the one number that says whether adjoining generators can
    /// move this corpus at all.
    pub fn mutable_over_permanent(&self) -> (u64, u64) {
        (self.oblique, self.mirrored + self.incongruent)
    }

    /// WHAT THE VALVE PASSED, AGAINST WHAT REACHED IT. A field that
    /// refuses everything and a field nothing reached both read `condensed
    /// 0`, and the pair tells them apart.
    ///
    /// The three arms are exclusive by construction, so they never sum past
    /// `offered`. Where they do, the instrument contradicts itself, and that
    /// reads `None` — a refusal — never a zero floored in its place
    /// (rule 11).
    pub fn reached_contact(&self) -> Option<u64> {
        self.offered
            .checked_sub(self.unlifted)?
            .checked_sub(self.apart)?
            .checked_sub(self.contained)
    }
}

/// WHICH INCIDENCE NAMED A PAIR (70.4): the Frame's star, a site two
/// members share, a corner two bodies meet at, or the condensation field.
/// Carried with the pair so the face built from it is counted where it came
/// from; a pair two incidences name is one face carrying both (N1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Incidence {
    Star,
    Site,
    Contact,
    Field,
}

/// A SITE OF THE OCCUPANCY: its own coordinates, and the members standing
/// on it in the order they were offered.
struct Occupied {
    site: Site,
    members: Vec<Vec<u8>>,
}

/// WHAT THE INCIDENCE NAMED BETWEEN TWO MEMBERS: every incidence that named
/// the pair (N1), and each interface a contact between them runs along
/// (C2, 70.6). The interfaces are sets of shared boundary elements. Two
/// different ones between one pair are two contacts, so they are held
/// apart and never merged into one set.
#[derive(Clone, Debug, Default)]
pub(crate) struct Named {
    pub(crate) hows: Vec<Incidence>,
    pub(crate) interfaces: Vec<Vec<Site>>,
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
    interface: Option<Vec<Site>>,
) {
    if related.get(&key).is_none() {
        related.insert(key.clone(), Named::default());
    }
    let Some(named) = related.get_mut(&key) else { return };
    if let Err(at) = named.hows.binary_search(&how) {
        named.hows.insert(at, how);
    }
    if let Some(i) = interface {
        if let Err(at) = named.interfaces.binary_search(&i) {
            named.interfaces.insert(at, i);
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

/// The two bodies a pair key names, read back as their addresses (A21).
/// `None` for bytes that are not two addresses exactly. The A21 property's
/// reader: the builder needs no read-back, because the pair's two sides ride
/// the gathered contact itself.
#[cfg(test)]
pub(crate) fn read_body_pair_key(k: &[u8]) -> Option<(coord::Piece, coord::Piece)> {
    // The first address's extent is read off its own layout — its tag, its
    // kind, its count of sites, and each site's own `n` — never searched
    // for: the second address starts where the first ends.
    let mut pos = 2usize;
    let count = u32::from_le_bytes(k.get(pos..pos + 4)?.try_into().ok()?);
    pos += 4;
    for _ in 0..count {
        crate::transducer::base::Site::take(k, &mut pos)?;
    }
    let (a, b) = k.split_at(pos);
    Some((coord::Piece::decode(a).ok()?, coord::Piece::decode(b).ok()?))
}

/// A CONTACT BETWEEN TWO BODIES, AS THE CORNER PASS GATHERS IT: the shared
/// boundary elements, and the members standing at each side of them.
#[derive(Default)]
struct Gathered {
    elements: Vec<Site>,
    left: Vec<Vec<u8>>,
    right: Vec<Vec<u8>>,
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
/// walking it is reading it (rule 1); a count of the integers with a keyed
/// read inside finds nothing for a vertex with no support and says nothing
/// about one the index holds and the count does not reach.
pub(crate) fn named_faces(
    frame: Option<&[u8]>,
    supports: &crate::network::nerve::Keyed<Vec<Site>>,
) -> Vec<NamedFace> {
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
    // occupancy already holds, never a second walk of the supports.
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
    let address_of: Vec<Vec<u8>> = support_of
        .into_iter()
        .map(|mut sup| {
            sup.sort_unstable();
            super::pieces::piece_at(crate::transducer::peel::ShellKind::Whole, &sup).encode().as_bytes().to_vec()
        })
        .collect();
    let mut contacts: crate::network::nerve::Keyed<Gathered> = crate::network::nerve::Keyed::new();
    for crate::transducer::sentence::SharedCell { i, j, .. } in
        crate::transducer::sentence::contact_cells(&positions)
    {
        let (Some(&bi), Some(&bj)) = (body.get(i), body.get(j)) else { continue };
        if bi == bj {
            continue;
        }
        let (Some(ai), Some(aj)) = (address_of.get(bi), address_of.get(bj)) else { continue };
        // The body pair is keyed by its two bodies' addresses, the lesser
        // first, and the element's sides are read in the same orientation.
        let ((x, ax), (y, ay)) = match ai <= aj {
            true => ((i, ai), (j, aj)),
            false => ((j, aj), (i, ai)),
        };
        let (Some(px), Some(py)) = (held.get(x), held.get(y)) else { continue };
        note_face_work();
        let k = body_pair_key(ax, ay);
        // The element's centre doubled, `p + q`: exact, integer, and a site
        // of the same base.
        let Some(element) = px.site.offset(py.site.axes()) else { continue };
        if contacts.get(&k).is_none() {
            contacts.insert(k.clone(), Gathered::default());
        }
        let Some(g) = contacts.get_mut(&k) else { continue };
        g.elements.push(element);
        g.left.extend(px.members.iter().cloned());
        g.right.extend(py.members.iter().cloned());
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
        let Some((a, b)) = ends else { continue };
        relate(&mut related, member_pair_key(a, b), Incidence::Contact, Some(elements));
    }
    // T4 — THE CONDENSATION PASS, WHERE THE STENCIL WAS (66, ruled
    // 2026-09-22). Incidence between two bodies is the exact conformal
    // strain between their nuclei, gated by three certificates and never by
    // a dilation of coordinates.
    //
    // THE METRIC GATE IS DERIVED, NOT DIALLED. Two supports sharing an EDGE
    // read `δ² = 4` exactly, at every size — an n×n block has `r² = n²/6`,
    // so `num = 2n²/3`, `den2 = n⁴/9`, `δ² = 4` independent of `n`, because
    // `δ` is a similarity invariant and adjacent n-blocks are similar to
    // adjacent unit cells. A corner neighbour reads 25 and a one-cell gap
    // reads 121. So `δ² ≤ 4` admits edge adjacency and refuses corner
    // adjacency, which is 51.3's own four-neighbour convention ARRIVING
    // FROM THE METRIC instead of from a stencil.
    //
    // AND IT IS WRITTEN ANTIPODALLY. `num² ≤ 4·den2` is `|δ| ≤ 2`, which
    // the involution `σ: s ↦ −s` fixes. Writing it as `num ≤ 0 ∨ num² ≤
    // 4·den2` would admit arbitrarily deep containment while refusing its
    // antipode — a half-spectrum silently admitted, which is the same shape
    // as the three-arm `Contact` collapse (65.1).
    // THE BLADES ARE LIFTED ONCE PER MEMBER, not once per pair. `shape_of`
    // integrates every lamina of a support, so computing it inside the pair
    // loop would lift each body `N` times — `O(N²·cells)` where `O(N·cells)`
    // is the whole cost. The field is a pair loop; the LIFT is not.
    let bodies: Vec<(Vec<u8>, blade::Blade)> = supports
        .scoped()
        .filter(|(mk, _)| Some(mk.as_slice()) != frame)
        .map(|(mk, supp)| (mk, blade::shape_of(supp).blade))
        .collect();
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            // ORDERED, so each pair is read once and the reading does not
            // depend on the order the members were offered.
            let ((ak, ba), (bk, bb)) = (&bodies[i], &bodies[j]);
            // THE FIELD'S COST IS NOT FACE WORK. `note_face_work` gauges
            // the OCCUPANCY-driven builder, whose whole claim is that it
            // is linear in occupied sites; the field is a pair loop over
            // members and counting it there fused two costs and broke the
            // gauge (36 against 528). `FieldTally::offered` is the field's
            // own carrier (law 12).
            note_field(|t| t.offered += 1);
            let Some(face) = blade::Interface::between(ba, bb) else {
                note_field(|t| t.unlifted += 1);
                continue;
            };
            // (1) THE METRIC GATE, antipodally symmetric.
            // THE CONTACT BAND IS `1 < δ ≤ 2`, NOT `|δ| ≤ 2`.
            //
            // `δ ≤ 1` is OVERLAP — the two bodies share support — and that
            // incidence already has a producer: the occupancy pass above,
            // which relates a site's occupants in a spanning CHAIN
            // precisely because they span a simplex and writing all
            // `k(k−1)/2` pairs would hand `orbits_of` a cycle basis with
            // nothing to fill. Admitting overlap here rebuilt exactly that
            // clique: eight bodies on one site condensed 28 faces where
            // the chain writes 7 (law 12, two producers of one fact).
            //
            // So the two operators divide by regime, as ruled: overlap is
            // the occupancy pass's, `1 < δ ≤ 2` is the field's, and it is
            // the band the ring stencil used to cover — genuine contact
            // WITHOUT shared support.
            //
            // AND THE BAND IS ONE-SIDED ON PURPOSE. Its antipode under
            // `σ` is `−2 ≤ δ < −1`, which is nesting, a DIFFERENT relation
            // wanting a different operator (65.3). The involution is
            // honoured by every regime having an operator, not by one gate
            // swallowing both poles.
            let four = crate::core::Q::from_integer(4.into());
            let sq = &face.strain.num * &face.strain.num;
            let adjacent = face.strain.num > crate::core::Q::from_integer(0.into())
                && sq > face.strain.den2
                && sq <= four * &face.strain.den2;
            if !adjacent {
                // WHICH POLE. `Apart` is `δ > 2` — too far to touch;
                // anything at or below `δ = 1` is overlap or nesting and
                // belongs to another producer.
                match face.strain.side() {
                    blade::Contact::Apart => note_field(|t| t.apart += 1),
                    _ => note_field(|t| t.contained += 1),
                }
                continue;
            }
            // THE HARMONIC CERTIFICATE STOOD HERE AND WAS IN THE WRONG SEAT
            // (66, measured 2026-09-22). It asked whether the two bodies
            // are DILATES of one another, and the census answered without
            // ambiguity:
            //
            //     308 offered, 0 unlifted, 190 off-horizon,
            //     118 SHEARED, 0 half-open, 0 condensed
            //
            // Every one of the 118 pairs that the metric gate found in
            // contact was then refused for not being a scaled copy of its
            // neighbour. In an ARC scene adjacent bodies are FUNCTIONAL
            // COMPLEMENTS — a line meeting a frame, an agent against an
            // obstacle, an L abutting a block — and are almost never
            // homothetic. The gate was requiring two touching puzzle pieces
            // to be the same shape before permitting them to touch.
            //
            // `dilation_certified` is an equivalence test for
            // REPRESENTATIONAL BLOAT — two cells that are one thing said
            // twice — and it belongs to the quotient operator, gated on a
            // free face, not to contact. A square may touch a rectangle.
            //
            // (2) THE PARITY GATE. A 1-cell may not enter `C¹` carrying a
            //     `±1` gauge ambiguity: `Frame::HalfOpen` is a projective
            //     class and not a determined rotor, and admitting it would
            //     let an unoriented reflection pass without paying. It
            //     refuses until the odd harmonic `m = 3` exists to resolve
            //     the half turn (57.3, 65.6).
            // On the sensor's plane, named (BQ4); FR1 reads the pose.
            match blade::relative_rotor(ba, bb, crate::transducer::base::GRID_PLANE) {
                Ok(blade::Frame::Turn(_)) => {}
                // A ROTOR EXISTS AND ITS PARITY DOES NOT. This is the
                // `m = 3` gap: a projective class, not a determined turn.
                Ok(blade::Frame::HalfOpen(_)) => {
                    note_field(|t| t.half_open += 1);
                    continue;
                }
                // AND THE FIVE REFUSALS, EACH AT ITS OWN NAME. `Oblique`
                // in particular is NOT a failure: the relative orientation
                // exists and the alphabet cannot spell it, which is 37.7.
                Err(why) => {
                    note_field(|t| match why {
                        blade::NoFrame::Bodiless => t.bodiless += 1,
                        blade::NoFrame::Isotropy => t.isotropy += 1,
                        blade::NoFrame::Unoriented => t.unoriented += 1,
                        blade::NoFrame::Oblique => t.oblique += 1,
                        blade::NoFrame::Mirrored => t.mirrored += 1,
                        blade::NoFrame::Incongruent => t.incongruent += 1,
                    });
                    continue;
                }
            }
            note_field(|t| t.condensed += 1);
            relate(&mut related, member_pair_key(ak, bk), Incidence::Field, None);
        }
    }

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
    related
        .scoped()
        .filter_map(|(k, named)| {
            read_member_pair_key(&k).map(|(a, b)| NamedFace { a, b, named: named.clone() })
        })
        .collect()
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
