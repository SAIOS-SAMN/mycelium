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
            genome_persisted: None,
            sealed: crate::network::nerve::Keyed::new(),
            contractions: crate::network::nerve::Keyed::new(),
            contracted: Vec::new(),
            owed: Vec::new(),
            owed_tries: crate::network::nerve::Keyed::new(),
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

    /// THIS MIND'S OWN K (45.4) — `Seat::k`, the next index on its own
    /// receipt chain, which counts the acts this voice had admitted and
    /// nothing else.
    ///
    /// It is the clock the dialect ages by (`shed_at`) and the only
    /// denominator a per-voice rate may use. [`Self::now_tu`] beside it is
    /// the MESH's height, and the book's k-step carried on a quote reply is
    /// a third quantity belonging to a hypha's chain; none of the three is
    /// interchangeable with another, and the stroke line prints this one
    /// and the height under their own names.
    pub fn own_k(&self) -> u64 {
        self.session.voice().k()
    }

    /// Open with CONTINUITY: restore the crystallized genome from `genome_path`
    /// if present (GEN4) — its words with their admitting stamps, and this
    /// voice's own ledger behind them, so its K continues across a boot
    /// rather than restarting at zero. The epigenome starts empty. An older
    /// file, or one whose ledger does not verify, is refused whole.
    pub fn open_persistent(addr: &str, kernel: Kernel, genome_path: &str, _thoughts_path: &str) -> Self {
        let mut e = Entity::open(addr, kernel);
        let Ok(bytes) = std::fs::read(genome_path) else {
            eprintln!("genome 0 words, epigenome empty");
            return e;
        };
        let Some(read) = e.epigenome.uncrystallize(&bytes) else {
            eprintln!("genome refused: the file is not a whole GEN4 tape");
            return e;
        };
        let mut pos = read.end;
        let restored = crate::chronometry::ReceiptChain::read(&bytes, &mut pos)
            .filter(|_| pos == bytes.len())
            .is_some_and(|chain| e.session.voice_mut().restore(chain, read.admitted));
        if !restored {
            eprintln!("genome refused: this voice's ledger does not verify under its key");
            return Entity::open(addr, e.session.kernel().clone());
        }
        let n = e.epigenome.genome_len();
        let kb = bytes.len() / 1024;
        eprintln!("genome {n} words (GEN4, {kb} KB), K {}, epigenome empty", e.own_k());
        e
    }

    /// Persist the genome atomically (write-then-rename): the words with
    /// their admitting stamps, then this voice's ledger on the ledger's own
    /// writer. Call from the syntropic phase when the genome has grown.
    pub fn save_genome(&mut self, genome_path: &str) -> io::Result<()> {
        let voice = self.session.voice();
        let mut bytes = self.epigenome.crystallize(|a| voice.admitted_at(a));
        if voice.chain().write(&mut bytes).is_none() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "genome refused: a node of this voice's ledger cannot be written"));
        }
        // ACT1: where this voice stood when the genome was taken, noted only
        // once the write lands (78.21(3)).
        let at = (self.now_tu(), self.own_k());
        let tmp = format!("{genome_path}.tmp");
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, genome_path)?;
        self.genome_persisted = Some(at);
        Ok(())
    }

    /// Persist open thoughts atomically.
    pub fn save_thoughts(&self, thoughts_path: &str) -> io::Result<()> {
        let tmp = format!("{thoughts_path}.tmp");
        let mut bytes = Vec::new();
        self.contemplation.to_bytes(&mut bytes);
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, thoughts_path)
    }

    pub fn kernel(&self) -> &Kernel {
        self.session.kernel()
    }

    pub fn axis_index_counts(&self) -> (usize, usize) {
        self.epigenome.axis_index_counts()
    }
}
