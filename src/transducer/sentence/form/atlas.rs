// AT1's READING (78.41(2), (2′)) — WHAT THE FORM HOLDS AS A TABLE, read
// before the atlas replaces it: the vertices that are charts, the faces as
// overlaps, the circuits recomposed, the searches at their sites, the
// frame-axis overlaps the atlas would add, and the bytes the ruled overlap
// keys would cost. Counters only — nothing here holds a coordinate, so the
// tally cannot grow with what it reads (R6).

/// ONE FORM'S ATLAS READING, read off its carriers at the percept's end.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FormAtlas {
    /// (r1) vertices standing on a receptor chart — Word, Rebased, Glued —
    /// and those that do not — Frame, Demand, Unfolded.
    pub charts: u64,
    pub non_charts: u64,
    /// (r7b) the atlas's charts at frame depth: every Word member at every
    /// frame it stands on, `class ‖ Invariant ‖ k` — summed per form, never
    /// identified across families (the frame is family-local).
    pub chart_frames: u64,
    /// (r2) faces with one end at the Frame; faces from a vertex to itself;
    /// SECTION faces, a later face on an ordered pair already joined; faces
    /// relating no member pair; and over the member pairs the faces relate,
    /// those whose two members stand on ONE chart Invariant against those
    /// that share none — the slot artefact the row names.
    pub star_faces: u64,
    pub self_faces: u64,
    pub section_faces: u64,
    pub unnamed_faces: u64,
    pub pairs_one_chart: u64,
    pub pairs_no_chart: u64,
    /// (r3) each held orbit's holonomy recomposed along its faces, equal to
    /// the one it holds or not; the orbits held against the member
    /// complex's cycle rank `E − rank ∂`, by the exact-rank reader.
    pub holonomy_equal: u64,
    pub holonomy_differ: u64,
    pub orbits: u64,
    pub cycle_rank: u64,
    /// AT1's ACCEPTANCE (ruled 2026-09-25): the orbits by kind — WITHIN one
    /// frame (every chart of the circuit at one frame, the old basis's
    /// per-frame lifts) and ACROSS frames (a frame-axis overlap on the
    /// circuit, the circuits AT1 exists for) — each by whether its holonomy
    /// turns (not the identity ray) or is flat.
    pub within_turned: u64,
    pub within_flat: u64,
    pub across_turned: u64,
    pub across_flat: u64,
    /// AT1's ACCEPTANCE (78.46(5)(b)–(c), ruled (A)): THE OLD BASIS,
    /// DEDUCED — the member complex with frames forgotten, each member pair
    /// one edge and each frameless pair a refused edge — its cycles read by
    /// kind: PRESENT (its pairs share a frame and its class key is among the
    /// atlas's circuits'), FRAMELESS (it crosses a frameless pair) carried
    /// ACROSS frames (its class among the across-frame circuits') or NOT,
    /// ACROSS (no frame shared and no frameless pair), LOST (a shared frame
    /// and its class not found), UNREAD (no class read on it), and the SLOT
    /// ARTEFACT (a face with no member pair), which no atlas carries.
    pub old_cycles: u64,
    pub old_present: u64,
    pub old_frameless_across: u64,
    pub old_frameless_not: u64,
    pub old_across: u64,
    pub old_lost: u64,
    pub old_unread: u64,
    pub old_slot: u64,
    /// (r6) the ruled overlap keys — `framed(class ‖ Invariant ‖ frame)` at
    /// each end — their count, total and largest bytes, against the form
    /// address's bytes.
    pub overlaps: u64,
    pub overlap_bytes: u64,
    pub overlap_max: u64,
    /// Member pairs whose frames read on neither member, or whose ends have
    /// no ruled coordinate: unread, never costed at a guess.
    pub overlap_unread: u64,
    pub address_bytes: u64,
}

