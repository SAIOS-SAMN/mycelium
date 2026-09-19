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
    /// The words the speech decomposed through, BY THEIR OWN COORDINATES.
    Decomposed(Vec<crate::transducer::address::Address>),
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
    /// The door DEFERRED the act on the lanes named (23.8, S13.4): the
    /// book's reserve is spent there and the voice paid nothing on them.
    /// Nothing was heard, docketed or drawn; the voice dwells on the lanes
    /// it carries and speaks again.
    Deferred([u64; 4]),
    /// 21 step 3, 50.2 step 2: THIS SENTENCE WAITS on a cited form this
    /// court has not ruled. It is docketed here and heard again when that
    /// address is ruled; nothing is refused, sealed or drawn. Unlike
    /// `Deferred`, which is the DOOR turning an act away on the lanes,
    /// this is per sentence — the rest of the frame is adjudicated — and
    /// it carries the cited address, because the voice holds that form
    /// and must utter it here first (chapter 6's depth order).
    Awaiting(crate::transducer::address::Address),
}

/// The Čech glue of a fetched word on an overlapping chart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Glue {
    Admitted,
    Attached,
    Dialect,
    Slashed(&'static str),
}

/// The result of the redundancy probe over the language: the held words a
/// word's substance decomposes through, BY THEIR OWN COORDINATES, or none.
/// `Irreducible` carries no element: what the language does not span is not
/// composed by the court (law 8).
///
/// They were "indices into the language the reduction ran over" — which
/// named nothing the moment that snapshot retired a word, and the snapshot
/// was never carried alongside them to say which one was meant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalForm {
    Decomposed(Vec<crate::transducer::address::Address>),
    Irreducible,
}

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
    pub words: Vec<std::sync::Arc<crate::transducer::element::Held>>,
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
        self.words.get(i).map(|w| w.element())
    }

    /// The held word at a slot: the element AND the form its tape decodes
    /// to (L16), for a reader that needs the form.
    pub fn held(&self, i: usize) -> Option<&crate::transducer::element::Held> {
        self.words.get(i).map(|w| w.as_ref())
    }

    /// The shape of word `i`, lifted on first use.
    pub fn shape(&self, i: usize) -> Option<&crate::transducer::blade::Shape> {
        let slot = self.shapes.get(i)?;
        slot.get_or_init(|| element::shape_of_chain(self.words[i].sub())).as_ref()
    }

    /// The index of the word at an address.
    pub fn at(&self, addr: &[u8]) -> Option<usize> {
        self.index.get(addr).copied()
    }

    /// THE WORD AT AN ADDRESS — the reading's own contract (law 2). The
    /// storage here is still a `Vec` with a `Keyed<usize>` beside it, the
    /// same duplicated-address defect the epigenome carried; it is named in
    /// the ledger and is the next carrier to be keyed by its coordinate.
    /// The reading no longer converts addresses to offsets and back, so
    /// nothing downstream depends on the slot any more.
    pub fn word_at(&self, addr: &[u8]) -> Option<&Element> {
        self.get(self.at(addr)?)
    }

    /// The shape of the word at an address, lifted on first use.
    pub fn shape_at(&self, addr: &[u8]) -> Option<&crate::transducer::blade::Shape> {
        self.shape(self.at(addr)?)
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
    closure: crate::network::nerve::Keyed<Option<usize>>,
    /// Per listing, its moduli — the coordinate that says whether a word
    /// added since could have closed it.
    pub invariants: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant>,
    /// The basis generation the closure was measured at.
    pub basis_gen: u64,
    /// The basis length and retirement generation at measurement: with no
    /// retirement since, every prior closure index still names its word.
    pub basis_len: usize,
    pub retire_gen: u64,
    /// THE COHESION READING, CARRIED WHERE THE CLOSURE IS BOUND: listings
    /// with a base point in the language, and listings without one. Kept
    /// by `put_closure` for the same reason `ListingField::nonflat` is
    /// kept by `write_field` — so a read never walks the book to count
    /// what the write already knew (rule 1). `cohesion_over` walked the
    /// whole closure map, and `synthesis` walked it a second time, with
    /// its own filter, to print one of the two numbers.
    closed: usize,
    open: usize,
}

impl Rank {
    /// Bind a listing's closure and keep the counts with it. The two are
    /// one fact written once (law 12): nothing else may touch `closure`,
    /// which is why it is private.
    pub fn put_closure(&mut self, addr: &[u8], closed: Option<usize>) {
        // A rebind carries its old reading out of the count first. A count
        // at zero holding an entry is a broken count and stays at zero,
        // which is `Keyed::remove`'s own treatment (rule 11: no floor).
        match self.closure.insert(addr, closed) {
            Some(Some(_)) => {
                if let Some(n) = self.closed.checked_sub(1) {
                    self.closed = n;
                }
            }
            Some(None) => {
                if let Some(n) = self.open.checked_sub(1) {
                    self.open = n;
                }
            }
            None => {}
        }
        match closed {
            Some(_) => self.closed += 1,
            None => self.open += 1,
        }
    }

    /// One listing's base point: `Some(None)` is a listing measured and
    /// not closed, `None` a listing this reading does not hold at all.
    pub fn closure_of(&self, addr: &[u8]) -> Option<&Option<usize>> {
        self.closure.get(addr)
    }

    /// Listings measured, closed and open. The reading `cohesion_over`
    /// returns and the ledger writes.
    pub fn counts(&self) -> (usize, usize) {
        (self.closed, self.open)
    }

    pub fn closure_len(&self) -> usize {
        self.closure.len()
    }

    pub fn closure_is_empty(&self) -> bool {
        self.closure.is_empty()
    }

    /// The closure map for the persist walk, which writes it whole.
    pub fn closure_carrier(&self) -> &crate::network::nerve::Keyed<Option<usize>> {
        &self.closure
    }
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
