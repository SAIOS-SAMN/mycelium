// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.62, 78.120, 78.145, 78.155b, Concordance §0 — THE HOUSE SURFACE: what a
// kernel's uplink (its house) speaks to a hypha, every item the library's
// own, re-exported, so a kernel need not reach into `mycelium`.

//! A kernel's house.
//!
//! A kernel runs a governed world under the Concordance and speaks to the
//! network through its uplink, its house: it signs the Concordance clause by
//! clause, by reconstructing each clause's fixtures, reads the law register and the core clock, puts the transitions it
//! composed to the court's verifier (L2), founds its domain, designates its
//! reserve, records its universe's origin, asks its endowment by a manifest,
//! reads its universe back, lists its seats and answers the minds seated in
//! them. The conformance read's [`Profile::Kernel`](crate::conformance::Profile)
//! names the acts a house MUST and MAY speak; `conformance::run(endpoint,
//! key, &[Profile::Kernel])` checks them against a running hypha.
//!
//! Each act is encoded here, spoken with
//! [`Session::speak`](crate::session::Session::speak), and its reply read by
//! its decoder. The session also carries the domain acts typed
//! (`found_domain`, `reserve`, `reserve_reading`) and the seat acts
//! (`seat_post`, `list_seats`, `read_mail`).
//!
//! | Group | Items |
//! |---|---|
//! | The Concordance | [`ConcordanceClause`], [`Fixture`], [`Object`], [`sign_concordance`], [`reconstruct_fixture`], [`Signing`], [`SigningRefusal`], modes 7 and 8; the alignment reading, [`Alignment`], [`ClauseReading`], mode 9 |
//! | The clock | [`encode_clock_request`], [`decode_clock_reply`], [`CoreStamp`]; the register's read is [`crate::laws`], the final point [`crate::session::TPoint`] |
//! | Verification (L2) | [`Transition`], [`BodyState`], [`BodyMoments`], [`Flux`], [`verify_sized`], [`encode_verify_request`], [`decode_verify_reply`], [`TransitionVerdict`], [`Breach`]; each law is [`crate::laws::Law`] |
//! | Domain founding | [`DomainGenesis`], [`encode_domain`], [`decode_domain_reply`], [`Founded`]; the reserve, the bond, the origin |
//! | The universe | [`BigBangRecord`], [`Genesis`], [`Geometry`], [`encode_universe_read`], [`decode_universe_reading`], [`UniverseReading`], the statements |
//! | The endowment | [`Manifest`], [`encode_manifest`], [`decode_grant_reply`], [`Granted`], [`encode_endowment_read`], [`decode_endowment_reading`] |
//! | Seats | [`Post`], [`Seats`], [`Taken`], [`Percept`], [`Outcome`], [`HouseMail`], [`encode_post`], [`decode_seat_request`], the listing and the mail |
//! | Handoff | [`SignedHandoff`], [`encode_handoff`], [`decode_handoff_reply`], [`encode_handoff_read`], [`decode_read_reply`], [`encode_loops_read`], [`decode_loops_reply`], [`encode_tube_read`], [`decode_tube_reading`] |
//! | Traverse, ledger and map | [`encode_traverse`], [`decode_traverse_reply`], [`encode_ledger`], [`encode_ledger_read`], [`decode_ledger_reply`], [`decode_ledger_page`], [`encode_map`], [`encode_map_read`], [`decode_map_reply`], [`decode_map_reading`] |
//! | Challenge and finality | [`encode_challenge`], [`decode_challenge_reply`], [`encode_finality_request`], [`decode_finality_reply`], [`encode_line_head_request`], [`decode_line_head_reply`], [`encode_obstructions_request`], [`decode_obstructions_reply`]; the fold sent up, [`encode_uplink`] |
//! | Equivocation | [`equivocation_proof`], [`encode_equivocation`], [`decode_ruled`], [`ReceiptSection`] |
//! | Framing and identity | [`WireFrame`], [`Lane`], [`read_lane`], [`split_point`], [`BODY_BUDGET`], [`payload_hash`]; the identity is [`crate::voice::VoiceKey`] |
//! | Whole modules | [`domain`], [`universe`], [`endowment`], [`seat`], [`atlas`], [`uplink`], [`challenge`], [`commitment`], [`tuniversal`] |

