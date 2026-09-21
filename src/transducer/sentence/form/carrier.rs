// 18.2 — THE FORM: vertices, faces, loops, and the reads that walk them.
//
//! A face is constant in the word between its two vertices; a loop is a
//! cycle of faces; the holonomy of a loop is the composite around it. All
//! three are structure, walked and never searched.

use super::*;

impl Form {
    /// THE WORDS A SENTENCE IS MADE OF — one holonomy per loop, with the
    /// class it belongs to.
    ///
    /// A sentence has no single spelling: it is a NERVE of germs whose
    /// vertices carry words and whose faces carry the conjugations between
    /// them (18.2). Asking it for `Element::letters` is a level error —
    /// letters spell a word, and a sentence is one level above a word. What
    /// it does have is a word per loop, its holonomy, already composed on
    /// the carrier, and the class that word belongs to.
    ///
    /// Order is a CONJUGACY invariant (15.2, 15.3): "closes in p steps" is a
    /// fact about the class, never about the spelling, which is why the
    /// class travels beside the word here and not the tape.
    /// ONE ENTRY PER MEMBER PATH, carrying the holonomy that path actually
    /// has: the edge composite where it has one, the loop's own word where it
    /// does not — `holonomy_words_on`, the same carrier the filer reads.
    ///
    /// THIS RETURNED `lp.holonomy` AND IT IS THE COURT'S METABOLISM THAT READ
    /// IT. `germinate/listing.rs` charges the physics off this: a loop of
    /// FINITE order raises pressure by its period, and a loop of INFINITE
    /// order charges TORSION — "what will not close". A face's word is the
    /// coboundary `w_j ∘ w_i⁻¹` and a product of coboundaries around a cycle
    /// telescopes to the identity, so every loop read `Finite(1)`, every
    /// sentence raised one unit of pressure per loop, and the branch that
    /// charges torsion was unreachable by construction.
    ///
    /// So `Physics.torsion` stayed `[0, 0, 0, 0]`, `order_flux` was
    /// `syntropic − 0`, and `yield_surface` — the per-lane dwell a reply
    /// carries — never saw a cycle. Measured at 7582ada: 196 mathematical
    /// torsion cycles filed by the voices, 0 charged on any court.
    ///
    /// D1 put the FILER on the true carrier and did not reach here; this is
    /// the same defect at the metabolism's own reader.
    pub fn loop_words(&self) -> Vec<(crate::transducer::alphabet::Word, Option<ClassVector>)> {
        let mut out = Vec::new();
        for i in 0..self.loops.len() {
            let Some(lp) = self.loops.get(i) else {
                continue;
            };
            let cvs = self.class_vectors(i);
            // One holonomy per member path, each its own obstruction, paired
            // with that path's own class vector rather than the loop's
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
/// stored on the form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Measure {
    Occupied,
    Remainder,
    Ring { anchor: u16 },
}

/// A FILED LOOP: a torsion cycle of a FORM, keyed by the class of the loop
/// that carries it. The same three members as an element's `TorsionCycle`
/// with one difference that matters — the third is the CHORD the loop closes
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

    /// Neither plane turns forever, so the loop closes — and the pair is
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
pub struct FiledLoop {
    /// The hue-map generator: the permutation one step around the loop.
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
    ///   beat, and two loops cannot resonate because no reader can compare
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
    /// The chord it closes on: an index into the form's faces.
    pub chord: u16,
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
}

impl Acting {
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
    pub fn key(&self) -> Key<coord::Section> {
        let mut k = Vec::new();
        crate::transducer::address::put_bytes(&mut k, self.chart.key().as_bytes());
        self.carriage.put_canonical(&mut k);
        Key::declared(&k)
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
        crate::transducer::address::put_bytes(&mut k, ck);
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
        /// other, not a one-directional descent.
        carriage: Box<crate::transducer::element::Versor>,
    },
    /// Vertex `of` read in its own frame as `by`'s word (3.2): the linear
    /// word is `by`'s, realised at `placement_by + shift`; the body it
    /// speaks to, its hue map and its dilation stay `of`'s own, so a
    /// rebased body keeps its shape, its colours and its scale.
    Rebased {
        by: u16,
        of: u16,
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
    Demand {
        kind: crate::transducer::sentence::Demand,
        chart: stalk::Invariant,
        bodies: Vec<Link>,
    },
}

/// Face of a `Form` (21 step 2, 3.1): two vertices, the word between them,
/// and what it carries AT EACH PAIR OF MEMBERS.
///
/// It held one `Transport` and one edge. 21 step 2 defines a face between
/// BODIES — "vertices whose after-supports overlap or sit at ring distance
/// one" — and D7 made one vertex cover several bodies on several charts, so
/// the face's endpoints became coarser than the face's meaning and one
/// transport was asserted across every member pair. Measured over the
/// families that mint: 87 faces, 19 of them between two multi-member
/// vertices and 52 with at least one.
///
/// The WORD stays single because it is not a member coordinate: `face_of`
/// reads it as `w_j ∘ w_i⁻¹` from the two derivations, and a derivation is a
/// vertex's. So the rotor half of the skeleton — holonomy, torsion, class
/// vectors' plane and off-plane coordinates, every word move the pivot makes
/// — is untouched by this, and only the carriage and the hue map fibre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormFace {
    pub i: u16,
    pub j: u16,
    /// `w_j ∘ w_i⁻¹`, one word for the face.
    pub word: Word,
    /// Carriage, hue map and 50.3 edge per member pair, keyed by
    /// `face_pair_key(chart_i, chart_j)`. Empty where the face records only
    /// its word — a section face pushed by `FormFace::new`.
    pub at: Keyed<FacePlace>,
    /// THE PAIRS OF THE BUNDLE THIS FACE'S WORD WAS MEASURED ON (21 step 2,
    /// S12.2). `None` is the unrestricted claim: nothing sectioned this
    /// face and it answers for every pair its two ends exist on. `Some(ps)`
    /// is a SECTION — the bundle spoke several words between these two ends
    /// and this face carries one of them, on the pairs it was spoken on.
    ///
    /// IT LIVES ON THE FACE BECAUSE NO KEY CAN REACH IT. The extractor kept
    /// these lists in `Form.sections` under `ends_pair_key(ea, eb)` and, for
    /// the alternates, that key with the word's key appended — a SECOND key
    /// family in a carrier whose only reader asks at `TransportClass::key()`
    /// (rule 6: one coordinate system, where a prefix is a stratum). Nothing
    /// read the face family, and nothing could have: `Form::regauge` rewrites
    /// EVERY face's word to `γ_j · w · γ_i⁻¹` at extraction step 7, and
    /// `rebase_named_faces` rewrites the star faces' words and turns a
    /// `Vertex::Word` into a `Vertex::Rebased` — after which `end_of`
    /// answers `None` and neither half of the key can be rebuilt. The key
    /// was stale by construction before the form ever left the extractor.
    ///
    /// Carried on the face, the section rides through both rewrites and
    /// through `retract_vertices` with the face it describes, and the court
    /// reads it where it stands.
    pub pairs: Option<Keyed<()>>,
}

/// A FRAME AS A COORDINATE (51.1, 51.8): the frame index along `e₃`,
/// written big-endian so the trie's order is the coordinate's order.
///
/// Little-endian would put 256 before 1, and a section enumerated in that
/// order is not the succession — `occupied_frames` already carries that
/// note for the fibers carrier, where the reading is sorted afterwards.
/// Here the key is the order, and nothing sorts.
pub fn frame_key(k: u16) -> [u8; 2] {
    k.to_be_bytes()
}

/// The frame a section's inner key names. `None` for a key that is not one.
pub fn frame_of_key(k: &[u8]) -> Option<u16> {
    Some(u16::from_be_bytes(k.try_into().ok()?))
}

/// The frames a section stands on, in succession. A reader that needs the
/// list — the tape, the form key — takes it here rather than keeping a
/// second copy beside the carrier (law 12).
pub fn frames_of_section(section: &Keyed<()>) -> Vec<u16> {
    section.scoped().filter_map(|(k, _)| frame_of_key(&k)).collect()
}

/// A section over the frames it stands on.
pub fn section_of_frames(frames: impl IntoIterator<Item = u16>) -> Keyed<()> {
    let mut out = Keyed::new();
    for k in frames {
        out.insert(frame_key(k), ());
    }
    out
}

impl FormFace {
    /// A face carrying only its word. The member pairs are filled by
    /// `Form::face_of` from the vertices, or by the incidence that created
    /// the face; they are never invented as the product of the two fibres.
    ///
    /// It claims every pair until something measures otherwise: a face with
    /// no section is the unrestricted claim, which is what every face made
    /// before sections existed.
    pub fn new(i: u16, j: u16, word: Word) -> FormFace {
        FormFace { i, j, word, at: Keyed::new(), pairs: None }
    }

    /// Is this face's word measured at `frame` of the family's one body
    /// (51.8)? A face that was never sectioned claims every frame, so it
    /// is; a sectioned face answers at the frames it names and at no
    /// others.
    ///
    /// A keyed read at the frame's own coordinate. It was
    /// `ps.contains(&(pair as u16))` — a walk of the whole list, once per
    /// face per frame in the court's edge replay.
    pub fn measured_on(&self, frame: usize) -> bool {
        match &self.pairs {
            None => true,
            Some(ps) => u16::try_from(frame).is_ok_and(|k| ps.contains(&frame_key(k))),
        }
    }

    /// What this face carries at one member pair. A keyed read: `None` is
    /// "this face does not relate those two bodies".
    pub fn place_at(&self, ci: &stalk::Invariant, cj: &stalk::Invariant) -> Option<&FacePlace> {
        self.at.get(&crate::transducer::sentence::face_pair_key(ci, cj))
    }

    /// 50.3's edge at one member pair; `None` is a remainder edge there, or a
    /// pair this face does not relate at all.
    pub fn edge_at(&self, ci: &stalk::Invariant, cj: &stalk::Invariant) -> Option<&Transport> {
        self.place_at(ci, cj)?.edge.as_ref()
    }

    /// Does any member pair of this face carry an edge? The reading a
    /// contraction makes when it asks whether a face is carried at all.
    pub fn has_edge(&self) -> bool {
        self.at.scoped().any(|(_, p)| p.edge.is_some())
    }

    /// Every member pair of this face becomes a remainder edge. A contraction
    /// that retracts a body cannot leave the edges that were measured on it.
    pub fn clear_edges(&mut self) {
        let pairs: Vec<Vec<u8>> = self.at.scoped().map(|(k, _)| k).collect();
        for k in pairs {
            if let Some(p) = self.at.get_mut(&k) {
                p.edge = None;
            }
        }
    }
}

/// Independent loop of a form: the closed vertex walk, the faces along it,
/// the chord (never a star face), and the oriented holonomy.
///
/// The HOLONOMY is a word and is the product of the faces' words around the
/// loop, which are vertex coordinates, so it is one word per loop and does
/// not fibre. The CARRIAGE is the product of the faces' carriages, which are
/// member coordinates, so it is one versor PER MEMBER PATH — the sequence of
/// charts the loop passes through, keyed by those charts in walk order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loop {
    /// THE CYCLE ITSELF, as an element of `Z₁(M)`: the MEMBER at each node,
    /// `member_key(vertex, chart)`, in walk order and closing on its first.
    ///
    /// `vertices` below is `π` of this — the projection that forgets the
    /// chart — and it is kept because every reader of a loop asks the form
    /// for a vertex. It is not the cycle. Under `π` a 1-cell between two
    /// charts AT ONE VERTEX, `((v,c_a),(v,c_b))`, collapses to the
    /// degenerate self-loop `(v,v)` and loses its orientation, which is
    /// exactly the defect that broke 62 of 62 live loops at `437d253`: the
    /// walker could not tell which way it ran because the projection had
    /// thrown the answer away.
    ///
    /// Carried, the cycle IS its own path. There is nothing to search for.
    pub members: Vec<Vec<u8>>,
    pub vertices: Vec<u16>,
    pub faces: Vec<u16>,
    pub chord: u16,
    pub holonomy: Word,
    /// Member path key → the carriage accumulated along it.
    pub carriages: Keyed<Versor>,
}

impl Loop {
    /// The key a member path is filed under: the chart key at each vertex of
    /// the loop, in walk order, each length-delimited. Content, so the same
    /// path reads the same on every node.
    pub fn path_key(charts: &[Vec<u8>]) -> Vec<u8> {
        let mut buf = Vec::new();
        for c in charts {
            crate::transducer::address::put_bytes(&mut buf, c);
        }
        buf
    }
}


/// A sentence as vertices, faces, loops, remainder.
#[derive(Debug, Clone)]
pub struct Form {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<FormFace>,
    pub loops: Vec<Loop>,
    /// A5's chain and the translator that places it (§2.1, 19). The proof
    /// rides the coordinate: a tape whose remainder puts two transitions on
    /// one site does not parse, and that refusal is the decoder's under
    /// `REFUSE_FORM_TAPE`, never a reader's downstream.
    ///
    /// `CanonicalChain` alone would centre it and lose the placement the
    /// Čech complex reads for overlap, which is the reading
    /// `two_bubbles_on_one_chart_share_an_edge_not_a_triangle` asserts.
    pub remainder: crate::transducer::element::PlacedChain,
    pub depth: u16,
    pub hue_maps: Keyed<Vec<(u8, u8)>>,
    pub ratio: ((usize, usize), (usize, usize)),
    /// THE SECTIONS ALONG `e₃` (51.8): class or member key → the FRAMES it
    /// stands on, each frame its own coordinate in the inner carrier.
    ///
    /// It was `Keyed<Vec<u16>>` and every reader asked it the same question
    /// — does this section claim frame `k` — with `ps.contains(&k)`, a walk
    /// of the whole list, once per vertex per member per frame in the
    /// court's replay. 51.8 rules the court's readers onto the coordinate:
    /// "the same refusal on a coordinate instead of on a list". Keyed by
    /// [`frame_key`] all the way down, the frame IS the coordinate and the
    /// question is a keyed read.
    ///
    /// The inner key is big-endian, so the trie's own order is the frame's
    /// order and a reader that enumerates a section — the tape, the form
    /// key — gets the frames in succession without sorting them (law 2).
    pub sections: Keyed<Keyed<()>, coord::MemberSection>,
    /// The certified pivots this form's crystallization rests on (A20).
    /// Evidence, replayed by the court; never part of the address.
    pub certificates: Keyed<PivotCertificate>,
    /// Filed torsion cycles, keyed by class key: `(σ, ℓ, chord face)`.
    /// THE FORM OWNS THIS CARRIER and there is no second copy of it: a reader
    /// holding an element reaches it through `sentence::form_torsion`, and an
    /// organ holding a word reads the form it decoded at admission
    /// (`Held::form`, L16). `Element::adopt_torsion` copied it into the
    /// element's own `torsion` field and is deleted.
    pub torsion: Keyed<FiledLoop, coord::Holonomy>,
    /// CHART AXES OPENED OVER HELD LOOPS (3.7, 15.5, 16.16 step 3; I6):
    /// the holonomy class opened on → the chord of the loop that is its
    /// section. The relation is all the form carries; the seed versor and
    /// A7's coordinate are re-derived from that loop on every read (law
    /// 12), never stored beside it. On the tape and in the address.
    pub axes: Keyed<u16, coord::Holonomy>,
    /// SITE → glued vertex index. Written by [`Form::glue`] and rebuilt by
    /// the decoder from Glued vertices.
    ///
    /// 54.7 B5a — THE DOC SAID "cited form address" AND THE CODE SAID SITE.
    /// The target left the vertex when it left `form_vertex_key`; this is
    /// keyed by `vertex_bytes`, the host's own boundary record, and the
    /// sentence that had drifted from the code is now the type.
    pub glued: Keyed<u16, coord::Site>,
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
    /// Word vertex address → index. The meet for A6 and Mayer–Vietoris.
    ///
    /// The address is `word_vertex_address`'s, which opens `TAG_FORM` — a
    /// MEMBER coordinate, strictly finer than the form's own address and
    /// coarser than a section (54.7 R5).
    pub by_address: Keyed<u16, coord::Member>,
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
    /// is that sentence where the compiler reads it (§4.4).
    pub by_class: Keyed<Vec<u16>, coord::Chart>,
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
    /// extraction walked every pair form's vertices and accumulated the
    /// correspondence itself — a second producer of one fact, beside the
    /// indexer whose job it is (law 12, 13.1).
    ///
    /// The key is `TransportClass::key()`'s own, taken from
    /// `family::transport_class(v)`, so a reader asks with the key it already
    /// builds and there is one definition of it.
    pub by_transport: Keyed<Vec<u16>, coord::Transport>,
    /// Vertex index key → Frame face index. Star incidence, one face per vertex.
    pub star: Keyed<u16, coord::VertexIx>,
    /// Word vertex address → carriage, filled at construction.
    pub carriages: Keyed<Versor>,
    /// Rebased vertex keyed by the `of` Word address — the same member
    /// coordinate `by_address` answers, over a different vertex kind.
    pub by_rebased: Keyed<u16, coord::Member>,
    /// Unfolded vertex keyed by its measure key.
    pub by_unfolded: Keyed<u16, coord::Unfolded>,
    /// Demand vertex keyed by (kind, chart) (50.5): the read that keeps a
    /// sentence to one demand vertex per pair. Written by `reindex`.
    pub demands: Keyed<u16, coord::DemandVertex>,
}

impl Default for Form {
    fn default() -> Self {
        Form {
            vertices: vec![Vertex::Frame],
            faces: Vec::new(),
            loops: Vec::new(),
            remainder: crate::transducer::element::PlacedChain::default(),
            depth: 0,
            hue_maps: Keyed::new(),
            ratio: ((1, 1), (1, 1)),
            sections: Keyed::new(),
            certificates: Keyed::new(),
            torsion: Keyed::new(),
            axes: Keyed::new(),
            glued: Keyed::new(),
            cover: Vec::new(),
            vortex: Keyed::new(),
            by_address: Keyed::new(),
            by_class: Keyed::new(),
            by_transport: Keyed::new(),
            star: Keyed::new(),
            carriages: Keyed::new(),
            by_rebased: Keyed::new(),
            by_unfolded: Keyed::new(),
            demands: Keyed::new(),
        }
    }
}

impl Form {
    /// The loop closing on `chord`, by index. A walk of this form's own
    /// loops — its boundary, never a search over anything else.
    pub(crate) fn loop_of_chord(&self, chord: u16) -> Option<usize> {
        for (i, lp) in self.loops.iter().enumerate() {
            if lp.chord == chord {
                return Some(i);
            }
        }
        None
    }

