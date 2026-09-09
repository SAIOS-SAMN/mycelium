// 23.8, 26 — THE LANGUAGE the mind reasons over, and what it answers with.
//
//! `Language` is the snapshot a reduction runs against with no lock held:
//! the ruled words, their stalks, the axis index and the dialect. `Outcome`,
//! `Glue`, `Adoption` and `MarketView` are what it answers with.

use super::*;

/// The verdict of ingesting one demand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The demand's substance DECOMPOSED into a sentence of installed
    /// generators: its normal form on one stratum, reached by reduction
    /// against the generators the language holds (A12, 1.8), unique because
    /// replay closes it and never because an order on tapes says so. The
    /// algebra spans it, nothing minted. An empty word is a pure
    /// transposition (no substance). One reduction, not two search kinds.
    Decomposed(Vec<usize>),
    /// A novel generator docketed on one vantage, awaiting a witness organ's
    /// independent re-derivation of the same address (47.3).
    GermPending,
    /// An irreducible residual that CONVERGED and was adjoined as a generator
    /// of the presentation (1.8): the base of the language grows, the group
    /// and the alphabet do not. Chapter 44 refuses a completion procedure.
    Installed(usize),
    /// A family member punctured as a section of held word `ti`. Not an install,
    /// and it does not mint.
    Sectioned(usize),
    /// The scene had no single motor; it fragmented into per-body sub-demands,
    /// each reduced independently and recomposed (the large-corpus path).
    Fragmented(Vec<Outcome>),
    /// A degenerate demand (empty, or mismatched body counts) — nothing to reduce.
    Pivot,
    /// The act is REFUSED with a reason of 34.1: a lane of the voice's room
    /// did not cover the draw (23.8). Nothing was docketed, drawn, escrowed
    /// or sealed; the refusal is the whole of what happened, and it travels
    /// to the voice.
    Refused(&'static str),
}

/// The Čech glue of a fetched word on an overlapping chart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Glue {
    Admitted,
    Attached,
    Dialect,
    Slashed(&'static str),
}

/// The result of reducing a demand's substance against the generator basis: either
/// a canonical sentence (fully decomposed; indices into the language the
/// reduction ran over) or an irreducible residual (a critical pair to complete
/// by germination).
#[derive(Debug, Clone)]
pub enum NormalForm {
    Decomposed(Vec<usize>),
    Irreducible(Box<Element>),
}

impl PartialEq for NormalForm {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NormalForm::Decomposed(a), NormalForm::Decomposed(b)) => a == b,
            (NormalForm::Irreducible(a), NormalForm::Irreducible(b)) => a.same_vertex(b),
            _ => false,
        }
    }
}
impl Eq for NormalForm {}

/// THE LANGUAGE SNAPSHOT: the held words and their atlas, immutable and
/// shared by pointer with every reflex worker. Reduction, congruence, and
/// quoting run over it with no lock held.
#[derive(Debug, Clone, Default)]
pub struct Language {
    /// Shared snapshot: each word is its own Arc, so a snapshot shares
    /// pointers and never copies the basis. The vector and the index are
    /// owned so an admit MODIFIES this snapshot in place through
    /// `Arc::make_mut` rather than building a new one beside it; a copy is
    /// paid once, and only when a reader is actually holding the old one.
    pub words: Vec<std::sync::Arc<Element>>,
    pub stalks: std::sync::Arc<crate::transducer::stalk::Stalks>,
    /// Each word's shape, lifted once on first use and shared by every
    /// lookup over this snapshot (a measurement memoized, never persisted).
    pub shapes: Vec<std::sync::OnceLock<Option<crate::transducer::blade::Shape>>>,
    /// The retirement generation of the basis this snapshot was taken at.
    pub retire_gen: u64,
    /// Each word reached by its address — a coordinate read, never a scan.
    pub index: crate::network::nerve::Keyed<usize>,
}

impl Language {
    pub fn get(&self, i: usize) -> Option<&Element> {
        self.words.get(i).map(|w| w.as_ref())
    }

    /// The shape of word `i`, lifted on first use.
    pub fn shape(&self, i: usize) -> Option<&crate::transducer::blade::Shape> {
        let cell = self.shapes.get(i)?;
        cell.get_or_init(|| element::shape_of_chain(self.words[i].sub())).as_ref()
    }

    /// The index of the word at an address.
    pub fn at(&self, addr: &[u8]) -> Option<usize> {
        self.index.get(addr).copied()
    }
}

/// WHAT AN OFFER STANDS AT, per witness (47.3, 47.5; S12.4d): pending
/// since the K it was offered at, or answered — the witness replayed and
/// sent its section, or named its refusal. An offer is asked again only
/// once the witness has answered nothing for that peer's Λ; a pending offer
/// is never re-sent on the next cycle, and an answered one never again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfferStanding {
    Pending { k: u64 },
    Answered,
}

