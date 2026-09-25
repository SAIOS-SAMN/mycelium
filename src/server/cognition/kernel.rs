// 23, 24 — THE KERNEL: everything one mind holds, and the reads that say
// how much of it there is.
//
//! The struct is the whole of a court's state: the basis of ruled words and
//! their indices, the book of listings and its fields, the dockets, the
//! dialect, the treasury, the receipt chain and the admissions it has made.

use super::*;

/// The kernel's evolving state: the installed generator basis + the germ
/// admission ledger + the Čech peer table.
#[derive(Default)]
pub struct Kernel {
    /// The germinated basis, each word reached by its address AND HELD WITH
    /// THE FORM ITS TAPE DECODES TO (L16): the decode happens once, where
    /// the court installs the word, not on the first read of its form.
    /// In-place writes go through `word_mut` / `touch_at`, and `word_mut`
    /// answers a `Held`, so a mutation re-decodes and the two cannot drift.
    /// Language snapshots still carry an insertion-order slice for
    /// Decomposed citations.
    pub(crate) basis: crate::network::nerve::Keyed<std::sync::Arc<crate::transducer::element::Held>>,
    /// Empty-sub group words by M1 action key, written on admit.
    pub(crate) by_action: crate::network::nerve::Keyed<crate::transducer::address::Address>,
    /// Chart+footprint → addresses on that base, written on admit.
    /// same_base reads the canonical member; condense reads the fiber.
    pub(crate) by_footprint: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// The discoverer of each basis germ, keyed by the word's address —
    /// retire must not shift a parallel Vec.
    pub(crate) discoverers: crate::network::nerve::Keyed<[u8; 32]>,
    /// The stalk space over `basis` (index-aligned): congruence lookup on
    /// the orbit quotient selects the fiber over a support's invariant and
    /// runs the exact orbit test there. Measured from the basis, never
    /// persisted; every basis mutation passes through `basis_push`,
    /// `basis_remove`, `basis_clear` so the alignment holds. Arc so a
    /// snapshot admits by pointer and copy-on-write, never a clone of the
    /// basis of elements.
    pub(crate) stalks: std::sync::Arc<crate::transducer::stalk::Stalks>,
    /// THE RULED COVER: the atlas over the words the court has finalized
    /// here — admitted when a verdict lands or a court adoption installs,
    /// removed on slash or retirement, measured once at restore. It is the
    /// cover the court reconciles against, so a stratum is discordant only
    /// where this hypha lacks federation words, never where it holds a
    /// provisional or corpus germ the court has not seen.
    pub(crate) ruled: crate::transducer::stalk::Stalks,
    /// The court's voice, heard on its first verdict: the one asker whose
    /// digest request is answered with the ruled cover.
    pub(crate) court_voice: Option<[u8; 32]>,
    /// Held words keyed by (chart, axis). Filled at `ruled.admit`, emptied
    /// at cavity relocation and basis remove.
    pub(crate) axis_index: crate::transducer::sentence::AxisIndex,
    /// Court replay of A20 certificates: citer → (certificate key → (ok,
    /// the head's place when it was replayed, none before genesis)).
    pub(crate) replay_memo:
        crate::network::nerve::Keyed<crate::network::nerve::Keyed<(bool, Option<crate::chronometry::Place>)>>,
    /// Cited form → citers. Slash of a cited form redockets each citer.
    pub(crate) cited_by: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// THE BOOK SNAPSHOT — the open listings as an immutable shared value
    /// sealed at a book generation; a market view shares them by pointer
    /// and the book is copied only when it moved.
    pub(crate) book: (u64, std::sync::Arc<crate::network::nerve::Keyed<Element>>),
    /// THE LANGUAGE SNAPSHOT — the basis as an immutable shared value the
    /// reflex path reduces over with no lock held. `basis_gen` counts every
    /// mutation of the basis (cards or fibres); the snapshot is rebuilt only
    /// when its generation lags, so a stable language is shared by pointer.
    pub(crate) basis_gen: u64,
    pub(crate) language_gen: u64,
    pub(crate) language: std::sync::Arc<Language>,
    /// THE MARKET CLOSURE — for every listing, the base point (an index into
    /// the language) its geometry sits on, or none (coverage work). A
    /// measured carrier sealed at (basis generation, book generation); a
    /// quote reads it, and it is recomputed off the lock only when either
    /// generation moved. `book_gen` counts every mutation of the open book.
    pub(crate) book_gen: u64,
    pub(crate) closure: (u64, u64, std::sync::Arc<Rank>),
    /// What each ORGAN re-derived, per object (court::convergence). Two
    /// organs reaching one address is the admission condition; a voice is a
    /// use and a settlement party, never a vantage.
    pub(crate) convergence: crate::court::convergence::Convergence,
    /// THE OPEN-DEMAND MARKET: irreducible residuals still awaiting consilience —
    /// the un-factored chains the mesh QUOTES as work (Vec-of-relations, capped;
    /// ephemeral — the market is runtime tension, not persisted language).
    /// THE OPEN-DEMAND MARKET — the listings. Where each stands on the
    /// ledger is its row's admitting stamp (`admitted_at`, the List act,
    /// 78.24), never a `K` carried beside it. Dwell is measured in ALGEBRAIC
    /// time (K-steps, court-replayable), never a wall clock. Capacity is
    /// thermodynamic (dynamic_lane_capacity — live memory headroom ×
    /// per-lane metabolic tension); eviction is by ACTION POTENTIAL, never
    /// FIFO. No static cap exists.
    pub(crate) pending: crate::network::nerve::Keyed<Element>,
    /// THE CHRONOMETRIC RECEIPT CHAIN — the K-step index is the mesh's temporal
    /// authority (algebraic time, no wall clocks); every install seals a
    /// parent-linked receipt (LAW-5 continuity). The Step-4 ledger's spine.
    pub(crate) chain: crate::chronometry::ReceiptChain,
    /// `ARC1` (28, 45.8): the eras this court folded, in K order — each
    /// the fold root, the identities its epoch rows named and the receipts
    /// of the prefix, cold. Written once at the fold, never rewritten;
    /// read to serve a K-range fetch or a court's replay of an era.
    pub(crate) archive: Vec<crate::chronometry::Era>,
    /// The fold awaiting a witness (47.3): the reading proposed on the
    /// syntropic circuit, carried to a bonded peer by the exchange, sealed
    /// only on both signatures.
    pub(crate) fold_pending: Option<crate::cognition::court::fold::Proposal>,
    /// The head hash of the court's chain as last ruled to this hypha —
    /// every settlement receipt here links to it, so the hypha's k-steps
    /// are causally ordered against the mesh's temporal authority.
    pub(crate) court_head: [u8; 32],
    /// Retirements of basis words, counted: while it stands still, every
    /// index into the basis names the same word it did.
    pub(crate) retire_gen: u64,
    /// Peer receipts admitted since this mind's last step, waiting to be
    /// CITED as parents of it (47.2 step 3). This is the only thing that
    /// makes height more than a rename of K: `append` takes
    /// `1 + max(h(prev), parent heights)`, and with no parent ever named
    /// the max was always this chain's own head, so TUniversal advanced on
    /// this mind's strokes rather than on the mesh's receipts (45.2).
    ///
    /// KEYED BY THE PARENT'S STAMP (78.24): a stamp names one receipt, and
    /// the carrier says so, so a duplicate is one keyed read.
    pub(crate) pending_parents: crate::network::nerve::Keyed<crate::chronometry::Parent>,
    /// THE RULED LOCATOR ON EVERY WORD ROW (78.24 (2b)): the stamp of the
    /// act that admitted each word this court holds — basis, provisional or
    /// cavity, one carrier keyed by the word so it moves with the word
    /// between rows and shares its address's bytes.
    pub(crate) admitted_at: crate::network::nerve::Keyed<crate::chronometry::Stamp>,
    /// The reverse: an admitting stamp to the word it admitted.
    pub(crate) held_at: crate::network::nerve::Keyed<Address>,
    /// THE RECEIPTS OF SPEECH: listing → voices that have spoken on it.
    pub(crate) spoken: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Transpose: voice → listings that voice holds. Written with `spoken`.
    pub(crate) spoken_by: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// THE QUOTE FRONTIER: remainder-of-g and unreached-moduli demands
    /// (empty-cites attest). Percept-pair coboundary tapes stay on the
    /// book for consilience but are not served on a quote page.
    pub(crate) quote_frontier: crate::network::nerve::Keyed<()>,
    /// THE REMAINING TENSION of a listing: derived from the union of
    /// remainders over the field's non-flat charts. Keyed, a vector, never
    /// primary — recomputed when a probe lands.
    pub(crate) tension: crate::network::nerve::Keyed<[u64; 4]>,
    /// THE COHERENCE FIELD: per listing, the charts it has been probed
    /// through and their closure. One entry per chart; a later probe on
    /// the same chart replaces its state. Never summed, never ordered.
    pub(crate) field: crate::network::nerve::Keyed<ListingField>,
    /// Listings still on the quote page (not relocated to the cavity atlas).
    pub(crate) on_page: crate::network::nerve::Keyed<()>,