    /// The chord of the loop this loop's class was opened on, if any (I6).
    pub fn opened_axis_of(&self, lp: &Loop) -> Option<u16> {
        opened_axis_on(&self.faces, &self.vertices, &self.axes, lp)
    }

    /// Law 12: the opened axis is a theorem the loop proves — the loop is
    /// the one the relation names, and its holonomy is a versor, so the
    /// seed (A20 `OpenAxis`) re-derives from it. Otherwise the relation is
    /// a stored coordinate the form cannot re-derive, and the loop stays a
    /// defect.
    /// AN AXIS IS KEYED BY A CLASS, SO THE CLASS IS WHAT IDENTIFIES IT
    /// (rule 15, 54.5 as ruled 2026-09-20).
    ///
    /// This read `chord == lp.chord` beside the seed, and the chord is a
    /// FACE INDEX — a position, not a coordinate. `form_vertex_key` knows
    /// that and writes an opened axis as its CLASS alone; the decoder has
    /// nothing to put in the chord slot and writes `0`. So on every form
    /// that came out of an address the stored chord was a placeholder, and
    /// this compared it as though it were real: the axis re-derived only
    /// when the loop's chord happened to be face zero, and otherwise a form
    /// that opened an axis was refused for it.
    ///
    /// `opened_axis_of` already finds the axis BY THE LOOP'S OWN CLASS,
    /// which is content. Two loops of one class are one obstruction and one
    /// axis serves both — that is what keying by class means — so there is
    /// nothing left for an index to disambiguate. What remains is the real
    /// question: does this loop have a holonomy for the axis to be the
    /// transition OF (16.20).
    pub fn axis_rederives(&self, lp: &Loop, _chord: u16) -> bool {
        self.axis_seed_of(lp).is_some()
    }

