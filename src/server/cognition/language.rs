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
    /// 78.71 — HELD ALREADY, FROM THIS PEER: the word is on the ruled cover
    /// and this court's complex already holds this peer's section of it.
    /// Hearing it again adds no vantage and no depth, so it writes nothing.
    Held,
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

/// ONE WORD OF THE SNAPSHOT, AT ITS OWN COORDINATE.
#[derive(Debug, Clone)]
pub struct LanguageWord {
    /// Its own Arc, so a snapshot shares pointers and never copies the
    /// basis.
    pub held: std::sync::Arc<crate::transducer::element::Held>,
    /// Its shape, lifted once on first use and shared by every lookup over
    /// this snapshot (a measurement memoized under the word it measures,
    /// never persisted).
    pub shape: std::sync::OnceLock<Option<crate::transducer::blade::Shape>>,
    /// The admission ordinal. Content: WHEN a word entered the basis is not
    /// readable off the word, and the rank's reuse read needs "added since".
    pub seq: u64,
}

/// THE LANGUAGE SNAPSHOT: the held words and their atlas, immutable and
/// shared by pointer with every reflex worker. Reduction, congruence, and
/// quoting run over it with no lock held.
#[derive(Debug, Clone, Default)]
pub struct Language {
    /// THE SNAPSHOT AT ITS COORDINATES. It was `Vec<Arc<Held>>` with a
    /// parallel `Vec<OnceLock<..>>` of shapes and a `Keyed<usize>` index
    /// beside them, and the comment on that index said "a coordinate read,
    /// never a scan" — which it was not: it read a coordinate, produced an
    /// OFFSET, and every caller immediately converted the offset back into
    /// a word. `language_rebuild` states the defect plainly: it reads
    /// `self.basis.scoped()`, which is ALREADY keyed by address, flattens
    /// it into a vector, and rebuilds an index to undo the flattening.
    ///
    /// Owned, so an admit MODIFIES this snapshot in place through
    /// `Arc::make_mut` rather than building a new one beside it; a copy is
    /// paid once, and only when a reader is actually holding the old one.
    pub words: crate::network::nerve::Keyed<LanguageWord>,
    pub stalks: std::sync::Arc<crate::transducer::stalk::Stalks>,
    /// The retirement generation of the basis this snapshot was taken at.
    pub retire_gen: u64,
    /// The next admission ordinal to hand out.
    pub next_seq: u64,
}

impl Language {
    /// THE WORD AT AN ADDRESS — the reading's own contract (law 2).
    pub fn word_at(&self, addr: &[u8]) -> Option<&Element> {
        self.words.get(addr).map(|w| w.held.element())
    }

    /// The held word at an address: the element AND the form its tape
    /// decodes to (L16), for a reader that needs the form.
    pub fn held_at(&self, addr: &[u8]) -> Option<&crate::transducer::element::Held> {
        self.words.get(addr).map(|w| w.held.as_ref())
    }

    /// The shape of the word at an address, lifted on first use.
    pub fn shape_at(&self, addr: &[u8]) -> Option<&crate::transducer::blade::Shape> {
        let w = self.words.get(addr)?;
        w.shape.get_or_init(|| element::shape_of_chain(w.held.sub())).as_ref()
    }

    /// Whether this snapshot holds a word at an address.
    pub fn holds(&self, addr: &[u8]) -> bool {
        self.words.contains(addr)
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    /// Every word, in the trie's order — the addresses' own order, content.
    pub fn words(&self) -> impl Iterator<Item = &crate::transducer::element::Held> {
        self.words.scoped().map(|(_, w)| w.held.as_ref())
    }

    /// The words admitted at or after an ordinal — "added since", read off
    /// each word's own admission and not off a length watermark.
    pub fn words_since(&self, seq: u64) -> Vec<&crate::transducer::element::Held> {
        self.words
            .scoped()
            .filter(|(_, w)| w.seq >= seq)
            .map(|(_, w)| w.held.as_ref())
            .collect()
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
    pub pending: std::sync::Arc<crate::network::nerve::Keyed<Element>>,
    /// `None` where the yield surface passes its carrier (STD1).
    pub surface: Option<[u64; 4]>,
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
    /// The base point of each listing, BY ITS COORDINATE. It was
    /// `Option<usize>` — a slot — and `basis_len`/`retire_gen` were carried
    /// beside it purely to notice when that slot had stopped naming its
    /// word. An address does not go stale: it either resolves in the
    /// snapshot being read or it does not.
    closure: crate::network::nerve::Keyed<Option<crate::transducer::address::Address>>,
    /// Per listing, its moduli — the coordinate that says whether a word
    /// added since could have closed it.
    pub invariants: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant>,
    /// The basis generation the closure was measured at.
    pub basis_gen: u64,
    /// The admission ordinal the basis stood at when this rank was
    /// measured — what "added since" is read against.
    pub basis_seq: u64,
    pub retire_gen: u64,
    /// THE COHESION READING, CARRIED WHERE THE CLOSURE IS BOUND: listings
    /// with a base point in the language, and listings without one. Kept
    /// by `put_closure` for the same reason `ListingField::nonflat` is
    /// kept by `write_field` — so a read never reads the whole book to count
    /// what the write already knew (rule 1). `cohesion_over` read the
    /// whole closure map, and `synthesis` read it a second time, with
    /// its own filter, to print one of the two numbers.
    closed: usize,
    open: usize,
}

impl Rank {
    /// Bind a listing's closure and keep the counts with it. The two are
    /// one fact written once (law 12): nothing else may touch `closure`,
    /// which is why it is private.
    pub fn put_closure(&mut self, addr: &[u8], closed: Option<crate::transducer::address::Address>) {
        // A rebind carries its old reading out of the count first. A count
        // at zero holding an entry is a broken count and stays at zero,
        // which is `Keyed::remove`'s own treatment (rule 11: no floor).
        let was_closed = closed.is_some();
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
        if was_closed {
            self.closed += 1;
        } else {
            self.open += 1;
        }
    }

    /// One listing's base point: `Some(None)` is a listing measured and
    /// not closed, `None` a listing this reading does not hold at all.
    pub fn closure_of(&self, addr: &[u8]) -> Option<&Option<crate::transducer::address::Address>> {
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
    pub fn closure_carrier(&self) -> &crate::network::nerve::Keyed<Option<crate::transducer::address::Address>> {
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
    pub stagnant: [u64; 4],
}

/// One listing's coherence field: the charts and the non-Flat count, updated
/// in write_field so a read never scans the entries for closure.
#[derive(Debug, Clone, Default)]
pub struct ListingField {
    pub entries: Vec<FieldEntry>,
    pub nonflat: u64,
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