    /// Voices that have closed a chart of a listing, keyed by chart then voice.
    pub(crate) flat_orbits: crate::network::nerve::Keyed<crate::network::nerve::Keyed<(crate::transducer::stalk::Invariant, element::Versor)>>,
    /// Atlas of listings and cavities: admitted under the base chart at
    /// list and under every face in the field. Drain never removes from here.
    pub(crate) listing_atlas: crate::transducer::stalk::Stalks,
    /// Relocated cavity elements, keyed, reachable through congruence.
    pub(crate) cavity_book: crate::network::nerve::Keyed<Element>,
    /// Unreachable listings for the court docket: the cavity is the
    /// element itself. Drained as the court pulls.
    pub(crate) cavities: Vec<Element>,
    /// First-vantage words already Offered.
    pub(crate) offered: crate::network::nerve::Keyed<()>,
    /// Per offered word, per witness: what the offer stands at (S12.4d).
    /// In-memory work, never persisted: a restart offers once more and the
    /// witness's own idempotence by receipt (23.8) answers the repeat.
    pub(crate) offered_to:
        crate::network::nerve::Keyed<crate::network::nerve::Keyed<crate::cognition::OfferStanding>>,
    /// The court organ's federation (ruled language) and index.
    pub(crate) court_federation: Vec<(Address, Element)>,
    pub(crate) court_fed_index: crate::network::nerve::Keyed<usize>,
    pub(crate) court_upheld: u64,
    pub(crate) court_slashed: u64,
    /// Demand vertices this court has listed under a sentence (50.5, L7):
    /// a wire read of the `court demand listed` lines, never branched on.
    pub(crate) court_demands_listed: u64,
    /// L8q — WHAT ANSWERED EACH DEMAND: per listing, per demand key, the
    /// address of the form the court replayed onto that demand's own links
    /// and the transport it was carried by. This is the ONLY demand
    /// carrier at the court: the demand vertices themselves live on the
    /// sentence, which the book already holds, and a second copy of them
    /// here would be the carrier that drifts (law 12).
    ///
    /// It is what item 5's expansion reads to know which vertex to expand
    /// and by what, and what makes a probe's closure a fact about ONE
    /// vertex rather than about a count of links.
    pub(crate) answered: crate::network::nerve::Keyed<
        crate::network::nerve::Keyed<(crate::transducer::address::Address, element::Versor)>,
    >,
    /// Čech complex per chart: vertices are covering hyphas, coefficients
    /// are Rotor. One Keyed entry per simplex; never Elements.
    pub(crate) complexes: crate::network::nerve::Keyed<crate::transducer::cech::Complex>,
    /// Unfilled simplices, keyed by (chart, simplex). Filled within spend_unit.
    pub(crate) unfilled: crate::network::nerve::Keyed<(crate::transducer::stalk::Invariant, crate::transducer::address::Address)>,
    /// Class simplices awaiting a prefix split.
    pub(crate) pending_refine: crate::network::nerve::Keyed<(
        crate::transducer::stalk::Invariant,
        crate::transducer::alphabet::Rotor,
        usize,
    )>,
    /// Addresses fetched as Dialect or Slashed — (chart key, stratum root at
    /// fetch). Dropped when confirm_peer_holds sees that root move.
    pub(crate) heard: crate::network::nerve::Keyed<(Vec<u8>, Vec<u8>)>,
    /// Heard addresses keyed by the chart they were heard under, so the
    /// two readers that want one chart's heard addresses read a fiber
    /// instead of the whole set — as `chart_listings` is to listings.
    ///
    /// `fetch_have` unioned ALL of `heard` into every chart's have-list,
    /// and it is called once per stratum per peer per exchange cycle;
    /// `confirm_peer_holds` read all of `heard` once per remote digest.
    /// Both are O(strata · |heard|), and `heard` grows with peers × corpus.
    pub(crate) heard_at: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Current dialect transition per (chart, hypha pair): one versor, the
    /// TUniversal stamp it stood at, the two hyphae, and the transport
    /// class key. A newer stamp supersedes; the chain keeps the old.
    pub(crate) cocycle_at: crate::network::nerve::Keyed<CocycleRow>,
    /// The stamp each standing cocycle stands at, so supersession is a
    /// question of causal order (45.3) and not of a height scalar. A row
    /// with no stamp here is UNMEASURED order and is therefore held, never
    /// overwritten.
    pub(crate) cocycle_receipt: crate::network::nerve::Keyed<crate::chronometry::Stamp>,
    /// Chart key → dialect row keys, so a fill or a refinement removes the row.
    pub(crate) cocycle_by_chart: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Every transition that ever stood between two hyphas on a chart, with
    /// the stamp of the receipt it stood at (78.27 (5)) — none where it stood
    /// at no act, which the snapshot refuses by name and counts.
    pub(crate) dialect_chain: Vec<(Vec<u8>, element::Versor, Option<crate::chronometry::Stamp>)>,
    /// Field entries of listings that settled Flat or relocated — FLD1 cold.
    pub(crate) field_cold: crate::network::nerve::Keyed<ListingField>,
    /// Settlements the court has read; the SET2 section holds them.
    pub(crate) settlements_cold: Vec<crate::anastome::dialogue::Settlement>,
    /// Test height: when set, height() returns this instead of the chain head.
    /// C6: a TEST knob, `None` on any live fleet. A kernel that can be told
    /// its own height is a kernel whose time is an input (law 9), so the
    /// SETTER is behind `cfg(test)` and nothing in production can reach it.
    pub(crate) height_override: Option<u64>,
    /// Set on home_install, glue Admitted, mint, and slash.
    pub(crate) persist_due: bool,
    /// 45.8 — THE COLD SEGMENT: where a sealed era's receipts live once
    /// the fold has shed them. Append-only, written once per fold, and
    /// never rewritten by a snapshot: the hot ledger carries only the era
    /// DESCRIPTORS, so persisting state does not re-serialise the whole
    /// history. `None` on a kernel with no home — the fixture and the
    /// tests — where a sealed era keeps its payload in hand as before.
    pub(crate) eras_path: Option<std::path::PathBuf>,
    /// C6: a TEST knob, `None` on any live fleet. Production reads
    /// `dynamic_lane_capacity`, which is measured; a cap that can be set is
    /// a free constant (rule 5), so the SETTER is behind `cfg(test)`.
    pub(crate) lane_cap: Option<[u64; 4]>,
    /// Height at first vantage of a standing carrier (listing or docket).
    pub(crate) standing_h: crate::network::nerve::Keyed<u64>,
    /// Listing address → corpus task id, so a verified solve pays the same escrow.
    pub(crate) listing_task: crate::network::nerve::Keyed<crate::transducer::address::Address>,
    /// Reverse: task → listings bound to it.
    pub(crate) task_listings: crate::network::nerve::Keyed<Vec<Address>>,
    /// Train pairs of a bound task. Install mints only what a rule speaks.
    pub(crate) task_family: crate::network::nerve::Keyed<Vec<crate::transducer::family::TrainPair>>,
    /// Sentence address → task, from perceive_speech. Replay reads the family.
    pub(crate) speech_task: crate::network::nerve::Keyed<Address>,
    /// THE SENTENCES THIS COURT REFUSED (50.9): the kind, the coordinate
    /// and the receipt it answered with, keyed by the refused address, so
    /// the contraction a voice cites next is verified against what this
    /// court actually named. A record leaves when its contraction is
    /// verified. In-memory, never persisted: a restart refuses afresh.
    pub(crate) refusals: crate::network::nerve::Keyed<crate::court::verdict::Refused>,
    /// THE SETTLEMENTS awaiting the court: (listing, its chain, the voice,
    /// the utterance, magnitude before, magnitude after). The court replays
    /// the utterance and measures the deflation itself — proof of
    /// convergence on the work that actually happens. Bounded by the
    /// reserve ceiling; drained as the court pulls the docket.
    pub(crate) settlements: Vec<crate::anastome::dialogue::Settlement>,
    /// The book generation the pressure was last measured at.
    pub(crate) pressure_gen: u64,
    /// THE DEFLATIONS: per provisional word, the magnitude of the listed
    /// demand its install settled — zero when the germ settled nothing on
    /// the book. The court's proof of convergence rides with the word.
    pub(crate) deflations: crate::network::nerve::Keyed<[u64; 4]>,
    /// ACT1 — OWN-LINE ACTS SEALED WITHOUT A FRAME (78.27 Q1(a)): acts that
    /// named a word no row of this court measures, so their μ⃗ was read as
    /// `[0; 4]` because nothing measured it. Counted at seal, since boot.
    pub(crate) unframed: u64,
    /// OK1 — CLOSES WITHHELD FOR SEPARATION: strokes at whose boundary this
    /// court's orbit frame coincided on every lane with a peer's closed at
    /// the same tick, so the close was withheld (78.23 as amended). Since
    /// boot.
    pub(crate) withheld: u64,
    /// LD3-a — THE WORDS BY FIBER, AT RUNTIME: T1's own key — a word's
    /// `Cite::Word{chart, class, at}` written by its `put_canonical` — to
    /// the word's address, filed where `admitted_at` is filed and rebuilt
    /// from T1 at restore. A prefix read by `chart‖class` is the fiber read.
    pub(crate) word_fibers: crate::network::nerve::Keyed<crate::transducer::address::Address>,
    /// Each admitting stamp's fiber key, so a word that leaves its last row
    /// takes its fiber entry with it once its element is gone.
    pub(crate) fiber_key_at: crate::network::nerve::Keyed<Vec<u8>>,
    /// RM1, 78.37(4) — THE REMAINDERS BY WHERE THEY WERE PERCEIVED: the
    /// residue's chart ‖ κ → the word whose torsion record carries it,
    /// derived from the records where a word is filed at its fiber and at
    /// restore (the word_fibers pattern); each word's keys; each refusal of a
    /// Torse mint by its own name — the record refused the remainder, nothing
    /// priced it, the mint refused it, the book did not take it — and
    /// remainders coinciding with one standing (one per class at a chart).
    pub(crate) remainder_fibers: crate::network::nerve::Keyed<crate::transducer::address::Address>,
    pub(crate) remainder_keys: crate::network::nerve::Keyed<Vec<Vec<u8>>>,
    pub(crate) remainders_refused: u64,
    pub(crate) remainders_past_carrier: u64,
    /// PD1 — cycles whose stored period was not the one read off their
    /// generator, refused by `record_cycle` (`Filed::Misspoken`).
    pub(crate) remainders_misspoken: u64,
    pub(crate) remainders_unpriced: u64,
    pub(crate) remainders_unminted: u64,
    pub(crate) remainders_unoffered: u64,
    pub(crate) remainders_coincident: u64,
    /// STD1 (rule 11) — THE ROOM READS THAT DID NOT ANSWER, each by its own
    /// name: a lane drawn past its cover (a broken count) and a cover or
    /// credit past its carrier. Atomic because the room is read through
    /// `&self` on the request path; a count of this process, volatile.
    pub(crate) room_broken: std::sync::atomic::AtomicU64,
    pub(crate) room_overflowed: std::sync::atomic::AtomicU64,
    /// STD1 — the court's own operations and readings refused past their
    /// carrier: an occupy (its listing refused), a recall owed, the book's
    /// T pressure, a relocation's total, a solve's paid sum. Volatile.
    pub(crate) overflowed: u64,
    /// RM1 (c) — voices' re-fill acts replayed here: applied (the Congruent
    /// simplex written Filled, the rewrite's remainder answered, the pressure
    /// minted under POUW), and refused by why.
    pub(crate) refills: super::court::fills::RefillTally,
    /// THE STAMP ALIASES filed on a word: the peer admitting stamps a fiber
    /// read located at this word (`held_at` entries beside its own), dropped
    /// with the word. Re-derived at restore from the ledger's peer nodes.
    pub(crate) aliases: crate::network::nerve::Keyed<Vec<crate::chronometry::Stamp>>,
    /// 78.27 (6) — SETTLEMENTS REFUSED FOR WANT OF AN ACT to stand at: a
    /// settlement recorded before this court sealed its first act. Since
    /// boot.
    pub(crate) settlements_unplaced: u64,
    /// 78.27 (3) — THE STAMP OF THE ACT THAT RULED EACH WORD INTO THE
    /// FEDERATION, by the word's address: distinct from `admitted_at` where
    /// the word was admitted earlier (a List, then a Ruling). The
    /// federation's order is re-derived from these at restore.
    pub(crate) fed_at: crate::network::nerve::Keyed<crate::chronometry::Stamp>,
    /// ACT1 (EC1) — PEERS' RULINGS ON THIS COURT'S BOOK, per ruling arm
    /// (`Ruled::index`): `[admitted, applied, awaiting, refused]`, where
    /// admitted = applied + awaiting + refused. Admitted counts a peer's
    /// Ruling receipt on a word this court can rule; awaiting is a level,
    /// the rulings held on an unheld citation; refused counts rulings whose
    /// replay did not re-derive (78.9). Per node, since boot.
    pub(crate) rulings: [[u64; 4]; 4],
    /// F5 — THE REFUSED RULINGS BY REASON, per arm (`Ruled::index`) and per
    /// reason (`RulingRefused::index`); each arm's row sums to its refused.
    pub(crate) ruling_refusals: [[u64; 3]; 4],
    /// 78.26(6), TP1 — closed rings of the cover's nerve the glue reopened
    /// because a gauge moved: `[filled, congruent]`, since boot.
    pub(crate) reopened: [u64; 2],
    /// EC1 — PEERS' RULINGS WAITING ON A CITATION THIS COURT DOES NOT HOLD,
    /// keyed by the unheld cite's STAMP — the node gives no address — and
    /// heard again when a word is filed at that stamp (`file_admitted`).
    pub(crate) awaiting_rulings: crate::network::nerve::Keyed<Vec<crate::chronometry::KStepReceipt>>,
    /// ACT1 (78.21(2)) — the stamp's height and `K` at which this court's
    /// snapshot was last written whole, noted after the write lands.
    pub(crate) persisted: Option<(u64, u64)>,
    /// Round-robin cursor for syntropic optimization — which generator to test for
    /// redundancy next (bounds consolidation to one reduce per depleted step).
    /// Last key the syntropic cursor yielded; the next stroke resumes after
    /// it. `None` is a circuit that has not started, which is an ABSENCE and
    /// not an empty key (rule 11): every one of the eight resume cursors
    /// below reads the same way, and `Keyed::cursor` takes the option.
    pub(crate) optimize_cursor: Option<Vec<u8>>,
    /// Keyed cursor into `open` for `syntropic_stroke` (rule 10, S13.3):
    /// the stagnation circuit resumes where the last stroke left off, so a
    /// docket beyond one budget is read whole across strokes.
    pub(crate) syntropic_cursor: Option<Vec<u8>>,
    /// Keyed cursor into `open` for `drain_unreachable` (rule 10, S13.3):
    /// the drain resumes where the last stroke left off.
    pub(crate) drain_cursor: Option<Vec<u8>>,
    /// Keyed cursor into `on_page` for the local `quote` (rule 10, S13.3):
    /// a page beyond one budget is served whole across quotes.
    pub(crate) quote_cursor: Option<Vec<u8>>,
    /// Polytonic kinematic pressure + torsion (per lane).
    pub(crate) physics: crate::physics::Physics,
    /// The settlement ledger (POW++-gated bounties, consilience splits).
    pub(crate) treasury: crate::economy::Treasury,
    /// This hypha's verifying key.
    pub(crate) identity: [u8; 32],
    /// Covering envelope of the genesis corpus, per lane. None is unmeasured.
    pub(crate) genesis_cover: Option<[u64; 4]>,
    /// Listing address → the voice whose bond the draw occupies.
    pub(crate) drawers: crate::network::nerve::Keyed<[u8; 32]>,
    /// 1.8, 50.2 step 2 — THE GENERATORS this court has ruled, by the chart
    /// of the versor each carries: the word vertices of the ruled forms,
    /// which is where a presentation's generators live since a ruled word
    /// became a form (L5). Filled beside the axis index in `ruled_admit`.
    pub(crate) generators: crate::transducer::sentence::GeneratorIndex,
    /// 21 step 3, 50.2 step 2 — SENTENCES AWAITING A CITED FORM. Keyed by
    /// the cited address this court has not ruled; each entry is the
    /// voice that spoke, the task it was bound to, and the tape. A voice
    /// one syllabus step ahead of this court's cover is not wrong and its
    /// sentence is not refused: it waits here, and `rehear_awaiting`
    /// hears it again when that address is ruled. Without this the voice
    /// received a refusal and sealed the address, losing the sentence
    /// permanently for being early.
    pub(crate) awaiting:
        crate::network::nerve::Keyed<Vec<([u8; 32], crate::transducer::address::Address, Element)>>,
    /// E2, 26 — THE INVERSE OF `discoverers`: voice → the addresses that
    /// voice discovered. `use_credit` asks "which forms are this voice's",
    /// and `discoverers` answers the other question; without this carrier
    /// the read covers every discovered address on every room check, which
    /// is the enumeration law 3 forbids. Written at the two sites that
    /// write `discoverers` and nowhere else, so the two cannot diverge.
    pub(crate) discovered_by: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// L15, 45.1, 1.9 — WHO ATTESTED WHAT: the committed address of every
    /// receipt this court holds → the signer and the kind of act, with
    /// the K it stood at. Outer key is the address; inner is the signer's
    /// verifying key followed by the act's kind byte, both written out
    /// (A21), so the reading is by content and never by an act digest —
    /// `operator_hash` names WHICH operation, and is never reasoned on.
    ///
    /// DEPTH IS NOT HERE. This is the index; the depth of an address is
    /// the number of DISTINCT signers among these entries whose kind is
    /// `Install`, `Ruling` or `Adopted`, computed at the read and stored
    /// nowhere (law 12). Without the index that read covers the whole
    /// chain per address, which is law 3's enumeration.
    ///
    /// Derived, never persisted: rebuilt from the chain's own receipts at
    /// restore, like the axis and moduli indices.
    pub(crate) attestors: crate::network::nerve::Keyed<crate::network::nerve::Keyed<crate::chronometry::Place>>,
    /// Per-voice occupancy of open draws, keyed by vk.
    pub(crate) drawn: crate::network::nerve::Keyed<[u64; 4]>,
    /// Per-voice recall fee it could not cover, per lane (MU-ECONOMY §3.1
    /// as ruled 2026-09-09, S13.15): while a claim of the voice is in
    /// flight on a lane it owes on, its room on that lane is closed; the
    /// record clears when that claim closes. Rebuilt from nothing at boot.
    pub(crate) recall_short: crate::network::nerve::Keyed<[u64; 4]>,
    /// Words installed on two-voice consilience but NOT yet court-final
    /// (Step 5): local dialect awaiting the admission docket, keyed by address.
    pub(crate) provisional: crate::network::nerve::Keyed<Element>,
    /// Listings whose field has a non-Flat chart — the court's docket, written
    /// in write_field, read in key order.
    pub(crate) open: crate::network::nerve::Keyed<()>,
    /// Keyed cursor into `open` for court_stroke.
    pub(crate) open_cursor: Option<Vec<u8>>,
    /// Keyed cursor into `pending` for the lapse (45.7): the stroke's circuit
    /// over the docket resumes where the last one left off, so a book that
    /// outgrows one stroke's budget is still read whole across strokes.
    pub(crate) lapse_cursor: Option<Vec<u8>>,
    /// The docket's circuits (45.7, S13.3): the circuits of `open` the court
    /// stroke completed and the strokes the last one took, written once
    /// per circuit as `court circuit docket` — the court's local time
    /// beside K.
    pub(crate) docket_circuits: crate::physics::Circuits,
    /// Where the phase's next window starts (45.9, S13.7): this court's
    /// head place (none before genesis) — the window is the line strictly
    /// after it — and the cover reading at the last docket circuit. A
    /// reading of the carriers, never a phase — the phase is computed and
    /// written.
    pub(crate) phase_mark: Option<(Option<crate::chronometry::Place>, crate::cognition::court::phase::CoverReading)>,
    /// Keyed cursors into `unfilled` and `pending_refine` for `stroke_fills`
    /// (rule 10): each stroke resumes where the last left off, so a tail
    /// beyond one budget is reached rather than starved by the head.
    pub(crate) fill_cursor: Option<Vec<u8>>,
    pub(crate) refine_cursor: Option<Vec<u8>>,
    /// This hypha's cover, admitted in write_field; own_cover_charts reads it.
    pub(crate) own_cover: crate::network::nerve::Cover,
    /// Listings keyed by chart, so rank_over's moved set is a fiber.
    pub(crate) chart_listings: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Ruled addresses beside the ruled cover trie.
    pub(crate) ruled_addrs: crate::network::nerve::Keyed<()>,
    /// Ruled cover root, maintained on admit/remove.
    pub(crate) ruled_root: [u8; 32],
    /// Whether the cover has moved since the root over it was measured.
    pub(crate) ruled_root_due: bool,
    /// Per-lane occupancy of in-flight execution leases, not the book
    /// row count. Cold listings exert zero occupancy. Lane 2 is quote
    /// leases keyed in `t_leases`.
    pub(crate) occupancy: [u64; 4],
    /// Voices that currently hold a T-lease (ACT_QUOTE checkout).
    pub(crate) t_leases: crate::network::nerve::Keyed<()>,
    /// Per-lane magnitude sum of the open book, updated on list/unlist.
    pub(crate) pressure_sum: [u64; 4],
    /// Listings standing on each lane, for eviction candidates.
    pub(crate) lane_listings: [crate::network::nerve::Keyed<()>; 4],
    /// Anchor evidence membership keyed by address+footprint.
    pub(crate) evidence_at: crate::network::nerve::Keyed<()>,
    /// SELECTION-FIBER CRITICAL PAIRS (Step 7): Q_anchor residuals — a word's
    /// substance matched a defect but its π refused the support. (word address,
    /// the symmetric difference, the scene it arose in). The court's admission
    /// docket refines these into rebound anchors. Bounded; in-memory work queue.
    pub(crate) anchor_mismatches: Vec<crate::transducer::anchor::AddressedEvidence>,
    /// BIRTH EVIDENCE for the selection fiber: (word address, the footprint it
    /// spoke on, the scene) — every germination and every witnessed placement
    /// deposits one; the court's refinement derives π over the accumulated
    /// family (one evidence never binds). Bounded; in-memory.
    pub(crate) anchor_evidence: Vec<crate::transducer::anchor::AddressedEvidence>,
}

