// 18.2 — THE FORM: vertices, faces, orbits, and the reads that traverse them.
//
//! A face is constant in the word between its two vertices; an orbit is a
//! cycle of faces; the holonomy of an orbit is the composite around it. All
//! three are structure, traversed and never searched.

use super::*;

impl Form {
    /// THE WORDS A SENTENCE IS MADE OF — one holonomy per orbit, with the
    /// class it belongs to.
    ///
    /// A sentence has no single spelling: it is a NERVE of germs whose
    /// vertices carry words and whose faces carry the conjugations between
    /// them (18.2). Asking it for `Element::letters` is a level error —
    /// letters spell a word, and a sentence is one level above a word. What
    /// it does have is a word per orbit, its holonomy, already composed on
    /// the carrier, and the class that word belongs to.
    ///
    /// Order is a CONJUGACY invariant (15.2, 15.3): "closes in p steps" is a
    /// fact about the class, never about the spelling, which is why the
    /// class travels beside the word here and not the tape.
    /// ONE ENTRY PER MEMBER PATH, carrying the holonomy that path actually
    /// has: the edge composite where it has one, the orbit's own word where it
    /// does not — `holonomy_words_on`, the same carrier the filer reads.
    ///
    /// THIS RETURNED `lp.holonomy` AND IT IS THE COURT'S METABOLISM THAT READ
    /// IT. `germinate/listing.rs` charges the physics off this: an orbit of
    /// FINITE order raises pressure by its period, and an orbit of INFINITE
    /// order charges TORSION — "what will not close". A face's word is the
    /// coboundary `w_j ∘ w_i⁻¹` and a product of coboundaries around a cycle
    /// telescopes to the identity, so every orbit read `Finite(1)`, every
    /// sentence raised one unit of pressure per orbit, and the branch that
    /// charges torsion was unreachable by construction.
    ///
    /// So `Physics.torsion` stayed `[0, 0, 0, 0]`, `order_flux` was
    /// `syntropic − 0`, and `yield_surface` — the per-lane dwell a reply
    /// carries — never saw a cycle. Measured at 7582ada: 196 mathematical
    /// torsion cycles filed by the voices, 0 charged on any court.
    ///
    /// D1 put the FILER on the true carrier and did not reach here; this is
    /// the same defect at the metabolism's own reader.
    pub fn orbit_words(&self) -> Vec<(crate::transducer::alphabet::Word, Option<ClassVector>)> {
        let mut out = Vec::new();
        for i in 0..self.orbits.len() {
            let Some(lp) = self.orbits.get(i) else {
                continue;
            };
            let cvs = self.class_vectors(i);
            // One holonomy per member path, each its own obstruction, paired
            // with that path's own class vector rather than the orbit's
            // collapsed one.
            let ws = crate::transducer::sentence::pivot::drive::holonomy_words_on(
                &self.faces,
                &self.vertices,
                lp,
            );
            for (k, w) in ws.scoped() {
                out.push((w.clone(), cvs.get(&k).cloned()));
            }
        }
        out
    }
}

/// How an unfolding stops. The step count is an instance coordinate, never
/// stored on the form. A ring stops at the vertex it is anchored on, named
/// by that vertex's coordinate (AT1), never by its place in a list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Measure {
    Occupied,
    Remainder,
    Ring { anchor: Box<coord::VertexAt> },
}

/// A FILED ORBIT: a torsion cycle of a FORM, keyed by the class of the orbit
/// that carries it. The same three members as an element's `TorsionCycle`
/// with one difference that matters — the third is the CHORD the orbit closes
/// on, an index into `faces`, not a fibre index. As one tuple the two were
/// interchangeable by type and only ever kept apart by which file was being
/// read.
///
/// THERE IS NO CONVERSION BETWEEN THEM, and the third coordinate is why: a
/// chord is a face of this form and a fibre index is a stratum of an element's
/// axis, so carrying one across as the other means inventing a coordinate the
/// source never held. `Element::adopt_torsion` did exactly that, writing
/// `FIBER_SPATIAL` in the chord's place, and is deleted. A form's cycles are
/// read where they are filed.
/// THE TWO-PLANE SPECTRUM OF A HOLONOMY: one period per invariant plane
/// of the double rotation, `None` where that factor does not close.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToralPeriod {
    pub plane_1: Option<std::num::NonZeroU32>,
    pub plane_2: Option<std::num::NonZeroU32>,
}

impl ToralPeriod {
    /// NEITHER PLANE CLOSES — the orbit is dense on the 2-torus and the
    /// residue is intrinsic torsion. A19 spelled this `0` on the class
    /// ledger; the type says it now, so a reader cannot mistake it for a
    /// period of no extent (rule 11).
    pub const OPEN: ToralPeriod = ToralPeriod { plane_1: None, plane_2: None };

    /// Both planes close, at the same period: an ISOCLINIC rotation, the
    /// Clifford translation of `S³`. Its stabiliser and its fixed-point set
    /// are not a planar rotation's, which is the distinction `lcm` erased.
    pub fn isoclinic(&self) -> Option<std::num::NonZeroU32> {
        match (self.plane_1, self.plane_2) {
            (Some(a), Some(b)) if a == b => Some(a),
            _ => None,
        }
    }

    /// Neither plane turns forever, so the orbit closes — and the pair is
    /// its beat, not one number.
    pub fn closes(&self) -> bool {
        self.plane_1.is_some() && self.plane_2.is_some()
    }

