// THE ACT TABLE — every act tag on the wire, every reply code, and the
// shape each act carries.
//
//! An act is one byte. The types below name what follows it, so a reader of
//! any codec file sees the shape of the act it spells without following it
//! into the encoder. Nothing here decides anything; it is the table.

use super::*;
use crate::physics::actions::Action;

/// Act tags — frame `body[0]`. 78.94: EVERY TAG IS AN ACTION'S COORDINATE,
/// spelled here and held by the one table (`physics::actions::Action`), which
/// says what kind of action each is, whether a court seals it, and the laws
/// it answers to; these names are the wire's spelling of it (law 12).
/// ACT_PERCEPT, ACT_GOSSIP and ACT_DEMAND are RETIRED (26, law 8, ruled
/// 2026-09-10): a court hears sentences and nothing else, and a frame
/// carrying one of these is refused at the door with `REFUSE_NOT_A_SENTENCE`.
/// The values stand so the door can name what it refused; nothing encodes
/// them.
pub const ACT_PERCEPT: u8 = Action::Percept.byte();
pub const ACT_GOSSIP: u8 = Action::Gossip.byte();
pub const ACT_DEMAND: u8 = Action::Demand.byte();
pub const ACT_QUOTE: u8 = Action::Quote.byte();
/// 78.94: retired — encoded, never handled; the table refuses it at the door.
pub const ACT_TEACH: u8 = Action::Teach.byte();
/// 78.94: retired — encoded, never handled; the table refuses it at the door.
pub const ACT_SETTLE: u8 = Action::Settle.byte();
/// Step 5 court act: the docket pull ("what awaits finality?"). The court
/// speaks the same wire as everyone — its authority is the replay, not the
/// socket. 0x07 is unassigned: a ruling reaches a node as a receipt through
/// the exchange and its back-fill (45.2, 47.2), never as a message.
/// 78.94: retired — encoded, never handled; the table refuses it at the door.
pub const ACT_DOCKET: u8 = Action::Docket.byte();
/// Step 7c: the court pulls the selection fibre's accumulated birth evidence.
pub const ACT_EVIDENCE: u8 = Action::Evidence.byte();
/// POUW dispatch: a client pulls a SIGNED WORK ENVELOPE — the hypha's
/// authoritative registry chooses; clients never arbitrate raw corpus.
pub const ACT_TASK: u8 = Action::Task.byte();
/// POUW settlement: a client submits predicted outputs for a dispatched
/// task_address; the hypha verifies against its held answers — first verified
/// solve takes the bounty, replays collapse to the verification floor.
pub const ACT_SOLVE: u8 = Action::Solve.byte();

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

pub const ACT_ADOPT: u8 = Action::Adopt.byte();
/// One family's cohering sentences. `[ACT_SPEECH][voice 32][task address][tu u64][n u32][len u32|element]…`.
pub const ACT_SPEECH: u8 = Action::Speech.byte();

/// Per-sentence: 0 Stood, 1 Dialect, 2 Refused, 3 Ratified. Since 50.9
/// (ruled 2026-09-10) these survive only as the adjudication's kind byte
/// on the wire and the mind's reading of it; the reply itself is an
/// `Adjudication` per sentence.
pub const SPEECH_STOOD: u8 = 0;
pub const SPEECH_DIALECT: u8 = 1;
pub const SPEECH_REFUSED: u8 = 2;
pub const SPEECH_RATIFIED: u8 = 3;