/// THE ATLAS TALLY: the forms' readings summed, the largest form's (r6)
/// kept whole, and the searches and frame-axis steps counted at their
/// sites.
#[derive(Clone, Debug, Default)]
pub struct AtlasTally {
    pub forms: u64,
    pub sum: FormAtlas,
    /// The form with the largest address, whole: r6 is read against it.
    pub largest: FormAtlas,
    /// (r4) `tree_path` calls and the nodes they visited; the carry's
    /// candidate rotors tried; the anchor's cuts and the candidates each
    /// dropped; `listings.rs`'s least-cycle choices and the cycles they
    /// chose among.
    pub tree_paths: u64,
    pub tree_nodes: u64,
    pub carry_candidates: u64,
    pub anchor_cuts: u64,
    pub anchor_dropped: u64,
    pub listing_choices: u64,
    pub listing_among: u64,
    /// (r5) MT1's member steps — the frame-axis overlaps the atlas would
    /// add — and the bytes their ruled keys would cost.
    pub frame_axis_steps: u64,
    pub frame_axis_bytes: u64,
}

thread_local! {
    static ATLAS: std::cell::RefCell<AtlasTally> = std::cell::RefCell::new(AtlasTally::default());
}

fn with(f: impl FnOnce(&mut AtlasTally)) {
    ATLAS.with(|t| f(&mut t.borrow_mut()));
}

/// THE FRAME'S SPELLING (AT1 Q4): a length byte, then the minimal
/// big-endian bytes — unbounded, and ordered by (length, bytes), which is
/// the numeric order the carrier reads the succession by (law 2).
pub fn frame_spelling(k: u64) -> Vec<u8> {
    let mut out = Vec::new();
    crate::network::nerve::coord::put_frame(k, &mut out);
    out
}

/// One ruled chart coordinate, framed: the vertex's coordinate, the chart
/// Invariant, the frame's spelling.
pub fn chart_coordinate(vertex: &[u8], chart: &[u8], frame: u64, out: &mut Vec<u8>) {
    let mut c = vertex.to_vec();
    crate::transducer::address::put_key_bytes(&mut c, chart);
    c.extend_from_slice(&frame_spelling(frame));
    crate::transducer::address::put_key_bytes(out, &c);
}

/// One form read: summed, and kept whole where its address is the largest.
pub fn note_form(f: FormAtlas) {
    with(|t| {
        t.forms += 1;
        let s = &mut t.sum;
        s.charts += f.charts;
        s.non_charts += f.non_charts;
        s.chart_frames += f.chart_frames;
        s.star_faces += f.star_faces;
        s.self_faces += f.self_faces;
        s.section_faces += f.section_faces;
        s.unnamed_faces += f.unnamed_faces;
        s.pairs_one_chart += f.pairs_one_chart;
        s.pairs_no_chart += f.pairs_no_chart;
        s.holonomy_equal += f.holonomy_equal;
        s.holonomy_differ += f.holonomy_differ;
        s.orbits += f.orbits;
        s.cycle_rank += f.cycle_rank;
        s.within_turned += f.within_turned;
        s.within_flat += f.within_flat;
        s.across_turned += f.across_turned;
        s.across_flat += f.across_flat;
        s.old_cycles += f.old_cycles;
        s.old_present += f.old_present;
        s.old_frameless_across += f.old_frameless_across;
        s.old_frameless_not += f.old_frameless_not;
        s.old_across += f.old_across;
        s.old_lost += f.old_lost;
        s.old_unread += f.old_unread;
        s.old_slot += f.old_slot;
        s.overlaps += f.overlaps;
        s.overlap_bytes += f.overlap_bytes;
        s.overlap_unread += f.overlap_unread;
        s.address_bytes += f.address_bytes;
        if f.overlap_max > s.overlap_max {
            s.overlap_max = f.overlap_max;
        }
        if f.address_bytes > t.largest.address_bytes {
            t.largest = f;
        }
    });
}

