// 78.94 — EVERY ACT IS AN ACTION OF THE PHYSICS (ruled
// 2026-10-04: every constant labelled ACT is carried into the physics as an
// ACTION, unified going forward and remediated from the progenitor).
//
//! One table for what happens in a universe: every action a party requests
//! on the wire and every act a court seals on its line, each by ONE
//! coordinate byte. An action states its physics as its type — what kind of
//! action it is, whether a court seals it, whether it changes the algorithm
//! the ledger holds, and the laws it answers to — so "an observation moves
//! nothing", "a retired action is refused" and "a reading writes no receipt"
//! are facts the table carries, never comments beside a byte.
//!
//! `chronometry::ActKind` is the ledger's name for [`Action`], and the wire's
//! `ACT_*` constants are spellings of its coordinates, derived from it (law
//! 12: one carrier).

use crate::chronometry::Ruled;

#[cfg(test)]
mod tests;

/// WHAT KIND OF ACTION IT IS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Measures and moves nothing: its frame is the zero reading and it
    /// never seals. Its cost is the court's stroke spent on the question,
    /// metered, and charged to the asker by tax once the financial mind
    /// stands (78.93 (d), 78.94).
    Observation,
    /// A party's act a court adjudicates: it moves the book, the treasury or
    /// the ledger.
    Transition,
    /// Court to court: covers, offers, installs, folds, burns.
    Exchange,
    /// What a court seals of itself.
    Court,
    /// Refused at the door by its type; its byte is never reused.
    Retired,
    /// 78.145 — HELD AND FORWARDED, JUDGED BY NOTHING: a signed relay between
    /// a game's house and the minds seated in it. It never seals, answers to
    /// no law (it moves no book), and what it holds is volatile and bounded.
    Relay,
}

/// EVERY ACTION OF THE UNIVERSE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    // Retired on the wire (26, law 8, ruled 2026-09-10): a court hears
    // sentences, never painted sites or chains; and three tags with encoders
    // and no handler (78.94).
    Percept,
    Gossip,
    Demand,
    DemandRouted,
    Teach,
    Settle,
    Docket,
    // Observations.
    Quote,
    Task,
    Digests,
    Fetch,
    Peers,
    Receipts,
    Rows,
    Observe,
    Laws,
    Why,
    Wallet,
    // Transitions.
    Evidence,
    Refine,
    RefineAlgebraic,
    Solve,
    Syllabus,
    Probe,
    Speech,
    /// 78.119 — A TIER'S FOLD ADMITTED (78.62 step 3): a kernel's settlement
    /// tier sends up its net positions per identity per lane, signed, with
    /// its prefix head; the court checks continuity and seals the fold's
    /// coordinates on its line. Sealed under its own coordinate, as the
    /// syllabus page and the fold are.
    Uplink,
    /// 78.120, 78.122 — A DOMAIN FOUNDED (78.62's GENESIS act): its key and the tier
    /// keys that settle for it, sealed under its own coordinate.
    Domain,
    /// 78.125 — A FOLD CHALLENGED within its window (2(c)): an availability
    /// demand, the tier line's prefix put to replay, a default claimed, a
    /// correction booking an arbitrage — sealed under its own coordinate.
    Challenge,
    /// 78.126 — MATTER OR A WORD CROSSING DOMAINS (2(b)): offered, accepted,
    /// refused or expired, each side consumed by one fold — sealed under its
    /// own coordinate. 0x07 is the last wire byte taken: 0x0A and 0x0F stay
    /// reserved, and the next act is a sub-kind under an existing one.
    Handoff,
    /// 78.145 — THE SEAT SURFACE: a game's house lists its seats, a mind
    /// takes one, the house perceives for it and the mind calls; the hypha
    /// holds and forwards, signed, judging nothing in-game. Its byte 0x53
    /// stands in the court's range by the wire agreed with the kernels, and
    /// is asked on the wire by its kind (`Kind::Relay`), never sealed.
    Seat,
    // Exchanges.
    Adopt,
    Root,
    Cover,
    Offer,
    Installed,
    Equivocation,
    Fold,
    /// 78.152b — A KEEPER'S RING PUSHED: court to court, the keeper's newest
    /// TUniversal readings on a dedicated act, heard at the door off the
    /// kernel, so the mesh's clock never waits on a court's load. Its byte
    /// 0x23 stands beside the cover, offer and install (ruled
    /// 2026-10-09: a free byte, WIRE-CHANGE, court to court only). Seals
    /// nothing.
    Tick,
    // What a court seals of itself.
    Attest,
    List,
    Install,
    /// A court's ruling on a word, the arm it ruled carried in its byte (EC1).
    Ruling(Ruled),
    Lapse,
    Adopted,
    Settled,
    /// 50.9 (L8), 78.93 B0: a court refused a sentence, on its line.
    Refused,
    /// 78.23 as amended, OK1 — THE CLOSE OF AN ORBIT: its frame the per-lane
    /// sum over the orbit's acts.
    Orbit,
    /// 78.68 — a ring returned, one full age.
    Return,
    /// 78.68 — a cell's holonomy algebra stepped in rank.
    RankStep,
    /// 78.71 (4) — the field reconnected.
    Reconnect,
    // Sealed by nothing since the voices' chains retired (78.49).
    Section,
    Stood,
    Uttered,
    Learned,
}