    /// THE YIELD ACROSS THE ACTUAL DEGREES OF FREEDOM: `(ℓ₁ − 1) + (ℓ₂ − 1)`.
    /// The court charged `(ℓ − 1)` on the `lcm`, which is neither plane's
    /// labour. `None` where a plane does not close: that is torsion and is
    /// charged as shear, never estimated through an invented finite order
    /// (rule 11).
    /// BOTH PLANES ON THE RECORD, each as a `u32` with `0` for the factor
    /// that does not close — A19's spelling, kept on the wire and made a
    /// type in memory. Written once here so no codec re-states it (law 12).
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        let of = |n: Option<std::num::NonZeroU32>| n.map_or(0u32, std::num::NonZeroU32::get);
        out.extend_from_slice(&of(self.plane_1).to_le_bytes());
        out.extend_from_slice(&of(self.plane_2).to_le_bytes());
    }

    pub fn from_pair(a: u32, b: u32) -> ToralPeriod {
        ToralPeriod {
            plane_1: std::num::NonZeroU32::new(a),
            plane_2: std::num::NonZeroU32::new(b),
        }
    }

    pub fn yield_units(&self) -> Option<u64> {
        let (a, b) = (self.plane_1?, self.plane_2?);
        Some(u64::from(a.get() - 1) + u64::from(b.get() - 1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiledOrbit {
    /// The hue-map generator: the permutation one step around the orbit.
    pub sigma: Vec<(u8, u8)>,
    /// THE TWO PLANE PERIODS OF THE HOLONOMY (54.7, as ruled 2026-09-21).
    ///
    /// It was `period: u32`, the `lcm` of the two, and the collapse
    /// destroys the eigenspace decomposition at the moment a cycle reaches
    /// the classifier. `Spin(4)` splits by the central idempotents
    /// `(1 ± I)/2` into two commuting quaternion factors, so
    ///
    /// ```text
    ///     Ω = exp(½θ₁B₁ + ½θ₂B₂) ,   B₁B₂ = B₂B₁ = I₄
    /// ```
    ///
    /// acts on two orthogonal invariant planes with their own periods.
    /// `lcm` throws three things away at once:
    ///
    ///   EIGENSPACE — a simple planar rotation `[ℓ, 1]` and an isoclinic
    ///   Clifford translation `[ℓ, ℓ]` both give `lcm = ℓ` and land in one
    ///   congruence class, with different stabilisers, different fixed
    ///   points and different actions.
    ///
    ///   INCOMMENSURABILITY — where `ℓ₁/ℓ₂ ∉ ℚ` the orbit is dense on the
    ///   2-torus and never closes. That is intrinsic torsion, and an
    ///   integer `lcm` reports it as a closed periodic orbit.
    ///
    ///   RESONANCE — commensurate periods beat at `|ℓ₁ − ℓ₂|⁻¹` under an
    ///   envelope of `lcm(ℓ₁, ℓ₂)`. Storing the envelope alone discards the
    ///   beat, and two orbits cannot resonate because no reader can compare
    ///   their harmonic ratio.
    ///
    /// `None` on a plane is infinite order — that factor turns forever —
    /// which A19 spelled as `0` on the class ledger and which is now said
    /// by the type (rule 11: a refusal is not a zero).
    ///
    /// IT LANDS INSIDE THE B6/B7 BOUNDARY. B6 already severed the sentence
    /// tape and moved the address to the head of the GEN3 record, so the
    /// recut is required anyway; leaving this scalar would persist collapsed
    /// periods into the fresh genome and demand a second recut to undo.
    pub periods: ToralPeriod,
    /// WHAT IT CLOSES ON (AT1, ruled 2026-09-25): the overlap of this form
    /// its cycle closes on, by its two charts, or the fibre's puncture.
    pub chord: Closes,
}

/// WHERE A FILED CYCLE CLOSES (AT1, ruled 2026-09-25). A cycle of the base
/// closes ON an overlap of the form, its chord. A fibre cycle — a bundle
/// connection's monodromy, `σ^ℓ = id` along the sections — winds the
/// PUNCTURE and closes on no overlap of the form; none is invented for it
/// (the slot era's `chord: 0` named a real face it did not close on). Its
/// own coordinate is its winding over the fibre's hole, WN1's.
///
/// Two arms, never an `Option` whose `None` must be read for its meaning:
/// the address writes the arm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Closes {
    On(coord::Overlap),
    Fibre,
}

impl Closes {
    /// The overlap it closes on, where it closes on one.
    pub fn overlap(&self) -> Option<&coord::Overlap> {
        match self {
            Closes::On(o) => Some(o),
            Closes::Fibre => None,
        }
    }

    /// The arm, written: `0 ‖ framed(overlap)` or `1`.
    pub fn put(&self, out: &mut Vec<u8>) {
        use crate::network::nerve::Coordinate;
        match self {
            Closes::On(o) => {
                out.push(0);
                crate::transducer::address::put_key_bytes(out, o.encode().as_bytes());
            }
            Closes::Fibre => out.push(1),
        }
    }

    /// The arm, read back; `None` on a tag or an overlap that does not read.
    pub fn take(b: &[u8], pos: &mut usize) -> Option<Closes> {
        use crate::network::nerve::Coordinate;
        let tag = *b.get(*pos)?;
        *pos += 1;
        match tag {
            0 => {
                let bytes = crate::transducer::address::take_key_bytes(b, pos)?;
                Some(Closes::On(coord::Overlap::decode(&bytes).ok()?))
            }
            1 => Some(Closes::Fibre),
            _ => None,
        }
    }
}

/// ONE CHART A WORD VERTEX ACTS ON, and what it acts by there: the A4
/// carriage that places it and the gauge that recolours it (D7).
///
/// The chart rides the member as well as keying it, which is how every
/// keyed carrier in this tree holds a coordinate that a reader needs back
/// — `finish`'s `by_demand` holds `(kind, chart, links)` under
/// `demand_key(kind, chart)` for the same reason. `Stalks::fiber` takes an
/// `Invariant` and not a key, so the member must carry the chart it is
/// filed under or the reader cannot ask the atlas anything.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Acting {
    /// The section coordinate naming the bodies this member speaks to.
    pub chart: stalk::Invariant,
    /// A4 carriage. Boxed: a versor is 1024 bytes and a member that holds
    /// one should not make every other vertex of a form carry that
    /// footprint.
    pub carriage: Box<Versor>,
    pub gauge: Vec<(u8, u8)>,
    /// AT1 (ruled) — THE FRAMES THIS MEMBER STANDS ON, each by its own
    /// spelling ([`frame_key`]): the member's charts are `class ‖ chart ‖
    /// frame` for each, held HERE and nowhere beside it. `Form.sections`
    /// read these off the members and retired as a carrier (law 12).
    pub frames: Keyed<()>,
}

impl Acting {
    /// DOES THIS MEMBER STAND AT FRAME `k` (AT1, ruled: a member holds its
    /// frames)? A member holding no frames is claimed whole along `e₃`, as the
    /// court has always read a member with no section (51.8).
    pub fn stands_at(&self, k: u64) -> bool {
        self.frames.is_empty() || self.frames.contains(&frame_key(k))
    }

    /// THE KEY A SECTION IS FILED UNDER: its chart AND the carriage it acts
    /// by (54.7 R1, as ruled 2026-09-21).
    ///
    /// Keyed by chart alone this held at most ONE section per chart and
    /// silently overwrote the second, so a vertex acting twice on one chart
    /// with two carriages kept one. That collapses the covering space: 53.6
    /// item 1 quotients two pieces of one rotor class to one base vertex
    /// `V ∈ B⁽⁰⁾` — correctly, so shape correspondence survives a change of
    /// frame — and 53.6 item 3 puts their meets in the TOTAL space, between
    /// `σ₁ = (V,U,φ₁)` and `σ₂ = (V,U,φ₂)` of the fibre `π⁻¹(V)`. With one
    /// section per chart the fibre is a point, `E` is `B`, and 18.2's own
    /// sentence — two carriages on one class are two sections — is not
    /// reachable through its own carrier.
    ///
    /// THE CHART LEADS AND IS LENGTH-PREFIXED, so it is an exact byte
    /// prefix and "every section on this chart" is a fibre read at that
    /// stratum (A21: a prefix of the key is a coarser stratum).
    ///
    /// IT IS A LAYOUT CHANGE (§10). `put_word_vertex` writes the CHART into
    /// the address and the members are written in this key's order, so
    /// widening it moves every address in the tree. A first attempt wrote
    /// the map's key into the address and 25 forms stopped decoding —
    /// `take_acts` parses an `Invariant` out of what it reads there, and a
    /// carriage tail is not one. The writer emits the chart and both sides
    /// ask THIS function for the carrier's key, which is the same
    /// one-writer rule that took `NerveEdge::host` off its second writer.
    ///
    /// 54.7 R5 — THE SECTION IS IN THE TYPE. A section key opens with
    /// `TAG_CHART`, exactly as a chart key does, so no byte check separates
    /// them and eight readers asked `by_class` — a CHART index — with one
    /// of these and got nothing back. `coord::Section` and `coord::Chart`
    /// are different types over the same tag, which is the only place that
    /// distinction can live.
    pub fn key(&self) -> coord::Section {
        coord::Section { chart: self.chart.clone(), carriage: (*self.carriage).clone() }
    }

    /// The prefix every section over `chart` shares — the coarser stratum
    /// of [`Acting::key`], and what a fibre read asks at.
    pub fn chart_prefix(chart: &stalk::Invariant) -> Vec<u8> {
        Self::chart_prefix_bytes(chart.key().as_bytes())
    }

    /// The same, from the chart's key as bytes — for a reader holding the
    /// key a face or an index recorded rather than the `Invariant`.
    pub fn chart_prefix_bytes(ck: &[u8]) -> Vec<u8> {
        let mut k = Vec::new();
        crate::transducer::address::put_key_bytes(&mut k, ck);
        k
    }
}

/// A vertex of a sentence form (chapter 18).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vertex {
    Frame,
    Word {
        address: crate::transducer::address::Address,
        /// What this vertex IS (50.2 step 2, law 12): the generators it
        /// factored through and the residue A22 peeled, the word read
        /// back with `derivation.word()`.
        derivation: crate::transducer::alphabet::Derivation,
        /// EVERY CHART THIS ONE GENERATOR ACTS ON, and what it acts by at
        /// each: `acts` is keyed by the chart and holds that chart's own
        /// carriage and gauge (D7, 18.2, 21 step 1).
        ///
        /// This was three single-valued fields — `class`, `carriage`,
        /// `gauge` — and 21 step 1 asks them to carry a section it cannot.
        /// Its two sentences are "two bodies of different shapes carried by
        /// one transport correspond", so they are ONE vertex, and "the
        /// vertex's class is the section coordinate", which is one
        /// `Invariant` and holds one of their charts. Measured on
        /// `05f2a901`: two bodies, two charts, one transport class, and
        /// `speak_rule`'s `atlas.fiber(class)` reaching one of the two. A
        /// section coordinate that is single-valued cannot carry a section.
        ///
        /// 18.2 already names the carrier and its intent in one line — two
        /// carriages on one class are two sections — and what it held was
        /// one versor. Keyed by chart all the way down, that sentence
        /// becomes true of its own type, and `speak_rule` reads the member
        /// at the body's own chart: a keyed read, never a search, and a
        /// body on a chart this rule has never seen finds no member and is
        /// honestly unspoken.
        acts: Keyed<Acting, coord::Section>,
        /// Stop ring. A form coordinate with the carriage and gauge; not M1.
        contact: Option<alphabet::Contact>,
    },
    /// AN OPEN BOUNDARY OF THIS CHART — and NOT what it mates with (54.7,
    /// B5a, as ruled 2026-09-21).
    ///
    /// THE TARGET IS GONE FROM HERE. It sat on the vertex and, through
    /// `put_vertex`, inside `form_vertex_key`, and that made the address
    /// space provably acyclic: `key(A) ⊃ key(B)` gives `|A| > |B|`, so by
    /// well-ordering no cycle `A → … → A` could exist. `b₁` of the nerve
    /// was zero by construction.
    ///
    /// What the host holds is its own boundary — the rank of chart it mates
    /// with, the depth that chart had when cited, the placement and the
    /// transition φ. All of it is LOCAL and bounded, `O(1)` per citation,
    /// and all of it stays. WHICH form the boundary was instantiated
    /// against is the cover's fact and lives on [`Form::cover`] as a
    /// `NerveEdge`.
    ///
    /// IT IS DELETED RATHER THAN EMPTIED. Left as a field, a form decoded
    /// from a key would carry an empty address and every reader would go on
    /// compiling while resolving nothing — the empty-carrier reading, which
    /// on `rule_matches_tests` means a court painting nothing for a rule
    /// the client painted, and every citing solve failing verification.
    /// Gone, each of those readers is a compile error until it takes the
    /// cover from where the cover is.
    Glued {
        chart: stalk::Invariant,
        magnitude: [u64; 4],
        depth: u16,
        placement: Word,
        /// THE TRANSITION VERSOR φ — HOW THE CITED FORM IS EMBEDDED HERE.
        ///
        /// A citation is not a foreign key. The cited form stands
        /// canonical and unmutated at its own address; it ACTS inside this
        /// one under a morphism, `x_host = φ x_cited φ̃`. Without φ the
        /// incidence is a static pointer and the embedding is not in the
        /// key, so `from_address` cannot know where the sub-complex sits.
        ///
        /// It was `placement: Word` alone — a rotor in `Spin(4,0)`, which
        /// can turn the cited form and cannot translate or dilate it —
        /// and `vertex_carriage_at` answered `identity_versor()` for every
        /// Glued vertex, so the placement face carried the identity too.
        /// Two forms citing one sub-complex at two places were one
        /// address, and the property test never saw it because every
        /// fixture glues at the origin.
        ///
        /// A conformal versor (A4) carries the rotor, the translator and
        /// the dilator together, is `O(1)` per citation whatever the cited
        /// form's size, and is exactly invertible by its reversion — so
        /// the incidence is a groupoid edge, `φ` one way and `φ̃` the
        /// other, not a one-directional reference.
        carriage: Box<crate::transducer::element::Versor>,
    },
    /// Vertex `of` read in its own frame as `by`'s word (3.2): the linear
    /// word is `by`'s, realised at `placement_by + shift`; the body it
    /// speaks to, its hue map and its dilation stay `of`'s own, so a
    /// rebased body keeps its shape, its colours and its scale.
    ///
    /// AT1 — `by` IS THE BASE'S COORDINATE, and `class` is the class `of`
    /// had before the rebase overwrote it: held, because the overwrite made
    /// it underivable, and the vertex is keyed by the base's class then this
    /// one (ruled). The `of` slot is gone; the vertex's own key replaces it.
    Rebased {
        by: coord::VertexAt,
        class: coord::Transport,
        /// EVERY CHART `of` ACTED ON, and what the rebase carries it by there:
        /// the member's `carriage` is the RELATIVE `t = C_of,chart · C_by⁻¹` at
        /// that chart and its `gauge` is `of`'s there (D10).
        ///
        /// These were three single-valued fields, and `Form::rebase` read `of`'s
        /// representative member to fill them — so rebasing a vertex that acted
        /// on two charts kept one and dropped the other's body, at the one move
        /// whose whole job is to REPLACE a vertex, leaving nothing to read the
        /// dropped member back from.
        ///
        /// THE RELATIVE DOES NOT RE-DERIVE, so law 12 does not retire it: the
        /// rebase overwrites `of`'s slot, `C_of` is gone, and `C_of = t · C_by`
        /// comes back FROM the stored `t`. What re-derives is the placement, and
        /// `vertex_carriage_at` is where it does.
        ///
        /// `by` stands at ONE chart — `Form::rebase` refuses otherwise — because
        /// `t` is measured against one member of the base and a fibre keyed by
        /// `of`'s chart has no room to say which.
        acts: Keyed<Acting, coord::Section>,
    },
    Unfolded { measure: Measure },
    /// 50.5: what the speaker could not spell or the eyes could not pair,
    /// one vertex per (kind, chart), carrying the bodies it stands for as
    /// the sentence's remainder. It has no word and reconstructs nothing.
    ///
    /// AT1, 54.5 ROAD (d) — THE BODIES ARE EVIDENCE AND RIDE THE RECORD,
    /// NEVER THE KEY. The key carries the kind, the chart, the chain's
    /// measure per lane and its piece count; the bodies ride the receipts
    /// envelope beside the cover and a form read back from its address
    /// holds none until the record supplies them, exactly as a Glued vertex
    /// holds no cover. `measure` and `pieces` are read off the bodies where
    /// the bodies are held ([`Vertex::demand`]), and held beside them only
    /// because a form read from its key has no bodies to read them from.
    Demand {
        kind: crate::transducer::sentence::Demand,
        chart: stalk::Invariant,
        measure: [u64; 4],
        pieces: u64,
        bodies: Vec<Link>,
    },
}