/// (r4) One `tree_path` call and the nodes it visited.
pub fn note_tree_path(nodes: usize) {
    with(|t| {
        t.tree_paths += 1;
        t.tree_nodes += nodes as u64;
    });
}

/// (r4) One candidate rotor the carry tried.
pub fn note_carry_candidate() {
    with(|t| t.carry_candidates += 1);
}

/// (r4) One anchor derivation cut to its thirty-two, and how many it dropped.
pub fn note_anchor_cut(dropped: usize) {
    with(|t| {
        t.anchor_cuts += 1;
        t.anchor_dropped += dropped as u64;
    });
}

/// (r4) One least-cycle choice in the listings, among `among` cycles.
pub fn note_listing_choice(among: usize) {
    with(|t| {
        t.listing_choices += 1;
        t.listing_among += among as u64;
    });
}

/// (r5) One member step along the frame axis, and its ruled key's bytes.
pub fn note_frame_axis_step(bytes: usize) {
    with(|t| {
        t.frame_axis_steps += 1;
        t.frame_axis_bytes += bytes as u64;
    });
}

/// The tally, without clearing (the per-percept running line).
pub fn atlas_tally() -> AtlasTally {
    ATLAS.with(|t| t.borrow().clone())
}

/// The tally, cleared: the corpus's one reader.
pub fn take_atlas_tally() -> AtlasTally {
    ATLAS.with(|t| t.replace(AtlasTally::default()))
}

impl FormAtlas {
    /// The old basis, deduced, by kind (78.46(5)).
    pub fn old_basis_line(&self) -> String {
        format!(
            "old basis: {} cycles | present {} | frameless {} (across {} / not {}) | across {} | lost {} | unread {} | slot artefact {}",
            self.old_cycles,
            self.old_present,
            self.old_frameless_across + self.old_frameless_not,
            self.old_frameless_across,
            self.old_frameless_not,
            self.old_across,
            self.old_lost,
            self.old_unread,
            self.old_slot
        )
    }

    /// One form's reading as its line's segment.
    pub fn line(&self) -> String {
        format!(
            "vertices: charts {} non-charts {} | charts at frames {} | faces: star {} self {} section {} unnamed {} | member pairs: one chart {} no chart {} \
             | holonomy recomposed: equal {} differ {} | orbits {} against cycle rank {} \
             | circuits within a frame: {} turned {} flat, across frames: {} turned {} flat \
             | overlap keys: {} of {} bytes, largest {}, unread {} | address {} bytes",
            self.charts,
            self.non_charts,
            self.chart_frames,
            self.star_faces,
            self.self_faces,
            self.section_faces,
            self.unnamed_faces,
            self.pairs_one_chart,
            self.pairs_no_chart,
            self.holonomy_equal,
            self.holonomy_differ,
            self.orbits,
            self.cycle_rank,
            self.within_turned,
            self.within_flat,
            self.across_turned,
            self.across_flat,
            self.overlaps,
            self.overlap_bytes,
            self.overlap_max,
            self.overlap_unread,
            self.address_bytes
        )
    }
}

impl AtlasTally {
    /// The searches at their sites and the frame-axis steps.
    pub fn searches_line(&self) -> String {
        format!(
            "searches: tree_path {} calls {} nodes | carry candidates {} | anchor cuts {} dropping {} | listing choices {} among {} \
             | frame-axis overlaps available {} of {} key bytes",
            self.tree_paths,
            self.tree_nodes,
            self.carry_candidates,
            self.anchor_cuts,
            self.anchor_dropped,
            self.listing_choices,
            self.listing_among,
            self.frame_axis_steps,
            self.frame_axis_bytes
        )
    }

    /// THE CORPUS LINE: the forms' sums, the largest form whole (r6 read
    /// against it), and the searches.
    pub fn line(&self) -> String {
        format!(
            "atlas over {} forms: {} | largest form: {} | {}",
            self.forms,
            self.sum.line(),
            self.largest.line(),
            self.searches_line()
        )
    }
}
