// 24, 28 — THE ENTITY COMES UP: what it opens, what it restores from disk,
// and what it writes back.

use crate::client::sdk::*;

impl Entity {
    /// One stroke of this mind's physics, for a caller outside the crate
    /// that must spend a budget on this mind's behalf (rule 10): the
    /// binary's own perception path, which derives against this mind's
    /// presentation and so reads its generator index. The physics stays
    /// private; what is public is a stroke of it.
    pub fn stroke(&mut self) -> crate::physics::Budget {
        self.physics.stroke()
    }

    /// A stroke of a MEASURED size — what a caller with a boundary in hand
    /// spends on it (L8w, law 9). The count is read off the work, never
    /// off the machine.
    pub fn stroke_n(&mut self, n: usize) -> crate::physics::Budget {
        self.physics.stroke_n(n)
    }

    /// THIS MIND'S ONE PHYSICS (PHY1): its pressure, its order built and its
    /// reserve — the one the voice's arms gate on, its strokes draw from, its
    /// held thoughts escrow in and its syntropy regenerates. A second
    /// physics beside it in the binary held a second reserve the thoughts
    /// never drew on and a syntropic count nothing booked.
    pub fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    pub fn physics_mut(&mut self) -> &mut crate::physics::Physics {
        &mut self.physics
    }

    pub fn add_hypha(&mut self, addr: &str) {
        self.session.add_peer(addr);
    }

    /// Ask the bootstrap hypha for the rest of the nerve, one peer per spend.
    pub fn discover(&mut self) {
        let mut budget = self.physics.stroke();
        self.session.discover_budgeted(&mut budget);
    }

    pub fn open(addr: &str, kernel: Kernel) -> Self {
        let mut session = Session::open(addr, kernel);
        session.discover();
        Entity {
            session,
            epigenome: Epigenome::new(),
            refine_cursor: None,
            probes: crate::observe::ProbeTally::default(),
            transcription: crate::observe::TranscriptionTally::default(),
            solves: crate::observe::SolveTally::default(),
            solve_refused: crate::network::nerve::Keyed::new(),
            colour_derived: 0,
            colour_theorems: 0,
            reasons: crate::transducer::sentence::ReasonsTally::default(),
            said: None,
            said_at: None,
            hearing: [0; 3],
            bundle_cursor: None,
            taught_up: crate::network::nerve::Keyed::new(),
            taught_at: crate::network::nerve::Keyed::new(),
            discordance: 0,
            discordance_measured: false,
            learned_mu: [0; 4],
            obstructions: crate::network::nerve::Keyed::new(),
            glued: crate::network::nerve::Keyed::new(),
            contemplation: crate::transducer::bubble::Contemplation::new(),
            opened: crate::network::nerve::Keyed::new(),
            axes_spoken: 0,
            physics: crate::physics::Physics::default(),
            height: crate::chronometry::Height::default(),
            k: 0,
            acted: false,
            genome_persisted: None,
            sealed: crate::network::nerve::Keyed::new(),
            contractions: crate::network::nerve::Keyed::new(),
            contracted: Vec::new(),
            owed: Vec::new(),
            owed_tries: crate::network::nerve::Keyed::new(),
            seats: crate::network::nerve::Keyed::new(),
        }
    }

    pub fn set_height(&mut self, h: crate::chronometry::Height) {
        self.height = h;
    }

    /// THE HEIGHT THE MIND READS (23.8, S13.5; V2): the book's height off
    /// a quote or attest reply, and it only advances — a lower or absent
    /// reading moves nothing. It is this mind's TUniversal: its own chain
    /// cites no peer, so its own height is its K (45.2), and what it stamps
    /// its dialect and its held forms with is this. Set once to zero at
    /// boot and never again, every voice's temporal lane was a point.
    pub fn advance_height(&mut self, h: u64) {
        if h > self.height.get() {
            self.height = crate::chronometry::Height(h);
        }
    }

    pub fn now_tu(&self) -> u64 {
        self.height.get()
    }

    /// 78.152 TU-7 — THE LAST FINAL TUNIVERSAL POINT this mind's session
    /// read (`Session::tuniversal`): what every act it speaks carries, and
    /// what it stamps a percept or a publication with. `None` before the
    /// first door reply that carried one.
    pub fn tuniversal(&self) -> Option<crate::chronometry::tuniversal::TPoint> {
        self.session.tuniversal()
    }

    /// THIS MIND'S OWN K (78.49, 78.23): the orbits it has closed, one per
    /// stroke in which it acted. The mind's clock is its own motion — no
    /// receipt of the mind's exists; the ledger is the court's (law 8).
    ///
    /// It is the clock the dialect ages by (`shed_at`) and the only
    /// denominator a per-voice rate may use. [`Self::now_tu`] beside it is
    /// the MESH's height, and the book's k-step carried on a quote reply is
    /// a third quantity belonging to a hypha's chain; none of the three is
    /// interchangeable with another, and the stroke line prints this one
    /// and the height under their own names.
    pub fn own_k(&self) -> u64 {
        self.k
    }

