// THE ACT TABLE — every act tag on the wire, every reply code, and the
// shape each act carries.
//
//! An act is one byte. The types below name what follows it, so a reader of
//! any codec file sees the shape of the act it spells without following it
//! into the encoder. Nothing here decides anything; it is the table.

use super::*;

/// Act tags — frame `body[0]`. ACT_PERCEPT/ACT_GOSSIP keep their historical
/// values so existing kernels speak unchanged.
pub const ACT_PERCEPT: u8 = 0x00;
pub const ACT_GOSSIP: u8 = 0x01;
pub const ACT_DEMAND: u8 = 0x02;
pub const ACT_QUOTE: u8 = 0x03;
pub const ACT_TEACH: u8 = 0x04;
pub const ACT_SETTLE: u8 = 0x05;
/// Step 5 court acts: the docket pull ("what awaits finality?") and the signed
/// verdict. The court speaks the same wire as everyone — its authority is the
/// replay, not the socket.
pub const ACT_DOCKET: u8 = 0x06;
pub const ACT_VERDICT: u8 = 0x07;
/// Step 7c: the court pulls the selection fiber's accumulated birth evidence.
pub const ACT_EVIDENCE: u8 = 0x08;
/// PoUW dispatch: a client pulls a SIGNED WORK ENVELOPE — the hypha's
/// authoritative registry chooses; clients never arbitrate raw corpus.
pub const ACT_TASK: u8 = 0x0B;
/// PoUW settlement: a client submits predicted outputs for a dispatched
/// task_id; the hypha verifies against its held answers — first verified
/// solve takes the bounty, replays collapse to the verification floor.
pub const ACT_SOLVE: u8 = 0x0C;
/// REFINEMENT BID (compression IS learning): a client claims basis word W
/// is derivable — the rest of the vocabulary already speaks it, a retirement
/// by derivation (3.8, 27; holonomy preserved because the replacement IS a
/// decomposition of W's own substance). The hypha verifies by deterministic
/// replay; on success the word retires, the vocabulary compresses, and the
/// bidder earns the compression yield. `[ACT_REFINE][voice 32][word address]`.
/// Receipt: solve-receipt shape — `[REFINE_ACCEPTED|REFINE_NOTHING][μ⃗ 32]`.
pub const ACT_REFINE: u8 = 0x09;
pub const REFINE_ACCEPTED: u8 = 1;
pub const REFINE_NOTHING: u8 = 0;

/// ADOPTION (genome synchronization): the court propagates an upheld word
/// to every server with ITS OWN VOICE as provenance — the ratifier is the
/// discoverer of record until origin voices ride the wire (freeze v2).
/// `[ACT_ADOPT][voice 32][element bytes]`. The reply acknowledges queueing
/// only; verification and installation run on the server's deliberation
/// worker (never on the request path).
/// Word-tape tag for a translation (gauge between two sections). No new
/// act: translations and formed thoughts travel as words. Same value as
/// `element::TRANSLATION_TAG`.
pub const TRANSLATION_TAG: u8 = 0x13;
/// Word-tape tag for an optimisation (a Rewrite). Same value as `element::REWRITE_TAG`.
pub const REWRITE_TAG: u8 = 0x14;
/// Genome-file section holding persisted thought bubbles.
/// Snapshot section holding Čech complexes (addresses and rotors only).
pub const COC1: &[u8; 4] = b"COC1";

pub const ACT_ADOPT: u8 = 0x0E;
/// One family's cohering sentences. `[ACT_SPEECH][voice 32][task address][tu u64][n u32][len u32|element]…`.
pub const ACT_SPEECH: u8 = 0x16;

/// Per-sentence: 0 Stood, 1 Dialect, 2 Refused.
pub const SPEECH_STOOD: u8 = 0;
pub const SPEECH_DIALECT: u8 = 1;
pub const SPEECH_REFUSED: u8 = 2;

/// GENESIS AUDIT (federation seeding): the court pages through a server's
/// held basis, blind-replays each word, and seals the verified core into
/// its federation — then propagates adoptions across the nerve so the
/// server plane converges without waiting for incidental consensus.
/// Request: `[ACT_GENESIS][voice 32][cursor u32]`. Reply shares the
/// syllabus page shape (`encode_syllabus_reply`); an empty page ends the
/// walk.
pub const ACT_GENESIS: u8 = 0x0F;