    /// The seed of the axis a loop opens: its own holonomy versor (16.20),
    /// which is the EDGE COMPOSITE where its paths carry one. Read off
    /// `lp.holonomy` — the word — it seeded the axis from a coboundary that
    /// telescopes to the identity, so the seed was the identity versor and
    /// the axis it opened stood for nothing the loop turns by. See
    /// [`crate::transducer::sentence::pivot::drive::loop_holonomy_on`].
    pub fn axis_seed_of(&self, lp: &Loop) -> Option<Versor> {
        element::versor_of_word(&self.loop_holonomy(lp)?)
    }

    /// THE HOLONOMY EVERY MEMBER PATH OF THIS LOOP AGREES ON, on this form.
    /// One reader for the whole `OpenAxis` pipeline — the gate, the seed,
    /// the class the axis is keyed by, and the row the audit reads — so the
    /// writer and the reader cannot key one carrier by two objects, which is
    /// exactly the split R8 removed from `file_loop`.
    pub(crate) fn loop_holonomy(&self, lp: &Loop) -> Option<Word> {
        crate::transducer::sentence::pivot::drive::loop_holonomy_on(
            &self.faces,
            &self.vertices,
            lp,
        )
    }

    /// Open a chart axis over the loop closing on `chord` (A20 `OpenAxis`).
    /// Refuses a chord with no loop, a holonomy with no class, and a
    /// holonomy with no versor to seed the axis.
    pub(crate) fn open_axis_on(&mut self, chord: u16) -> Option<()> {
        let i = self.loop_of_chord(chord)?;
        let lp = self.loops.get(i)?;
        let class = self.loop_holonomy(lp)?.rotor().holonomy_key()?;
        self.axis_seed_of(lp)?;
        self.axes.put(&class, chord);
        Some(())
    }

