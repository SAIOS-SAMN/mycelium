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
            refine_cursor: 0,
            taught_up: crate::network::nerve::Keyed::new(),
            taught_at: crate::network::nerve::Keyed::new(),
            discordance: 0,
            discordance_measured: false,
            obstructions: crate::network::nerve::Keyed::new(),
            glued: crate::network::nerve::Keyed::new(),
            contemplation: crate::transducer::bubble::Contemplation::new(),
            thought_gen: 0,
            opened: crate::network::nerve::Keyed::new(),
            axes_spoken: 0,
            physics: crate::physics::Physics::default(),
            height: crate::chronometry::Height::default(),
            sealed: crate::network::nerve::Keyed::new(),
            contractions: crate::network::nerve::Keyed::new(),
            contracted: Vec::new(),
            owed: Vec::new(),
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

    /// Open with CONTINUITY: restore the crystallized genome from `genome_path`
    /// if present (GEN3). The epigenome starts empty. An older file is refused.
    pub fn open_persistent(addr: &str, kernel: Kernel, genome_path: &str, _thoughts_path: &str) -> Self {
        let mut e = Entity::open(addr, kernel);
        if let Ok(bytes) = std::fs::read(genome_path) {
            if e.epigenome.uncrystallize(&bytes) {
                let n = e.epigenome.genome_len();
                // A file the host will not size reports no size; the word
                // count above is the reading that matters here.
                let kb = match std::fs::metadata(genome_path) {
                    Ok(m) => m.len() / 1024,
                    Err(_) => 0,
                };
                eprintln!("genome {n} words (GEN3, {kb} KB), epigenome empty");
            }
        } else {
            eprintln!("genome 0 words, epigenome empty");
        }
        e.thought_gen = e.epigenome.genome_len();
        e
    }

    /// Persist the genome atomically (write-then-rename). Call from the
    /// syntropic phase when the genome has grown.
    pub fn save_genome(&self, genome_path: &str) -> io::Result<()> {
        let tmp = format!("{genome_path}.tmp");
        let bytes = self.epigenome.crystallize();
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, genome_path)
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

}
