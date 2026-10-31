// 24 — the mind: a voice with its genome, and what it owes its courts.

use super::*;

/// ONE SAYING HELD FOR THE MIND'S RETURN: what leaves when it is run there.
pub(crate) type HeldSaying = Box<dyn FnOnce(&mut Mind)>;

/// THE MIND — a voice with its genome/epigenome, conversing with
/// its court through the hypha carrier (MU-CLIENT §5.1): it speaks sentences,
/// learns what the mesh teaches (comprehending, not storing), asks quotes, and
/// probes listings, and never waits on the answer.
pub struct Mind {
    pub(crate) hypha: super::entity::hypha::Hypha,
    /// MU-CLIENT §6 — THE SEAT'S OWN CARRIER: the seat surface's link to the
    /// hypha on its own thread (`Mind::ask_seat`), so a slow house holds
    /// that carrier and never the mind.
    pub(crate) seat: super::entity::hypha::Hypha,
    /// THE SEAT'S PERCEIVING CARRIER (2026-10-23): the held mail read alone
    /// (`Mind::ask_held`), on its own thread and session. The court holds
    /// that read to its next event; on the seat's carrier it blocked every
    /// take, solve and declination queued behind it, and a call the court
    /// never received could not be the event that released the read.
    pub(crate) held: super::entity::hypha::Hypha,
    pub epigenome: Epigenome,
    /// Rotating cursor for the refinement probe — one genome word examined
    /// per stroke, so the compression search is metabolic, never a sweep.
    /// The rule the court last refused for each task, by task id — so the
    /// same rule is not sent again (`Mind::solve_task`). One per task.
    pub(crate) solve_refused: crate::network::nerve::Keyed<Vec<u8>>,
    /// Words this voice has spoken up for adoption: said once, the judge
    /// decided (installed, derivable, or refused). Membership is vertex
    /// equality, never the digest alone. MU-CLIENT §4.13 — KEYED BY THE
    /// ADDRESS'S 32-BYTE HASH (`Mind::taught_key`, ruled
    /// 2026-10-16: a hash compacts coordinates where nothing is lost), each
    /// with the stratum its vertex was filed under in `taught_at` — none for
    /// a sentence, which is one vertex by its address alone. The word itself
    /// stands where the language holds it (the dialect keeps a taught word
    /// until the genome does), and the exact test reads it there.
    pub(crate) taught_up: crate::network::nerve::Keyed<Option<Vec<u8>>>,
    /// `taught_up` INDEXED BY VERTEX STRATUM: the M1 key of a word's rotor
    /// (or its canonical chain, where it has no letters) to the addresses
    /// uttered at it.
    ///
    /// `already_taught` read the whole of `taught_up` doing an exact
    /// `same_vertex` against every word this mind had ever uttered, on
    /// every sentence of every speak — law 3's "no loop over pairs the
    /// carrier does not already name", and its answer: "the carrier is
    /// missing a key and that key is the first thing to write". On a mesh
    /// whose voices all produce language, that read grew with the mesh's
    /// whole output.
    ///
    /// The key is NECESSARY and not sufficient: `same_vertex` requires the
    /// rotors to be projectively equal, so two words that are one vertex
    /// share this key, and the exact test still decides over the fibre it
    /// names. That is A21 §3's own shape — "equal keys are necessary for
    /// two classes to be the same; sufficiency … is `solve_gauge_between`".
    pub(crate) taught_at: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// The discordance measured at the last sync (strata whose roots differ
    /// from the hypha's); zero is a glued genome — the condition under which
    /// this mind speaks proofs. Unmeasured: the open thought's boundary
    /// cochain norm, never usize::MAX.
    pub(crate) discordance: usize,
    pub(crate) discordance_measured: bool,
    /// WHAT THIS MIND LEARNED SINCE THE STROKE LAST TOOK IT (24.3): the
    /// measure, per lane, of every page `absorb_taught` admitted — the
    /// outcome's `learned` mode, accumulated at the admission and taken
    /// once per stroke by `take_learned_mu`.
    pub(crate) learned_mu: [u64; 4],
    /// ČECH AFTER FETCH: the obstructions measured on the fibre when a
    /// fetched word met the words already held there, each by its coordinates
    /// — the stratum, then the fetched word's address, to the address of the
    /// held word whose transition to it did not vanish. The digest only
    /// indexed the wire; this is the glue, on the elements. A stratum that
    /// glues lets its obstructions go.
    pub(crate) obstructions: crate::network::nerve::Keyed<crate::network::nerve::Keyed<crate::transducer::address::Address>>,
    /// Strata the fibre Čech has already glued (a fetched word's transition
    /// to the held representative vanished): the outer index is not walked
    /// there again. The digest names what differs by address; the geometry
    /// decides what is fetched.
    pub(crate) glued: crate::network::nerve::Keyed<()>,
    /// OPEN THOUGHTS: the contemplation over bubbles, the one carrier of
    /// held forms (24.1 as ruled 2026-09-09); a cite reads the forms' words at
    /// the listing body's chart. Remainders persist; residuals are never
    /// lost on restart.
    pub(crate) contemplation: crate::transducer::bubble::Contemplation,
    pub(crate) physics: crate::physics::Physics,
    pub(crate) height: crate::chronometry::Height,
    /// 78.49, 78.23, MU-CLIENT §4.4(4) — THIS MIND'S OWN K: its completed
    /// orbits — every orbit of its field that returned and every thought
    /// whose winding came round — counted at the close of the stroke they
    /// returned in. The mind's clock is its own motion; the ledger is the
    /// court's (law 8), and no receipt of the mind's exists.
    pub(crate) k: u64,
    /// MU-CLIENT §4.4(4), §4.13, §7.1 R2 — WHAT THE MIND SAYS, HELD FOR THE
    /// RETURN OF THE THOUGHT IT SPEAKS FOR: speech to a court, a solve, a SAY
    /// to its peers, by the scene whose thought it speaks for (empty for what
    /// the mind says for itself), each posted where a thought of that scene
    /// returns — or, with no scene, at the mind's next return — and nowhere
    /// else, so its outflow is bounded by its thoughts' own periods. Within a
    /// scene each is held BY ITS COORDINATES: a thing said twice before its
    /// return is one thing, its latest saying in the place of its first —
    /// never by a count. An answer to a counterparty's frame is never held
    /// here: it leaves where the frame lands.
    pub(crate) at_return: crate::network::nerve::Keyed<Vec<(Vec<u8>, HeldSaying)>>,
    /// MU-CLIENT §3.1, §4.9 — THE SCENES NOT YET COMPREHENDED, by the task's
    /// citation, each with the language root it was last read at: read again
    /// at the mind's returns, gone with the last thought of each.
    pub(crate) comprehending: crate::network::nerve::Keyed<(crate::anastome::dialogue::TaskEnvelope, crate::transducer::address::Address)>,
    /// The citation last read again (`comprehend_waiting`): each waiting
    /// scene is read in turn.
    pub(crate) comprehended_after: Vec<u8>,
    /// ACT1 (78.21(3)) — the court's height this voice had heard, and its
    /// own `K`, when its genome was last written whole; none before.
    pub(crate) genome_persisted: Option<(u64, u64)>,
    /// THE SEALED SET (50.9): addresses a court refused, each with the mind's
    /// own K at which it heard the refusal (78.49). Read before any frame is
    /// built; a sealed tape is refused by the mind itself.
    pub(crate) sealed: crate::network::nerve::Keyed<u64>,
    /// Contraction certificates by the contracted form's vertex key, cited
    /// on the next utterance of that form (50.9).
    pub(crate) contractions: crate::network::nerve::Keyed<crate::transducer::sentence::Contraction>,
    /// The contracted sentences waiting to be uttered, each with the family
    /// and the task it was minted for.
    pub(crate) contracted: Vec<Pending>,
    /// 21 step 3, chapter 6's depth order — WHAT THIS MIND OWES A COURT
    /// BEFORE IT SPEAKS AGAIN. A court that answered `Awaiting` has not
    /// ruled a form this mind cited; the mind HOLDS that form, so it
    /// utters the cited form first and the citer after it. Nothing was
    /// sealed and nothing was refused: this is the mind teaching the
    /// court what it was speaking from, in the order a cited form must
    /// precede its citer.
    pub(crate) owed: Vec<Owed>,
    /// HOW MANY COURTS A WAITING CITER HAS BEEN OWED TO, by its address:
    /// an owed frame goes to the courts in turn, so a citer is owed at
    /// most once per court this mind knows and then waits for the family
    /// to come round again. Cleared when the citer is admitted or sealed.
    pub(crate) owed_tries: crate::network::nerve::Keyed<usize>,
    /// 37.7 — WHAT THIS MIND SPOKE AND NO COURT HAS TAUGHT YET, by address:
    /// a letter or crystallized idea a court deferred stands in the dialect,
    /// and the guard that kept it from being spoken twice kept it from ever
    /// being spoken again (courts deferred 211/126/442 acts, epoch 146). Said
    /// again at the mind's returns until a court stands or ratifies it, or
    /// seals it. MU-CLIENT §4.13 — HELD BY ITS COORDINATES: the word's
    /// address, with the scene it was spoken under (§5.4, E1); the word is
    /// the one its dialect stands, read there, and it leaves where the
    /// dialect sheds it — bounded by the language, never by a count.
    pub(crate) unheard: crate::network::nerve::Keyed<Vec<u8>>,
    /// MU-CLIENT §4.13 — WHAT THIS MIND HAS SAID AND NO COURT HAS ANSWERED
    /// YET, by address: a speech stood and held for its scene's return, or
    /// posted and not yet landed. Each is an unjudged sentence the dialect
    /// keeps (`Mind::shed`); it leaves where the court's answer lands.
    pub(crate) speaking: crate::network::nerve::Keyed<()>,
    /// 78.153 — THE SOLVES THE COURT HAS NOT JUDGED YET (`SOLVE_UNREAD`), by
    /// task: the rule and the outputs, asked again each stroke until the
    /// court rules, its replay resuming where its stroke ran out. Without it
    /// the task arm moved to the next task and the solve was never asked
    /// again: 38 of 41 verdicts on the development deployment were "unread" (2026-10-10).
    /// MU-CLIENT §4.13 — one per task, released by the court's verdict:
    /// bounded by what the court holds unjudged.
    pub(crate) unread_solves: crate::network::nerve::Keyed<(Element, Vec<Grid>)>,
    /// The task coordinate the last unread solve was asked again at: the
    /// next asking takes the least after it, so every one is asked in turn.
    pub(crate) unread_after: Vec<u8>,
    /// 78.146 — THE SEATS THIS MIND SITS IN A GAME, by domain ‖ seat: what
    /// it perceives there and what its calls were seen to do (volatile, the
    /// mind's own, never persisted).
    pub(crate) seats: crate::network::nerve::Keyed<super::seat_mind::SeatMind>,
    /// 78.161 — this mind's chatter with other minds, where it opened one.
    pub(crate) chat: Option<super::entity::chatter::Chat>,
    /// 78.164 — the boundaries this mind is open on, and what its field
    /// keeps from tick to tick to meet them.
    pub(crate) board: super::entity::winding::Board,
    /// MU-CLIENT §4.2 — THIS MIND'S OWN FIELD: its tubes and orbits, its
    /// lanes, its lexicon, the words taught it, the covers it parked; it
    /// thinks in it for the length of each tick.
    pub(crate) field: crate::transducer::mind::MindField,
    /// MU-CLIENT §4.9 — WHAT THIS MIND REMEMBERS HEARING: the rings it has
    /// heard, by size (the ring's return), each by its coordinates — the
    /// mind's memory, harmonically resonant, never a vocabulary and never the
    /// ear's (the ear is a tuned resonator that lifts signal from noise and
    /// keeps nothing past the moment). What a heard ring is read against to
    /// answer "does this SOUND like something the mind has heard?" A ring heard
    /// again is remembered once.
    pub(crate) remembered: crate::network::nerve::Keyed<Vec<(Vec<u8>, crate::transducer::cga::Mv)>>,
    /// MU-CLIENT §5.3 — THE QUESTIONS THIS MIND HAS OPEN: keyed by the mind
    /// asked and the question's coordinates, each the word posited over what
    /// the mind knows and the residue asked about, until a relation derives
    /// its meaning or it stands an aperture cycle unanswered. The mind's,
    /// never its conversation's carrier's.
    pub(crate) questions: crate::network::nerve::Keyed<super::entity::chatter::Question>,
    /// MU-CLIENT §5.5 — THE SCENES THIS MIND SPOKE WORDS ON, what a heard
    /// word is read against, and the convergences found there
    /// (`entity::converge`).
    pub(crate) scenes: super::entity::converge::Scenes,
}

/// A cited form owed to a court, with the citer that waits on it and the
/// family and task both were spoken over.
pub struct Owed {
    pub form: Element,
    pub citer: Element,
    pub train: Vec<crate::transducer::family::TrainPair>,
    pub task: Vec<u8>,
}

/// A CONTRACTED SENTENCE WAITING TO BE UTTERED (50.9): the sentence the
/// move produced, the family it was minted over, the task it answers, and
/// the certificate it cites.
#[derive(Clone, Debug)]
pub struct Pending {
    pub sentence: Element,
    pub train: Vec<crate::transducer::family::TrainPair>,
    pub task: Vec<u8>,
    pub certificate: crate::transducer::sentence::Contraction,
}
