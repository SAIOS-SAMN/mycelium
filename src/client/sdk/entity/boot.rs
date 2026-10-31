// 24, 28 — THE ENTITY COMES UP: what it opens, what it restores from disk,
// and what it writes back.

use crate::client::sdk::*;

impl Entity {
    /// One stroke of this mind's physics, for a caller outside the crate
    /// that must spend a budget on this mind's behalf (rule 10): the
    /// binary's own perception path, which derives against this mind's
    /// presentation and so reads its generator index. The physics stays
    /// private; what is public is a stroke of it.
    pub(crate) fn stroke(&mut self) -> crate::physics::Budget {
        self.physics.stroke()
    }

    /// A stroke of a MEASURED size — what a caller with a boundary in hand
    /// spends on it (L8w, law 9). The count is read off the work, never
    /// off the machine.
    pub(crate) fn stroke_n(&mut self, n: usize) -> crate::physics::Budget {
        self.physics.stroke_n(n)
    }

    /// THIS MIND'S ONE PHYSICS (PHY1): its pressure, its order built and its
    /// reserve — the one the voice's arms gate on, its strokes draw from, its
    /// held thoughts escrow in and its syntropy regenerates. A second
    /// physics beside it in the binary held a second reserve the thoughts
    /// never drew on and a syntropic count nothing booked.
    pub(crate) fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    pub(crate) fn physics_mut(&mut self) -> &mut crate::physics::Physics {
        &mut self.physics
    }

    /// Ask the bootstrap hypha for the rest of the nerve, one peer per spend,
    /// on the carrier.
    pub fn discover(&mut self) {
        let mut budget = self.physics.stroke();
        self.ask(move |s| s.discover_budgeted(&mut budget), |_, ()| {});
    }