/// WHICH ORGAN PRODUCED AN OVERLAP'S TRANSITION (78.41(2′)(a): every
/// transition names what built it). A PLACE is the relative of two members'
/// placements, measured where the face was made (21 step 2); a MOTOR is the
/// frame-axis transition, a member's body carried from one occupied frame
/// to the next and read by its action (MT1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Producer {
    Place,
    Motor,
}

/// A FACE OF THE ATLAS: ONE OVERLAP OF TWO CHARTS, keyed by the two
/// (`coord::Overlap`, AT1 Q2) and never by two positions in a list.
///
/// It holds two kinds of content, and the carrier says which (ruled):
///
/// - MEASURED — the place (the relative carriage, the relative hue map,
///   50.3's edge, BQ5a's refraction), the interface of a contact, and the
///   producer that measured it.
/// - PRODUCED — `left` and `right`, the identity by default and written
///   ONLY by a pivot's Word move, the one producer that writes a word on a
///   face; the certificate whose chord names this overlap is that factor's
///   producer (A20).
///
/// THE WORD IS DERIVED, never held: `left · (w_to ∘ w_from⁻¹) · right⁻¹`,
/// read from the two ends' vertices ([`Form::face_word`]). A face carrying
/// a word its ends do not derive is the table 78.41(2′) names; a section
/// is a vertex of its own class, not a word on a face.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormFace {
    pub place: FacePlace,
    pub producer: Producer,
    /// The pivot's factor on the left of the derived word.
    pub left: Word,
    /// The pivot's factor on the right, entering inverted.
    pub right: Word,
    /// BQ5b, 72.2, A.10 — THE INTERFACE A CONTACT FACE CARRIES, WHOLE: the
    /// versor `V = s_b s_a` between its two bodies' nuclei and the grade of
    /// the contact's largest shared cell. WHAT two bodies are to each other
    /// (conformal), read where the face is built; WHETHER they touch is the
    /// lattice's shared cells (A.14, topological), the witness perception
    /// reads and drops. `None` on a face that is not a contact, and on a
    /// contact whose body pair's nuclei did not read (`unnucleated`). The
    /// face's CONTENT, never its identity: two pairs may share one `V`.
    pub interface: Option<Interface>,
}

