// MU-RECURSION §0.0 — THE FOUNDING PRINCIPLES: what the mathematics is
// derived from. Every axiom of this register, every law the court serves and
// every pivot a mind makes is read from one of the three, and each carries a
// test in `tests/founding.rs` that could fail.
//
//! THE PHYSICS FIRST, THE MATHEMATICS FROM IT. A motion of `Cl(4,1)` over ℚ
//! splits exactly into modes (MR1), and a mode is one of three kinds. Each
//! kind is a principle:
//!
//! - **HARMONIC** — the elliptic mode: a turn, which RETURNS. Its order is read
//!   off it (Niven: 1, 2, 3, 4, 6 among rational turns), or it has none and
//!   never returns (a rosette, whose winding is content). Return is the
//!   elastic vent, the period, the class that comes back.
//! - **LUMINAL** — the parabolic mode: a null generator, `B² = 0`, on the
//!   light cone. It has no length, only its ray and its ratios on it; flux
//!   along it is frozen into its tube and conserved by the flow.
//! - **HOLONOMIC** — the closed orbit: what a circuit of transports composes to
//!   is its content. Moving the orbit's base point conjugates it and changes
//!   none of its modes, so the class is the orbit's, never the frame's.
//!
//! The hyperbolic mode (a boost or a dilation) is the luminal one off the
//! cone: two null rays, the field's expansion between them.
//!
//! TWO COROLLARIES BIND EVERY READER:
//!
//! - **EVERY DIMENSION IS AN EXPONENT.** A grade is the number of factors in a
//!   product of orthogonal generators; a cell's degree is its vertices less
//!   one; a base of `n` axes has a unit cell of `2ⁿ` vertices. Nothing caps a
//!   degree: the complex climbs as far as its faces stand (A.6).
//! - **NO CONSTANT.** A number in the physics is read off the field or the
//!   geometry, or it is undefined physics, named in [`OPEN_PHYSICS`] until it
//!   is derived. A bare number standing for a reading is refused.

/// HARMONIC — a turn returns at its read order, or never.
pub const HARMONIC_RETURN: &str =
    "an elliptic mode returns: its order is read off it (Niven's 1, 2, 3, 4, 6), or it has none and never returns, its winding content";

/// LUMINAL — a null generator has no length; its flux is frozen in.
pub const LUMINAL_NULL_FLUX: &str =
    "a parabolic mode is null: B² = 0, no length, only its ray and ratios; conjugation keeps it null and its flux on its tube";

/// HOLONOMIC — an orbit's composite is its content, invariant under its base.
pub const HOLONOMIC_CONTENT: &str =
    "a closed loop's holonomy is its content: moving its base point conjugates it, and its modes — its class — are unchanged";

/// AN ORBIT IS A RETURN (harmonic): a mode's flow on its plane, composed
/// exactly, returning where its point is the identity (78.73, 78.75). A tick
/// is the host handing a mind a turn, and a node's `K` is a coordinate of its
/// ledger (OK1): neither is an orbit, and no orbit is counted by either.
pub const ORBIT_IS_A_RETURN: &str =
    "an orbit is a mode's flow on its plane, returning where its point is the identity; a tick is a turn of the host and K a ledger coordinate, never an orbit";

/// EVERY DIMENSION IS AN EXPONENT.
pub const DIMENSION_IS_EXPONENT: &str =
    "a grade is the count of orthogonal factors in a product; a cell's degree its vertices less one; a base of n axes has 2ⁿ vertices in its unit cell; no degree is capped";

/// NO CONSTANT.
pub const NO_CONSTANT: &str =
    "a number in the physics is read off the field or the geometry, or it is undefined physics named until it is derived";

/// What a registered number decides (MU-DERIVATIONS §0.3, D1.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Governs {
    Finality,
    Admission,
    Mu,
    Liveness,
    ProtocolSize,
    Reading,
    Memory,
    Instrument,
}

/// Which process retires a registered number (MU-DERIVATIONS D1.4): a law
/// amendment under the Concordance (`Physics`), the wire contract
/// (`Protocol`), or MU-STANDARDS (`Engineering`: memory, cost, instruments).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenKind {
    Physics,
    Protocol,
    Engineering,
}

/// ONE NUMBER STANDING WHERE A READING BELONGS, named until derived
/// (MU-DERIVATIONS D1.4). The numbers gate (`gate::numbers`, N5) reads
/// every entry: its `site` is an item tagged `REGISTERED: <name>`, and its
/// `value` is that item's initializer as written.
#[derive(Debug, Clone, Copy)]
pub struct OpenNumber {
    /// The item's name.
    pub name: &'static str,
    /// The initializer exactly as `syn` prints it.
    pub value: &'static str,
    /// `<file>::[<owner>::]<NAME>`, the item's coordinates in the tree.
    pub site: &'static str,
    pub governs: Governs,
    /// The governing sections.
    pub doc: &'static str,
    /// What will derive it, or `Open: …` with its candidates.
    pub derivation: &'static str,
    pub kind: OpenKind,
}

