// 78.122 — THE SHARED LEXICON'S REGISTER: the vocabulary the server defines
// (`docs/MU-LEXICON.md`), served by address beside the law register
// (`ACT_LAWS` mode 3) so a connecting kernel checks its terms against the
// server's, at a version — never silently mistranslated.
//
//! This table is the one carrier of the served terms; `MU-LEXICON.md` is their
//! exposition, and a test holds every term and the version to stand in it.
//! No word belongs to two vocabularies.

/// The lexicon's version: an amendment raises it.
pub const LEXICON_VERSION: u8 = 5;

/// WHICH VOCABULARY A TERM BELONGS TO.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vocabulary {
    /// The spatial ladder and what is in it.
    Physical,
    /// Who holds, governs and settles what.
    Civic,
    /// The hyphae and what runs on them.
    Network,
}

impl Vocabulary {
    pub const fn byte(self) -> u8 {
        match self {
            Vocabulary::Physical => 0,
            Vocabulary::Civic => 1,
            Vocabulary::Network => 2,
        }
    }

    pub fn of(b: u8) -> Option<Vocabulary> {
        [Vocabulary::Physical, Vocabulary::Civic, Vocabulary::Network].into_iter().find(|v| v.byte() == b)
    }
}

/// ONE TERM: its name, its vocabulary, its meaning in one line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Term {
    pub name: &'static str,
    pub vocabulary: Vocabulary,
    pub meaning: &'static str,
}

const fn t(name: &'static str, vocabulary: Vocabulary, meaning: &'static str) -> Term {
    Term { name, vocabulary, meaning }
}

use Vocabulary::{Civic, Network, Physical};

/// EVERY TERM, by vocabulary, in `MU-LEXICON.md`'s order.
pub const TERMS: [Term; 48] = [
    t("Universe", Physical, "everything that shares one origin: one seed, one law set, one clock"),
    t("Cluster", Physical, "galaxies bound together by gravity"),
    t("Galaxy", Physical, "a disc and halo of stars"),
    t("Region", Physical, "a spatial cell of a galaxy's disc"),
    t("System", Physical, "one sector holding a star or stars and its bodies, on exact orbits"),
    t("Sector", Physical, "one node of the warp graph: a system, or null space"),
    t("Star", Physical, "a system's stellar body or bodies"),
    t("Body", Physical, "anything that orbits: a planet, a moon, a belt, a station"),
    t("World", Physical, "informal only: an inhabited or habitable body; never a type or wire name"),
    t("Black hole", Physical, "a sink of mass and value, absorbing at its place as continuity allows"),
    t("White hole", Physical, "a source, emitting at its place as continuity allows; funds nothing"),
    t("Wormhole", Physical, "a gateway joining places the lanes do not, crossed at a toll and a tithe"),
    t("Lane", Physical, "a warp connection between adjacent sectors"),
    t("People", Civic, "a species become a polity, with its own biology and value channels"),
    t("Realm", Civic, "a group of constellations of inhabited systems monitored by stewards; never a region"),
    t("Ancient house", Civic, "an ancient people's estate and its line"),
    t("Steward", Civic, "a seat that monitors a realm"),
    t("Sovereign", Civic, "a seat that governs"),
    t("Progenitor", Civic, "a seat whose line keeps the wormholes' throats"),
    t("Ancient", Civic, "a seat that touches the network's μ"),
    t("Value channel", Civic, "any mechanism moving claims between holders with conserved backing"),
    t("Money", Civic, "a value channel's unit of account"),
    t("Stardock", Civic, "a port, and the ledger of every transfer made at it"),
    t("Ledger tree", Civic, "stardock, realm, progenitor network, ancient houses, anchored on the hypha"),
    t("Network universe", Network, "a genesis record, a law version and its causal clock"),
    t("Domain", Network, "an application's namespace under a universe, founded on a court by its genesis"),
    t("Tier", Network, "a signing key that folds a ledger for a domain, named at the domain's genesis"),
    t("Hypha", Network, "a node of the mesh: the courts hold the laws and adjudicate"),
    t("Court", Network, "a hypha's adjudicating organ and its line"),
    t("Voice", Network, "a kernel that speaks to a court"),
    t("Mind", Network, "what a voice thinks with"),
    t("Entity", Network, "a party holding a key"),
    t("Cohort", Network, "clients working on alike problems"),
    t("Allocation", Network, "a client's bond and lane budget"),
    t("μ", Network, "the network's four-lane token, never summed across lanes"),
    t("POW++", Network, "useful work, work convergence, work retirement"),
    t("Matter lane", Network, "a fold's S lane: what has extent and is hauled; never translated"),
    t("Value lane", Network, "a fold's G lane: money and value, exchanged at a rate; the one lane translated"),
    t("Duration lane", Network, "a fold's T lane: time owed or served, leases, contracts, upkeep; never translated"),
    t("Information lane", Network, "a fold's H lane: deliveries of words and proofs, each issued word a declared source"),
    t("Rate", Network, "a child fold's translation into its parent's units per lane, exact and strictly positive"),
    t("Open remainder", Network, "a child fold's value not yet netted by a parent: in flight, never a breach"),
    t("House", Network, "a domain's sitting reserve speaking for its game: it lists the seats and answers the minds in them"),
    t("Seat surface", Network, "the hypha's signed relay between a house and the minds that take its seats; it judges nothing in-game"),
    t("Task citation", Network, "a task's name on every wire: 32 bytes, the citation of its registry content address, which stays court-side"),
    t("Sub-origin", Network, "a sub-universe of a domain's universe, named by its 32-byte citation; with the domain and the seat id it names a seat"),
    t("Relay epoch", Network, "a value the seat relay draws when it is made, changed by every hypha restart, so a reader tells a restart from progress"),
    t("Axis coordinate", Network, "a universe's own time u, its committed cycle, carried on a percept beside the house's final point"),
];
