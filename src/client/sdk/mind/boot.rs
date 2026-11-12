// 24, 28 — THE MIND COMES UP: what it opens, what it restores from disk,
// and what it writes back.

use crate::client::sdk::*;

impl Mind {
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
    /// never drew on and a syntropic count nothing recorded.
    pub(crate) fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    pub(crate) fn physics_mut(&mut self) -> &mut crate::physics::Physics {
        &mut self.physics
    }

    /// Ask the bootstrap hypha for the rest of the nerve, one peer per spend,
    /// on the carrier. One discovery is in flight at a time.
    pub fn discover(&mut self) {
        if self.board.court.discovering {
            return;
        }
        self.board.court.discovering = true;
        let mut budget = self.physics.stroke();
        self.ask(move |s| s.discover_budgeted(&mut budget), |entity, ()| {
            entity.board.court.discovering = false;
        });
    }

    /// THE MIND OPENED toward the hypha at `endpoint`: its carrier discovers the
    /// nerve on its own thread before it carries the first ask. Every session
    /// and its chatter speak plaintext, named; `open_with_noise` seals them.
    pub fn open(endpoint: &str, voice: Voice) -> Self {
        Mind::open_on(endpoint, voice, None)
    }

    /// THE MIND OPENED INSIDE NOISE (MU-PRECURSORS §3.1): `open`, with every
    /// session dialled on `home`'s static key (`Session::with_noise`, each
    /// hypha pinned under the address dialled) and the chatter it opens
    /// sending, asking and listening on the same key (`chatter::send_over`,
    /// `Post::listen_over`), each mind pinned under its voice.
    pub fn open_with_noise(endpoint: &str, voice: Voice, home: crate::network::noise::NoiseHome) -> Self {
        Mind::open_on(endpoint, voice, Some(home))
    }

    fn open_on(endpoint: &str, voice: Voice, noise: Option<crate::network::noise::NoiseHome>) -> Self {
        let session = |voice: Voice| match &noise {
            Some(home) => Session::open(endpoint, voice).with_noise(home.clone()),
            None => Session::open(endpoint, voice),
        };
        // The complex listens from the first pivot (MU-CLIENT §4.5): every
        // tether condensed and face sealed winds its strain into the field.
        let mut contemplation = crate::transducer::field::Contemplation::new();
        contemplation.complex.arm_outflow();
        // The seat's carrier lands on the mind's own waker: a house's percept
        // wakes the resting mind that is to take it up.
        let hypha = super::hypha::HyphaLink::open(session(voice.clone()));
        let seat = super::hypha::HyphaLink::open_beside(session(voice.clone()), hypha.waker());
        // The held mail read on a carrier of its own, so it never stands in
        // front of a take, a solve or a declination.
        let held = super::hypha::HyphaLink::open_beside(session(voice), hypha.waker());
        Mind {
            seat,
            held,
            hypha,
            language: MindLanguage::new(),
            solve_refused: crate::network::nerve::Keyed::new(),
            mint_refused: crate::network::nerve::Keyed::new(),
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
            amendments: Vec::new(),
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
            closed_answers: crate::network::nerve::Keyed::new(),
            scenes: Default::default(),
        }
    }

    pub fn set_height(&mut self, h: crate::chronometry::Height) {
        self.height = h;
    }

    /// THE HEIGHT THE MIND READS (23.8, S13.5; V2): the court's height off
    /// a quote or attest reply, and it only advances — a lower or absent
    /// reading moves nothing. It is this mind's TUniversal: its own chain
    /// cites no peer (45.2), so the height read is its extent, and what it stamps
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
    /// Before the first, the height it was set at boot — never a court's
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
    /// the MESH's height, and the court's receipt line position `i` carried
    /// on a quote reply is a third quantity belonging to a hypha's chain; none of the three is
    /// interchangeable with another, and the stroke line prints this one
    /// and the height, each under its own name.
    pub(crate) fn own_k(&self) -> u64 {
        self.k
    }

