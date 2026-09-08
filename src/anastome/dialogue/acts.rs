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
/// is derivable — the rest of the vocabulary already speaks it (S' ≺_DM S,
/// holonomy preserved because the replacement IS a decomposition of W's own
/// substance). The hypha verifies by deterministic replay; on success the
/// word retires, the vocabulary compresses, and the bidder earns the
/// compression yield. `[ACT_REFINE][voice 32][word address]`.
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

/// A citation on the wire: (word address, standing, transport, gauge, body).
/// Standing is one byte beside the address (`Standing::as_u8`): Ruled, Stood,
/// or Held — a coordinate, so the hypha can weight a held idea below a
/// ruled word. The versor and the body are the operator; the hearer
/// verifies them and never fetches a copy of the word to verify against.
pub type CiteWire = (crate::transducer::a21::Address, u8, crate::transducer::element::Versor, u32, Vec<Cell>);

/// A fetch names every discordant chart with the asker's have-list in each;
/// the request itself is filled within the frame budget.
/// `[ACT_FETCH][voice 32][k u32]{[len u8][inv u64·len][n u32][addr]·n}·k`.
pub type Chart = (crate::transducer::stalk::Invariant, Vec<Address>);

/// THE SYLLABUS (distributed cognition, operator-ruled): court-ratified
/// vocabulary PUSHED to every enrolled voice on its next contact — teaching
/// as an act of the network, not a passive download. The client asks with
/// its voice; the hypha serves the ratified stream from that voice's cursor
/// (each ratification reaches each voice exactly once, in ruling order).
/// Request: `[ACT_SYLLABUS][voice 32]`. Reply: `[n u8][(len u32)(word
/// bytes)·n]` — full lossless element forms (Epigenome::learn refreshes by
/// address, so rebinds propagate too).
pub const ACT_SYLLABUS: u8 = 0x0D;

/// DECODED ACT SHAPES. Each alias names one act's field list -- the tuple
/// is the act, written out (A21), and the alias gives it a name rather than
/// hiding it behind a struct nobody constructs. Introduced because these
/// signatures were unreadable in place and clippy was right to say so.
///
/// `(voice, task, K, sentences)`
pub type Speech = ([u8; 32], Address, u64, Vec<Vec<u8>>);
/// `(voice, task, K, sentences, standing)`
pub type SpeechStanding = ([u8; 32], Address, u64, Vec<Vec<u8>>, Vec<u8>);
/// `(vk, bind, charts)`
pub type Cover = ([u8; 32], String, Vec<(u8, crate::transducer::stalk::Invariant)>);
/// `(address, element bytes, voices, deflation, K, sig, install sig)`
pub type Offer = (Address, Vec<u8>, Vec<[u8; 32]>, [u64; 4], u64, [u8; 64], [u8; 64]);
/// `(address, element bytes, home sig, witness sig)`
pub type Installed = (Address, Vec<u8>, [u8; 64], [u8; 64]);
/// `(voice, task, grids, rule)`
pub type Solve = ([u8; 32], Address, Vec<Vec<Vec<u8>>>, Option<Vec<u8>>);
/// `(voice, chain, cites, height)`
pub type DemandAt = ([u8; 32], Vec<Cell>, Vec<CiteWire>, u64);
/// One selection's birth evidence: `(address, positions, scene)`
pub type EvidenceItem = EvidenceOf<Address>;

/// The same item as a SENDER holds it, before the address is decoded: the
/// key is whatever bytes it is filed under.
pub type EvidenceOf<A> = (A, Vec<(i64, i64)>, Vec<(i64, i64, u8)>);
/// `(voice, items)`
pub type EvidenceDeposit = ([u8; 32], Vec<EvidenceItem>);
/// 47.1's receipt section: `(prefix, signature over it, the receipt)`
pub type ReceiptSection = (
    crate::court::admission::SignedPrefix,
    [u8; 64],
    crate::chronometry::KStepReceipt,
);
/// The cover digest's fields: `(charts, room, cover roots, ruled root, vk,
/// bind, sig)`
pub type CoverDigest = (
    Vec<(u8, crate::transducer::stalk::Invariant)>,
    [u64; 4],
    [[u8; 32]; 4],
    [u8; 32],
    [u8; 32],
    String,
    [u8; 64],
);


/// solve receipt: `[code u8][μ⃗ 32]` — code 2 = BOUNTY (first verified solve),
/// 1 = verification floor (replay), 0 = nothing (wrong or unregistered).
pub const SOLVE_BOUNTY: u8 = 2;
pub const SOLVE_FLOOR: u8 = 1;
pub const SOLVE_NOTHING: u8 = 0;

/// One centered coboundary cell `(row, col, before, after)`.
pub type Cell = (i64, i64, u8, u8);

/// One quoted work item: the residual's address, its priced μ⃗, the chain,
/// and the open (non-flat) charts of its coherence field.
pub type WorkItem = (Address, [u64; 4], Vec<Cell>, Vec<crate::transducer::stalk::Invariant>);

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
pub type Docket = (crate::transducer::a21::Address, Vec<u8>, Vec<[u8; 32]>, [u64; 4], [u64; 4], u64);

/// A SETTLEMENT on the docket: the listing, its chain, the voice, the
/// utterance that settled it, and the magnitudes before and after as the
/// hypha measured them. The court replays the utterance and measures for
/// itself: two measurements of one deflation are the proof of convergence.
/// The last two fields are the hypha's k-step (order inside one kernel)
/// and the TUniversal stamp (ledger, cross-node).
pub type Settlement = (crate::transducer::a21::Address, Vec<Cell>, [u8; 32], Vec<CiteWire>, [u64; 4], [u64; 4], u64, u64);

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

/// A decoded verdict: (court voice, word address, ruling, receipt, π* for a
/// rebind, (base, g) for an attach).
pub type Verdict = (
    [u8; 32],
    Address,
    u8,
    [u8; 32],
    Option<crate::transducer::anchor::Sel>,
    Option<(Address, crate::transducer::element::Versor)>,
);