/// The soak's instruments stroke a family once under a stroke of `2²⁰`
/// reads; one entry per instrument, all retired by one reading.
const SOAK_STROKE: &str = "Open: an instrument's stroke is the need of what it reads (78.111, the D3.1 pattern), read off the \
                           family it extracts; until each instrument sizes its own, the soak strokes under one chosen size";

/// UNDEFINED PHYSICS, NAMED: each physical invariant the tree still
/// declares where a reading belongs, with what it decides, the sections
/// that govern it and what will derive it (MU-DERIVATIONS D1.4). Only
/// `OpenKind::Physics` stands here (78.182 item 20); an engineering or
/// protocol number stands in [`OPEN_ENGINEERING`].
pub const OPEN_PHYSICS: [OpenNumber; 2] = [
    OpenNumber {
        name: "THROAT_LEVEL_BITS",
        value: "64",
        site: "src/server/cognition/domain_passage.rs::THROAT_LEVEL_BITS",
        governs: Governs::Admission,
        doc: "MU-RECURSION 78.134, 78.162 (5); MU-CONCORDANCE 2.6; MU-DERIVATIONS D1.2",
        derivation: "the 2^-64 grid a throat's cosh horizon is enclosed on (78.134): an approximation; the throat is a Cl(4,1) \
                     round carried by its bank's modes, fit by meet, and the grid retires with it",
        kind: OpenKind::Physics,
    },
    OpenNumber {
        name: "CANDIDATES_PER_PAIR",
        value: "8",
        site: "src/transducer/blade/carry.rs::CANDIDATES_PER_PAIR",
        governs: Governs::Reading,
        doc: "MU-RECURSION 65.5; MU-DERIVATIONS §9",
        derivation: "two parities × the fourth harmonic's four 90° turns of a transition (65.5); EXACT once a test reads both \
                     factors off the transition's held groups",
        kind: OpenKind::Physics,
    },
];

/// THE SIBLING REGISTER (D1.4 O2, 78.182 item 20): each engineering or
/// protocol number the tree declares where a reading belongs (memory,
/// cost, liveness, the soak's instruments, the wire's sizes), named until
/// derived, read by the numbers gate as `OPEN_PHYSICS` is. Retired by
/// MU-STANDARDS or the wire contract, never by a law amendment.
pub const OPEN_ENGINEERING: [OpenNumber; 15] = [
    OpenNumber {
        name: "POOL_DIVISOR",
        value: "2",
        site: "src/bin/mycelium-server/main.rs::POOL_DIVISOR",
        governs: Governs::Liveness,
        doc: "MU-DERIVATIONS D4.5",
        derivation: "Open (D4.5 O1): the workers W = 1 + ⌈off/under⌉ (strokes of a worker's act off the court lock over those \
                     under it, exact over ℚ, ceiled to threads), the readers the cores less W and the court thread, each \
                     queue W deep; derived once the off-lock strokes are metered beside `physics::spent_here`",
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "BODY_BUDGET",
        value: "768 * 1024",
        site: "src/network/wire.rs::BODY_BUDGET",
        governs: Governs::ProtocolSize,
        doc: "MU-RECURSION PH1; MU-DERIVATIONS D1.3; served on ACT_LAWS mode 5 (PARAMETERS)",
        derivation: "Open (D1.3 O1, read 2026-10-31: (b)): B(k) = 65519·k − 89, whole Noise messages, with k read off the \
                     largest unit that cannot page — but a receipt's cites carry u32-counted fields (a class's cover, a \
                     demand's), no u16 wall, so MAX_WIRE(receipt) is not finite; until those units page, the budget is an \
                     open protocol parameter, served so every kernel reads it",
        kind: OpenKind::Protocol,
    },
    OpenNumber {
        name: "ACCOUNT_SHARDS",
        value: "256",
        site: "src/server/cognition/published/publish.rs::ACCOUNT_SHARDS",
        governs: Governs::Memory,
        doc: "MU-DERIVATIONS D1.4 O2, §9",
        derivation: "Open (D1.4 O2): a cost and memory layout keyed by the first byte of a party's key; derived as shards per \
                     the published account count's growth, or kept as an engineering layout by ruling",
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "N",
        value: "1000",
        site: "src/bin/mycelium-soak/calibrate.rs::N",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4",
        derivation: "Open: the count of objects held at once for a per-object footprint, past the allocator's granularity; \
                     read off the allocator's measured granularity",
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/bridge.rs::STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/confluence.rs::STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "DELTA_STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/delta.rs::DELTA_STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/incongruence.rs::STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/interpret.rs::STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/invert.rs::STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/luminal.rs::STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "MINT_STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/mint.rs::MINT_STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/reconstruct.rs::STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/enclosure.rs::STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
    OpenNumber {
        name: "STROKE",
        value: "1 << 20",
        site: "src/bin/mycelium-soak/volume.rs::STROKE",
        governs: Governs::Instrument,
        doc: "MU-DERIVATIONS D1.4, D3.1",
        derivation: SOAK_STROKE,
        kind: OpenKind::Engineering,
    },
];