    /// The inverse move: the axis opened over the loop closing on `chord`
    /// leaves the form (the court's un-pivot, A20 step 3). Keyed by the same
    /// composite the opening used, or it removes nothing and the axis stands
    /// with no loop claiming it.
    pub(crate) fn close_axis_on(&mut self, chord: u16) -> Option<()> {
        let i = self.loop_of_chord(chord)?;
        let lp = self.loops.get(i)?;
        let class = self.loop_holonomy(lp)?.rotor().holonomy_key()?;
        self.axes.drop_at(&class)?;
        Some(())
    }

    /// The row one loop contributes to the rank audit (A18): an opened
    /// axis, or what its holonomy is; `None` when the holonomy has no class.
    pub fn loop_row_at(&self, idx: usize) -> Option<crate::transducer::sentence::LoopRow> {
        loop_row_on(&self.faces, &self.vertices, &self.axes, self.loops.get(idx)?)
    }

    pub fn loop_rows(&self) -> Option<Vec<crate::transducer::sentence::LoopRow>> {
        (0..self.loops.len()).map(|i| self.loop_row_at(i)).collect()
    }

    /// A18 over this form's loops. `None` when a holonomy has no class or
    /// the ℤ-rank overflows: unmeasured, and the caller refuses.
    pub fn rank(&self) -> Option<crate::transducer::sentence::Rank> {
        crate::transducer::sentence::rank_of_rows(&self.loop_rows()?)
    }
}

impl Form {
    /// Fill by_address, by_class, star, carriages (by transport-class key)
    /// and hue_maps from the vertices and faces, in one pass over the
    /// form's own vertices. Two gauges on one word class are two hue
    /// sections, each kept. Two carriages on one transport class stay two
    /// sections: that class is refused as one carriage. Two Word vertices
    /// on one chart are both on that chart's fiber (24.1).
    pub fn reindex(&mut self) {
        let mut by_address: Keyed<u16, coord::Member> = Keyed::new();
        let mut by_class: Keyed<Vec<u16>, coord::Chart> = Keyed::new();
        let mut by_transport: Keyed<Vec<u16>, coord::Transport> = Keyed::new();
        let mut by_rebased: Keyed<u16, coord::Member> = Keyed::new();
        let mut by_unfolded: Keyed<u16, coord::Unfolded> = Keyed::new();
        let mut demands: Keyed<u16, coord::DemandVertex> = Keyed::new();
        // `glued` IS BUILT HERE TOO. `Form::glue` wrote it as it appended each
        // vertex and the tape decoder rebuilt it in a walk of its own; now that
        // `depth_of` reads it, whoever builds it is the indexer and that is
        // this pass (law 12).
        let mut glued: Keyed<u16, coord::Site> = Keyed::new();
        let mut hue_maps = std::mem::take(&mut self.hue_maps);
        let mut places: Keyed<Versor> = Keyed::new();
        let mut star: Keyed<u16, coord::VertexIx> = Keyed::new();
        for (i, v) in self.vertices.iter().enumerate() {
            match v {
                Vertex::Word { address, derivation, acts, .. } => {
                    let rotor = derivation.rotor();
                    let Ok(member_key) = Key::<coord::Member>::from_address(address) else {
                        continue;
                    };
                    by_address.put(&member_key, i as u16);
                    // THE TRANSPORT CLASS IS THE VERTEX'S, not a member's, so
                    // this files the vertex once however many charts it acts
                    // on. A rotor with no class has no transport class (15.3)
                    // and is filed under none: no key of its bytes stands in.
                    if let Some(tk) = crate::transducer::family::transport_class(v).map(|c| c.key())
                    {
                        // 21 step 1's own key, declared as the transport
                        // class it is. `TransportClass::key` is the one
                        // writer of these bytes, so this is the one place
                        // they become a coordinate of that system.
                        let tk = Key::<coord::Transport>::declared(&tk);
                        match by_transport.at_mut(&tk) {
                            Some(on) => on.push(i as u16),
                            None => {
                                by_transport.put(&tk, vec![i as u16]);
                            }
                        }
                    }
                    // EVERY CHART THIS VERTEX ACTS ON is a chart it stands
                    // at, so `by_class` files it under each of them. One
                    // member is the single-chart case and reads as before.
                    for (_, a) in acts.scoped() {
                        // KEYED BY THE CHART IT INDEXES (54.7 R1). It used
                        // the member's own key, which WAS the chart until
                        // the fibre had to separate its sections; now the
                        // key is `chart ‖ carriage` and a reader asking
                        // `by_class` for a chart found nothing. An index is
                        // keyed by the coordinate it answers, not by
                        // whatever the carrier it was built from used.
                        //
                        // 54.7 R5 — AND NOW THE COMPILER READS IT. `ck` is
                        // written as a chart coordinate here or this vertex
                        // stands on no chart at all; the member's key is a
                        // different type and cannot reach this carrier.
                        let Ok(chart_key) = Key::<coord::Chart>::from_address(&a.chart.key())
                        else {
                            continue;
                        };
                        let ck = chart_key.as_bytes().to_vec();
                        match by_class.at_mut(&chart_key) {
                            Some(on) => {
                                if !on.contains(&(i as u16)) {
                                    on.push(i as u16);
                                }
                            }
                            None => {
                                by_class.put(&chart_key, vec![i as u16]);
                            }
                        }
                        // `hue_maps` and `carriages` are DERIVATIONS OF THE
                        // FIBRE and are rebuilt from it here, or they are a
                        // second carrier for the fact the fibre now holds
                        // (law 12, D7) — the same fault as keeping the three
                        // fields beside it, one level up.
                        if let Some(hk) = hue_section_key_of(&rotor, &a.gauge) {
                            hue_maps.insert(hk, a.gauge.clone());
                        }
                        // 18.2 FINALLY TRUE OF ITS OWN TYPE. The key is the
                        // transport class AND the chart, so two carriages on
                        // one class are two entries — two sections — where
                        // before they were one entry refused by
                        // `place_conflict` and replaced by nothing. The
                        // comment here claimed the class "stays two
                        // sections" and no section was ever written; it is
                        // written now, and `place_conflict` is gone with the
                        // claim it could not keep.
                        if let Some(tk) = vertex_transport_key_of(&rotor, &a.gauge) {
                            let mut key = tk.into_vec();
                            crate::transducer::address::put_bytes(&mut key, &ck);
                            places.insert(key, (*a.carriage).clone());
                        }
                    }
                }
                Vertex::Rebased { of, .. } => {
                    if let Some(Vertex::Word { address, .. }) = self.vertices.get(*of as usize) {
                        if let Ok(k) = Key::<coord::Member>::from_address(address) {
                            by_rebased.put(&k, i as u16);
                        }
                    }
                }
                Vertex::Unfolded { measure } => {
                    by_unfolded.put(&unfolded_key(measure), i as u16);
                }
                Vertex::Demand { kind, chart, .. } => {
                    demands.put(
                        &crate::transducer::sentence::demand_key(kind, chart),
                        i as u16,
                    );
                }
                // 54.7 B5a — KEYED BY THE SITE, which is what the vertex
                // now holds. It was keyed by the cited address, and that
                // address is the cover's fact on `Form::cover`.
                v @ Vertex::Glued { .. } => {
                    glued.put(
                        &Key::<coord::Site>::declared(
                            crate::transducer::sentence::vertex_bytes(v),
                        ),
                        i as u16,
                    );
                }
                _ => {}
            }
        }
        // EVERY PLACE IS A SECTION AND NONE IS REFUSED. The key is the
        // transport class AND the chart, so two carriages on one class are
        // two entries rather than one entry conflicting with itself, and
        // 18.2's sentence is finally the type's. `place_conflict` marked
        // the conflict, dropped the class from `carriages` entirely, and
        // wrote no section anywhere — a local that died exactly as
        // `after_chain` does — so the one index keyed by transport class
        // went empty on precisely the classes the defect was about.
        let mut carriages = Keyed::new();
        for (k, p) in places.scoped() {
            carriages.insert(&k, p.clone());
        }
        // ONE STAR FACE PER VERTEX: the rule's face, the first between the
        // Frame and that vertex. A later face between the same two is a
        // section over some pairs (21 step 2, S12.2) and stays as the
        // bundle spoke it; the rebase pass re-derives star faces from their
        // vertices, and a section it re-derived was a section erased.
        for (fi, f) in self.faces.iter().enumerate() {
            let key = if f.i == 0 {
                vertex_ix_key(f.j)
            } else if f.j == 0 {
                vertex_ix_key(f.i)
            } else {
                continue;
            };
            if !star.holds(&key) {
                star.put(&key, fi as u16);
            }
        }
        self.by_address = by_address;
        self.by_class = by_class;
        self.by_transport = by_transport;
        self.by_rebased = by_rebased;
        self.by_unfolded = by_unfolded;
        self.demands = demands;
        self.glued = glued;
        self.star = star;
        self.carriages = carriages;
        self.hue_maps = hue_maps;
    }
}

// `impl Vertex` — `word_in`, `class`, `depth` — was here and is now
// `form/vertex.rs`. `word_in` WALKS the rebase chain instead of
// recursing on it (L20), and this file went over the §1.1 reading bound
// with the walk in it. Moved, not rewritten (§1.6).


impl Form {