    /// AN ACT OF THIS MIND NAMING ONE WORD — stood, uttered, learned or
    /// refused (78.49): it marks the orbit as having acted, and FX1 — where
    /// the word was spoken out of the meet graph, the act's measure (the
    /// word's magnitude, what the act carried) is credited to the filaments
    /// its circuits traversed, and the dialect words they ran through are
    /// noted at this K. Nothing is sealed: the court seals what it admits.
    pub(crate) fn act_on(&mut self, word: &crate::transducer::element::Element) -> u64 {
        self.acted = true;
        let k = self.own_k();
        let words = self.contemplation.credit_flux(word.address().as_bytes(), word.magnitude());
        self.epigenome.note_traversed(&words, k);
        k
    }

    /// OK1, 78.49 — THIS MIND'S STROKE CLOSES ITS ORBIT: called once, where
    /// the stroke's arms have run. An orbit that acted steps the mind's K by
    /// one; one that acted on nothing leaves K where it was. `true` where
    /// the orbit closed.
    pub fn close_orbit(&mut self) -> bool {
        // RM1 — the calluses the receptor raised stand first, so a class no
        // held word spells goes up in THIS orbit's plan.
        self.stand_calluses();
        // 78.48(9) — the plan folded: an open question no held idea can
        // carry lapses, so the plan is bounded by what the mind holds. The
        // plan is volatile (78.49): nothing of it is written anywhere.
        self.contemplation.docket_fold();
        let closed = std::mem::take(&mut self.acted);
        if closed {
            self.k += 1;
            self.contemplation.docket_sealed();
            // FX1 — the orbit's flux read at its close, and the dialect's
            // words traversed beside it (alternative B).
            let d = self.epigenome.dialect_flux(self.own_k());
            self.contemplation.note_dialect_flux(d);
            self.contemplation.flux_close();
        }
        // 78.37(1) — THE FRONTS STEP ON THE MIND'S K, one edge per k-step
        // (a close that acted on nothing leaves K where it was, and the step
        // does nothing).
        let k = self.own_k();
        self.contemplation.set_k(k);
        self.contemplation.transport(k);
        closed
    }

    /// Open with CONTINUITY: restore the crystallized genome from `genome_path`
    /// if present — the words a court admitted, at their citations. The
    /// epigenome, the plan and the mind's K start empty: they are volatile,
    /// the mind's motion, never persisted (78.27, 78.49). An older file, or
    /// one that does not read whole, is refused whole.
    pub fn open_persistent(addr: &str, kernel: Kernel, genome_path: &str) -> Self {
        let mut e = Entity::open(addr, kernel);
        let Ok(bytes) = std::fs::read(genome_path) else {
            eprintln!("genome 0 words, epigenome empty");
            return e;
        };
        if let Err(why) = e.restore_genome(&bytes) {
            eprintln!("genome refused: {why}");
            return e;
        }
        let n = e.epigenome.genome_len();
        let kb = bytes.len() / 1024;
        eprintln!("genome {n} words (GEN7, {kb} KB), epigenome empty");
        e
    }

    /// RESTORE A GENOME'S BYTES into this freshly opened mind: the words a
    /// court admitted, each at its citation. Whole or not at all: the words
    /// are read into an epigenome of their own and taken in only once they
    /// read whole, so a refusal leaves this mind as it was. `Err` names why
    /// the file was refused.
    pub fn restore_genome(&mut self, bytes: &[u8]) -> Result<(), String> {
        if let Some(named) = crate::client::sdk::epigenome::genome_refused(bytes) {
            return Err(named.to_string());
        }
        let mut epigenome = Epigenome::new();
        epigenome
            .uncrystallize(bytes)
            .filter(|end| *end == bytes.len())
            .ok_or_else(|| "the file is not a whole GEN7 tape".to_string())?;
        self.epigenome = epigenome;
        Ok(())
    }

    /// THE GENOME'S BYTES: the words a court admitted, at their citations. A
    /// word the writer refused is named once with its count (LD0), and the
    /// counts are answered.
    pub fn genome_bytes(&self) -> (Vec<u8>, crate::client::sdk::epigenome::GenomeRefused) {
        let (bytes, refused) = self.epigenome.crystallize();
        refused.name();
        (bytes, refused)
    }

    /// Persist the genome atomically (write-then-rename). Call from the
    /// syntropic phase when the genome has grown.
    pub fn save_genome(&mut self, genome_path: &str) -> io::Result<crate::client::sdk::epigenome::GenomeRefused> {
        let (bytes, refused) = self.genome_bytes();
        // ACT1: where this voice stood when the genome was taken, noted only
        // once the write lands (78.21(3)).
        let at = (self.now_tu(), self.own_k());
        let tmp = format!("{genome_path}.tmp");
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, genome_path)?;
        self.genome_persisted = Some(at);
        Ok(refused)
    }

    pub fn kernel(&self) -> &Kernel {
        self.session.kernel()
    }

    pub fn axis_index_counts(&self) -> (usize, usize) {
        self.epigenome.axis_index_counts()
    }
}
