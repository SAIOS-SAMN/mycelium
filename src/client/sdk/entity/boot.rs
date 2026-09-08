// 24, 28 — THE ENTITY COMES UP: what it opens, what it restores from disk,
// and what it writes back.

use crate::client::sdk::*;

impl Entity {
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
            ideas: crate::network::nerve::Keyed::new(),
            physics: crate::physics::Physics::default(),
            height: crate::chronometry::Height::default(),
        }
    }

    pub fn set_height(&mut self, h: crate::chronometry::Height) {
        self.height = h;
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
                let kb = std::fs::metadata(genome_path).map(|m| m.len() / 1024).unwrap_or(0);
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