/// BQ5b — A CONTACT'S INTERFACE: `V = s_b s_a`, held as its RAY (the
/// versor's canonical form, VA1: `λV` is one interface), and the grade of
/// the contact's largest shared cell. Its kind is `Contact::of(V)`, read
/// where asked and never stored.
#[derive(Debug, Clone)]
pub struct Interface {
    pub v: crate::transducer::cga::Mv,
    pub grade: u8,
}

impl Interface {
    /// THE ONE PRODUCER: the tether between two bodies' nuclei — the body
    /// at the face's `i` end first — as its ray, with the contact's grade.
    /// `None` where the tether has no ray (zero).
    pub fn between(a: &crate::transducer::cga::Mv, b: &crate::transducer::cga::Mv, grade: u8) -> Option<Interface> {
        let v = crate::transducer::blade::tether_of(a, b).canonical()?;
        Some(Interface { v, grade })
    }

    /// The interface as the address writes it: `V`'s ray, then the grade.
    pub fn put(&self, out: &mut Vec<u8>) {
        self.v.put_canonical(out);
        out.push(self.grade);
    }

    pub fn take(b: &[u8], p: &mut usize) -> Option<Interface> {
        let v = crate::transducer::cga::Mv::from_bytes(b, p)?;
        let grade = *b.get(*p)?;
        *p += 1;
        Some(Interface { v, grade })
    }
}