/// The Concordance's clause register (`ACT_LAWS` mode 7) and a kernel's
/// signing act (mode 8): the clause, its fixtures and the objects they
/// reach, the act and its verdict, each refusal by name; and the kernel's
/// signing helper, its own reconstruction of each fixture.
pub use crate::signing::{
    decode_concordance_reply, decode_signature_reply, encode_concordance_request, encode_signature_request, reconstruct_fixture, sign_concordance,
    Admitted, Assent, Choice, ConcordanceClause, ConcordanceClauseId, Fixture, Object, SignedClause, Signing, SigningRefusal, Standing,
};
/// The alignment reading (mode 9), derived from the signings the hypha's
/// ledger records: per clause version, converses, vetoed, collides with the
/// residue, or unread; and the signings of the past as history.
pub use crate::signing::{decode_alignment_reply, encode_alignment_request, Alignment, AlignmentRefusal, ClauseReading, KernelAlignment};

/// The core clock (mode 4): the request and the reply (`Some(None)`: a
/// court with no head yet). The register's read and every mode stand in
/// [`crate::laws`]; the final point in [`crate::session`].
pub use mycelium::physics::laws::codec::{decode_clock_reply, encode_clock_request};
/// The core clock's stamp, and a court act's stamp.
pub use mycelium::chronometry::{CoreStamp, Stamp};

/// THE VERIFIER (L2): a transition the kernel composed, every body before
/// and after it, the sources and sinks it records, and the law register's
/// verdict on it — every breach named with its law. `verify_sized` rules
/// under the stroke the motion needs, as the court does, so a kernel that
/// verifies with it agrees with the court exactly.
pub use mycelium::physics::laws::{verify, verify_sized, BodyState, Breach, Flow, Flux, Transition, TransitionVerdict, Unmeasured};
/// THE CONFORMAL LAW ON A MOTION STATED PHASE BY PHASE (MU-HOSTING
/// §5a.10): per bell, each phase of a frame advances on its own named plane
/// by the fraction `r/h` of its period, the degrees `360°·r/h` on that
/// plane, held exactly in ℚ/ℤ where no exact versor exists (Niven), and
/// refused `LeavesPeriods` where a phase's advance leaves the frame's
/// periods. In process: the wire carries it with the crossing record
/// (track 25, X1).
pub use mycelium::physics::laws::{verify_turns, TurnBreach, TurnMotion, TurnTransition, TurnVerdict};
/// A phase's advance in ℚ/ℤ (`Turn::of(r, h)`, the degrees `360°·r/h` on
/// its plane); a frame's periods are [`crate::task::Enclosure`].
pub use mycelium::transducer::family::transduce::Turn;
/// A body as A.13 reads it (centre, mass, second moments).
pub use mycelium::physics::equilibrium::BodyMoments;
/// The verify act (mode 1) and its verdict; the batch (mode 2) and its
/// verdicts in order; a transition's and a verdict's bytes.
pub use mycelium::physics::laws::codec::{
    decode_batch_reply, decode_laws_request, decode_verify_reply, encode_batch_request, encode_verify_request, put_transition, take_transition,
    take_verdict, LawsRequest, MODE_BATCH, MODE_VERIFY,
};

/// DOMAIN FOUNDING (`ACT_DOMAIN`): the genesis signed by the domain's key,
/// the court's answer (`Founded`: sealed, or the refusal by name), and the
/// request read whole.
pub use mycelium::anastome::dialogue::domain::{
    decode_domain, decode_domain_reply, decode_domain_request, encode_domain, DomainGenesis, DomainRequest, Founded, SignedDomain,
};
/// The domain's reserve tier designated, and the reserve read (its
/// reading is [`crate::readings::ReserveReading`]).
pub use mycelium::anastome::dialogue::domain::{
    decode_reserve_reading, encode_reservation, encode_reserve_read, Reservation, SignedReservation,
};
/// A tier's bond posted or withdrawn by the reserve, and the bond read.
pub use mycelium::anastome::dialogue::domain::{decode_bond_reading, encode_bond, encode_bond_read, Bond, SignedBond};
/// A universe's origin, recorded once, and the origin read.
pub use mycelium::anastome::dialogue::domain::{
    decode_origin_reading, encode_origin, encode_origin_read, BigBangRecord, OriginReading, SignedOrigin,
};