/// Every action, in coordinate order. 0x0A and 0x0F stand unassigned (0x0F
/// was the retired genesis page, 2026-09-29) and RESERVED (78.126): the wire's
/// bytes below the court's range are spent, so the next act is a sub-kind
/// under an existing one, never a new byte. 78.145's seat relay is one
/// exception, at 0x53 past the court's acts, by the wire the kernels agreed;
/// 78.152b's keeper tick the other, at 0x23 among the courts' exchanges, by
/// ruling (2026-10-09).
pub const ACTIONS: [Action; 57] = [
    Action::Percept,
    Action::Gossip,
    Action::Demand,
    Action::Quote,
    Action::Teach,
    Action::Settle,
    Action::Docket,
    Action::Handoff,
    Action::Evidence,
    Action::Refine,
    Action::Task,
    Action::Solve,
    Action::Syllabus,
    Action::Adopt,
    Action::Digests,
    Action::Fetch,
    Action::Probe,
    Action::Root,
    Action::DemandRouted,
    Action::Peers,
    Action::Speech,
    Action::RefineAlgebraic,
    Action::Cover,
    Action::Offer,
    Action::Installed,
    Action::Tick,
    Action::Equivocation,
    Action::Receipts,
    Action::Fold,
    Action::Rows,
    Action::Observe,
    Action::Laws,
    Action::Why,
    Action::Wallet,
    Action::Uplink,
    Action::Domain,
    Action::Challenge,
    Action::Attest,
    Action::List,
    Action::Section,
    Action::Install,
    Action::Ruling(Ruled::Upheld),
    Action::Ruling(Ruled::Demoted),
    Action::Ruling(Ruled::Slashed),
    Action::Ruling(Ruled::Retired),
    Action::Lapse,
    Action::Adopted,
    Action::Stood,
    Action::Uttered,
    Action::Learned,
    Action::Settled,
    Action::Refused,
    Action::Orbit,
    Action::Return,
    Action::RankStep,
    Action::Reconnect,
    Action::Seat,
];

/// The first coordinate of a court's own acts: below it, the wire's.
pub const COURT_RANGE: u8 = 0x40;