/// THE SHEAF SYNC (Q3): a holder asks for the other's section digests over
/// the atlas; a discordant stratum is then fetched by chart, with the
/// asker's have-list, so only the lacking words travel (sections and
/// connection riding with each). Cost scales with divergence.
/// Digests request: `[ACT_DIGESTS][voice 32]`. Reply: `[n u32]` then per
/// stratum `[len u8][inv u64·len][count u32][root 32]`.
pub const ACT_DIGESTS: u8 = 0x10;
/// Fetch request: `[ACT_FETCH][voice 32][len u8][inv u64·len][n u32][addr 32·n]`.
/// Reply: the syllabus page shape.
pub const ACT_FETCH: u8 = 0x11;
/// Hypha-to-hypha court organ: cover, offer, installed. Clients never send these.
pub const ACT_COVER: u8 = 0x20;
pub const ACT_OFFER: u8 = 0x21;
pub const ACT_INSTALLED: u8 = 0x22;

/// THE PROBE (interrogative grammar): a tentative sentence spoken against an
/// open listing — citations (word address, transport g, gauge coordinate)
/// the hypha replays linearly. Yield settles on graded descent of the
/// residual without germination; closure clears the listing.
/// Request: `[ACT_PROBE][voice 32][target 32][n u32][(addr 32)(g 17)(asg u32)]·n`.
/// Reply: `[code][paid μ⃗ 4×u64][remaining μ⃗ 4×u64]`.
pub const ACT_PROBE: u8 = 0x12;
pub const PROBE_NOTHING: u8 = 0;
pub const PROBE_DESCENT: u8 = 1;
pub const PROBE_CLOSED: u8 = 2;
/// The listing is no longer on the book in the state it was quoted:
/// another voice settled or descended it first.
pub const PROBE_SETTLED: u8 = 3;

/// A PROVEN ROOT — a family a voice solved and a registry verified —
/// gossiped across the nerve: `[ACT_ROOT][voice 32][task 32][solver 32]`.
/// Every hypha holding the task re-dispatches it first; a voice that
/// reproduces the solve from the taught language proves it shares the root.
pub const ACT_ROOT: u8 = 0x13;

/// `[ACT_ROOT][voice 32][task 32][solver 32][ticks u64]` — the root's stamp
/// on the speaker's clock. The reply is the receiver's record for that
/// task: `[solver 32 (zeros: none)][ticks u64]`, its own clock, so the
/// speaker knows the root was delivered later than it was spoken.
/// THE PEERS an entity routes over: `[ACT_PEERS][voice 32]` asks a hypha for
/// the addresses of the nerve it stands in (itself and its configured
/// peers); the reply is `[n u16] ([len u16][addr bytes])…`. The entity
/// builds the same XOR router the hyphas use, so every act it speaks goes
/// to the key's home.
pub const ACT_PEERS: u8 = 0x15;

/// A DEMAND ROUTED ON THE NERVE to the listing key's home hypha — so the
/// second voice on a listing can be a stranger's, not a local twin's:
/// `[ACT_DEMAND_ROUTED][origin hypha 32][the demand body as spoken]`.
/// Heard once as a demand at the home; never routed again.
pub const ACT_DEMAND_ROUTED: u8 = 0x14;

/// A CITATION ON THE WIRE. The versor and the body ARE the operator: the
/// hearer verifies them by replay and never fetches a copy of the word to
/// verify against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiteWire {
    /// The word cited, by address.
    pub addr: crate::transducer::address::Address,
    /// Its STANDING (`Standing::as_u8`): Ruled, Stood or Held. A
    /// coordinate, so a hypha can weight a held idea below a ruled word —
    /// never a filter that drops one.
    pub standing: u8,
    /// The transport carrying the word's chain into the utterance's frame.
    pub transport: crate::transducer::element::Versor,
    /// The gauge coordinate. `u32::MAX` is the SECTION marker: this cite is
    /// a puncture, not an assignment index.
    pub gauge: u32,
    /// The word's cells as the speaker says them, under the body's own
    /// transitions.
    pub body: Vec<Cell>,
}