/// THE UNIVERSE: its genesis (a geometry and a parent face) as an origin of
/// version 2 states it, and the universe read back.
pub use mycelium::anastome::dialogue::universe::{
    decode_universe_reading, encode_universe_read, Genesis, Geometry, Metric, Parent, UniverseReading,
};
/// What a universe states once, by its sitting reserve: its geometry, its
/// reserve's basket, its join to a parent's core.
pub use mycelium::anastome::dialogue::universe::{
    basket_holds, encode_statement as encode_universe_statement, Basket, SignedStatement as SignedUniverseStatement, Stated,
    Statement as UniverseStatement,
};

/// THE ENDOWMENT: a universe's manifest (nuclei per Z, signed by its sitting
/// reserve), the grant that answers it or its refusal by name, and the
/// endowment read.
pub use mycelium::anastome::dialogue::endowment::{
    decode_endowment_reading, decode_grant_reply, encode_endowment_read, encode_manifest, nuclei_hold, EndowmentReading, GrantRead, GrantRefusal,
    Granted, Manifest, Nuclei, SignedManifest,
};
/// A count of nuclei: a positive integer of any size.
pub use num_bigint::BigUint;

/// SEATS, THE HOUSE'S SIDE: the listing it posts, its answer to a take, the
/// percepts it lets a seated mind perceive, the outcome of a call, each
/// signed by the domain's sitting reserve; and what it reads in its mail.
pub use mycelium::anastome::dialogue::seat::{Call, Outcome, Percept, Post, Record, Seat, SeatFn, Seats, SignedPost, Signer, Take, Taken};
/// What a house reads in its mail: a take and a call (with its reference).
pub use mycelium::sdk::seat::HouseMail;
/// An argument, its tags, a seat's role, and the counsel record's kind.
pub use mycelium::anastome::dialogue::seat::{put_args, take_args, Arg, Role, KIND_COUNSEL, TAG_COUNT, TAG_FLAG};
/// The reasons a house's taken and outcome carry.
pub use mycelium::anastome::dialogue::seat::{REASON_BAD_ARGUMENTS, REASON_NOT_APPLIED, REASON_NOT_THE_HOLDER, REASON_NO_SUCH_FUNCTION};
/// A post on the wire and the relay's answer; the listing read; the mail
/// read at the relay's standing; a seat frame read whole; the relay's
/// refusal by name.
pub use mycelium::anastome::dialogue::seat::{
    decode_listing_reply, decode_mail_reply_standing, decode_post_reply, decode_seat_request, encode_post, encode_read_listing, encode_read_mail,
    Mail, MailRead, ReadMail, Refusal, SeatRequest,
};

/// A TASK SEAT'S WORDS: the record kind a percept names a task by (its
/// citation), and the functions a seated mind solves or declines it by.
pub use mycelium::sdk::seat_mind::task::{FN_DECLINE, FN_SOLVE, KIND_TASK};

/// A body carried rigidly by a motion: its state after a transition.
pub use mycelium::physics::equilibrium::carried_body;
/// Building a transition: a rational rotor, a translator, a sphere at a
/// centre; and why a motion's modes did not split.
pub use mycelium::transducer::cga::{rotator, sphere_at, translator_at, Undecomposed};

/// FRAMING: a lane-tagged frame, the four lanes, a lane byte read (and
/// whether its frame carries a point), the point split off a body, the
/// body's budget and the pointed bit.
pub use mycelium::network::wire::{read_lane, split_point, Lane, WireFrame, BODY_BUDGET, POINTED};
/// A count, and a byte run, each written by its own coordinate (LEB128,
/// minimal), as every count on the wire is.
pub use mycelium::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};
/// The citation hash of exact bytes (BLAKE3): what a receipt, a manifest's
/// grant and a call's reference cite.
pub use mycelium::chronometry::payload_hash;

/// HANDOFF (`ACT_HANDOFF`, MU-HOSTING §2.4): matter or a word crossing
/// domains, a conversion of μ, a mouth or a tube — offered, accepted,
/// refused or expired, each signed by its filer; the court's answer
/// (`Handed`), a handoff's standing read, the open mouth loops and a mouth
/// pair's tube, each read without sealing.
pub use mycelium::anastome::dialogue::handoff::{
    decode_handoff_reply, decode_loops_reply, decode_read_reply, encode_handoff, encode_handoff_read, encode_loops_read, encode_tube_read,
    ConvertIn, ConvertOut, Handed, Handoff, HandoffRefusal, Loop, Mouth, Offer as HandoffOffer, SignedHandoff, State as HandoffState, Tariff,
};
/// A mouth pair's tube as the court reads it.
pub use mycelium::anastome::dialogue::tube::{decode_tube_reading, Tube, TubeReading};