/// One interface and another are one where their rays write the same
/// coordinates and their grades agree.
impl PartialEq for Interface {
    fn eq(&self, other: &Interface) -> bool {
        let (mut a, mut b) = (Vec::new(), Vec::new());
        self.put(&mut a);
        other.put(&mut b);
        a == b
    }
}

impl Eq for Interface {}

/// A FRAME AS A COORDINATE (51.1, 51.8, AT1 Q4): the frame along `e₃` by
/// its own spelling — a length byte, then the minimal big-endian bytes — so
/// the trie's order is the succession and no width walls it.
pub fn frame_key(k: u64) -> Vec<u8> {
    let mut out = Vec::new();
    coord::put_frame(k, &mut out);
    out
}

/// The frame a section's inner key names. `None` for a key that is not one.
pub fn frame_of_key(k: &[u8]) -> Option<u64> {
    let mut pos = 0usize;
    let f = coord::take_frame(k, &mut pos)?;
    (pos == k.len()).then_some(f)
}

/// The frames a section stands on, in succession. A reader that needs the
/// list — the tape, the form key — takes it here rather than keeping a
/// second copy beside the carrier (law 12).
pub fn frames_of_section(section: &Keyed<()>) -> Vec<u64> {
    section.scoped().filter_map(|(k, _)| frame_of_key(&k)).collect()
}

/// A section over the frames it stands on.
pub fn section_of_frames(frames: impl IntoIterator<Item = u64>) -> Keyed<()> {
    let mut out = Keyed::new();
    for k in frames {
        out.insert(frame_key(k), ());
    }
    out
}

impl FormFace {
    /// An overlap carrying its measured place, unpivoted.
    pub fn placed(place: FacePlace, producer: Producer) -> FormFace {
        FormFace { place, producer, left: Word::identity(), right: Word::identity(), interface: None }
    }

    // `interface_tower` STOOD HERE AND IS DELETED (BQ5b): it lifted the
    // shared-cell set as a support, a tower of the witness of WHETHER, and
    // nothing read it. The interface is `V` whole.

    /// Does this overlap carry 50.3's edge? The reading a contraction makes
    /// when it asks whether a face is carried at all.
    pub fn has_edge(&self) -> bool {
        self.place.edge.is_some()
    }

    /// The overlap's edge becomes a remainder edge. A contraction that
    /// retracts a body cannot leave the edge that was measured on it.
    pub fn clear_edges(&mut self) {
        self.place.edge = None;
    }

    /// Has a pivot written a factor on this overlap?
    pub fn is_pivoted(&self) -> bool {
        !self.left.is_identity() || !self.right.is_identity()
    }
}

impl coord::Overlap {
    /// The vertex at the overlap's leaving end; the Frame's own coordinate
    /// where the chart is the Frame's.
    pub fn from_vertex(&self) -> coord::VertexAt {
        self.from.vertex()
    }

    /// The vertex at its arriving end.
    pub fn to_vertex(&self) -> coord::VertexAt {
        self.to.vertex()
    }

    /// The frame this overlap stands on: the member end's, the Frame having
    /// none. A frame-axis overlap stands on two, and answers its leaving one.
    pub fn frame(&self) -> Option<u64> {
        self.from.frame().or_else(|| self.to.frame())
    }

    /// The same overlap read backward.
    pub fn reversed(&self) -> coord::Overlap {
        coord::Overlap { from: self.to.clone(), to: self.from.clone() }
    }
}

impl coord::ChartAt {
    /// The vertex this chart belongs to; the Frame's chart is the Frame's.
    pub fn vertex(&self) -> coord::VertexAt {
        match self {
            coord::ChartAt::Frame => coord::VertexAt::Frame,
            coord::ChartAt::At(m) => m.vertex.clone(),
        }
    }

    /// The Invariant the member acts on; `None` at the Frame.
    pub fn invariant(&self) -> Option<&stalk::Invariant> {
        match self {
            coord::ChartAt::Frame => None,
            coord::ChartAt::At(m) => Some(&m.chart),
        }
    }

    /// The frame; `None` at the Frame.
    pub fn frame(&self) -> Option<u64> {
        match self {
            coord::ChartAt::Frame => None,
            coord::ChartAt::At(m) => Some(m.frame),
        }
    }
}

/// THE ATLAS'S VERTICES AND FACES, as every reader takes them.
pub type Vertices = Keyed<Vertex, coord::VertexAt>;
pub type Faces = Keyed<FormFace, coord::Overlap>;

/// ONE STEP OF A CIRCUIT: an overlap, crossed with its orientation or
/// against it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrbitTransition {
    pub overlap: coord::Overlap,
    pub forward: bool,
}