/// A fetch names every discordant chart with the asker's have-list in each;
/// the request itself is filled within the frame budget.
/// `[ACT_FETCH][voice 32][k u32]{[len u8][inv u64·len][n u32][addr]·n}·k`.
pub struct ChartOf<A> {
    /// The chart, by invariant.
    pub inv: crate::transducer::stalk::Invariant,
    /// What the ASKER already holds on it, so the server sends the
    /// difference and never the stratum.
    pub have: Vec<A>,
}

/// A chart as the receiver reads it, with addresses decoded.
pub type Chart = ChartOf<Address>;

/// THE SYLLABUS (distributed cognition, operator-ruled): court-ratified
/// vocabulary PUSHED to every enrolled voice on its next contact — teaching
/// as an act of the network, not a passive download. The client asks with
/// its voice; the hypha serves the ratified stream from that voice's cursor
/// (each ratification reaches each voice exactly once, in ruling order).
/// Request: `[ACT_SYLLABUS][voice 32]`. Reply: `[n u8][(len u32)(word
/// bytes)·n]` — full lossless element forms (Epigenome::learn refreshes by
/// address, so rebinds propagate too).
pub const ACT_SYLLABUS: u8 = 0x0D;

/// DECODED ACT SHAPES. One struct per act, each field named and carrying
/// what the field means. An alias over a tuple names the act and none of
/// its coordinates, which is the `#[allow]` attribute by another route
/// (F3ah).
///
/// SPEECH: what a voice says it derived, with the standing it claims for
/// each sentence.
pub struct SpeechAct {
    /// The speaking voice.
    pub voice: [u8; 32],
    /// The task the sentences answer.
    pub task: Address,
    /// The speaker's K — its own receipt-chain depth, not the hearer's.
    pub k: u64,
    /// The sentences, as full element tapes.
    pub sentences: Vec<Vec<u8>>,
    /// The standing claimed for each sentence, in the same order: Stood
    /// where the speaker holds it, Ruled where a court already has. A
    /// missing entry is Stood, never Ruled.
    pub standing: Vec<u8>,
}
/// A COVER announcement: who, where, and what it covers.
pub struct Cover {
    /// The announcing key.
    pub vk: [u8; 32],
    /// Where to reach it.
    pub bind: String,
    /// The charts it covers, each on its lane.
    pub charts: Vec<(u8, crate::transducer::stalk::Invariant)>,
}
/// AN OFFER: a home hypha asking a witness to re-derive a word (47.3).
#[derive(Clone)]
pub struct Offer {
    /// The word's address.
    pub addr: Address,
    /// THE HOME'S KEY, named by the act. The witness glues the offer as
    /// this vantage and holds its own separately, which is what makes the
    /// Čech cell have two vertices; a witness that does not know who
    /// offered can only glue the form as itself, and then it compares the
    /// form with itself and agrees every time.
    pub home: [u8; 32],
    /// The element as a full tape. The witness replays THIS on its own
    /// tapes; it never fetches the home's copy to compare bytes.
    pub elem: Vec<u8>,
    /// The voices that attested it at home.
    pub voices: Vec<[u8; 32]>,
    /// The deflation the home measured.
    pub deflation: [u64; 4],
    /// THE HOME'S CAUSAL POSITION AND ITS SIGNATURE, as 47.1 writes them.
    ///
    /// It carried `k: u64`, `sig: [u8; 64]` and `install_sig: [u8; 64]`,
    /// signed over `offer_signed_prefix` — a bespoke prefix, where 47.1
    /// says "Every signature in the tree is over exactly these bytes" and
    /// 26 says "Every signed frame signs the receipt prefix of 47.1". The
    /// three fields had no production writer: every offer left with
    /// `k: 0`, `sig: [0u8; 64]`, `install_sig: [0u8; 64]`, which is rule
    /// 8's named shape ("all-zero signatures"), and no reader anywhere ran
    /// 47.2 on them. Any host that could open a socket could post an offer
    /// naming any `home` and have the receiving court write a Čech section
    /// at that stranger's vantage.
    ///
    /// `install_sig` is gone rather than zeroed: 47.3's second signature
    /// belongs to the install, and `Installed` already carries `sig_home`
    /// and `sig_peer` for it.
    pub section: ReceiptSection,
}