/// THE JUDICIAL REPLY (50.9, 26): one per sentence sent, in the order
/// sent, never an opaque byte. `Stood` is the docket's receipt; `Ratified`
/// the language's — a use of a ruled word here, the second organ's
/// co-signature on the syllabus page; `Refused` names the kind (34.1) and
/// the coordinate as content. The receipt is the court's replay receipt,
/// its chain hash (47.1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Adjudication {
    /// 78.92, 78.93 B0 — `at` is WHERE THE WORD STANDS on the court's
    /// ledger: the stamp of the act that admitted it, the coordinate any
    /// party asks "why" by (`ACT_WHY`). `None` where no act has admitted it
    /// yet — a reading of absence, never a stamp invented for it.
    Stood { receipt: [u8; 32], at: Option<crate::chronometry::Stamp> },
    Ratified { receipt: [u8; 32], at: Option<crate::chronometry::Stamp> },
    Refused { kind: crate::transducer::sentence::RefusalKind, at: crate::transducer::sentence::At, receipt: [u8; 32] },
    /// 21 step 3, 50.2 step 2 — THIS SENTENCE WAITS ON A FORM THIS COURT
    /// HAS NOT RULED. Not a refusal and not a verdict: the voice spoke
    /// one syllabus step ahead of this court's cover, so nothing is
    /// sealed and the sentence is docketed here under `form`. It carries
    /// no receipt because no act was admitted — a receipt would be a
    /// chain position for something that did not happen (rule 11) — and
    /// it carries the CITED address instead, which is what the voice
    /// needs: it holds that form, and utters it to this court before it
    /// utters the citer again (chapter 6's depth order).
    ///
    /// It is per sentence. The rest of the frame is adjudicated as
    /// normal, so a sentence that stood beside it keeps its verdict.
    Awaiting { form: crate::transducer::address::Address },
}

impl Adjudication {
    /// The kind byte a mind reads: `SPEECH_STOOD`, `SPEECH_RATIFIED`,
    /// `SPEECH_REFUSED`, or `SPEECH_DIALECT` for a sentence that waits —
    /// "stood there, unanswered here, spoken again later".
    pub fn code(&self) -> u8 {
        match self {
            Adjudication::Stood { .. } => SPEECH_STOOD,
            Adjudication::Ratified { .. } => SPEECH_RATIFIED,
            Adjudication::Refused { .. } => SPEECH_REFUSED,
            Adjudication::Awaiting { .. } => SPEECH_DIALECT,
        }
    }

    /// The court's chain hash after the act. `None` where no act was
    /// admitted: a wait is not a position on any chain.
    pub fn receipt(&self) -> Option<[u8; 32]> {
        match self {
            Adjudication::Stood { receipt, .. } | Adjudication::Ratified { receipt, .. } | Adjudication::Refused { receipt, .. } => Some(*receipt),
            Adjudication::Awaiting { .. } => None,
        }
    }

    /// Where the word stands on the court's ledger, where an act admitted it.
    pub fn at(&self) -> Option<crate::chronometry::Stamp> {
        match self {
            Adjudication::Stood { at, .. } | Adjudication::Ratified { at, .. } => *at,
            Adjudication::Refused { .. } | Adjudication::Awaiting { .. } => None,
        }
    }
}

// 0x0F was ACT_GENESIS, a basis page by cursor. RETIRED 2026-09-29
// (ruled): nothing sent it; the syllabus and the fetch carry every
// ruled word to a new node, and a cursor into the basis's key order moved
// under it at every ruling. An unknown tag is not answered.