impl Action {
    /// THE ACTION'S ONE COORDINATE, on the wire and on the ledger alike.
    pub const fn byte(self) -> u8 {
        match self {
            Action::Percept => 0x00,
            Action::Gossip => 0x01,
            Action::Demand => 0x02,
            Action::Quote => 0x03,
            Action::Teach => 0x04,
            Action::Settle => 0x05,
            Action::Docket => 0x06,
            Action::Handoff => 0x07,
            Action::Evidence => 0x08,
            Action::Refine => 0x09,
            Action::Task => 0x0B,
            Action::Solve => 0x0C,
            Action::Syllabus => 0x0D,
            Action::Adopt => 0x0E,
            Action::Digests => 0x10,
            Action::Fetch => 0x11,
            Action::Probe => 0x12,
            Action::Root => 0x13,
            Action::DemandRouted => 0x14,
            Action::Peers => 0x15,
            Action::Speech => 0x16,
            Action::RefineAlgebraic => 0x17,
            Action::Cover => 0x20,
            Action::Offer => 0x21,
            Action::Installed => 0x22,
            Action::Tick => 0x23,
            Action::Equivocation => 0x35,
            Action::Receipts => 0x36,
            Action::Fold => 0x37,
            Action::Rows => 0x38,
            Action::Observe => 0x39,
            Action::Laws => 0x3A,
            Action::Why => 0x3B,
            Action::Wallet => 0x3C,
            Action::Uplink => 0x3D,
            Action::Domain => 0x3E,
            Action::Challenge => 0x3F,
            Action::Attest => 0x40,
            Action::List => 0x41,
            Action::Section => 0x42,
            Action::Install => 0x43,
            Action::Ruling(Ruled::Upheld) => 0x44,
            Action::Ruling(Ruled::Demoted) => 0x45,
            Action::Ruling(Ruled::Slashed) => 0x46,
            Action::Ruling(Ruled::Retired) => 0x47,
            Action::Lapse => 0x48,
            Action::Adopted => 0x49,
            Action::Stood => 0x4A,
            Action::Uttered => 0x4B,
            Action::Learned => 0x4C,
            Action::Settled => 0x4D,
            Action::Refused => 0x4E,
            Action::Orbit => 0x4F,
            Action::Return => 0x50,
            Action::RankStep => 0x51,
            Action::Reconnect => 0x52,
            Action::Seat => 0x53,
        }
    }

    /// The action a byte names, or none.
    pub fn of(b: u8) -> Option<Action> {
        ACTIONS.iter().copied().find(|a| a.byte() == b)
    }

    /// WHAT KIND OF ACTION IT IS.
    pub const fn kind(self) -> Kind {
        match self {
            Action::Percept
            | Action::Gossip
            | Action::Demand
            | Action::DemandRouted
            | Action::Teach
            | Action::Settle
            | Action::Docket
            | Action::Section
            | Action::Stood
            | Action::Uttered
            | Action::Learned
            // 2026-10-11 — a mind proposes an optimization as a Retire
            // rewrite it speaks (MU-CLIENT §5.4, S3); the two bids retire.
            | Action::Refine
            | Action::RefineAlgebraic => Kind::Retired,
            Action::Quote
            | Action::Task
            | Action::Digests
            | Action::Fetch
            | Action::Peers
            | Action::Receipts
            | Action::Rows
            | Action::Observe
            | Action::Laws
            | Action::Why
            | Action::Wallet => Kind::Observation,
            Action::Evidence
            | Action::Solve
            | Action::Syllabus
            | Action::Probe
            | Action::Speech
            | Action::Uplink
            | Action::Domain
            | Action::Challenge
            | Action::Handoff => Kind::Transition,
            Action::Adopt
            | Action::Root
            | Action::Cover
            | Action::Offer
            | Action::Installed
            | Action::Equivocation
            | Action::Fold
            | Action::Tick => Kind::Exchange,
            Action::Attest
            | Action::List
            | Action::Install
            | Action::Ruling(_)
            | Action::Lapse
            | Action::Adopted
            | Action::Settled
            | Action::Refused
            | Action::Orbit
            | Action::Return
            | Action::RankStep
            | Action::Reconnect => Kind::Court,
            Action::Seat => Kind::Relay,
        }
    }

    /// WHETHER A PARTY ASKS IT ON THE WIRE: its coordinate stands below the
    /// court's range, or it is a relay (78.145). A court act is never
    /// requested.
    pub const fn on_wire(self) -> bool {
        self.byte() < COURT_RANGE || matches!(self.kind(), Kind::Relay)
    }

    /// THE ACTION A WIRE TAG NAMES, or none: a court act is never asked for.
    pub fn of_tag(b: u8) -> Option<Action> {
        Action::of(b).filter(|a| a.on_wire())
    }