impl OrbitTransition {
    /// The chart the step leaves.
    pub fn leaves(&self) -> &coord::ChartAt {
        match self.forward {
            true => &self.overlap.from,
            false => &self.overlap.to,
        }
    }

    /// The chart it arrives at.
    pub fn arrives(&self) -> &coord::ChartAt {
        match self.forward {
            true => &self.overlap.to,
            false => &self.overlap.from,
        }
    }
}

/// AN INDEPENDENT ORBIT OF A FORM, AS THE SEQUENCE OF OVERLAPS IT CROSSES
/// (AT1, ruled): the cycle, closing where it began; the chord that closed it
/// (never a star overlap); and the forest's stem from its component's root
/// to where it is based (Axiom A).
///
/// NOTHING THE STEPS DERIVE IS HELD. The holonomy word, the carriage along
/// the cycle and the transport `θ` to the basepoint are READ by composing
/// the overlaps (`Form::orbit_word`, `orbit_transport_on`,
/// `Form::orbit_based_at`); a holonomy stored beside the transitions it
/// composes is the table 78.41(2′) names. Each overlap relates ONE member
/// pair at one frame, so a cycle is one member path and carries one
/// transport: the frontier over a face's several pairs is gone with the
/// faces that had several.
///
/// The stem and the chord stay until (iii) retires the forest; they are
/// sequences of coordinates, never positions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Orbit {
    pub transitions: Vec<OrbitTransition>,
    pub chord: coord::Overlap,
    pub stem: Vec<OrbitTransition>,
}

impl Orbit {
    /// The charts the cycle stands at, in circuit order: each step's leaving
    /// chart.
    pub fn charts(&self) -> Vec<coord::ChartAt> {
        self.transitions.iter().map(|s| s.leaves().clone()).collect()
    }

    /// The vertices it passes through, in circuit order (π of the charts).
    pub fn vertices(&self) -> Vec<coord::VertexAt> {
        self.transitions.iter().map(|s| s.leaves().vertex()).collect()
    }

    /// THE KEY THE CYCLE'S ONE MEMBER PATH IS FILED UNDER: its charts in
    /// circuit order, each length-delimited. Content, so the same path reads
    /// the same on every node.
    pub fn path_key(&self) -> Vec<u8> {
        use crate::network::nerve::Coordinate;
        let mut buf = Vec::new();
        for c in self.charts() {
            crate::transducer::address::put_key_bytes(&mut buf, c.encode().as_bytes());
        }
        buf
    }
}


/// NB1, 78.2 — ONE CERTIFIED CELL OF THE CONTACT COMPLEX: the bodies
/// standing together at one element of the base, each by its PIECE — its
/// support, canonical, the coordinate a body has (78.14, BD2; C3): a member
/// key names a vertex on a chart, whose support can fall into several
/// bodies, and a nucleus is the body's order-0 and order-1 reading, a
/// projection — in their own coordinate order; every element standing
/// under exactly them,
/// its witnesses; and the MEET of their nuclei — the round the bodies
/// share, at the grade their number gives (`k + 1` bodies, a `k`-cell).
///
/// Perception's reading, a fibre of the epigenome: held on the form in
/// memory and never written to the tape; a cell a circuit seals is a tether
/// of the genome and reaches the tape at the recut (78.6, NB1 choice (c)).
/// It carries no transport: that is deposited by the circuit that closes on
/// it (`Complex::ascend`), in the circuit's own order (NB1 choice (b)).
#[derive(Debug, Clone)]
pub struct FormCell {
    pub bodies: Vec<Vec<u8>>,
    pub witnesses: Vec<crate::transducer::base::Site>,
    /// CS1 — THE CELL'S CONSTRUCTORS: its bodies' nuclei, in the bodies'
    /// order, each a reflection. `None` where a body's nucleus did not read
    /// back (rule 11). The meet is read off them, never stored beside them.
    pub nuclei: Option<Vec<crate::transducer::cga::Mv>>,
}

impl FormCell {
    /// `k + 1` bodies make a `k`-cell; a cell is minted over two or more.
    pub fn grade(&self) -> usize {
        self.bodies.len() - 1
    }

    /// THE MEET OF THE CELL'S NUCLEI — the round its bodies share, the
    /// cell's shape (78.2, NB1) — read off its constructors.
    pub fn meet(&self) -> Option<crate::transducer::cga::Mv> {
        crate::transducer::cech::meet_of(self.nuclei.as_deref()?)
    }
}

/// A CHAIN BY ITS COORDINATES (54.5 Road (d), AT1): its measure per lane,
/// its pieces and its corner — what the key writes of a demand's bodies or
/// of the remainder; the chain itself is evidence and rides the record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainAt {
    pub measure: [u64; 4],
    pub pieces: u64,
    pub corner: (i64, i64),
}

impl ChainAt {
    /// The coordinates of a chain, read off it; `None` for the empty chain,
    /// which has no corner and is written as the absence it is.
    #[expect(clippy::disallowed_methods, reason = "rule 4 — the min corner is the chain's extent, content; §1.4, the sensor boundary where a site's coordinates are read")]
    pub fn of(chain: &[Link]) -> Option<ChainAt> {
        let r = chain.iter().map(|(r, _, _, _)| *r).min()?;
        let c = chain.iter().map(|(_, c, _, _)| *c).min()?;
        let (measure, pieces) = demand_coordinates(chain);
        Some(ChainAt { measure, pieces, corner: (r, c) })
    }
}