/// THE SHEAF SYNC (Q3): a holder asks for the other's section digests over
/// the atlas; a discordant stratum is then fetched by chart, with the
/// asker's have-list, so only the lacking words travel (sections and
/// connection riding with each). Cost scales with divergence.
/// Digests request: `[ACT_DIGESTS][voice 32]`. Reply: `[n u32]` then per
/// stratum `[len u8][inv u64·len][count u32][root 32]`.
pub const ACT_DIGESTS: u8 = Action::Digests.byte();
/// Fetch request: `[ACT_FETCH][voice 32][len u8][inv u64·len][n u32][addr 32·n]`.
/// Reply: the syllabus page shape.
pub const ACT_FETCH: u8 = Action::Fetch.byte();
/// Hypha-to-hypha court organ: cover, offer, installed. Clients never send these.
pub const ACT_COVER: u8 = Action::Cover.byte();
pub const ACT_OFFER: u8 = Action::Offer.byte();
pub const ACT_INSTALLED: u8 = Action::Installed.byte();
/// 78.152b — A KEEPER'S RING PUSHED, court to court:
/// `[ACT_TICK][keeper vk 32][ring]`, the ring in the cover digest's own codec
/// (`chronometry::tuniversal::reading::put_ring`). Sent on the nerve when the
/// keeper signs a reading in its court loop; heard at the door off the kernel.
/// Reply: one byte, `TICK_HEARD` or `TICK_REFUSED`. WIRE-CHANGE.
pub const ACT_TICK: u8 = Action::Tick.byte();
/// The ring was admitted, or held already.
pub const TICK_HEARD: u8 = 1;
/// The frame did not read whole, its keeper is no known peer court, or a
/// reading of the ring was refused (`tuniversal::KeeperRefusal`).
pub const TICK_REFUSED: u8 = 0;
/// AN EQUIVOCATION PROOF (47.4, `MU-XSEPTUM.md` §5.5): any node presents two
/// receipt sections it holds under one key at one K with two chain
/// hashes, and the court burns the signer's bond. The one xSEPTUM act
/// landed; 0x30–0x34 stay reserved.
/// `[ACT_EQUIVOCATION][version u8][len32 section_a][len32 section_b]`.
/// Reply: `Ruled` — `Applied` burned, `Inert` nothing left to burn,
/// `Refused` naming the 34.1 constant.
pub const ACT_EQUIVOCATION: u8 = Action::Equivocation.byte();
/// THE K-RANGE ACT (47.2 step 3, 45.8; S13.10):
/// `[ACT_RECEIPTS][voice 32][signer 32][from u64][to u64]` asks for the
/// signer's receipts `from..=to` — its own from its hot chain or `ARC1`,
/// or the ones a holder admitted from it — as many as fit the frame
/// budget, from `from` upward. Reply: `[n u32][receipt…]`. A gap's
/// back-fill and an archived prefix are both fetched by it.
pub const ACT_RECEIPTS: u8 = Action::Receipts.byte();
/// THE FOLD PROPOSAL (47.3, 45.8; S13.10): a home asks a bonded peer to
/// witness its fold — `[ACT_FOLD][k0 u64][root k u64 ‖ hash 32 ‖ height
/// u64][n u32][(who 32 ‖ μ⃗ 4×u64)…][47.1 receipt section, payload the root
/// hash]`. The witness fetches the prefix by `ACT_RECEIPTS` and `ACT_ROWS`,
/// re-derives it, and answers refused, unmeasured, or co-signed with its
/// own signed position over the root.
pub const ACT_FOLD: u8 = Action::Fold.byte();
/// THE ROWS A FOLD RE-EXPRESSES (§6 across a fold): `[ACT_ROWS][voice 32]
/// [signer 32][from u64][to u64][cursor u32]` asks for the signer's own
/// ledger rows anchored in `from..=to`, from `cursor`; reply `[total u32]
/// [m u32][row…]`, a page within the frame budget.
pub const ACT_ROWS: u8 = Action::Rows.byte();
/// THE OBSERVER (§8.1): a voice pushes its readings to its hypha, and any
/// process queries a hypha for the readings it holds by key prefix. The
/// layout and its codec are [`crate::observe`]'s, the one writer of it.
pub const ACT_OBSERVE: u8 = Action::Observe.byte();
/// THE LAW REGISTER (78.62 steps 2 and 3, 78.64 H4): any kernel asks a hypha
/// for the universal laws it holds, each by its coordinate and statement
/// (`[ACT_LAWS][0]`), or puts a transition it composed to the court's
/// verifier (`[ACT_LAWS][1][transition]`), answered by the verdict — every
/// breach with its law. The layout and its codec are `physics::laws::codec`'s,
/// the one writer of it. It admits nothing and writes no receipt: a verdict
/// is a reading the kernel acts on.
pub const ACT_LAWS: u8 = Action::Laws.byte();
/// WHY AN ACT STANDS (78.92): any process asks a court about one act on its
/// ledger — a place on its own line (`[ACT_WHY][0][place]`) or a stamp on any
/// line it holds (`[ACT_WHY][1][stamp]`) — and is answered the act and every
/// word it names, each with the reasons read off the word itself. The layout
/// and its codec are `dialogue::why`'s. It admits nothing.
pub const ACT_WHY: u8 = Action::Why.byte();
/// A PARTY'S WALLET AT THIS COURT (78.93 B1): `[ACT_WALLET][who 32]`, answered
/// the party's balance, bond and holds per lane and the court's head stamp
/// the reading stands at. Every party holds one — entities, kernels, humans.
/// The layout and its codec are `dialogue::wallet`'s. It moves nothing.
pub const ACT_WALLET: u8 = Action::Wallet.byte();
/// A TIER'S FOLD ADMITTED (78.119, 78.62 step 3): a kernel's settlement tier
/// sends up its net positions per identity per lane and its declared sources
/// and sinks, with the hash of the prefix head it nets, signed by the tier;
/// the court checks the signature and continuity and seals the fold's
/// coordinates on its line, answered by the receipt's stamp and chain hash.
/// The layout and its codec are `dialogue::uplink`'s.
pub const ACT_UPLINK: u8 = Action::Uplink.byte();
/// A DOMAIN FOUNDED (78.120, 78.122, 78.62's GENESIS act): a domain's key and
/// the tier keys that settle for it, signed by the domain, sealed on the
/// court's line; thereafter only a named tier's fold is admitted. The layout
/// and its codec are `dialogue::domain`'s.
pub const ACT_DOMAIN: u8 = Action::Domain.byte();
/// A FOLD CHALLENGED (78.125, 2(c)): within its window, a demand that its tier
/// publish the prefix it nets, the prefix itself put to replay, a default
/// claimed past a demand's deadline, or a correction booking an arbitrage;
/// and, sealing nothing, the fold's finality read. The layout and its codec
/// are `dialogue::challenge`'s.
pub const ACT_CHALLENGE: u8 = Action::Challenge.byte();
/// MATTER OR A WORD CROSSING DOMAINS (78.126, 2(b)): an offer by the sending
/// tier, accepted whole or refused by the receiving one, or expired; each
/// side consumed by one fold. The layout and its codec are
/// `dialogue::handoff`'s.
pub const ACT_HANDOFF: u8 = Action::Handoff.byte();
/// THE SEAT SURFACE (78.145): a game's house lists its seats, a mind takes
/// one, the house perceives for it, the mind calls and the house answers the
/// outcome — each signed, held and forwarded by the hypha, judged by nothing
/// in-game. The layout and its codec are `dialogue::seat`'s.
pub const ACT_SEAT: u8 = Action::Seat.byte();