    /// THE WORD BETWEEN TWO VERTICES: `w_j ∘ w_i⁻¹`, read from their
    /// derivations. A vertex coordinate, so it is the same at every member
    /// pair of the face.
    pub fn face_word(&self, i: u16, j: u16) -> Option<Word> {
        let wa = self.vertices.get(i as usize)?.word_in(self)?;
        let wb = self.vertices.get(j as usize)?.word_in(self)?;
        Some(wb.compose(&wa.inverse()).normal_form())
    }

    /// WHAT THE FACE CARRIES AT ONE NAMED MEMBER PAIR: the relative carriage
    /// of the body on `cj` against the body on `ci` under the face's word, and
    /// the relative hue map between their two gauges. The edge is not derived
    /// here — 50.3's edge is measured on the union of the two bodies by
    /// whatever created the face, and re-deriving a placement must not erase
    /// it, so it is carried in by the caller.
    pub fn face_place(
        &self,
        i: u16,
        ci: &stalk::Invariant,
        j: u16,
        cj: &stalk::Invariant,
        edge: Option<Transport>,
    ) -> Option<FacePlace> {
        self.face_place_by(i, ci, j, cj, &self.face_word(i, j)?, edge)
    }

    /// The same placement UNDER A NAMED WORD. A section face (21 step 2, S12.2)
    /// carries a word the bundle spoke between these two ends that is NOT the
    /// coboundary of their placements, and its carriage is the relative under
    /// THAT word — computing it under the coboundary word would place the
    /// section by the face it is a section of.
    pub fn face_place_by(
        &self,
        i: u16,
        ci: &stalk::Invariant,
        j: u16,
        cj: &stalk::Invariant,
        word: &Word,
        edge: Option<Transport>,
    ) -> Option<FacePlace> {
        use crate::transducer::sentence::transport::{vertex_carriage_at, vertex_gauge_at};
        let va = self.vertices.get(i as usize)?;
        let vb = self.vertices.get(j as usize)?;
        let ca = vertex_carriage_at(va, self, ci)?;
        let cb = vertex_carriage_at(vb, self, cj)?;
        let wv = element::versor_of_word(word)?;
        let inv = wv.versor_inverse()?;
        let conjugated = wv.geo(&ca).geo(&inv);
        let carriage = cb.geo(&conjugated.versor_inverse()?);
        let gauge = relative_gauge(&vertex_gauge_at(va, ci), &vertex_gauge_at(vb, cj));
        Some(FacePlace { carriage, gauge, edge })
    }

