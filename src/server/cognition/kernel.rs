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
    /// The germinated basis, each word reached by its address. In-place
    /// writes go through `word_mut` / `touch_at`. Language snapshots still
    /// carry an insertion-order slice for Decomposed citations.
    pub(crate) basis: crate::network::nerve::Keyed<std::sync::Arc<Element>>,
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
    /// Court replay of A20 certificates: citer → (certificate key → (ok, K)).
    pub(crate) replay_memo: crate::network::nerve::Keyed<crate::network::nerve::Keyed<(bool, u64)>>,
    /// Cited form → citers. Slash of a cited form redockets each citer.
    pub(crate) cited_by: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// THE BOOK SNAPSHOT — the open listings as an immutable shared value
    /// sealed at a book generation; a market view shares them by pointer
    /// and the book is copied only when it moved.
    pub(crate) book: (u64, std::sync::Arc<crate::network::nerve::Keyed<(Element, u64)>>),
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
    /// THE OPEN-DEMAND MARKET — (listing, the K-step it listed at). Dwell is
    /// measured in ALGEBRAIC time (K-steps, court-replayable), never a wall
    /// clock. Capacity is thermodynamic (dynamic_lane_capacity — live memory
    /// headroom × per-lane metabolic tension); eviction is by ACTION
    /// POTENTIAL, never FIFO. No static cap exists.
    pub(crate) pending: crate::network::nerve::Keyed<(Element, u64)>,
    /// THE CHRONOMETRIC RECEIPT CHAIN — the K-step index is the mesh's temporal
    /// authority (algebraic time, no wall clocks); every install seals a
    /// parent-linked receipt (LAW-5 continuity). The Step-4 ledger's spine.
    pub(crate) chain: crate::chronometry::ReceiptChain,
    /// The head hash of the court's chain as last ruled to this hypha —
    /// every settlement receipt here links to it, so the hypha's k-steps
    /// are causally ordered against the mesh's temporal authority.
    pub(crate) court_head: [u8; 32],
    /// Retirements of basis words, counted: while it stands still, every
    /// index into the basis names the same word it did.
    pub(crate) retire_gen: u64,
    /// 47.2's admission state: what this mind knows of each hypha's chain
    /// head. Empty until a receipt is admitted, and nothing admits one
    /// except `admit_receipt`.
    pub(crate) admissions: crate::court::admission::Admissions,
    /// Peer receipts admitted since this mind's last step, waiting to be
    /// CITED as parents of it (47.2 step 3). This is the only thing that
    /// makes height more than a rename of K: `append` takes
    /// `1 + max(h(prev), parent heights)`, and with no parent ever named
    /// the max was always this chain's own head, so TUniversal advanced on
    /// this mind's strokes rather than on the mesh's receipts (45.2).
    ///
    /// KEYED BY CHAIN HASH. It was a `Vec` and every admission scanned the
    /// whole queue for a duplicate (rule 10). Between two of this mind's
    /// steps a mesh admits one receipt per speaking peer, so the queue is
    /// as long as the mesh is wide and the scan ran once per arrival: the
    /// cost of admitting a peer grew with the number of peers. A chain
    /// hash names one receipt, and the carrier says so.
    pub(crate) pending_parents: crate::network::nerve::Keyed<crate::chronometry::Parent>,
    /// THE RECEIPTS OF SPEECH: listing → voices that have spoken on it.
    pub(crate) spoken: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Transpose: voice → listings that voice holds. Written with `spoken`.
    pub(crate) spoken_by: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Idempotence of an utterance: address → voice → K → the reply last given.
    /// The receipt is (vk, M2 address, K); a second frame with the same
    /// triple is answered from here, never re-docketed or re-charged.
    pub(crate) utterances: crate::network::nerve::Keyed<crate::network::nerve::Keyed<crate::network::nerve::Keyed<Outcome>>>,
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
    pub(crate) flat_loops: crate::network::nerve::Keyed<crate::network::nerve::Keyed<(crate::transducer::stalk::Invariant, element::Versor)>>,
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
    pub(crate) crt_imported: bool,
    /// Čech complex per chart: vertices are covering hyphas, coefficients
    /// are Rotor. One Keyed entry per cell; never Elements.
    pub(crate) complexes: crate::network::nerve::Keyed<crate::transducer::cech::Complex>,
    /// Unfilled cells, keyed by (chart, cell). Filled within spend_unit.
    pub(crate) unfilled: crate::network::nerve::Keyed<(crate::transducer::stalk::Invariant, crate::transducer::address::Address)>,
    /// Class cells awaiting a prefix split.
    pub(crate) pending_refine: crate::network::nerve::Keyed<(
        crate::transducer::stalk::Invariant,
        crate::transducer::alphabet::Rotor,
        u8,
    )>,
    /// Addresses fetched as Dialect or Slashed — (chart key, stratum root at
    /// fetch). Dropped when confirm_peer_holds sees that root move.
    pub(crate) heard: crate::network::nerve::Keyed<(Vec<u8>, Vec<u8>)>,
    /// Current dialect transition per (chart, hypha pair): one versor, the
    /// TUniversal stamp it stood at, the two hyphae, and the transport
    /// class key. A newer stamp supersedes; the chain keeps the old.
    pub(crate) cocycle_at: crate::network::nerve::Keyed<CocycleRow>,
    /// The receipt each standing cocycle stands at, so supersession is a
    /// question of causal order (45.3) and not of a height scalar. A row
    /// with no receipt here — one restored from a snapshot written before
    /// this — is UNMEASURED order and is therefore held, never overwritten.
    pub(crate) cocycle_receipt: crate::network::nerve::Keyed<[u8; 32]>,
    /// Chart key → dialect row keys, so a fill or a refinement removes the row.
    pub(crate) cocycle_by_chart: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Every transition that ever stood between two hyphas on a chart.
    pub(crate) dialect_chain: Vec<(Vec<u8>, element::Versor, u64)>,
    /// Field entries of listings that settled Flat or relocated — FLD1 cold.
    pub(crate) field_cold: crate::network::nerve::Keyed<ListingField>,
    /// Settlements the court has read; the SET1 section holds them.
    pub(crate) settlements_cold: Vec<crate::anastome::dialogue::Settlement>,
    /// Test height: when set, height() returns this instead of the chain head.
    /// C6: a TEST knob, `None` on any live fleet. A kernel that can be told
    /// its own height is a kernel whose time is an input (law 9), so the
    /// SETTER is behind `cfg(test)` and nothing in production can reach it.
    pub(crate) height_override: Option<u64>,
    /// Set on home_install, glue Admitted, mint, and slash.
    pub(crate) persist_due: bool,
    /// C6: a TEST knob, `None` on any live fleet. Production reads
    /// `dynamic_lane_capacity`, which is measured; a cap that can be set is
    /// a free constant (rule 5), so the SETTER is behind `cfg(test)`.
    pub(crate) lane_cap: Option<[usize; 4]>,
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
    /// The k-step at which each provisional word was installed — the
    /// coordinate a court's "since" is read against (order inside one kernel).
    pub(crate) provisional_k: crate::network::nerve::Keyed<u64>,
    /// Height at first vantage of each provisional — standing extent.
    pub(crate) provisional_h: crate::network::nerve::Keyed<u64>,
    /// Round-robin cursor for syntropic optimization — which generator to test for
    /// redundancy next (bounds consolidation to one reduce per depleted step).
    /// Last key the syntropic cursor yielded; the next stroke resumes after it.
    pub(crate) optimize_cursor: Vec<u8>,
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
    /// Per-voice occupancy of open draws, keyed by vk.
    pub(crate) drawn: crate::network::nerve::Keyed<[u64; 4]>,
    /// Words installed on two-voice consilience but NOT yet court-final
    /// (Step 5): local dialect awaiting the admission docket, keyed by address.
    pub(crate) provisional: crate::network::nerve::Keyed<Element>,
    /// Listings whose field has a non-Flat chart — the court's docket, written
    /// in write_field, read in key order.
    pub(crate) open: crate::network::nerve::Keyed<()>,
    /// Keyed cursor into `open` for court_stroke.
    pub(crate) open_cursor: Vec<u8>,
    /// Keyed cursor into `pending` for the lapse (45.7): the stroke's orbit
    /// over the docket resumes where the last one left off, so a book that
    /// outgrows one stroke's budget is still walked whole across strokes.
    pub(crate) lapse_cursor: Vec<u8>,
    /// This hypha's cover, admitted in write_field; own_cover_charts reads it.
    pub(crate) own_cover: crate::network::nerve::Cover,
    /// Listings keyed by chart, so rank_over's moved set is a fiber.
    pub(crate) chart_listings: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Ruled addresses beside the ruled cover trie.
    pub(crate) ruled_addrs: crate::network::nerve::Keyed<()>,
    /// Ruled cover root, maintained on admit/remove.
    pub(crate) ruled_root: [u8; 32],
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
    /// Anchor evidence/mismatch membership keyed by address+footprint.
    pub(crate) evidence_at: crate::network::nerve::Keyed<()>,
    pub(crate) mismatch_at: crate::network::nerve::Keyed<()>,
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
        if verified.signer() != r.vk {
            return crate::court::admission::Admitted::Refused(
                crate::court::admission::REFUSE_RECEIPT,
            );
        }
        let chain = &self.chain;
        let verdict = self
            .admissions
            .admit(origin, verified.prefix(), r, |p| chain.height_of(&p.hash));
        match &verdict {
            crate::court::admission::Admitted::Parent(p) => {
                self.chain.admit(r.clone());
                self.pending_parents.insert(p.hash, p.clone());
            }
            crate::court::admission::Admitted::Equivocation { k, .. } => {
                self.burn_equivocation(r.vk, *k);
            }
            _ => {}
        }
        verdict
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
        self.burn_equivocation(proof.vk, proof.k)
    }

    /// 47.4: TWO SIGNED RECEIPTS UNDER ONE KEY AT ONE K WITH DIFFERENT
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
    fn burn_equivocation(&mut self, vk: [u8; 32], k: u64) -> Option<[u64; 4]> {
        let bond = self.treasury.bond_of(&vk)?;
        let anchor = self.anchor();
        let burned = self.treasury.burn(vk, bond, anchor);
        if crate::economy::lanes_short(&burned).is_empty() {
            return None;
        }
        eprintln!(
            "  court equivocation {:02x}{:02x}… K {k} burned {burned:?}",
            vk[0], vk[1]
        );
        Some(burned)
    }

    /// Receipts admitted and not yet cited. A step consumes them.
    pub fn pending_parents(&self) -> usize {
        self.pending_parents.len()
    }

    /// What this mind knows of a hypha's chain head, if it has admitted one.
    pub fn known_head(&self, vk: &[u8; 32]) -> Option<(u64, [u8; 32])> {
        self.admissions.head(vk)
    }

    pub fn height(&self) -> u64 {
        self.height_override.unwrap_or_else(|| self.chain.head_height())
    }

    /// C6: test only. Time is not an input to a kernel (law 9).
    #[cfg(test)]
    pub fn set_height(&mut self, h: u64) {
        self.height_override = Some(h);
    }

    /// C6: test only. Capacity is measured, never set (rule 5).
    #[cfg(test)]
    pub fn set_lane_cap(&mut self, cap: [usize; 4]) {
        self.lane_cap = Some(cap);
    }

    pub(crate) fn mark_persist(&mut self) {
        self.persist_due = true;
    }

    pub fn take_persist(&mut self) -> bool {
        let due = self.persist_due;
        self.persist_due = false;
        due
    }

    pub fn basis_len(&self) -> usize {
        self.basis.len()
    }

    pub fn ruled_root(&self) -> [u8; 32] {
        self.ruled_root
    }

    pub fn court_federation_len(&self) -> usize {
        self.court_federation.len()
    }

    pub fn crt_imported(&self) -> bool {
        self.crt_imported
    }

    pub fn offered(&self, addr: &[u8]) -> bool {
        self.offered.get(addr).is_some()
    }

    pub fn dialect_len(&self) -> usize {
        self.cocycle_at.len()
    }

    pub fn dialect_chain_len(&self) -> usize {
        self.dialect_chain.len()
    }

    pub fn heard_len(&self) -> usize {
        self.heard.len()
    }

    pub fn field_hot_len(&self) -> usize {
        self.field.len()
    }

    pub fn field_cold_len(&self) -> usize {
        self.field_cold.len()
    }

    pub fn settlements_hot_len(&self) -> usize {
        self.settlements.len()
    }

    pub fn settlements_cold_len(&self) -> usize {
        self.settlements_cold.len()
    }

}
