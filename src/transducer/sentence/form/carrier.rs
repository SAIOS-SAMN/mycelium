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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiledLoop {
    /// The hue-map generator: the permutation one step around the loop.
    pub sigma: Vec<(u8, u8)>,
    /// The period it closes at — the order of the loop's holonomy; `0` is
    /// infinite order, as A19 spells it on the class ledger (52.3).
    pub period: u32,
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
    /// The key this member is filed under: its chart's own.
    pub fn key(&self) -> Vec<u8> {
        self.chart.key().into_vec()
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
        acts: Keyed<Acting>,
        /// Stop ring. A form coordinate with the carriage and gauge; not M1.
        contact: Option<alphabet::Contact>,
    },
    Glued {
        address: crate::transducer::address::Address,
        chart: stalk::Invariant,
        magnitude: [u64; 4],
        depth: u16,
        placement: Word,
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
        acts: Keyed<Acting>,
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
    pub pairs: Option<Vec<u16>>,
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

    /// Is this face's word measured on `pair` of the bundle? A face that
    /// was never sectioned claims them all, so it is; a sectioned face
    /// answers for the pairs it names and for no others.
    pub fn measured_on(&self, pair: usize) -> bool {
        match &self.pairs {
            None => true,
            Some(ps) => ps.contains(&(pair as u16)),
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
    pub sections: Keyed<Vec<u16>>,
    /// The certified pivots this form's crystallization rests on (A20).
    /// Evidence, replayed by the court; never part of the address.
    pub certificates: Keyed<PivotCertificate>,
    /// Filed torsion cycles, keyed by class key: `(σ, ℓ, chord face)`.
    /// THE FORM OWNS THIS CARRIER and there is no second copy of it: a reader
    /// holding an element reaches it through `sentence::form_torsion`, and an
    /// organ holding a word reads the form it decoded at admission
    /// (`Held::form`, L16). `Element::adopt_torsion` copied it into the
    /// element's own `torsion` field and is deleted.
    pub torsion: Keyed<FiledLoop>,
    /// CHART AXES OPENED OVER HELD LOOPS (3.7, 15.5, 16.16 step 3; I6):
    /// the holonomy class opened on → the chord of the loop that is its
    /// section. The relation is all the form carries; the seed versor and
    /// A7's coordinate are re-derived from that loop on every read (law
    /// 12), never stored beside it. On the tape and in the address.
    pub axes: Keyed<u16>,
    /// Cited form address → glued vertex index. Written by [`Form::glue`]
    /// and rebuilt by the decoder from Glued vertices.
    pub glued: Keyed<u16>,
    /// Incomparable discharges held as candidates until a glue or ruling.
    /// Outer key is the after class; inner is the discharging rotor A1 key.
    pub vortex: Keyed<Keyed<PivotCertificate>>,
    /// Word vertex address → index. The meet for A6 and Mayer–Vietoris.
    pub by_address: Keyed<u16>,
    /// Chart key → the Word vertices on that chart (18.2): the fiber a
    /// listing body's chart reads when a cite asks what this form holds
    /// there (24.1 as ruled 2026-09-09). Derived by `reindex`, never on
    /// the tape.
    pub by_class: Keyed<Vec<u16>>,
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
    pub by_transport: Keyed<Vec<u16>>,
    /// Vertex index key → Frame face index. Star incidence, one face per vertex.
    pub star: Keyed<u16>,
    /// Word vertex address → carriage, filled at construction.
    pub carriages: Keyed<Versor>,
    /// Rebased vertex keyed by the `of` Word address.
    pub by_rebased: Keyed<u16>,
    /// Unfolded vertex keyed by its measure key.
    pub by_unfolded: Keyed<u16>,
    /// Demand vertex keyed by (kind, chart) (50.5): the read that keeps a
    /// sentence to one demand vertex per pair. Written by `reindex`.
    pub demands: Keyed<u16>,
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
    pub fn axis_rederives(&self, lp: &Loop, chord: u16) -> bool {
        chord == lp.chord && self.axis_seed_of(lp).is_some()
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
        let class = self.loop_holonomy(lp)?.rotor().class_key()?;
        self.axis_seed_of(lp)?;
        self.axes.insert(class, chord);
        Some(())
    }

    /// The inverse move: the axis opened over the loop closing on `chord`
    /// leaves the form (the court's un-pivot, A20 step 3). Keyed by the same
    /// composite the opening used, or it removes nothing and the axis stands
    /// with no loop claiming it.
    pub(crate) fn close_axis_on(&mut self, chord: u16) -> Option<()> {
        let i = self.loop_of_chord(chord)?;
        let lp = self.loops.get(i)?;
        let class = self.loop_holonomy(lp)?.rotor().class_key()?;
        self.axes.remove(&class)?;
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
        let mut by_address = Keyed::new();
        let mut by_class: Keyed<Vec<u16>> = Keyed::new();
        let mut by_transport: Keyed<Vec<u16>> = Keyed::new();
        let mut by_rebased = Keyed::new();
        let mut by_unfolded = Keyed::new();
        let mut demands = Keyed::new();
        // `glued` IS BUILT HERE TOO. `Form::glue` wrote it as it appended each
        // vertex and the tape decoder rebuilt it in a walk of its own; now that
        // `depth_of` reads it, whoever builds it is the indexer and that is
        // this pass (law 12).
        let mut glued = Keyed::new();
        let mut hue_maps = std::mem::take(&mut self.hue_maps);
        let mut places: Keyed<Versor> = Keyed::new();
        let mut star = Keyed::new();
        for (i, v) in self.vertices.iter().enumerate() {
            match v {
                Vertex::Word { address, derivation, acts, .. } => {
                    let rotor = derivation.rotor();
                    by_address.insert(address, i as u16);
                    // THE TRANSPORT CLASS IS THE VERTEX'S, not a member's, so
                    // this files the vertex once however many charts it acts
                    // on. A rotor with no class has no transport class (15.3)
                    // and is filed under none: no key of its bytes stands in.
                    if let Some(tk) = crate::transducer::family::transport_class(v).map(|c| c.key())
                    {
                        match by_transport.get_mut(&tk) {
                            Some(on) => on.push(i as u16),
                            None => {
                                by_transport.insert(&tk, vec![i as u16]);
                            }
                        }
                    }
                    // EVERY CHART THIS VERTEX ACTS ON is a chart it stands
                    // at, so `by_class` files it under each of them. One
                    // member is the single-chart case and reads as before.
                    for (ck, a) in acts.scoped() {
                        match by_class.get_mut(&ck) {
                            Some(on) => on.push(i as u16),
                            None => {
                                by_class.insert(&ck, vec![i as u16]);
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
                        by_rebased.insert(address, i as u16);
                    }
                }
                Vertex::Unfolded { measure } => {
                    by_unfolded.insert(unfolded_key(measure), i as u16);
                }
                Vertex::Demand { kind, chart, .. } => {
                    demands.insert(crate::transducer::sentence::demand_key(kind, chart), i as u16);
                }
                Vertex::Glued { address, .. } => {
                    glued.insert(address, i as u16);
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
            if !star.contains(&key) {
                star.insert(key, fi as u16);
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
        crate::transducer::family::trace::trace_at("face_at", j as usize);
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
                crate::transducer::family::trace::trace_at("face_at", j as usize);
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

    /// `δ¹g = 1` on every loop (10.3), where a loop whose class is an axis
    /// the form opened is coherent on the refined cover: its holonomy is
    /// the transition on that axis, re-derived from the loop (I6, 15.5).
    pub fn coheres(&self) -> bool {
        self.loops.iter().all(|lp| {
            (lp.holonomy.rotor().projective_eq(&Rotor::identity())
                || self.opened_axis_of(lp).is_some_and(|chord| self.axis_rederives(lp, chord)))
                && self.edge_coheres(lp)
        })
    }

    /// Closed: coheres, no remainder, and no demand vertex (2.2, 50.1);
    /// a sentence with demands is valid, never closed.
    pub fn is_closed(&self) -> bool {
        self.coheres() && self.remainder.is_empty() && self.demand_vertices().next().is_none()
    }

    /// The demand vertices of this sentence, with their indices (50.5),
    /// read off `demands`, which `reindex` built: one entry per (kind,
    /// chart), a keyed read and never a walk with a filter.
    pub fn demand_vertices(&self) -> impl Iterator<Item = (u16, &crate::transducer::sentence::Demand, &stalk::Invariant, &[Link])> {
        self.demands.scoped().filter_map(move |(_, &i)| {
            match self.vertices.get(i as usize) {
                Some(Vertex::Demand { kind, chart, bodies }) => {
                    Some((i, kind, chart, bodies.as_slice()))
                }
                _ => None,
            }
        })
    }

    /// One more than the deepest glued vertex; zero where none is glued.
    ///
    /// READS `glued`, WHICH `reindex` BUILT: the same walk with a filter, and
    /// the index is keyed by the cited address, so a glued vertex reachable by
    /// the address it cites is reachable here.
    pub fn depth_of(&self) -> u16 {
        let mut d = 0u16;
        for (_, &i) in self.glued.scoped() {
            if let Some(Vertex::Glued { depth, .. }) = self.vertices.get(i as usize) {
                d = d.max(depth.saturating_add(1));
            }
        }
        d
    }

    /// S9.13'S REACH — the span this form's fibre spans ABOVE ITS BASE, which
    /// is what E2 prices as a closure's yield. One filed cycle of period ℓ
    /// reaches ℓ−1 members past the one it opens on, and speaks every one of
    /// them in O(1) storage: that is the capacity a condensation earns.
    ///
    /// THE FORM IS THE CARRIER, so this is where the reading lives. A minted
    /// sentence's element leaves its own two fibre fields empty because the
    /// cycles are on the form, so `Element::fibre_volume` answers 0 for it and
    /// is not this reading — asking the element was how the instrument came to
    /// report 1 everywhere by construction.
    ///
    /// A FORM HAS NO FREE-SECTION CARRIER and none is substituted. `sections`
    /// here is `Keyed<Vec<u16>>` holding PAIR INDICES of the bundle — which
    /// pairs a class was measured on — where `Element.sections` holds
    /// transition ASSIGNMENTS over a base. Two objects, and summing the first
    /// as the second is the fault a rename row is owed on. A form's free part
    /// is absent, not zero.
    ///
    /// A finite period is exact and at least one (A17 — a loop is filed at
    /// the period it CLOSES at), so the subtraction is total; a class of
    /// infinite order, filed at `0` (52.3), condenses no members and reaches
    /// nothing here. No rounding, nothing saturating: this is a sum over an
    /// exact carrier, and an empty carrier sums to 0 because it is empty.
    pub fn fibre_volume(&self) -> usize {
        self.torsion
            .scoped()
            .map(|(_, l)| match Order::from_u32(l.period) {
                Order::Finite(ell) => ell as usize - 1,
                Order::Infinite => 0,
            })
            .sum()
    }

    pub fn is_cauterized(&self) -> bool {
        if !self.remainder.is_empty() || self.demand_vertices().next().is_some() {
            return false;
        }
        for lp in &self.loops {
            if lp.holonomy.is_identity() {
                continue;
            }
            if let Some(chord) = self.opened_axis_of(lp) {
                if self.axis_rederives(lp, chord) {
                    continue;
                }
                return false;
            }
            // Filed under its class at the period its order reads, `0` for
            // infinite order (52.3, A19). A holonomy that is not a versor
            // has no class (15.3) and nothing files it: the form is not
            // cauterized. No key of the rotor's bytes stands in for the
            // class it lacks.
            let Some(k) = lp.holonomy.rotor().class_key() else {
                return false;
            };
            let ell = lp.holonomy.order().to_u32();
            match self.torsion.get(&k) {
                Some(filed) if filed.period == ell => {}
                _ => return false,
            }
        }
        true
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
    for v in &f.vertices {
        match v {
            Vertex::Glued { address, .. } => out.push(address.clone()),
            Vertex::Word { derivation, .. } => {
                for (form_addr, _, _) in &derivation.generators {
                    out.push(form_addr.clone());
                }
            }
            _ => {}
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
    axes: &Keyed<u16>,
    lp: &Loop,
) -> Option<u16> {
    let class = crate::transducer::sentence::pivot::drive::loop_holonomy_on(faces, vertices, lp)?
        .rotor()
        .class_key()?;
    axes.get(&class).copied()
}

/// The row one loop contributes to the rank audit (A18), read off the axis
/// carrier and the loop's own holonomy — the edge composite, the same object
/// the axis is keyed by and the class vector is read from.
pub(crate) fn loop_row_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    axes: &Keyed<u16>,
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