    /// THE FACE BETWEEN TWO VERTICES, AT THE MEMBER PAIRS THE INCIDENCE NAMES.
    ///
    /// `pairs` is the incidence: which member of `i` relates to which member
    /// of `j`. It comes from the site occupancy that created the face
    /// (`star_and_site_faces`) or from the pairs a face already carries when
    /// its placements are re-derived. IT IS NEVER THE PRODUCT OF THE TWO
    /// FIBRES — a face exists because two BODIES' supports meet, and that
    /// incidence is between bodies, so it says which member relates to which.
    ///
    /// `None` when the word does not read; a member pair whose placement does
    /// not derive is left out of the fibre rather than given a versor it does
    /// not have (rule 11).
    pub fn face_at(
        &self,
        i: u16,
        j: u16,
        pairs: &[(stalk::Invariant, stalk::Invariant)],
    ) -> Option<FormFace> {
        self.face_at_by(i, j, pairs, &self.face_word(i, j)?)
    }

    /// The same face UNDER A NAMED WORD, for a section face: the word is the
    /// one the bundle spoke between these ends, and the placements are the
    /// relatives under it.
    pub fn face_at_by(
        &self,
        i: u16,
        j: u16,
        pairs: &[(stalk::Invariant, stalk::Invariant)],
        word: &Word,
    ) -> Option<FormFace> {
        crate::transducer::family::trace::trace("face_at", crate::transducer::family::trace::Wrote::Key(&vertex_ix_key(j)), None);
        let mut at = Keyed::new();
        for (ci, cj) in pairs {
            if let Some(p) = self.face_place_by(i, ci, j, cj, word, None) {
                at.insert(crate::transducer::sentence::face_pair_key(ci, cj), p);
            }
        }
        note_unplaced(i, j, word, pairs.len(), at.len());
        Some(FormFace { i, j, word: word.clone(), at, pairs: None })
    }