impl Kernel {
    /// THE FIXTURE KERNEL (§3.4): a bootstrap identity `[0x11; 32]` funded
    /// and staked at the six-family cover below, so a test can list, quote
    /// and settle without registering a corpus. No production path calls
    /// it: the hypha boots by [`Kernel::boot`], whose cover is measured from
    /// the corpus that registers and whose bond is the hypha's own key (rule
    /// 8). Booted from here, every live hypha of the 2026-09-08 soak carried
    /// a stake for a key the mesh record never named and dropped it at boot
    /// (I38).
    pub fn new() -> Self {
        let mut k = Kernel { identity: [0x11; 32], ..Kernel::default() };
        if let Some(cover) = Self::measured_test_cover() {
            k.set_genesis_cover(cover);
            k.enroll([0x11; 32], cover);
        }
        k
    }

    /// THE HYPHA'S KERNEL AS IT BOOTS: no identity until `bind_identity`, no
    /// cover until the corpus registers (`genesis::register_corpora` measures
    /// it and stakes the hypha's own key), nothing funded and nothing staked
    /// that the mesh record does not name.
    pub fn boot() -> Self {
        Kernel::default()
    }

    /// Bootstrap genesis: the lane-wise measure of a named genesis family,
    /// which a server replaces the moment a corpus registers (the cover is
    /// measured from the registered tasks and re-staked,
    /// `mycelium-server.rs` `set_genesis_cover`). It is a starting corpus,
    /// not a measurement of the live one, and it is a bond a voice may work
    /// within before any corpus arrives. It is measured by `family_cover`,
    /// so it is the lane-wise SUM over these pairs (12.6) and covers each of
    /// them; an envelope over them would not.
    pub(crate) fn measured_test_cover() -> Option<[u64; 4]> {
        crate::arc::family_cover(&[
            (vec![vec![1, 1], vec![1, 0]], vec![vec![0, 1], vec![1, 1]]),
            (vec![vec![1, 1]], vec![vec![2, 2]]),
            (vec![vec![1, 1, 0, 0], vec![0, 0, 0, 0]], vec![vec![0, 0, 1, 1], vec![0, 0, 0, 0]]),
            (vec![vec![1, 1, 1, 3]], vec![vec![2, 2, 1, 4]]),
            (
                vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
                vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]],
            ),
            (vec![vec![1, 1, 1, 1, 1, 1]], vec![vec![2, 2, 2, 2, 2, 2]]),
        ])
    }

    /// The genesis covering bond, if measured.
    pub fn set_genesis_cover(&mut self, bond: [u64; 4]) {
        self.genesis_cover = Some(bond);
    }

    /// Bind the cold segment a fold sheds into (45.8). Called at boot
    /// beside the ledger path; a kernel without one keeps a sealed era's
    /// receipts in hand, which is what a fixture wants and what a live
    /// court must not do.
    pub fn bind_eras(&mut self, path: std::path::PathBuf) {
        self.eras_path = Some(path);
    }

    /// File a sealed era: its payload appended to the cold segment, and
    /// the descriptor — root, signers, signatures, K range and offset —
    /// kept. `None` from the append leaves the era whole in memory rather
    /// than losing it (rule 11: a write that did not happen is not a write
    /// that did).
    pub(crate) fn file_era(&mut self, era: crate::chronometry::Era) {
        // AN ERA IS FILED ONCE. `fold_round` re-asks every witness on every
        // exchange cycle while a proposal is pending, and a witness filed
        // whatever it was asked to co-sign — so one uncommitted proposal
        // was re-fetched and re-written by every peer, every cycle. At
        // epoch 13 that put 7.3 GiB per court into the cold segments in
        // about four minutes and filled the disk.
        //
        // The root names the era, and the archive is in K order, so the
        // question is a bisection and not a scan (law 3).
        // AND IT IS FILED IN K ORDER. `push` appended, so an archive that
        // interleaves signers — a witness holds its own folds and the ones
        // it co-signed — was not ordered by `root.k` at all, and the
        // bisection `eras_over` makes over it was unsound. Inserting at the
        // position the coordinate names keeps the one invariant both reads
        // depend on.
        let seat = self.archive.partition_point(|e| e.root.at < era.root.at);
        if let Some(run) = self.archive.get(seat..) {
            for held in run {
                if held.root.at != era.root.at {
                    break;
                }
                if held.root.hash == era.root.hash && held.signer == era.signer {
                    return;
                }
            }
        }
        let Some(path) = self.eras_path.clone() else {
            self.archive.insert(seat, era);
            return;
        };
        let Some(payload) = era.payload() else {
            eprintln!("  era refused: a receipt of its payload cannot be written");
            return;
        };
        let at = match std::fs::OpenOptions::new().create(true).append(true).open(&path) {
            Ok(mut f) => {
                use std::io::{Seek, Write};
                f.stream_position().and_then(|at| f.write_all(&payload).map(|()| at)).ok()
            }
            Err(_) => None,
        };
        match at {
            Some(at) => self.archive.insert(seat, era.filed_at(at)),
            None => self.archive.insert(seat, era),
        }
    }

    /// A filed era's receipts, paged from the cold segment on demand. An
    /// era that still holds its payload answers from memory.
    pub(crate) fn era_receipts(
        &self,
        era: &crate::chronometry::Era,
    ) -> Option<Vec<crate::chronometry::KStepReceipt>> {
        let Some(at) = era.at else {
            return Some(era.receipts.clone());
        };
        let path = self.eras_path.as_ref()?;
        let bytes = std::fs::read(path).ok()?;
        let mut pos = usize::try_from(at).ok()?;
        crate::chronometry::Era::read_payload(&bytes, &mut pos)
    }

    pub fn genesis_cover(&self) -> Option<[u64; 4]> {
        self.genesis_cover
    }

    /// 47.2, on the voice: admit one hypha's receipt, carried with the
    /// prefix it was signed over. `origin` is this node's own
    /// `(H_genesis, E_origin)`. Only `Parent` touches a carrier -- it puts
    /// the receipt on the chain's known set and queues it to be cited by
    /// this mind's next step, which is how height climbs on ADMITTED
    /// receipts rather than on this mind's own strokes.
    ///
    /// Step 2 is the caller's and it is PROVED, not assumed: `verified` is
    /// built only by `court::admission::{admit_signature, verify_signature}`
    /// over exactly the prefix it carries, so no call site can reach the
    /// chain checks — or the burn on the `Equivocation` arm (47.4) — with
    /// bytes nobody verified. A receipt whose signer is not the key that
    /// verified is two claims and is `REFUSE_RECEIPT` (A2's rule, one
    /// signer).
    pub fn admit_receipt(
        &mut self,
        origin: ([u8; 32], u32),
        verified: crate::court::admission::Verified<'_>,
        r: &crate::chronometry::KStepReceipt,
    ) -> crate::court::admission::Admitted {
        if verified.signer() != r.stamp.vk {
            return crate::court::admission::Admitted::Refused(
                crate::court::admission::REFUSE_RECEIPT,
            );
        }
        let verdict = crate::court::admission::admit(origin, verified.prefix(), r, &self.chain);
        match &verdict {
            crate::court::admission::Admitted::Parent(p) => {
                self.chain.admit(r.clone());
                self.note_attestor(r);
                self.pending_parents.insert(p.at.key(), *p);
                // LD3-a: a word the peer's act admitted, located here by its
                // fiber where exactly one word stands on it.
                self.alias_admitting(r);
                // EC1: a peer's ruling applies at the node that admits its
                // receipt, through the receipt path — the arm its kind
                // carries, on the word its citation locates here.
                if let crate::chronometry::ActKind::Ruling(arm) = r.kind {
                    self.hear_ruling(r, arm);
                }
                // RM1 (c): a voice's re-fill, replayed here — the court's one
                // door to its Congruent simplex.
                if r.kind == crate::chronometry::ActKind::Settled {
                    self.hear_refill(r);
                }
            }
            crate::court::admission::Admitted::Equivocation { at, .. } => {
                self.burn_equivocation(r.stamp.vk, *at);
            }
            _ => {}
        }
        verdict
    }

    /// 47.2 STEP 3'S BACK-FILL (S13.10): receipts fetched by place range
    /// from the signer or a holder, admitted in order by their chain and
    /// held as known so later heights re-derive through them; one already
    /// held at its place is passed over. Nothing here is cited as a parent:
    /// the signed frame that closes the gap is, when it is admitted again.
    /// Answers how many the page advanced by AND THE CITATION THAT STOPPED
    /// IT, when one did: the stamp of a receipt this node does not
    /// hold and cannot re-derive a height without.
    ///
    /// THE MESH IS A DIRECTED ACYCLIC GRAPH, NOT ONE SIGNER'S LOG. This
    /// broke on anything that was not `Parent` or `Stale`, so a back-fill
    /// paging A's chain died the moment A's receipt cited B — which every
    /// receipt on a live mesh does. On the fleet at epoch 10 that read
    /// `backfill ce02… 940..940 admitted 0`, fifty-two times, forever.
    pub fn admit_backfill(
        &mut self,
        receipts: &[crate::chronometry::KStepReceipt],
    ) -> (usize, Option<crate::chronometry::Parent>) {
        let mut n = 0usize;
        for r in receipts {
            match crate::court::admission::admit_backfilled(r, &self.chain) {
                crate::court::admission::Admitted::Parent(_) => {
                    self.chain.admit(r.clone());
                    self.note_attestor(r);
                    self.alias_admitting(r);
                    n += 1;
                }
                crate::court::admission::Admitted::Stale => n += 1,
                // The hole is named and handed back; the receipt that
                // exposed it is not admitted and not refused, it waits for
                // the citation to arrive (topological order).
                crate::court::admission::Admitted::MissingParent(p) => return (n, Some(p)),
                _ => break,
            }
        }
        (n, None)
    }

    /// 47.4 ON A THIRD PARTY'S PROOF (`ACT_EQUIVOCATION`, I21): the proof
    /// was decided by `admission::admit_equivocation` over two verified
    /// signatures, and this court burns the signer's bond as it does on the
    /// `Equivocation` arm of its own admission. `Some(burned)` is what left
    /// circulation; `None` is a signer with no bond left to take — the same
    /// proof presented again, or a signer never bonded here — and nothing is
    /// written for it.
    pub fn accept_equivocation(
        &mut self,
        proof: crate::court::admission::Equivocated,
    ) -> Option<[u64; 4]> {
        self.burn_equivocation(proof.vk, proof.at)
    }

    /// 47.4: TWO SIGNED RECEIPTS UNDER ONE KEY AT ONE PLACE WITH DIFFERENT
    /// CHAIN HASHES IS AN EQUIVOCATION PROOF, and the court burns the
    /// signer's bond on all four lanes — one `burn` entry, supply reduced
    /// by the same vector, so §6's equation holds through the forfeiture
    /// (R9). Nothing is replayed and nothing is redistributed.
    ///
    /// `Admissions::admit` has held exactly that proof since it was
    /// written: the hash it holds at that K, and the one claimed. Both
    /// signatures were verified under the claimed key at 47.2 step 2, the
    /// held one when it was admitted and the claimed one by this caller.
    /// Every reader of the verdict answered `REFUSE_EQUIVOCATION` and one
    /// counted a tally; nothing burned. A signer could therefore tell two
    /// peers two truths, be caught by both, and pay nothing — which is the
    /// one line of 48's table that 47.4 is the whole of.
    ///
    /// A second frame at the same K burns nothing more: the bond is gone,
    /// `Treasury::burn` takes what is held and writes no entry for a
    /// vector of zeros, and the line is written only for what left.
    ///
    /// Reached from this node's own admission (the `Equivocation` arm) and
    /// from a third party's presentation (`accept_equivocation`, 0x35);
    /// both hold the two verified signatures 47.4 asks for. `Some` is the
    /// vector that left circulation, `None` a signer with nothing left to
    /// burn, for which nothing is written.
    fn burn_equivocation(&mut self, vk: [u8; 32], at: crate::chronometry::Place) -> Option<[u64; 4]> {
        let bond = self.treasury.bond_of(&vk)?;
        let anchor = self.anchor();
        let burned = self.treasury.burn(vk, bond, anchor);
        if crate::economy::lanes_short(&burned).is_empty() {
            return None;
        }
        eprintln!(
            "  court equivocation {:02x}{:02x}… K {} burned {burned:?}",
            vk[0],
            vk[1],
            at.print()
        );
        Some(burned)
    }

    /// Receipts admitted and not yet cited. A step consumes them.
    pub fn pending_parents(&self) -> usize {
        self.pending_parents.len()
    }

    /// What this mind knows of a hypha's chain head, if it has admitted one:
    /// the node at the top of that signer's line in the ledger.
    pub fn known_head(&self, vk: &[u8; 32]) -> Option<(crate::chronometry::Place, [u8; 32])> {
        self.chain.line_head(vk).map(|r| (r.stamp.place(), r.chain_hash))
    }

    pub fn height(&self) -> u64 {
        // A test may hold the height; otherwise it is the chain's own head
        // (law 9), which is where the court's clock actually stands.
        match self.height_override {
            Some(h) => h,
            None => self.chain.head_height(),
        }
    }

    /// C6: test only. Time is not an input to a kernel (law 9).
    #[cfg(test)]
    pub fn set_height(&mut self, h: u64) {
        self.height_override = Some(h);
    }

    /// C6: test only. Capacity is measured, never set (rule 5).
    #[cfg(test)]
    pub fn set_lane_cap(&mut self, cap: [u64; 4]) {
        self.lane_cap = Some(cap);
    }

}