/// The market as seen at one instant (see [`Kernel::market_open`]).
pub struct MarketView {
    pub language: std::sync::Arc<Language>,
    pub pending: std::sync::Arc<crate::network::nerve::Keyed<(Element, u64)>>,
    pub surface: [u64; 4],
    /// The open book's tension per lane — the denominator every price is a
    /// share of.
    pub pressure: [u64; 4],
    pub pool: Option<[u64; 4]>,
    pub k_step: u64,
    /// Receipt-chain head K — the kernel's own receipt index, packed on Quote.
    pub head_k: u64,
    /// The chain's head height (45.2): the book's height the quote reply
    /// carries, so a voice paces on the ledger and never on a count.
    pub head_height: u64,
    /// Standing extent per listing, in heights.
    pub standing_t: std::sync::Arc<crate::network::nerve::Keyed<u64>>,
    pub reserve: [u64; 4],
    /// The (basis, book) generations this view was taken at.
    pub gens: (u64, u64),
    /// The sealed rank if it is current at `gens`; absent when either
    /// generation moved, so the reader measures it over the snapshot.
    pub closure: Option<std::sync::Arc<Rank>>,
    /// The last sealed rank regardless of currency — the memo a fresh
    /// measurement reuses closures from at the same basis generation.
    pub prior: std::sync::Arc<Rank>,
    /// Listings keyed by chart, so rank_over's moved set is a fiber.
    pub chart_listings: std::sync::Arc<crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>>,
}

/// THE RANK OF THE BOOK — a sealed carrier: for each listing its base point
/// in the language (closure) or none (coverage). There is no scalar order
/// over the book; quotes are incidence on charts (quote_incidence).
/// Identity is the listing address; there is no parallel Vec of positions.
#[derive(Debug, Clone, Default)]
pub struct Rank {
    pub closure: crate::network::nerve::Keyed<Option<usize>>,
    /// Per listing, its moduli — the coordinate that says whether a word
    /// added since could have closed it.
    pub invariants: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant>,
    /// The basis generation the closure was measured at.
    pub basis_gen: u64,
    /// The basis length and retirement generation at measurement: with no
    /// retirement since, every prior closure index still names its word.
    pub basis_len: usize,
    pub retire_gen: u64,
}

/// One chart a listing has been probed through, with its closure, the
/// remainder that still stands on that chart, and per-lane stagnation on
/// that chart. The closure is never a number.
#[derive(Debug, Clone)]
pub struct FieldEntry {
    pub chart: crate::transducer::stalk::Invariant,
    pub state: crate::transducer::stalk::Closure,
    pub remainder: Vec<(i64, i64, u8, u8)>,
    pub stagnant: [u32; 4],
}

/// One listing's coherence field: the charts and the non-Flat count, updated
/// in write_field so a read never scans the entries for closure.
#[derive(Debug, Clone, Default)]
pub struct ListingField {
    pub entries: Vec<FieldEntry>,
    pub nonflat: u32,
}

/// ONE PRICED ITEM ON A QUOTE.
pub struct Priced {
    /// The listed word, whole — the quote serves the substance, not a digest.
    pub word: crate::transducer::element::Element,
    /// What it is priced at, per lane: magnitude · (1 + surface), never a
    /// scalar and never summed.
    pub mu: [u64; 4],
    /// The charts it leaves OPEN — the incidence a voice reads to decide
    /// whether this item is work it can do.
    pub open: Vec<crate::transducer::stalk::Invariant>,
}

/// A SELECTION CRITICAL PAIR: the index of the word that was placed and the
/// support it was placed on.
pub type CriticalPair = (usize, Vec<(i64, i64)>);

/// A reduction computed off the lock, ready to commit: the germ, its context,
/// the normal form over `language` (the snapshot it reduced over, so cited
/// indices resolve to addresses at commit), and the selection critical pairs.
pub struct Reduction {
    pub germ: Element,
    pub form: NormalForm,
    pub qs: Vec<CriticalPair>,
    pub language: std::sync::Arc<Language>,
    /// The listing this reduction settles: the address of the whole
    /// utterance's coboundary (a percept's pair, a demand's chain), where
    /// the deflation an installed word earns is read.
    pub listing: crate::transducer::address::Address,
    /// Empty-cites irreducible: remainder of a spoken connection or an
    /// unreached-moduli demand. Quoted. A percept-pair tape is not.
    pub frontier: bool,
}

/// THE SHARED PROJECTIVE DECOMPOSITION — speculative projection over a held
/// language, SECTION-AWARE (a bundle speaks its whole family through the
/// punctured axis). This is one mathematics worn by two stacks: the kernel's
/// reduction (perception-side) and the C-stack's COMPREHENSION — a received
/// utterance decomposed toward understanding over the receiver's genome, never
/// re-cognized, never recalled.
///
/// The adoption outcome — a deterministic status, never a boolean gate.
/// Every arm names what actually happened; the caller maps states, it does
/// not branch on success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Adoption {
    /// The word entered the universal genome here.
    Installed,
    /// Already present — the genome had converged at this address.
    Known,
    /// The current language already derives it — adoption would add
    /// redundancy, not vocabulary.
    Derivable,
    /// The word does not replay from its own evidence — not ratifiable
    /// speech; the reason names the failed axis.
    Refused(&'static str),
}