/// A sentence as vertices, faces, orbits, remainder.
#[derive(Debug, Clone)]
pub struct Form {
    /// 71.2(1) — THE `n` OF THE BASE THE FORM WAS READ ON: the count of a
    /// site's coordinates, read at intake, and the algebra `Cl(n+1, 1)`
    /// every versor the form carries stands in. The coarsest coordinate of
    /// the form, so it is written FIRST on its address and a reader takes
    /// every versor after it at this `n`.
    pub n: u8,
    /// AT1 — THE VERTICES BY THEIR COORDINATES (Q1): a Word by its
    /// transport class, the Frame by its tag alone, every other kind by the
    /// coordinate its kind names. Never a position: a vertex is reached by
    /// what it is, and two forms holding one vertex hold it at one key.
    pub vertices: Vertices,
    /// AT1 — THE FACES BY THEIR OVERLAPS (Q2): one per member pair per
    /// frame, keyed by the two charts, the order the orientation.
    pub faces: Faces,
    pub orbits: Vec<Orbit>,
    /// A5's chain and the translator that places it (§2.1, 19). The proof
    /// rides the coordinate: a tape whose remainder puts two transitions on
    /// one site does not parse, and that refusal is the decoder's under
    /// `REFUSE_FORM_TAPE`, never a reader's downstream.
    ///
    /// `CanonicalChain` alone would centre it and lose the placement the
    /// Čech complex reads for overlap, which is the reading
    /// `two_bubbles_on_one_chart_share_an_edge_not_a_triangle` asserts.
    pub remainder: crate::transducer::element::PlacedChain,
    /// AT1, 54.5 ROAD (d) — THE REMAINDER'S COORDINATES where its chain is
    /// not held: a form read back from its address carries the chain's
    /// measure, pieces and corner and not the chain, which rides the record
    /// exactly as a demand's bodies do. `None` wherever `remainder` holds the
    /// chain, whose coordinates are then read off it ([`ChainAt::of`]).
    pub remainder_at: Option<ChainAt>,
    pub depth: u16,
    pub hue_maps: Keyed<Vec<(u8, u8)>>,
    pub ratio: ((usize, usize), (usize, usize)),
    // `sections` STOOD HERE AND RETIRED AS A CARRIER (AT1, ruled): class or
    // member key → the frames it stands on. Each member now holds its own
    // frames (`Acting.frames`), so the sections are READ off the members
    // ([`Form::sections`]) and nothing beside them has to agree with them
    // (law 12).
    /// The certified pivots this form's crystallization rests on (A20).
    /// Evidence, replayed by the court; never part of the address.
    pub certificates: Keyed<PivotCertificate>,
    /// 78.112 H7b — THE TORSION HEARING THIS PERCEPTION'S RINGS WOUND, carried
    /// from the percept to the hold, where it is wound on the perception's
    /// vertex. Never part of the address: a form read back from its address
    /// heard nothing.
    pub heard: Option<crate::transducer::cga::Mv>,
    /// Filed torsion cycles, keyed by class key: `(σ, ℓ, chord face)`.
    /// THE FORM OWNS THIS CARRIER and there is no second copy of it: a reader
    /// holding an element reaches it through `sentence::form_torsion`, and an
    /// organ holding a word reads the form it decoded at admission
    /// (`Held::form`, L16). `Element::adopt_torsion` copied it into the
    /// element's own `torsion` field and is deleted.
    pub torsion: Keyed<FiledOrbit, coord::Holonomy>,
    /// CHART AXES OPENED OVER HELD ORBITS (3.7, 15.5, 16.16 step 3; I6):
    /// the holonomy class opened on → the chord of the orbit that is its
    /// section. The relation is all the form carries; the seed versor and
    /// A7's coordinate are re-derived from that orbit on every read (law
    /// 12), never stored beside it. On the tape and in the address. The
    /// chord is the overlap (AT1); a form read from its address holds the
    /// class alone and finds the orbit by it.
    pub axes: Keyed<Option<coord::Overlap>, coord::Holonomy>,
    /// THE COVER: WHICH FORM EACH OPEN BOUNDARY MATES WITH (54.7, B5a, as
    /// ruled 2026-09-21).
    ///
    /// This was `Vertex::Glued.address` — the target sitting on the vertex
    /// and, through `put_vertex`, inside `form_vertex_key`. Both are wrong,
    /// and the second is the one that forced the topology: with `key(A) ⊃
    /// key(B)` for every citation, `|A| > |B|`, and by well-ordering no
    /// cycle `A → … → A` could exist. The manifold was a directed acyclic
    /// forest because of how it was NAMED, and `b₁` was zero by
    /// construction rather than by measurement.
    ///
    /// A CHART DOES NOT KNOW WHAT IT OVERLAPS. The host states its own
    /// boundary — a rank, a depth, a placement, a φ — and that is all the
    /// vertex carries and all the key writes. WHICH form that boundary was
    /// instantiated against is a fact about the COVER, so the form carries
    /// it here as `N(𝒰)`'s 1-skeleton and the key does not carry it at all.
    ///
    /// IT IS NOT AN INDEX, AND IT IS NOT KEYED BY SITE. `glued` above maps
    /// a cited address to a vertex index, and an index is a position rather
    /// than a coordinate (rule 15). A map keyed the other way — site to
    /// edge — fails for a different reason: once the target leaves the key,
    /// two citations of two forms under one φ at one chart rank stand at
    /// ONE site (gauge universality), so a keyed carrier would hold one and
    /// silently drop the other. The cover is a list of edges, ordered by
    /// their own coordinates when the envelope is built.
    ///
    /// EMPTY IS HONEST. `from_address` reconstitutes a form from its key,
    /// and the key does not name the cover, so the form it returns has
    /// none. `from_element_form` has the word's `ReceiptsEnvelope` in hand
    /// and fills it. A reader that finds it empty is reading a form whose
    /// citations nothing told it — which is the truth, not a zero.
    pub cover: Vec<crate::transducer::element::NerveEdge>,
    /// Incomparable discharges held as candidates until a glue or ruling.
    /// Outer key is the after class; inner is the discharging rotor A1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// Chart key → the Word vertices on that chart (18.2): the fiber a
    /// listing body's chart reads when a cite asks what this form holds
    /// there (24.1 as ruled 2026-09-09). Derived by `reindex`, never on
    /// the tape.
    ///
    /// 54.7 R5 — THE CHART IS IN THE TYPE. This sentence was a doc comment
    /// over a `Keyed<Vec<u16>>` identical to every other index in the tree,
    /// and `reindex` filed the MEMBER's key under it — `chart ‖ carriage`,
    /// which opens with `TAG_CHART` exactly as a chart key does, so no byte
    /// check could have separated them. Every reader asked for a chart and
    /// got nothing, `incident_chart` read every idea as standing on no
    /// chart, and the defect surfaced only when a cite broke. `coord::Chart`
    /// is that sentence where the compiler reads it (§4.4). It holds the
    /// vertices' COORDINATES (AT1), never their positions.
    pub by_class: Keyed<Vec<coord::VertexAt>, coord::Chart>,
    /// TRANSPORT CLASS KEY → the Word vertices of that class (21 step 1).
    /// Derived by `reindex`, never on the tape.
    ///
    /// The fiber correspondence needs, and it did not exist. Correspondence
    /// matches by TRANSPORT CLASS — kind and the conjugacy of the word, never
    /// the hue map and never the body's moduli, which are section coordinates
    /// — and none of this form's carriers was keyed by it: `by_class` is
    /// keyed by the CHART, and one class spans several charts (`05f2a901`:
    /// one class, two charts), so it cannot say which vertices correspond;
    /// `carriages` holds versors rather than indices and its key carries the
    /// gauge, which 21 step 1 excludes from matching. Lacking the fiber,
    /// extraction read every pair form's vertices and accumulated the
    /// correspondence itself — a second producer of one fact, beside the
    /// indexer whose job it is (law 12, 13.1).
    ///
    /// The key is `TransportClass::key()`'s own, taken from
    /// `family::transport_class(v)`, so a reader asks with the key it already
    /// builds and there is one definition of it. It holds the vertices'
    /// COORDINATES (AT1): a Word's is its class, and a Rebased vertex on a
    /// base of the class stands under it too.
    pub by_transport: Keyed<Vec<coord::VertexAt>, coord::Transport>,
    /// Word vertex address → carriage, filled at construction.
    pub carriages: Keyed<Versor>,
    /// NB1 — THE CONTACT COMPLEX'S CERTIFIED CELLS, keyed by their bodies'
    /// pieces written inline, each framed: one per standing set, read by
    /// perception and held in memory only (see [`FormCell`]).
    pub cells: Keyed<FormCell>,
    /// BD2 — THE BODY PAIR EACH CONTACT FACE RUNS BETWEEN, by its overlap:
    /// the two bodies' pieces in the overlap's orientation, the body at its
    /// leaving end first. `FormFace` carries no body, so a circuit's step
    /// reads its body off the overlap it crosses here — never off its member
    /// (checklist (m)). In memory only; the tape at the recut (BQ5b).
    pub face_bodies: Keyed<(Vec<u8>, Vec<u8>), coord::Overlap>,
}