/// THE PROBE (interrogative grammar): a tentative sentence spoken against an
/// open listing — citations (word address, transport g, gauge coordinate)
/// the hypha replays linearly. Yield settles on the graded trajectory of
/// the residual without germination; closure clears the listing.
/// Request: `[ACT_PROBE][voice 32][target 32][n u32][(addr 32)(g 17)(asg u32)]·n`.
/// Reply: the probe's READING (PR1) — `[settled μ⃗ 4×u64][closure][remaining
/// μ⃗ 4×u64]`, never an outcome code.
pub const ACT_PROBE: u8 = Action::Probe.byte();

/// PR1 — WHERE A PROBE LEFT THE LISTING: closed; a residual, with the versor
/// it stands off the listing by where a residual body reads on both sides
/// (`V = s_b s_a` between the listing's residual body and the probe's, the
/// one producer `tether_of`), `None` — unread by name — where one does not;
/// nothing covered or settled; or the listing gone from the book in the
/// state it was quoted (another voice settled or moved it first).
#[derive(Clone, Debug)]
pub enum ProbeClosure {
    Closed,
    Residual(Option<crate::transducer::cga::Mv>),
    Nothing,
    Gone,
}

impl PartialEq for ProbeClosure {
    /// Residuals are equal where the two versors write one ray.
    fn eq(&self, other: &ProbeClosure) -> bool {
        let ray = |v: &Option<crate::transducer::cga::Mv>| {
            v.as_ref().map(|m| {
                let mut b = Vec::new();
                m.put_canonical(&mut b);
                b
            })
        };
        match (self, other) {
            (ProbeClosure::Closed, ProbeClosure::Closed)
            | (ProbeClosure::Nothing, ProbeClosure::Nothing)
            | (ProbeClosure::Gone, ProbeClosure::Gone) => true,
            (ProbeClosure::Residual(a), ProbeClosure::Residual(b)) => ray(a) == ray(b),
            _ => false,
        }
    }
}