    /// THE MEMBER PAIRS A FACE BETWEEN TWO VERTICES HAS where one endpoint
    /// stands at a single chart — the star case, and every face this tree makes
    /// outside a cover's meets. `None` where both carry several: the pairing is
    /// the scene's fact and this does not have it.
    pub fn face_pairs(&self, i: u16, j: u16) -> Option<Vec<(stalk::Invariant, stalk::Invariant)>> {
        let ci = self.vertices.get(i as usize)?.charts();
        let cj = self.vertices.get(j as usize)?.charts();
        match (ci.as_slice(), cj.as_slice()) {
            ([one], many) => Some(many.iter().map(|c| (one.clone(), c.clone())).collect()),
            (many, [one]) => Some(many.iter().map(|c| (c.clone(), one.clone())).collect()),
            _ => None,
        }
    }

    /// A FACE CARRYING A NAMED WORD, placed at the member pairs its two
    /// vertices name. This is what `FormFace::new` was for — a section face, or
    /// a chord — except that `new` recorded the word and left the placement
    /// empty, and a face with no placement contributes no member path to any
    /// loop, so every loop through it lost its carriage.
    pub fn face_carrying(&self, i: u16, j: u16, word: Word) -> Option<FormFace> {
        self.face_carrying_with_edge(i, j, word, None)
    }

    /// A FACE CARRYING A NAMED WORD AND AN EDGE TRANSPORT.
    pub fn face_carrying_with_edge(
        &self,
        i: u16,
        j: u16,
        word: Word,
        edge: Option<Transport>,
    ) -> Option<FormFace> {
        match self.face_pairs(i, j) {
            Some(pairs) => {
                crate::transducer::family::trace::trace("face_at", crate::transducer::family::trace::Wrote::Key(&vertex_ix_key(j)), None);
                let mut at = Keyed::new();
                for (ci, cj) in &pairs {
                    if let Some(p) = self.face_place_by(i, ci, j, cj, &word, edge.clone()) {
                        at.insert(crate::transducer::sentence::face_pair_key(ci, cj), p);
                    }
                }
                note_unplaced(i, j, &word, pairs.len(), at.len());
                Some(FormFace { i, j, word, at, pairs: None })
            }
            None => {
                let mut f = FormFace::new(i, j, word);
                let ends = (
                    self.vertices.get(i as usize).and_then(|v| v.face_chart()),
                    self.vertices.get(j as usize).and_then(|v| v.face_chart()),
                );
                if let (Some(ci), Some(cj)) = &ends {
                    if let Some(p) = self.face_place_by(i, ci, j, cj, &f.word, edge) {
                        f.at.insert(crate::transducer::sentence::face_pair_key(ci, cj), p);
                    }
                }
                note_unplaced(i, j, &f.word, usize::from(matches!(ends, (Some(_), Some(_)))), f.at.len());
                Some(f)
            }
        }
    }

    /// THE STAR FACE OF A VERTEX, or any face one of whose endpoints stands at
    /// a single chart. There the incidence needs no reading: one side has one
    /// member, so each member of the other side relates to it and to nothing
    /// else. The Frame is the case that matters — every star face is
    /// Frame→vertex and the Frame stands at the root stratum alone.
    ///
    /// `None` where BOTH endpoints carry several members: the pairing is then
    /// a fact about the scene and this function does not have it. Callers with
    /// an incidence use `face_at`.
    pub fn face_of(&self, i: u16, j: u16) -> Option<FormFace> {
        self.face_at(i, j, &self.face_pairs(i, j)?)
    }