    /// WHETHER A COURT SEALS IT ON ITS LINE — the kinds a receipt carries.
    /// An observation never seals; a retired kind is sealed by nothing.
    pub const fn sealed(self) -> bool {
        matches!(self.kind(), Kind::Court) || matches!(self, Action::Syllabus | Action::Fold | Action::Uplink | Action::Domain | Action::Challenge | Action::Handoff)
    }

    /// THE SEALED ACT A LEDGER BYTE NAMES, or none: a byte naming an action
    /// no court seals refuses the read (45.1).
    pub fn from_byte(b: u8) -> Option<Action> {
        Action::of(b).filter(|a| a.sealed())
    }

    /// The coordinate the node carries (the ledger's spelling of [`byte`]).
    ///
    /// [`byte`]: Action::byte
    pub const fn to_byte(self) -> u8 {
        self.byte()
    }

    /// THE L0 LAWS IT ANSWERS TO: every law for an action that moves
    /// anything; none for an observation, which cannot break one by its
    /// kind, nor for a retired one, which never runs.
    pub fn laws(self) -> &'static [crate::physics::laws::Law] {
        match self.kind() {
            Kind::Observation | Kind::Retired | Kind::Relay => &[],
            Kind::Transition | Kind::Exchange | Kind::Court => &crate::physics::laws::LAWS,
        }
    }

    /// 78.68, 78.69 — DOES THIS ACTION CHANGE THE ALGORITHM THE LEDGER
    /// HOLDS: a ring's return, a rank step, the field reconnecting, or a
    /// generator entering the presentation (`Install`, `Ruling(Upheld)`,
    /// `Adopted`). The one reading of a fold's transition.
    pub const fn changes_algorithm(self) -> bool {
        matches!(
            self,
            Action::Return
                | Action::RankStep
                | Action::Reconnect
                | Action::Install
                | Action::Ruling(Ruled::Upheld)
                | Action::Adopted
        )
    }

    /// The action's name, the bytes an act's operator digest is taken over.
    pub const fn name(self) -> &'static [u8] {
        match self {
            Action::Percept => b"percept",
            Action::Gossip => b"gossip",
            Action::Demand => b"demand",
            Action::DemandRouted => b"demand routed",
            Action::Teach => b"teach",
            Action::Settle => b"settle",
            Action::Docket => b"docket",
            Action::Quote => b"quote",
            Action::Task => b"task",
            Action::Digests => b"digests",
            Action::Fetch => b"fetch",
            Action::Peers => b"peers",
            Action::Receipts => b"receipts",
            Action::Rows => b"rows",
            Action::Observe => b"observe",
            Action::Laws => b"laws",
            Action::Why => b"why",
            Action::Wallet => b"wallet",
            Action::Evidence => b"evidence",
            Action::Refine => b"refine",
            Action::RefineAlgebraic => b"refine algebraic",
            Action::Solve => b"solve",
            Action::Syllabus => b"syllabus",
            Action::Probe => b"probe",
            Action::Speech => b"speech",
            Action::Uplink => b"uplink",
            Action::Domain => b"domain",
            Action::Challenge => b"challenge",
            Action::Handoff => b"handoff",
            Action::Adopt => b"adopt",
            Action::Root => b"root",
            Action::Cover => b"cover",
            Action::Offer => b"offer",
            Action::Installed => b"installed",
            Action::Equivocation => b"equivocation",
            Action::Fold => b"fold",
            Action::Tick => b"tick",
            Action::Attest => b"attest",
            Action::List => b"list",
            Action::Install => b"install",
            Action::Ruling(_) => b"ruling",
            Action::Lapse => b"lapse",
            Action::Adopted => b"adopted",
            Action::Settled => b"settled",
            Action::Refused => b"refused",
            Action::Orbit => b"orbit",
            Action::Return => b"return",
            Action::RankStep => b"rank step",
            Action::Reconnect => b"reconnect",
            Action::Section => b"section",
            Action::Stood => b"stood",
            Action::Uttered => b"uttered",
            Action::Learned => b"learned",
            Action::Seat => b"seat",
        }
    }

    /// The operator digest of an act of this action (never reasoned on).
    pub fn operator(self) -> [u8; 32] {
        crate::chronometry::payload_hash(self.name())
    }
}