/// PR1 — A PROBE'S READING: the μ⃗ it settled, per lane; where it left the
/// listing; the μ⃗ remaining on it, per lane.
#[derive(Clone, Debug, PartialEq)]
pub struct ProbeReading {
    pub settled: [u64; 4],
    pub closure: ProbeClosure,
    pub remaining: [u64; 4],
}

impl ProbeReading {
    /// A probe that settled nothing and left nothing standing to read.
    pub fn nothing() -> ProbeReading {
        ProbeReading { settled: [0; 4], closure: ProbeClosure::Nothing, remaining: [0; 4] }
    }

    /// The listing was gone from the book.
    pub fn gone() -> ProbeReading {
        ProbeReading { settled: [0; 4], closure: ProbeClosure::Gone, remaining: [0; 4] }
    }

    /// Did the probe move the listing — close it or leave a residual?
    pub fn moved(&self) -> bool {
        matches!(self.closure, ProbeClosure::Closed | ProbeClosure::Residual(_))
    }
}

/// A PROVEN ROOT — a family a voice solved and a registry verified —
/// gossiped across the nerve: `[ACT_ROOT][voice 32][task 32][solver 32]`.
/// Every hypha holding the task re-dispatches it first; a voice that
/// reproduces the solve from the taught language proves it shares the root.
pub const ACT_ROOT: u8 = Action::Root.byte();

/// `[ACT_ROOT][voice 32][task 32][solver 32][ticks u64]` — the root's stamp
/// on the speaker's clock. The reply is the receiver's record for that
/// task: `[solver 32 (zeros: none)][ticks u64]`, its own clock, so the
/// speaker knows the root was delivered later than it was spoken.
/// THE PEERS an entity routes over: `[ACT_PEERS][voice 32]` asks a hypha for
/// the addresses of the nerve it stands in (itself and its configured
/// peers); the reply is `[n u16] ([len u16][addr bytes])…`. The entity
/// builds the same XOR router the hyphas use, so every act it speaks goes
/// to the key's home.
pub const ACT_PEERS: u8 = Action::Peers.byte();

/// RETIRED with the demand (26, law 8, ruled 2026-09-10): a demand routed on
/// the nerve to a listing's home. Refused at the door, never routed again.
pub const ACT_DEMAND_ROUTED: u8 = Action::DemandRouted.byte();

/// A CITATION ON THE WIRE. The versor and the body ARE the operator: the
/// hearer verifies them by replay and never fetches a copy of the word to
/// verify against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiteWire {
    /// The word cited, by address.
    pub addr: crate::transducer::address::Address,
    /// Its STANDING (`Standing::byte`): Ruled, Stood or Held. A
    /// coordinate, so a hypha can weight a held idea below a ruled word —
    /// never a filter that drops one.
    pub standing: u8,
    /// The transport carrying the word's chain into the utterance's frame.
    pub transport: crate::transducer::element::Versor,
    // THE GAUGE IS NOT ON THE WIRE. It was a `u32` index into the word's
    // assignments beside a body that already carries the assignment (law
    // 12), with `u32::MAX` a sentinel for "a section" — the speaker's claim,
    // which the hypha verifies anyway by asking the word (rule 11). The
    // body says the assignment; the hypha reads whether it is a section.
    /// The word's links as the speaker says them, under the body's own
    /// transitions.
    pub body: Vec<Link>,
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