    pub fn delta1(&self) -> Vec<(u16, Word)> {
        self.loops
            .iter()
            .enumerate()
            .map(|(i, lp)| (i as u16, lp.holonomy.clone()))
            .collect()
    }

}

/// §8.1, measurement only: a face whose word has no sensor action places
/// no member pair — `face_place_by` derives each pair's carriage through
/// `versor_of_word`, one plane by construction (51.2) — and every loop
/// through it has no member path and no class. Said once per face, against
/// the pairs the incidence named.
fn note_unplaced(i: u16, j: u16, word: &Word, named: usize, placed: usize) {
    if named > 0 && placed == 0 && element::versor_of_word(word).is_none() {
        eprintln!("  face {i}-{j} placed no member pair: the word has no sensor action");
    }
}

/// WHAT A FORM CITES, at both levels (26 as ruled 2026-09-10): a `Glued`
/// vertex cites a whole form, and a word vertex's derivation cites a
/// `Word` vertex inside one (50.2 step 2). In vertex order, with no
/// deduplication — a form cited twice is cited twice, and the caller
/// that counts uses decides what a repeat means.
///
/// One carrier for the reading. The court's use-credit walked the
/// vertices for it and so did the retirement scope; two walks of one fact
/// are two chances to disagree about what a citation is (law 12).
pub fn cited_addresses(f: &Form) -> Vec<crate::transducer::address::Address> {
    let mut out = Vec::new();
    // CHART CITING CHART, off the cover (54.7 B5a). It walked the `Glued`
    // vertices, and a vertex no longer names the form it mates with; the
    // 1-skeleton of `N(𝒰)` does.
    for e in &f.cover {
        out.push(e.target.clone());
    }
    for v in &f.vertices {
        if let Vertex::Word { derivation, .. } = v {
            for (form_addr, _, _) in &derivation.generators {
                out.push(form_addr.clone());
            }
        }
    }
    out
}

/// THE AXIS CARRIER AND THE LOOP, without the form between them. A loop's
/// opened axis is a keyed read of the form's axis carrier at the loop's own
/// class, so a trial holding that carrier reads it directly (I6).
/// THE FACES AND VERTICES ARE THE CARRIER, not decoration on the signature.
/// The axis is keyed by the loop's holonomy CLASS, and a loop's holonomy is
/// its edge composite wherever a path carries one — which cannot be read
/// from `lp` alone, because `lp.holonomy` is the word and the word
/// telescopes to the identity on any coboundary cycle. Read there, this
/// answered `axes.get(identity_class)` for every loop on the corpus and the
/// opening was invisible to the audit that has to count it.
pub(crate) fn opened_axis_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    axes: &Keyed<u16, coord::Holonomy>,
    lp: &Loop,
) -> Option<u16> {
    let class = crate::transducer::sentence::pivot::drive::loop_holonomy_on(faces, vertices, lp)?
        .rotor()
        .holonomy_key()?;
    axes.at(&class).copied()
}

/// The row one loop contributes to the rank audit (A18), read off the axis
/// carrier and the loop's own holonomy — the edge composite, the same object
/// the axis is keyed by and the class vector is read from.
pub(crate) fn loop_row_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    axes: &Keyed<u16, coord::Holonomy>,
    lp: &Loop,
) -> Option<crate::transducer::sentence::LoopRow> {
    // The class the axis was opened under IS the row's coordinate, so it is
    // carried: `rank_of_rows` counts axes by class, and one opening over a
    // class shared by two loops is one axis, not two.
    if opened_axis_on(faces, vertices, axes, lp).is_some() {
        let class = crate::transducer::sentence::pivot::drive::loop_holonomy_on(faces, vertices, lp)?
            .rotor()
            .class_key()?;
        return Some(crate::transducer::sentence::LoopRow::Opened(class));
    }
    // The row is the obstruction's, so it is read off the obstruction the
    // rest of this pipeline names. A loop whose paths disagree has no single
    // holonomy and therefore no row (rule 11), which is the same refusal
    // `loop_row` makes for a holonomy that is not a versor.
    crate::transducer::sentence::loop_row(&crate::transducer::sentence::pivot::drive::loop_holonomy_on(
        faces, vertices, lp,
    )?)
}

pub(crate) fn form_face_step(faces: &[FormFace], fi: usize, from: usize, to: usize) -> Option<Word> {
    let f = faces.get(fi)?;
    if f.i as usize == from && f.j as usize == to {
        Some(f.word.clone())
    } else {
        Some(f.word.inverse())
    }
}

pub(crate) fn form_named_holonomy(faces: &[FormFace], path: &[usize], walked: &[usize]) -> Word {
    let mut w = Word::identity();
    for (win, &fi) in path.windows(2).zip(walked) {
        if let Some(s) = form_face_step(faces, fi, win[0], win[1]) {
            w = w.compose(&s);
        }
    }
    w.normal_form()
}

/// The tree path from `src` to `dst` as its vertices and the faces walked
/// between them, named in the same pass that finds them: each vertex is
/// reached through one face of its own adjacency, so no face is searched
/// for afterwards.
pub(crate) fn tree_path(adj: &[Vec<(usize, usize)>], src: usize, dst: usize) -> Option<(Vec<usize>, Vec<usize>)> {
    if src == dst {
        return Some((vec![src], Vec::new()));
    }
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; adj.len()];
    prev[src] = Some((src, 0));
    let mut q = vec![src];
    let mut qi = 0usize;
    while qi < q.len() {
        let u = q[qi];
        qi += 1;
        if u == dst {
            break;
        }
        for &(v, fi) in &adj[u] {
            if prev[v].is_some() {
                continue;
            }
            prev[v] = Some((u, fi));
            q.push(v);
        }
    }
    prev[dst]?;
    let mut path = vec![dst];
    let mut walked = Vec::new();
    let mut cur = dst;
    while cur != src {
        let (p, fi) = prev[cur]?;
        walked.push(fi);
        path.push(p);
        cur = p;
    }
    path.reverse();
    walked.reverse();
    Some((path, walked))
}