/// WHAT A WITNESS ANSWERS AN OFFER WITH (47.3, 13.9). Three arms, and the
/// home matches on which one it is — never a flag, never an emptiness.
///
/// It carried `glued: bool` first and the home branched on the bit. R1
/// replaced that with a struct of two byte vectors, `reached` and
/// `refused`, and the home then told the arms apart by
/// `!w.refused.is_empty()` and by whether `reached` decoded — checklist
/// (b), "any `len()`, `count()`, `is_empty()` that decides a branch rather
/// than an emptiness of a fiber". Neither vector is a fiber; both were
/// flags in a longer form, and the doc above them claimed the arms were
/// "read off the content and never off a flag" while the content that
/// named the arm was two lengths.
///
/// Every arm carries the witness's own verifying key: the home reads the
/// second vantage off that and never off the address it happened to dial.
/// An address is where a hypha answers; a vantage is a key.
pub enum Witnessed {
    /// The witness replayed the offer and holds a form of its own on the
    /// class: the second vantage, as a tape. Never the offered form handed
    /// back, which is the byte exactness of 1.7.
    Reached { vk: [u8; 32], form: Vec<u8> },
    /// The replay slashed the offer, naming the 34.1 constant it refused
    /// with, so the home logs and acts on the reason rather than on the
    /// fact that there was one.
    Refused { vk: [u8; 32], why: Vec<u8> },
    /// The offer survived the replay and this court has re-derived nothing
    /// of its own on the class: unmeasured, which is neither agreement nor
    /// refusal, and the object keeps waiting.
    Unmeasured { vk: [u8; 32] },
}

impl Witnessed {
    /// The key that answered, on every arm.
    pub fn vk(&self) -> [u8; 32] {
        match self {
            Witnessed::Reached { vk, .. }
            | Witnessed::Refused { vk, .. }
            | Witnessed::Unmeasured { vk } => *vk,
        }
    }
}

/// WHAT A RULING DID AT A RECEIVER (42, 47.2). Four arms, and the ruling
/// court must be able to tell them apart.
///
/// It was one byte, `vec![applied as u8]`, and every distinguishable
/// outcome answered `0`: a node on another mesh, a signature that did not
/// verify, a prefix naming another address, a poisoned lock, a chain GAP,
/// a stale frame, an equivocation, and a ruling that simply found nothing
/// to move. A gap means "send me the back-fill"; an equivocation is 47.4's
/// burn proof, the mesh's only bond-slashing evidence; `REFUSE_ORIGIN`
/// means "you are on another mesh". All three read identically to "your
/// ruling did not land", so the sender's only recovery was blind retry and
/// equivocation proofs died silently at the door (MU-STANDARDS §2.3: "every
/// caller that needs to act recomputes what the bit threw away").
pub enum Ruled {
    /// The ruling landed and this court's book moved.
    Applied,
    /// Admitted under 47.2, and the arm found nothing to move: the word is
    /// not held here, or it was ruled already.
    Inert,
    /// 47.2 refused the frame, naming the 34.1 constant it refused with.
    Refused(Vec<u8>),
    /// 47.2 step 3: this receiver's knowledge of the signer's chain has a
    /// gap, and it is asking for the receipts between `from` and `to`. Not
    /// a refusal — "the back-fill fetches what makes it re-derive".
    Gap { from: u64, to: u64 },
}

/// AN INSTALLED word, carrying both hyphae's signatures.
pub struct Installed {
    /// The word's address.
    pub addr: Address,
    /// The element as a full tape.
    pub elem: Vec<u8>,
    /// The HOME's signature.
    pub sig_home: [u8; 64],
    /// The WITNESS's signature. Two signatures, two vantages: the pair is
    /// what makes the install a convergence and not an assertion.
    pub sig_peer: [u8; 64],
}
/// A SOLVE submission: the answers a voice claims for a registered task.
pub struct SolveAct {
    /// The solving voice.
    pub voice: [u8; 32],
    /// The task answered.
    pub task: Address,
    /// One output grid per test input, in the order they were served.
    pub outputs: Vec<Vec<Vec<u8>>>,
    /// The rule the voice extracted, when it speaks one. `None` is an
    /// answer with no rule behind it — the registry prices it differently.
    pub rule: Option<Vec<u8>>,
}