    /// AN ACT OF THIS MIND — a word stood, asserted, learned or refused
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
    /// one that does not read whole, is refused whole. What the file came to
    /// is answered beside the mind ([`GenomeReading`]); the caller prints it.
    /// With `noise`, the mind opens inside Noise (`open_with_noise`).
    pub fn open_persistent(endpoint: &str, voice: Voice, noise: Option<crate::network::noise::NoiseHome>, genome_path: &str) -> (Self, GenomeReading) {
        let mut e = Mind::open_on(endpoint, voice, noise);
        let Ok(bytes) = std::fs::read(genome_path) else {
            return (e, GenomeReading::Absent);
        };
        if let Err(why) = e.restore_genome(&bytes) {
            return (e, GenomeReading::Refused(why));
        }
        let words = e.language.genome_len();
        (e, GenomeReading::Restored { words, bytes: bytes.len() })
    }

    /// RESTORE A GENOME'S BYTES into this freshly opened mind: the words a
    /// court admitted, each at its citation. Whole or not at all: the words
    /// are read into a separate `MindLanguage` and taken in only once they
    /// read whole, so a refusal leaves this mind as it was. `Err` names why
    /// the file was refused.
    pub(crate) fn restore_genome(&mut self, bytes: &[u8]) -> Result<(), String> {
        if let Some(named) = crate::client::sdk::vocabulary::genome_refused(bytes) {
            return Err(named.to_string());
        }
        let mut language = MindLanguage::new();
        language
            .uncrystallize(bytes)
            .filter(|end| *end == bytes.len())
            .ok_or_else(|| "the file is not a whole GEN7 tape".to_string())?;
        // A file holding what the genome leaves out (residue chains, rewrites)
        // is written again at the first stroke.
        self.board.genome_unwritten = !language.restored_residues.is_empty() || language.restored_dropped > 0;
        // MU-CLIENT §7.1 — the residue chains the file held are open
        // questions, handed to the mind's carrier, never words.
        for (addr, chain, stratum) in std::mem::take(&mut language.restored_residues) {
            self.place_question(&addr, &chain);
            self.board.open_residues.insert(&addr, chain);
            self.board.residue_strata.insert(&stratum, ());
        }
        self.language = language;
        Ok(())
    }

    /// THE GENOME'S BYTES: the words a court admitted, each at its citation. A
    /// word the writer refused is named once with its count (LD0), and the
    /// counts are answered.
    pub(crate) fn genome_bytes(&self) -> (Vec<u8>, crate::client::sdk::vocabulary::GenomeWriteTally) {
        let (bytes, refused) = self.language.crystallize();
        refused.name();
        (bytes, refused)
    }

    /// Persist the genome atomically (write-then-rename). Call from the
    /// syntropic phase when the genome has grown.
    pub(crate) fn save_genome(&mut self, genome_path: &str) -> io::Result<crate::client::sdk::vocabulary::GenomeWriteTally> {
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
    pub(crate) fn voice(&self) -> &Voice {
        self.hypha.voice()
    }
}

/// WHAT A GENOME FILE CAME TO at `Mind::open_persistent`: none on disk, refused
/// whole by name, or restored. The library prints nothing; a binary prints
/// this reading (`Display`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenomeReading {
    /// No file at the path: the mind opens with no words.
    Absent,
    /// The file was refused whole; the reason names why.
    Refused(String),
    /// The file was restored: the words the genome holds and the file's
    /// length in bytes.
    Restored { words: usize, bytes: usize },
}

impl std::fmt::Display for GenomeReading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenomeReading::Absent => write!(f, "genome 0 words, epigenome empty"),
            GenomeReading::Refused(why) => write!(f, "genome refused: {why}"),
            GenomeReading::Restored { words, bytes } => {
                write!(f, "genome {words} words (GEN7, {} KB), epigenome empty", bytes / 1024)
            }
        }
    }
}
