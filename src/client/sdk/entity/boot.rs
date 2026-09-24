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

    /// OK1 — THIS MIND'S STROKE CLOSES ITS ORBIT BY NAME (see
    /// [`crate::voice::Voice::orbit`]): called once, where the stroke's
    /// arms have run.
    pub fn close_orbit(&mut self) -> crate::chronometry::Close {
        let close = self.session.voice_mut().orbit();
        // 78.37(1) — THE FRONTS STEP ON THE VOICE'S K, one edge per k-step:
        // stamped once the close has sealed, and stepped at that K (a close
        // that sealed nothing leaves K where it was, and the step does nothing).
        let k = self.own_k();
        self.contemplation.set_k(k);
        self.contemplation.transport(k);
        close
    }

    /// Open with CONTINUITY: restore the crystallized genome from `genome_path`
    /// if present (GEN6) — its words at their citations, and this voice's
    /// own ledger behind them, so its K continues across a boot rather than
    /// restarting at zero. The epigenome starts empty: it is volatile, in
    /// memory and never persisted (78.27). An older file, one whose ledger
    /// does not verify, or one with a word admitted on this voice's line at
    /// a stamp its ledger does not hold, is refused whole.
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
        eprintln!("genome {n} words (GEN6, {kb} KB), K {}, epigenome empty", e.own_k());
        e
    }

    /// RESTORE A GENOME'S BYTES into this freshly opened mind: the words at
    /// their citations, then this voice's ledger. Whole or not at all: the
    /// words are read into an epigenome of their own and taken in only once
    /// the ledger has restored, so a refusal leaves this mind as it was.
    /// `Err` names why the file was refused.
    pub fn restore_genome(&mut self, bytes: &[u8]) -> Result<(), String> {
        if let Some(named) = crate::client::sdk::epigenome::genome_refused(bytes) {
            return Err(named.to_string());
        }
        let mut epigenome = Epigenome::new();
        let read = epigenome
            .uncrystallize(bytes)
            .ok_or_else(|| "the file is not a whole GEN6 tape".to_string())?;
        let mut pos = read.end;
        let chain = crate::chronometry::ReceiptChain::take_tries(bytes, &mut pos)
            .filter(|_| pos == bytes.len())
            .ok_or_else(|| "this voice's ledger does not read whole".to_string())?;
        // THE GENOME'S admitted_at FROM ITS OWN LEDGER (78.27): a word
        // admitted on this voice's line stands at a node of the ledger just
        // read; a stamp on a peer's line — the court's syllabus stamp — is a
        // citation and stands as one.
        let orphaned = read.admitted.iter().filter(|(_, at)| at.vk == chain.vk() && chain.node(at).is_none()).count();
        if orphaned > 0 {
            return Err(format!("{orphaned} words admitted at stamps on this voice's line its ledger does not hold"));
        }
        if !self.session.voice_mut().restore(chain, read.admitted) {
            return Err("this voice's ledger does not verify under its key".to_string());
        }
        self.epigenome = epigenome;
        Ok(())
    }

    /// THE GENOME'S BYTES: the words at their citations, then this voice's
    /// ledger as its tries. A word the writer refused is named once with its
    /// count (LD0), and the counts are answered; `None` where a node of the
    /// ledger cannot be written, and nothing is written then.
    pub fn genome_bytes(&self) -> Option<(Vec<u8>, crate::client::sdk::epigenome::GenomeRefused)> {
        let voice = self.session.voice();
        let (mut bytes, refused) = self.epigenome.crystallize(|a| voice.admitted_at(a));
        refused.name();
        voice.chain().put_tries(&mut bytes)?;
        Some((bytes, refused))
    }

    /// Persist the genome atomically (write-then-rename). Call from the
    /// syntropic phase when the genome has grown.
    pub fn save_genome(&mut self, genome_path: &str) -> io::Result<crate::client::sdk::epigenome::GenomeRefused> {
        let Some((bytes, refused)) = self.genome_bytes() else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "genome refused: a node of this voice's ledger cannot be written"));
        };
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
