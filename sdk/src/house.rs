// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.62, 78.120, 78.145, 78.155b, Concordance §0 — THE HOUSE SURFACE: what a
// kernel's uplink (its house) speaks to a hypha, every item the library's
// own, re-exported, so a kernel need not reach into `mycelium`.

//! A kernel's house.
//!
//! A kernel runs a governed world under the Concordance and speaks to the
//! network through its uplink, its house: it signs the Concordance's hash,
//! reads the law register and the core clock, puts the transitions it
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
//! | The Concordance | [`CONCORDANCE_HASH`], [`ConcordanceSignature`], [`verify_concordance_signature`], modes 7 and 8 |
//! | The register and clock | [`encode_register_request`], [`decode_register_reply`], [`encode_clock_request`], [`decode_clock_reply`], [`CoreStamp`], [`TPoint`] |
//! | Verification (L2) | [`Transition`], [`BodyState`], [`Body`], [`Flux`], [`verify_sized`], [`encode_verify_request`], [`decode_verify_reply`], [`Verdict`], [`Breach`], [`Law`] |
//! | Domain founding | [`DomainGenesis`], [`encode_domain`], [`decode_domain_reply`], [`Founded`]; the reserve, the bond, the origin |
//! | The universe | [`Origin`], [`Genesis`], [`Geometry`], [`encode_universe_read`], [`decode_universe_reading`], [`UniverseReading`], the statements |
//! | The endowment | [`Manifest`], [`encode_manifest`], [`decode_grant_reply`], [`Granted`], [`encode_endowment_read`], [`decode_endowment_reading`] |
//! | Seats | [`Post`], [`Seats`], [`Taken`], [`Percept`], [`Outcome`], [`HouseMail`], [`encode_post`], [`decode_seat_request`], the listing and the mail |
//! | Framing and identity | [`Frame`], [`Stratum`], [`read_lane`], [`split_point`], [`BODY_BUDGET`], [`HyphaKey`], [`payload_hash`] |
//! | Whole modules | [`domain`], [`universe`], [`endowment`], [`seat`], [`atlas`], [`uplink`], [`challenge`], [`commitment`], [`tuniversal`] |

/// The Concordance's hash (BLAKE3 of its exact bytes at build time), a
/// kernel's signature over it, its verifier and its refusal by name.
pub use mycelium::physics::laws::concordance::{
    verify_concordance_signature, ConcordanceRefusal, ConcordanceSignature, CONCORDANCE_HASH, SIGNATURE_LEN,
};
/// The hash read (`ACT_LAWS` mode 7) and the signature presented and judged
/// (mode 8).
pub use mycelium::physics::laws::codec::{
    decode_concordance_reply, decode_signature_reply, encode_concordance_request, encode_signature_request, MODE_CONCORDANCE, MODE_SIGNATURE,
};

/// The law register's read (mode 0): the request, the reply at this
/// register's version, and one law as served.
pub use mycelium::physics::laws::codec::{decode_register_reply, encode_register_request, Served, MODE_REGISTER};
/// The core clock (mode 4): the request, the reply (`Some(None)`: a court
/// with no head yet), and the head's stamp.
pub use mycelium::physics::laws::codec::{decode_clock_reply, encode_clock_request, MODE_CLOCK};
/// The core clock's stamp, and a court act's stamp.
pub use mycelium::chronometry::{CoreStamp, Stamp};
/// A final point of TUniversal: `origin/h:S:G:T:H:K`.
pub use mycelium::chronometry::tuniversal::TPoint;

/// THE VERIFIER (L2): a transition the kernel composed, every body before
/// and after it, the sources and sinks it records, and the law register's
/// verdict on it — every breach named with its law. `verify_sized` rules
/// under the stroke the motion needs, as the court does, so a kernel that
/// verifies with it agrees with the court exactly.
pub use mycelium::physics::laws::{verify, verify_sized, BodyState, Breach, Flow, Flux, Law, Transition, Unmeasured, Verdict};
/// A body as A.13 reads it (centre, mass, second moments).
pub use mycelium::physics::tensegrity::Body;
/// The multivector of Cl(4,1) over ℚ a motion and a nucleus are written in.
pub use mycelium::transducer::cga::Mv;
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
/// The domain's reserve tier designated, and the reserve read.
pub use mycelium::anastome::dialogue::domain::{
    decode_reserve_reading, encode_reservation, encode_reserve_read, Reservation, ReserveReading, SignedReservation,
};
/// A tier's bond posted or withdrawn by the reserve, and the bond read.
pub use mycelium::anastome::dialogue::domain::{decode_bond_reading, encode_bond, encode_bond_read, Bond, SignedBond};
/// A universe's origin, recorded once, and the origin read.
pub use mycelium::anastome::dialogue::domain::{
    decode_origin_reading, encode_origin, encode_origin_read, Origin, OriginReading, SignedOrigin,
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
pub use mycelium::physics::tensegrity::carried_body;
/// Building a transition: a rational rotor, a translator, a sphere at a
/// centre; and why a motion's modes did not split.
pub use mycelium::transducer::cga::{rotator, sphere_at, translator_at, Undecomposed};

/// FRAMING: a lane-tagged frame, the four lanes, a lane byte read (and
/// whether its frame carries a point), the point split off a body, the
/// body's budget and the pointed bit.
pub use mycelium::network::wire::{read_lane, split_point, Frame, Stratum, BODY_BUDGET, POINTED};
/// A count, and a byte run, each written by its own coordinate (LEB128,
/// minimal), as every count on the wire is.
pub use mycelium::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};
/// The citation hash of exact bytes (BLAKE3): what a receipt, a manifest's
/// grant and a call's reference cite.
pub use mycelium::chronometry::payload_hash;
/// A hypha's (or a house's) ed25519 identity, loaded or minted at a path.
pub use mycelium::court::identity::HyphaKey;

/// THE HOUSE'S WIRE MODULES WHOLE, each with every public encoder, decoder
/// and type: the domain (founding, reserve, bond, origin), the universe's
/// statements, the endowment, the seat wire, the atlas (a universe's ledger
/// and map), a tier's fold (`uplink`), a fold's challenges and demands
/// (`challenge`), and a rows commitment (`commitment`).
pub use mycelium::anastome::dialogue::{atlas, challenge, commitment, domain, endowment, seat, universe, uplink};
/// TUniversal: the final point and its origin.
pub use mycelium::chronometry::tuniversal;