/// A DEMAND with the citations that came with it.
pub struct DemandAct {
    /// The demanding voice.
    pub voice: [u8; 32],
    /// The chain it demands a word for.
    pub chain: Vec<Cell>,
    /// What it could already cite.
    pub cites: Vec<CiteWire>,
    /// The demander's own K, so a duplicate (voice, address, K) is answered
    /// from the docket rather than re-reduced.
    pub k: u64,
}
/// One selection's birth evidence: `(address, positions, scene)`
pub type EvidenceItem = crate::transducer::anchor::AddressedEvidence;

/// The same item as a SENDER holds it, before the address is decoded: the
/// key is whatever bytes it is filed under.
pub use crate::transducer::anchor::EvidenceOf;
/// `(voice, items)`
pub type EvidenceDeposit = ([u8; 32], Vec<EvidenceItem>);
/// 47.1's RECEIPT SECTION. A truncated section is refused whole, never a
/// partial admission (rule 11).
#[derive(Clone)]
pub struct ReceiptSection {
    /// The signed prefix, as the signer wrote it.
    pub prefix: crate::court::admission::SignedPrefix,
    /// The signature over that prefix.
    pub sig: [u8; 64],
    /// The receipt the signature binds to.
    pub receipt: crate::chronometry::KStepReceipt,
}

/// A COVER DIGEST: what a hypha publishes about itself.
pub struct CoverDigest {
    /// The charts it covers, each on its lane.
    pub charts: Vec<(u8, crate::transducer::stalk::Invariant)>,
    /// Its room, per lane.
    pub room: [u64; 4],
    /// The four stratum roots of its cover.
    pub cover_roots: [[u8; 32]; 4],
    /// The root of its RULED language — a different tree from the four
    /// above, and the pair is what a peer compares to see it moved.
    pub ruled_root: [u8; 32],
    /// Its verifying key.
    pub vk: [u8; 32],
    /// Where to reach it.
    pub bind: String,
    /// The signature over the prefix `cover_signed_prefix` builds.
    pub sig: [u8; 64],
}


/// solve receipt: `[code u8][μ⃗ 32]` — code 2 = BOUNTY (first verified solve),
/// 1 = verification floor (replay), 0 = nothing (wrong or unregistered).
pub const SOLVE_BOUNTY: u8 = 2;
pub const SOLVE_FLOOR: u8 = 1;
pub const SOLVE_NOTHING: u8 = 0;

/// One centered coboundary cell `(row, col, before, after)`.
pub type Cell = (i64, i64, u8, u8);

/// One quoted work item: the residual's address, its priced μ⃗, the chain,
/// and the open (non-flat) charts of its coherence field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItem {
    /// The residual's address — what a demand for it is filed under.
    pub addr: Address,
    /// Its priced μ⃗, per lane.
    pub mu: [u64; 4],
    /// The chain itself: real, attemptable work, not a pointer to it.
    pub chain: Vec<Cell>,
    /// The OPEN (non-flat) charts of its coherence field.
    pub open: Vec<crate::transducer::stalk::Invariant>,
}

/// `[ACT_QUOTE][height u64][surface 4×u64][K u64][n u8][item: addr 32 | μ⃗ 4×u64 | chain]…`
/// — height is the book generation; the u64 K is receipt-chain head
/// (verified consensus depth). TUniversal is not this field.
/// The offset of the served count in a quote reply (act, height, surface, K).
pub const QUOTE_COUNT_AT: usize = 1 + 8 + 32 + 8;