    /// THE MIND OPENED toward the hypha at `addr`: its carrier discovers the
    /// nerve on its own thread before it carries the first ask.
    pub(crate) fn open(addr: &str, kernel: Kernel) -> Self {
        // The complex listens from the first pivot (MU-CLIENT §4.5): every
        // tether condensed and face sealed winds its strain into the field.
        let mut contemplation = crate::transducer::bubble::Contemplation::new();
        contemplation.complex.arm_outflow();
        // The seat's carrier lands on the mind's own waker: a house's percept
        // wakes the resting mind that is to take it up.
        let hypha = super::hypha::Hypha::open(Session::open(addr, kernel.clone()));
        let seat = super::hypha::Hypha::open_beside(Session::open(addr, kernel.clone()), hypha.waker());
        // The held mail read on a carrier of its own, so it never stands in
        // front of a take, a solve or a declination.
        let held = super::hypha::Hypha::open_beside(Session::open(addr, kernel), hypha.waker());
        Entity {
            seat,
            held,
            hypha,
            epigenome: Epigenome::new(),
            solve_refused: crate::network::nerve::Keyed::new(),
            taught_up: crate::network::nerve::Keyed::new(),
            taught_at: crate::network::nerve::Keyed::new(),
            discordance: 0,
            discordance_measured: false,
            learned_mu: [0; 4],
            obstructions: crate::network::nerve::Keyed::new(),
            glued: crate::network::nerve::Keyed::new(),
            contemplation,
            physics: crate::physics::Physics::default(),
            height: crate::chronometry::Height::default(),
            k: 0,
            at_return: crate::network::nerve::Keyed::new(),
            comprehending: crate::network::nerve::Keyed::new(),
            comprehended_after: Vec::new(),
            genome_persisted: None,
            sealed: crate::network::nerve::Keyed::new(),
            contractions: crate::network::nerve::Keyed::new(),
            contracted: Vec::new(),
            owed: Vec::new(),
            owed_tries: crate::network::nerve::Keyed::new(),
            unheard: crate::network::nerve::Keyed::new(),
            speaking: crate::network::nerve::Keyed::new(),
            unread_solves: crate::network::nerve::Keyed::new(),
            unread_after: Vec::new(),
            seats: crate::network::nerve::Keyed::new(),
            chat: None,
            board: Default::default(),
            field: Default::default(),
            remembered: crate::network::nerve::Keyed::new(),
            questions: crate::network::nerve::Keyed::new(),
            scenes: Default::default(),
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
    #[cfg(test)]
    pub(crate) fn advance_height(&mut self, h: u64) {
        if h > self.height.get() {
            self.height = crate::chronometry::Height(h);
        }
    }

    /// 78.152 TU-7 — THE MIND KEEPS TIME BY TUNIVERSAL: the height of the
    /// last final point its door replies carried (`Session::tuniversal`).
    /// Before the first, the height it was set at boot — never a book's
    /// height read off a quote, which is another coordinate.
    pub(crate) fn now_tu(&self) -> u64 {
        match self.hypha.tuniversal() {
            Some(p) => p.h,
            None => self.height.get(),
        }
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
    /// and the height, each under its own name.
    pub(crate) fn own_k(&self) -> u64 {
        self.k
    }

    /// AN ACT OF THIS MIND — a word stood, uttered, learned or refused
    /// (78.49): it stands at the mind's K, which it answers. An act moves no
    /// clock: K moves by the orbits that return (`close_cycle`). Nothing is
    /// sealed: the court seals what it admits.
    pub(crate) fn act(&mut self) -> u64 {
        self.own_k()
    }

    /// OK1, 78.49, MU-CLIENT §4.4(4) — THE STROKE CLOSES BY ITS RETURNS:
    /// `returns` is how many of the mind's orbits came round in it — orbits
    /// of its field that returned, thoughts whose winding completed a
    /// return. K steps by exactly that, and a stroke in which nothing
    /// returned leaves K where it was: the mind's clock is its own completed
    /// orbits, never its acts. `true` where K moved.
    pub(crate) fn close_cycle(&mut self, returns: u64) -> bool {
        // RM1 — the calluses the receptor raised stand first, so a class no
        // held word spells goes up in THIS orbit's plan.
        self.stand_calluses();
        // 78.48(9) — the plan folded: an open question no held idea can
        // carry lapses, so the plan is bounded by what the mind holds. The
        // plan is volatile (78.49): nothing of it is written anywhere.
        self.contemplation.docket_fold();
        // A count past its carrier is refused, and K stands where it was.
        let closed = returns > 0 && crate::physics::metabolism::step_or_refuse(&mut self.k, returns);
        if closed {
            self.contemplation.docket_sealed();
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
    /// if present — the words a court admitted, each at its citation. The
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
    /// are read into a separate epigenome and taken in only once they
    /// read whole, so a refusal leaves this mind as it was. `Err` names why
    /// the file was refused.
    pub(crate) fn restore_genome(&mut self, bytes: &[u8]) -> Result<(), String> {
        if let Some(named) = crate::client::sdk::epigenome::genome_refused(bytes) {
            return Err(named.to_string());
        }
        let mut epigenome = Epigenome::new();
        epigenome
            .uncrystallize(bytes)
            .filter(|end| *end == bytes.len())
            .ok_or_else(|| "the file is not a whole GEN7 tape".to_string())?;
        // A file holding what the genome leaves out (residual chains, rewrites)
        // is written again at the first stroke.
        self.board.genome_unwritten = !epigenome.restored_residuals.is_empty() || epigenome.restored_dropped > 0;
        // MU-CLIENT §7.1 — the residual chains the file held are open
        // questions, handed to the mind's carrier, never words.
        for (addr, chain, stratum) in std::mem::take(&mut epigenome.restored_residuals) {
            self.place_question(&addr, &chain);
            self.board.open_residuals.insert(&addr, chain);
            self.board.residual_strata.insert(&stratum, ());
        }
        self.epigenome = epigenome;
        Ok(())
    }

    /// THE GENOME'S BYTES: the words a court admitted, each at its citation. A
    /// word the writer refused is named once with its count (LD0), and the
    /// counts are answered.
    pub(crate) fn genome_bytes(&self) -> (Vec<u8>, crate::client::sdk::epigenome::GenomeRefused) {
        let (bytes, refused) = self.epigenome.crystallize();
        refused.name();
        (bytes, refused)
    }

    /// Persist the genome atomically (write-then-rename). Call from the
    /// syntropic phase when the genome has grown.
    pub(crate) fn save_genome(&mut self, genome_path: &str) -> io::Result<crate::client::sdk::epigenome::GenomeRefused> {
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

    #[cfg(test)]
    pub(crate) fn kernel(&self) -> &Kernel {
        self.hypha.kernel()
    }
}