/// THE SYLLABUS (distributed cognition, as ruled): court-ratified
/// vocabulary PUSHED to every enrolled voice on its next contact — teaching
/// as an act of the network, not a passive download. The client asks with
/// its voice; the hypha serves the ratified stream from that voice's cursor
/// (each ratification reaches each voice exactly once, in ruling order).
/// Request: `[ACT_SYLLABUS][voice 32]`. Reply: `[n u8][(len u32)(word
/// bytes)·n]` — full lossless element forms (Epigenome::learn refreshes by
/// address, so rebinds propagate too).
pub const ACT_SYLLABUS: u8 = Action::Syllabus.byte();

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
    /// The speaker's K — the orbits its mind acted in, not the hearer's.
    pub k: u64,
    /// The sentences, as full element tapes.
    pub sentences: Vec<Vec<u8>>,
    /// The standing claimed for each sentence, in the same order: Stood
    /// where the speaker holds it, Ruled where a court already has. A
    /// missing entry is Stood, never Ruled.
    pub standing: Vec<u8>,
    /// Per sentence, in the same order, the contraction certificate it
    /// cites (50.9): the refusal receipt this sentence is the contraction
    /// of, or none for a sentence spoken fresh.
    pub contractions: Vec<Option<crate::transducer::sentence::Contraction>>,
    /// MU-CLIENT §5.5 — per sentence, the signed HOLD frame of another mind
    /// that agreed it, or none: the court verifies it and seals that mind's
    /// attestation.
    pub agreements: Vec<Option<Vec<u8>>>,
    // 50.2 step 2 as ruled 2026-09-10: `paths` was here. The derivation
    // of every word vertex travels inside the tape at v7, so two voices
    // sending one address are told apart by the tapes themselves and the
    // court pays each one's generators from what it replayed (26).
}
/// THE FOUR STRATUM ROOTS a court signs, with the ruled root over them —
/// the whole of what a cover digest says about what that court holds (A4).
/// One carrier for the pair (law 12): the hypha's exchange, the voice's
/// session and the digest codec all read the same two coordinates, and a
/// reader that held its own shape would compare roots against a shape the
/// signer never wrote.
pub type Roots = ([[u8; 32]; 4], [u8; 32]);

/// A COVER, one page: who, where, and the charts under the roots it just
/// signed. Read by `ACT_COVER` when those roots move, never on the digest.
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
    /// Čech simplex have two vertices; a witness that does not know who
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
/// than an emptiness of a fibre". Neither vector is a fibre; both were
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
    /// The vantage that answered, on every arm.
    pub fn vantage(&self) -> [u8; 32] {
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
/// to move. A gap means "send the receiving court the back-fill"; an
/// equivocation is 47.4's burn proof, the mesh's only bond-slashing
/// evidence; `REFUSE_ORIGIN` means "the sender is on another mesh". All
/// three read identically to "the sender's ruling did not land", so the sender's only recovery was blind retry and
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
    /// 47.2 step 3: this receiver's knowledge of the signer's line has a
    /// gap, and it is asking for the receipts strictly after `from` (from
    /// the line's start where `None`) up to `to`. Not a refusal — "the
    /// back-fill fetches what makes it re-derive".
    Gap { from: Option<crate::chronometry::Place>, to: crate::chronometry::Place },
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
    /// The task answered, by its citation (wire v1.2): the door resolves it
    /// to the task's content address (`TaskRegistry::by_citation`).
    pub task: [u8; 32],
    /// One output grid per test input, in the order they were served.
    pub outputs: Vec<Vec<Vec<u8>>>,
    /// The rule the voice extracted, when it speaks one. `None` is an
    /// answer with no rule behind it — the registry prices it differently.
    pub rule: Option<Vec<u8>>,
    /// 78.141 — whether the voice signed the act. The court pays the voice
    /// a solve names, so an unsigned solve names no payee and is refused.
    pub signed: bool,
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
    /// 78.152 TU-2 — ITS KEEPER RING: its newest TUniversal readings, oldest
    /// first, each signed by `vk` on its own and parent-linked (the digest
    /// now carries the signer's point, which 45.2's digest never did).
    pub ring: Vec<crate::chronometry::tuniversal::TickReading>,
    /// The signature over the prefix `cover_signed_prefix` builds.
    pub sig: [u8; 64],
}