impl Default for Form {
    fn default() -> Self {
        let mut vertices = Keyed::new();
        vertices.put(&coord::VertexAt::Frame, Vertex::Frame);
        Form {
            n: crate::transducer::element::GRID_N,
            vertices,
            faces: Keyed::new(),
            orbits: Vec::new(),
            remainder: crate::transducer::element::PlacedChain::default(),
            remainder_at: None,
            depth: 0,
            hue_maps: Keyed::new(),
            ratio: ((1, 1), (1, 1)),
            certificates: Keyed::new(),
            heard: None,
            torsion: Keyed::new(),
            axes: Keyed::new(),
            cover: Vec::new(),
            vortex: Keyed::new(),
            by_class: Keyed::new(),
            by_transport: Keyed::new(),
            carriages: Keyed::new(),
            cells: Keyed::new(),
            face_bodies: Keyed::new(),
        }
    }
}

// `impl Vertex` — `word_in`, `class`, `depth` — was here and is now
// `form/vertex.rs`. `word_in` TRAVERSES the rebase chain instead of
// recursing on it (L20), and this file went over the §1.1 reading bound
// with the traversal in it. Moved, not rewritten (§1.6).


/// WHAT A FORM CITES, at both levels (26 as ruled 2026-09-10): a `Glued`
/// vertex cites a whole form, and a word vertex's derivation cites a
/// `Word` vertex inside one (50.2 step 2). In vertex order, with no
/// deduplication — a form cited twice is cited twice, and the caller
/// that counts uses decides what a repeat means.
///
/// One carrier for the reading. The court's use-credit read the
/// vertices for it and so did the retirement scope; two reads of one fact
/// are two chances to disagree about what a citation is (law 12).
pub fn cited_addresses(f: &Form) -> Vec<crate::transducer::address::Address> {
    let mut out = Vec::new();
    // CHART CITING CHART, off the cover (54.7 B5a). It read the `Glued`
    // vertices, and a vertex no longer names the form it mates with; the
    // 1-skeleton of `N(𝒰)` does.
    for e in &f.cover {
        out.push(e.target.clone());
    }
    for (_, v) in f.vertices.scoped() {
        if let Vertex::Word { derivation, .. } = v {
            for (form_addr, _, _) in &derivation.generators {
                out.push(form_addr.clone());
            }
        }
    }
    out
}

/// THE WORD ACROSS ONE OVERLAP, DERIVED (AT1 Q2, ruled): the pivot's left
/// factor, the coboundary `w_to ∘ w_from⁻¹` of the two ends' vertices, and
/// the pivot's right factor inverted. Never held: a face carries only what
/// a producer wrote on it. `None` where an end speaks no word (a demand,
/// 50.5) or is not a vertex of these.
pub fn face_word_on(vertices: &Vertices, overlap: &coord::Overlap, face: &FormFace) -> Option<Word> {
    let wa = vertices.at(&overlap.from_vertex())?.word_in(vertices)?;
    let wb = vertices.at(&overlap.to_vertex())?.word_in(vertices)?;
    Some(face.left.compose(&wb.compose(&wa.inverse())).compose(&face.right.inverse()).normal_form())
}

/// THE WORD AN ORBIT COMPOSES, read along its steps (ruled: a holonomy is
/// never held beside the transitions it composes). `None` where a step
/// names an overlap these faces do not hold.
pub fn orbit_word_on(faces: &Faces, vertices: &Vertices, lp: &Orbit) -> Option<Word> {
    let mut w = Word::identity();
    for s in &lp.transitions {
        let step = face_word_on(vertices, &s.overlap, faces.at(&s.overlap)?)?;
        let step = match s.forward {
            true => step,
            false => step.inverse(),
        };
        // The order `form_named_holonomy` composed in, kept, so the word an
        // orbit reads is the one it read before the atlas, not a conjugate.
        w = w.compose(&step);
    }
    Some(w.normal_form())
}