/// docket reply: `[n u32] ( [addr 32][elen u32][element][nv u32][voices 32·nv][μ⃗ 32][Δμ⃗ 32] )…`
/// — Δμ⃗ is the deflation the word's install measured on a listed demand
/// (zero when the germ settled nothing on the book): the court's proof of
/// convergence rides with the word.
/// The last field is the hypha's k-step at which the word was installed:
/// the coordinate a court's "since" is read against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Docket {
    /// The word's address.
    pub addr: crate::transducer::address::Address,
    /// The element itself, as a full tape — the court replays it blind.
    pub elem: Vec<u8>,
    /// The voices attesting it.
    pub voices: Vec<[u8; 32]>,
    /// The μ⃗ ESCROWED against it: a claim on issuance.
    pub escrowed: [u64; 4],
    /// The DEFLATION its install measured on a listed demand — zero when
    /// the germ settled nothing. The proof of convergence rides with the
    /// word, and this is not the escrow.
    pub deflation: [u64; 4],
    /// The hypha's k-step at install: the coordinate a court's "since" is
    /// read against.
    pub k: u64,
}

/// A SETTLEMENT on the docket: the listing, its chain, the voice, the
/// utterance that settled it, and the magnitudes before and after as the
/// hypha measured them. The court replays the utterance and measures for
/// itself: two measurements of one deflation are the proof of convergence.
/// The last two fields are the hypha's k-step (order inside one kernel)
/// and the TUniversal stamp (ledger, cross-node).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settlement {
    /// The listing settled.
    pub listing: crate::transducer::address::Address,
    /// Its chain.
    pub chain: Vec<Cell>,
    /// The voice that settled it.
    pub voice: [u8; 32],
    /// The utterance: the citations the voice spoke. The court replays
    /// these and measures for itself — two measurements of one deflation
    /// are the proof of convergence.
    pub cites: Vec<CiteWire>,
    /// The magnitude BEFORE, as the hypha measured it.
    pub before: [u64; 4],
    /// The magnitude AFTER. `before − after` is the deflation claimed.
    pub after: [u64; 4],
    /// The hypha's k-step: order inside one kernel.
    pub k: u64,
    /// The TUniversal stamp: the ledger coordinate, across nodes.
    pub t_universal: u64,
}

/// Verdict rulings — the court's closed vocabulary.
pub const RULING_UPHELD: u8 = 1;
pub const RULING_DEMOTED: u8 = 2;
pub const RULING_SLASHED: u8 = 0;
/// Step 7c: rebind a word's anchor fiber — the verdict carries π* (a Sel term)
/// derived from family-wide evidence; the kernel upgrades γ = (⊤, Δ) → (π*, Δ)
/// in place (canonical substance identity untouched).
pub const RULING_REBIND: u8 = 3;
/// The bundle connects: the ruled word is a placement of a federated base
/// geometry — the verdict carries the base address and the group element g
/// that transports the word into the base frame. A hypha holding both
/// attaches the word as a section of the base and retires its card.
pub const RULING_ATTACHED: u8 = 4;
/// Two independent voices verified the same task: a proven root.
pub const RULING_ROOT: u8 = 5;

/// A DECODED VERDICT.
pub struct Verdict {
    /// The COURT's voice — the ruling's author, not the word's.
    pub voice: [u8; 32],
    /// The word ruled on.
    pub addr: Address,
    /// The ruling itself, from the closed vocabulary above.
    pub ruling: u8,
    /// The head of the court's own receipt chain, binding the ruling to a
    /// tamper-evident history.
    pub receipt: [u8; 32],
    /// π* — present only on RULING_REBIND, the selection the court rebinds
    /// the word to.
    pub pi: Option<crate::transducer::anchor::Sel>,
    /// The base and the transport — present only on RULING_ATTACHED.
    pub attach: Option<(Address, crate::transducer::element::Versor)>,
    /// THE COURT'S CAUSAL POSITION, whole: 47.1's signed prefix with the
    /// ruled address as its payload, the signature over exactly those
    /// bytes, and the receipt the prefix names.
    ///
    /// The receipt travels because 47.2 is four steps and the first cut
    /// carried only the prefix and the signature, which decide steps 1 and
    /// 2. Steps 3 and 4 — K against the signer's known head, and a height
    /// that re-derives from the parents the receipt names — need the
    /// receipt, so without it a verdict was admitted at any K and any
    /// height, and a frame replayed from a spent position moved a court
    /// head as readily as a fresh one.
    pub section: ReceiptSection,
}