/// solve receipt: `[code u8][μ⃗ 32]` — code 2 = BOUNTY (first verified solve),
/// 1 = verification floor (replay), 0 = nothing (wrong or unregistered).
pub const SOLVE_BOUNTY: u8 = 2;
pub const SOLVE_FLOOR: u8 = 1;
pub const SOLVE_NOTHING: u8 = 0;
/// 78.153 — NOT JUDGED YET: the court's stroke ran out before the replay
/// ruled; asked again, the replay resumes where it stopped. A RETRY, never a
/// refusal — distinct from SOLVE_NOTHING, which since 78.149 is final ("no
/// work"): a house re-files on this code and drops on that one (ruled
/// 2026-10-09).
pub const SOLVE_UNREAD: u8 = 3;

/// One link of a centred coboundary chain: `(row, col, before, after)`.
pub type Link = (i64, i64, u8, u8);

/// One quoted work item: the listing's address, its priced μ⃗, the listing
/// WHOLE, and the open (non-flat) charts of its coherence field.
///
/// THE WHOLE ELEMENT, NOT ITS LINKS (L8q). A `chain: Vec<Link>` here — a
/// flat projection of a coboundary — would leave a voice one move, to
/// subtract links it held, which is deflation and not comprehension. A
/// listing is an `Element`, and served whole it may be a SENTENCE carrying
/// demand vertices (50.5) naming what it could not spell; answering one
/// with a word another mind holds is the market.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItem {
    /// The listing's address — what a demand for it is filed under.
    pub addr: Address,
    /// Its priced μ⃗, per lane.
    pub mu: [u64; 4],
    /// The listing itself. The encoding is injective — `from_element_form`
    /// rebuilds the same vertices, faces, orbits and demands — so nothing is
    /// flattened by crossing; what is refused is READING these bytes, since
    /// identity is re-derived by `same_vertex` (law 2, 1.7).
    pub element: Vec<u8>,
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
    /// The hypha's place on its line at install — the orbit and the act's
    /// place in it: the coordinate a court's "since" is read against.
    pub at: crate::chronometry::Place,
}

/// A SETTLEMENT on the docket: the listing, its chain, the voice, the
/// utterance that settled it, and the magnitudes before and after as the
/// hypha measured them. The court replays the utterance and measures for
/// itself: two measurements of one deflation are the proof of convergence.
/// The last field is the stamp of the act the settlement follows on the
/// hypha's own line (78.27 (6)): its place orders it inside one kernel and
/// its tick is TUniversal, one coordinate for both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settlement {
    /// The listing settled.
    pub listing: crate::transducer::address::Address,
    /// Its chain.
    pub chain: Vec<Link>,
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
    /// The stamp of the act on the hypha's own line this settlement
    /// follows — the head when it was recorded (78.27 (6)). The `k` and the
    /// `t_universal` it replaced are its place and its tick.
    pub at: crate::chronometry::Stamp,
}

/// Verdict rulings — the court's closed vocabulary.
pub const RULING_UPHELD: u8 = 1;
pub const RULING_DEMOTED: u8 = 2;
pub const RULING_SLASHED: u8 = 0;
// 3 and 4 are RETIRED (EC1, 2026-09-24): REBIND carried π* (a Selection term) and
// ATTACHED carried a base and a transport g — a Selection term or a versor on a
// node is a blob where a coordinate belongs, and neither had a producer.
// Attachment returns with its producer (an inter-form 2-cell attached along
// a round) as a cite of the base by its stamp and g as a class record. The
// bytes are not reused.
/// Two independent voices verified the same task: a proven root.
pub const RULING_ROOT: u8 = 5;