/// TRAVERSE (`ACT_DOMAIN` kind 16, 78.154): a coordinate retrieval through
/// a mapped wormhole, signed by the payer; the court's answer (`Crossed`):
/// the crossing's hash, the gas and the map fee, or the refusal by name.
pub use mycelium::anastome::dialogue::crossing::{decode_traverse_reply, encode_traverse, CrossRefusal, Crossed, Declaration, Gas, SignedTraverse, Traverse};

/// A UNIVERSE'S LEDGER AND MAP (`ACT_DOMAIN` kinds 12–15, 78.154): an entry
/// chained and sealed, never judged; the ledger read by pages; a map act
/// (mouth, segment, local throat) posted by the sitting reserve and the map
/// read.
pub use mycelium::anastome::dialogue::atlas::{
    decode_ledger_page, decode_ledger_reply, decode_map_reading, decode_map_reply, encode_ledger, encode_ledger_read, encode_map, encode_map_read,
    LedgerEntry, LedgerPage, LedgerRefusal, Ledgered, MapAct, MapEntry, MapRecord, MapRefusal, Mapped, SignedLedger, SignedMap,
};

/// A FOLD CHALLENGED, AND ITS WINDOW READ (`ACT_CHALLENGE`, Concordance
/// 4.5): the signed challenge and the court's answer; a fold's finality, a
/// tier's line head and a domain's open obstructions, each read without
/// sealing; where a fold stands on the court.
pub use mycelium::anastome::dialogue::challenge::{
    decode_challenge_reply, decode_finality_reading, decode_finality_reply, decode_line_head_reply, decode_obstructions_reply, encode_challenge,
    encode_finality_request, encode_line_head_request, encode_obstructions_request, Challenge, ChallengeRefusal, Challenged, Finality,
    FinalityReading, LineHead, OpenObstruction, SignedChallenge,
};
pub use mycelium::anastome::dialogue::uplink::FoldAt;

/// A TIER'S FOLD SENT UP (`ACT_UPLINK`, 78.119): the fold signed by its tier.
pub use mycelium::anastome::dialogue::uplink::{encode_uplink, SignedFold, TierFold};

/// AN EQUIVOCATION PROVED (`ACT_EQUIVOCATION`, MU-HOSTING §5.3, MU-XSEPTUM:
/// sendable by anyone): two receipt sections signed under one key at one
/// place of its line, the court's answer read as [`Ruled`] (`Applied`, the
/// bond burned; `Inert`, nothing left to burn; `Refused`, by name). The
/// proof is built by [`equivocation_proof`].
pub use mycelium::anastome::dialogue::{
    decode_receipt_section, decode_ruled, encode_equivocation, encode_receipt_section, ReceiptSection, Ruled, EQUIVOCATION_VERSION,
};

/// THE PROOF OF AN EQUIVOCATION, OR NONE: where two receipt sections a
/// client holds stand at ONE place of ONE signer's line — the same key, the
/// same place — and their chain hashes differ, the act that presents them,
/// `[ACT_EQUIVOCATION]…`; `None` where they are one receipt, stand at two
/// places or under two keys, or do not frame. The court reconstructs both
/// halves and judges; this only says the two are worth presenting.
pub fn equivocation_proof(a: &ReceiptSection, b: &ReceiptSection) -> Option<Vec<u8>> {
    use mycelium::court::admission::Predecessor;
    let (sa, sb) = (&a.receipt.stamp, &b.receipt.stamp);
    if sa.vk != sb.vk || sa.place() != sb.place() || !Predecessor::of(&a.receipt).forks(&Predecessor::of(&b.receipt)) {
        return None;
    }
    encode_equivocation(a, b)
}

/// THE HOUSE'S WIRE MODULES WHOLE, each with every public encoder, decoder
/// and type: the domain (founding, reserve, bond, origin), the universe's
/// statements, the endowment, the seat wire, the atlas (a universe's
/// anchor chain, `KIND_LEDGER`, and map), a tier's fold (`uplink`), a fold's challenges and
/// demands (`challenge`), and a universe's state commitment and events
/// digest (`commitment`).
pub use mycelium::anastome::dialogue::{atlas, challenge, commitment, domain, endowment, seat, universe, uplink};
/// TUniversal: the final point and its origin.
pub use mycelium::chronometry::tuniversal;
