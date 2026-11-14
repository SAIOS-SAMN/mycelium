// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 P2 — THE SDK'S SURFACE CLAIM: every public item of the
// curated surface, imported one `use` per item. The file compiles exactly
// where every item stands at its path; an item that disappears or moves
// fails the build by name. A new item enters the surface with its line here.

#![expect(unused_imports, reason = "each import is the claim: it resolves, and is never used")]

// The crate root.
use mycelium_sdk::encode_deferral as _;
use mycelium_sdk::forms as _;
use mycelium_sdk::solve_kind as _;
use mycelium_sdk::ACT_TABLE as _;
use mycelium_sdk::PENDING_CODE as _;
use mycelium_sdk::SOLVE_KIND_FACTORISATION as _;
use mycelium_sdk::SOLVE_KIND_OUTPUTS as _;
use mycelium_sdk::WIRE_VERSION as _;

// voice
use mycelium_sdk::voice::hex32 as _;
use mycelium_sdk::voice::load_or_migrate_voice as _;
use mycelium_sdk::voice::load_or_mint as _;
use mycelium_sdk::voice::parse_hex32 as _;
use mycelium_sdk::voice::MeshRecord as _;
use mycelium_sdk::voice::SigningKey as _;
use mycelium_sdk::voice::VerifyingKey as _;
use mycelium_sdk::voice::Voice as _;
use mycelium_sdk::voice::VoiceKey as _;

// session
use mycelium_sdk::session::deferral_of as _;
use mycelium_sdk::session::read_deferral as _;
use mycelium_sdk::session::Deferral as _;
use mycelium_sdk::session::GenomeReading as _;
use mycelium_sdk::session::Mind as _;
use mycelium_sdk::session::Session as _;
use mycelium_sdk::session::TPoint as _;
use mycelium_sdk::session::TransportAperture as _;
use mycelium_sdk::session::encode_digests_request as _;
use mycelium_sdk::session::encode_evidence_request as _;
use mycelium_sdk::session::encode_fetch as _;
use mycelium_sdk::session::encode_peers_request as _;
use mycelium_sdk::session::encode_probe as _;
use mycelium_sdk::session::encode_quote_request as _;
use mycelium_sdk::session::encode_speech as _;
use mycelium_sdk::session::encode_syllabus_request as _;
use mycelium_sdk::session::ChartOf as _;

// task
use mycelium_sdk::task::decode_posted_samples as _;
use mycelium_sdk::task::decode_samples as _;
use mycelium_sdk::task::decode_solve_receipt as _;
use mycelium_sdk::task::decode_solve_verdict as _;
use mycelium_sdk::task::decode_task as _;
use mycelium_sdk::task::encode_posted_samples as _;
use mycelium_sdk::task::encode_samples as _;
use mycelium_sdk::task::encode_solve as _;
use mycelium_sdk::task::encode_solve_spoken as _;
use mycelium_sdk::task::encode_task as _;
use mycelium_sdk::task::position as _;
use mycelium_sdk::task::push_sample as _;
use mycelium_sdk::task::read_sample as _;
use mycelium_sdk::task::spoken_outputs as _;
use mycelium_sdk::task::Member as _;
use mycelium_sdk::task::SectionRefusal as _;
use mycelium_sdk::task::Place as _;
use mycelium_sdk::task::PostedTask as _;
use mycelium_sdk::task::Sample as _;
use mycelium_sdk::task::SampleRefusal as _;
use mycelium_sdk::task::SampleWire as _;
use mycelium_sdk::task::Split as _;
use mycelium_sdk::task::TaskSection as _;
use mycelium_sdk::task::encode_solve_with_rule as _;
use mycelium_sdk::task::encode_task_by_citation as _;
use mycelium_sdk::task::encode_task_request as _;
use mycelium_sdk::task::Modes as _;
use mycelium_sdk::task::SolveRefusal as _;
use mycelium_sdk::task::SolveVerdictRead as _;
use mycelium_sdk::task::TaskFamily as _;
use mycelium_sdk::task::SOLVE_BOUNTY as _;
use mycelium_sdk::task::SOLVE_AGAIN as _;
use mycelium_sdk::task::SOLVE_NOTHING as _;
use mycelium_sdk::task::SOLVE_UNREAD as _;

// rule
use mycelium_sdk::rule::extract_rule as _;
use mycelium_sdk::rule::extraction_spend as _;
use mycelium_sdk::rule::perceive_family as _;
use mycelium_sdk::rule::Frames as _;
use mycelium_sdk::rule::mint_sentence_why as _;
use mycelium_sdk::rule::sentence as _;
use mycelium_sdk::rule::speak_rule as _;
use mycelium_sdk::rule::About as _;
use mycelium_sdk::rule::Act as _;
use mycelium_sdk::rule::Budget as _;
use mycelium_sdk::rule::Clause as _;
use mycelium_sdk::rule::Comparison as _;
use mycelium_sdk::rule::Coord as _;
use mycelium_sdk::rule::Create as _;
use mycelium_sdk::rule::Crystal as _;
use mycelium_sdk::rule::Element as _;
use mycelium_sdk::rule::Extent as _;
use mycelium_sdk::rule::Mv as _;
use mycelium_sdk::rule::NoCrystal as _;
use mycelium_sdk::rule::NoRule as _;
use mycelium_sdk::rule::ObstructionRefusal as _;
use mycelium_sdk::rule::Order as _;
use mycelium_sdk::rule::Paint as _;
use mycelium_sdk::rule::Physics as _;
use mycelium_sdk::rule::Pred as _;
use mycelium_sdk::rule::RegionKind as _;
use mycelium_sdk::rule::SentenceRule as _;
use mycelium_sdk::rule::Speech as _;
use mycelium_sdk::rule::Step as _;
use mycelium_sdk::rule::Stop as _;
use mycelium_sdk::rule::Symmetry as _;
use mycelium_sdk::rule::Tie as _;
use mycelium_sdk::rule::TrainPair as _;
use mycelium_sdk::rule::Unstated as _;
use mycelium_sdk::rule::Value as _;

// seat
use mycelium_sdk::seat::Arg as _;
use mycelium_sdk::seat::Call as _;
use mycelium_sdk::seat::HouseMail as _;
use mycelium_sdk::seat::Mail as _;
use mycelium_sdk::seat::MailRead as _;
use mycelium_sdk::seat::MindMail as _;
use mycelium_sdk::seat::MindRead as _;
use mycelium_sdk::seat::Outcome as _;
use mycelium_sdk::seat::Percept as _;
use mycelium_sdk::seat::Post as _;
use mycelium_sdk::seat::Record as _;
use mycelium_sdk::seat::Refusal as _;
use mycelium_sdk::seat::Role as _;
use mycelium_sdk::seat::Seat as _;
use mycelium_sdk::seat::SeatFn as _;
use mycelium_sdk::seat::SeatName as _;
use mycelium_sdk::seat::Seats as _;
use mycelium_sdk::seat::SignedPost as _;
use mycelium_sdk::seat::Signer as _;
use mycelium_sdk::seat::Take as _;
use mycelium_sdk::seat::Taken as _;
use mycelium_sdk::seat::REASON_BAD_ARGUMENTS as _;
use mycelium_sdk::seat::REASON_NOT_APPLIED as _;
use mycelium_sdk::seat::REASON_NOT_THE_HOLDER as _;
use mycelium_sdk::seat::REASON_NO_SUCH_FUNCTION as _;

// wallet
use mycelium_sdk::wallet::decode_lease_reply as _;
use mycelium_sdk::wallet::decode_stake_reply as _;
use mycelium_sdk::wallet::decode_wallet_reply as _;
use mycelium_sdk::wallet::encode_close as _;
use mycelium_sdk::wallet::encode_lease as _;
use mycelium_sdk::wallet::encode_stake as _;
use mycelium_sdk::wallet::encode_wallet as _;
use mycelium_sdk::wallet::Account as _;
use mycelium_sdk::wallet::CloseLease as _;
use mycelium_sdk::wallet::LeaseAct as _;
use mycelium_sdk::wallet::LeaseRefusal as _;
use mycelium_sdk::wallet::LeaseReply as _;
use mycelium_sdk::wallet::SignedClose as _;
use mycelium_sdk::wallet::SignedLease as _;
use mycelium_sdk::wallet::SignedStake as _;
use mycelium_sdk::wallet::StakeAct as _;
use mycelium_sdk::wallet::StakeRefusal as _;
use mycelium_sdk::wallet::Wallet as _;

// readings
use mycelium_sdk::readings::decode_lexicon_reply as _;
use mycelium_sdk::readings::decode_parameters_reply as _;
use mycelium_sdk::readings::decode_receipts_reply as _;
use mycelium_sdk::readings::decode_reply as _;
use mycelium_sdk::readings::decode_entries_reply as _;
use mycelium_sdk::readings::decode_why_reply as _;
use mycelium_sdk::readings::encode_lexicon_request as _;
use mycelium_sdk::readings::encode_parameters_request as _;
use mycelium_sdk::readings::encode_query as _;
use mycelium_sdk::readings::encode_receipts_request as _;
use mycelium_sdk::readings::encode_entries_request as _;
use mycelium_sdk::readings::encode_why as _;
use mycelium_sdk::readings::KeyedReading as _;
use mycelium_sdk::readings::Place as _;
use mycelium_sdk::readings::ProbeReading as _;
use mycelium_sdk::readings::Reading as _;
use mycelium_sdk::readings::ReserveReading as _;
use mycelium_sdk::readings::ServedTerm as _;
use mycelium_sdk::readings::Vocabulary as _;
use mycelium_sdk::readings::Why as _;
use mycelium_sdk::readings::WhyAsk as _;

// laws
use mycelium_sdk::laws::decode_register_reply as _;
use mycelium_sdk::laws::CLAUSES as _;
use mycelium_sdk::laws::MODE_CONCORDANCE as _;
use mycelium_sdk::laws::MODE_SIGNATURE as _;
use mycelium_sdk::laws::encode_register_request as _;
use mycelium_sdk::laws::law_at as _;
use mycelium_sdk::laws::Law as _;
use mycelium_sdk::laws::Served as _;
use mycelium_sdk::laws::FORM_LAW_IDS as _;
use mycelium_sdk::laws::LAWS as _;
use mycelium_sdk::laws::LAW_VERSION as _;
use mycelium_sdk::laws::MODE_CLOCK as _;
use mycelium_sdk::laws::MODE_LEXICON as _;
use mycelium_sdk::laws::MODE_PARAMETERS as _;
use mycelium_sdk::laws::MODE_REGISTER as _;
use mycelium_sdk::laws::MODE_STATEMENT as _;
use mycelium_sdk::laws::PARAMETERS as _;

// conformance
use mycelium_sdk::conformance::check_codecs as _;
use mycelium_sdk::conformance::decode_statement_reply as _;
use mycelium_sdk::conformance::encode_statement as _;
use mycelium_sdk::conformance::encode_statement_request as _;
use mycelium_sdk::conformance::run as _;
use mycelium_sdk::conformance::ActRead as _;
use mycelium_sdk::conformance::ActRequirement as _;
use mycelium_sdk::conformance::Finding as _;
use mycelium_sdk::conformance::DEFERRAL as _;
use mycelium_sdk::conformance::Need as _;
use mycelium_sdk::conformance::Profile as _;
use mycelium_sdk::conformance::ProfileRead as _;
use mycelium_sdk::conformance::ProfileStatement as _;
use mycelium_sdk::conformance::Report as _;
use mycelium_sdk::conformance::Statement as _;
use mycelium_sdk::conformance::StatementRead as _;
use mycelium_sdk::conformance::FORM_PROFILES as _;
use mycelium_sdk::conformance::STATEMENT as _;

// house: the Concordance
use mycelium_sdk::house::decode_concordance_reply as _;
use mycelium_sdk::house::decode_signature_reply as _;
use mycelium_sdk::house::encode_concordance_request as _;
use mycelium_sdk::house::encode_signature_request as _;
use mycelium_sdk::house::reconstruct_fixture as _;
use mycelium_sdk::house::sign_concordance as _;
use mycelium_sdk::house::Admitted as _;
use mycelium_sdk::house::Assent as _;
use mycelium_sdk::house::Choice as _;
use mycelium_sdk::house::ConcordanceClause as _;
use mycelium_sdk::house::ConcordanceClauseId as _;
use mycelium_sdk::house::Fixture as _;
use mycelium_sdk::house::Object as _;
use mycelium_sdk::house::SignedClause as _;
use mycelium_sdk::house::Signing as _;
use mycelium_sdk::house::SigningRefusal as _;
use mycelium_sdk::house::Standing as _;

// signing: the kernel's helper
use mycelium_sdk::signing::reconstruct_fixture as _;
use mycelium_sdk::signing::sign_concordance as _;
use mycelium_sdk::signing::RegisterRefusal as _;

// house: the clock
use mycelium_sdk::house::decode_clock_reply as _;
use mycelium_sdk::house::encode_clock_request as _;
use mycelium_sdk::house::tuniversal::Origin as _;
use mycelium_sdk::house::tuniversal::TPoint as _;
use mycelium_sdk::house::CoreStamp as _;
use mycelium_sdk::house::Stamp as _;

// house: verification (L2)
use mycelium_sdk::house::carried_body as _;
use mycelium_sdk::house::decode_batch_reply as _;
use mycelium_sdk::house::decode_laws_request as _;
use mycelium_sdk::house::decode_verify_reply as _;
use mycelium_sdk::house::encode_batch_request as _;
use mycelium_sdk::house::encode_verify_request as _;
use mycelium_sdk::house::put_transition as _;
use mycelium_sdk::house::rotator as _;
use mycelium_sdk::house::membrane_at as _;
use mycelium_sdk::house::take_transition as _;
use mycelium_sdk::house::take_verdict as _;
use mycelium_sdk::house::translator_at as _;
use mycelium_sdk::house::verify as _;
use mycelium_sdk::house::verify_sized as _;
use mycelium_sdk::house::BodyMoments as _;
use mycelium_sdk::house::BodyState as _;
use mycelium_sdk::house::Breach as _;
use mycelium_sdk::house::Flow as _;
use mycelium_sdk::house::Flux as _;
use mycelium_sdk::house::LawsRequest as _;
use mycelium_sdk::house::Transition as _;
use mycelium_sdk::house::TransitionVerdict as _;
use mycelium_sdk::house::Undecomposed as _;
use mycelium_sdk::house::Unmeasured as _;
use mycelium_sdk::house::MODE_BATCH as _;
use mycelium_sdk::house::MODE_VERIFY as _;

// house: domain founding, the reserve, the bond, the origin
use mycelium_sdk::house::decode_bond_reading as _;
use mycelium_sdk::house::decode_domain as _;
use mycelium_sdk::house::decode_domain_reply as _;
use mycelium_sdk::house::decode_domain_request as _;
use mycelium_sdk::house::decode_origin_reading as _;
use mycelium_sdk::house::decode_reserve_reading as _;
use mycelium_sdk::house::encode_bond as _;
use mycelium_sdk::house::encode_bond_read as _;
use mycelium_sdk::house::encode_domain as _;
use mycelium_sdk::house::encode_origin as _;
use mycelium_sdk::house::encode_origin_read as _;
use mycelium_sdk::house::encode_reservation as _;
use mycelium_sdk::house::encode_reserve_read as _;
use mycelium_sdk::house::Bond as _;
use mycelium_sdk::house::DomainGenesis as _;
use mycelium_sdk::house::DomainRequest as _;
use mycelium_sdk::house::Founded as _;
use mycelium_sdk::house::BigBangRecord as _;
use mycelium_sdk::house::OriginReading as _;
use mycelium_sdk::house::Reservation as _;
use mycelium_sdk::house::SignedBond as _;
use mycelium_sdk::house::SignedDomain as _;
use mycelium_sdk::house::SignedOrigin as _;
use mycelium_sdk::house::SignedReservation as _;
use mycelium_sdk::house::domain::decode_domain_request as _;
use mycelium_sdk::house::domain::DomainGenesis as _;
use mycelium_sdk::house::domain::DomainRequest as _;
use mycelium_sdk::house::domain::Founded as _;
use mycelium_sdk::house::domain::BigBangRecord as _;
use mycelium_sdk::house::domain::Reservation as _;
use mycelium_sdk::house::domain::SignedDomain as _;
use mycelium_sdk::house::domain::SignedOrigin as _;
use mycelium_sdk::house::domain::SignedReservation as _;

// house: the universe
use mycelium_sdk::house::basket_holds as _;
use mycelium_sdk::house::decode_universe_reading as _;
use mycelium_sdk::house::encode_universe_read as _;
use mycelium_sdk::house::encode_universe_statement as _;
use mycelium_sdk::house::Basket as _;
use mycelium_sdk::house::Genesis as _;
use mycelium_sdk::house::Geometry as _;
use mycelium_sdk::house::Metric as _;
use mycelium_sdk::house::Parent as _;
use mycelium_sdk::house::SignedUniverseStatement as _;
use mycelium_sdk::house::Stated as _;
use mycelium_sdk::house::UniverseReading as _;
use mycelium_sdk::house::UniverseStatement as _;
use mycelium_sdk::house::universe::Genesis as _;
use mycelium_sdk::house::universe::Geometry as _;
use mycelium_sdk::house::universe::Parent as _;

// house: the endowment
use mycelium_sdk::house::decode_endowment_reading as _;
use mycelium_sdk::house::decode_grant_reply as _;
use mycelium_sdk::house::encode_endowment_read as _;
use mycelium_sdk::house::encode_manifest as _;
use mycelium_sdk::house::nuclei_hold as _;
use mycelium_sdk::house::BigUint as _;
use mycelium_sdk::house::EndowmentReading as _;
use mycelium_sdk::house::GrantRead as _;
use mycelium_sdk::house::GrantRefusal as _;
use mycelium_sdk::house::Granted as _;
use mycelium_sdk::house::Manifest as _;
use mycelium_sdk::house::Nuclei as _;
use mycelium_sdk::house::Nucleus as _;
use mycelium_sdk::house::SignedManifest as _;

// house: seats, the house's side
use mycelium_sdk::house::decode_listing_reply as _;
use mycelium_sdk::house::decode_mail_reply_standing as _;
use mycelium_sdk::house::decode_post_reply as _;
use mycelium_sdk::house::decode_seat_request as _;
use mycelium_sdk::house::encode_post as _;
use mycelium_sdk::house::encode_read_listing as _;
use mycelium_sdk::house::encode_read_mail as _;
use mycelium_sdk::house::put_args as _;
use mycelium_sdk::house::take_args as _;
use mycelium_sdk::house::Arg as _;
use mycelium_sdk::house::Call as _;
use mycelium_sdk::house::HouseMail as _;
use mycelium_sdk::house::Mail as _;
use mycelium_sdk::house::MailRead as _;
use mycelium_sdk::house::Outcome as _;
use mycelium_sdk::house::Percept as _;
use mycelium_sdk::house::Post as _;
use mycelium_sdk::house::ReadMail as _;
use mycelium_sdk::house::Record as _;
use mycelium_sdk::house::Refusal as _;
use mycelium_sdk::house::Role as _;
use mycelium_sdk::house::Seat as _;
use mycelium_sdk::house::SeatFn as _;
use mycelium_sdk::house::SeatRequest as _;
use mycelium_sdk::house::Seats as _;
use mycelium_sdk::house::SignedPost as _;
use mycelium_sdk::house::Signer as _;
use mycelium_sdk::house::Take as _;
use mycelium_sdk::house::Taken as _;
use mycelium_sdk::house::FN_DECLINE as _;
use mycelium_sdk::house::FN_SOLVE as _;
use mycelium_sdk::house::KIND_COUNSEL as _;
use mycelium_sdk::house::KIND_TASK as _;
use mycelium_sdk::house::REASON_BAD_ARGUMENTS as _;
use mycelium_sdk::house::REASON_NOT_APPLIED as _;
use mycelium_sdk::house::REASON_NOT_THE_HOLDER as _;
use mycelium_sdk::house::REASON_NO_SUCH_FUNCTION as _;
use mycelium_sdk::house::TAG_COUNT as _;
use mycelium_sdk::house::TAG_FLAG as _;
use mycelium_sdk::house::seat::Arg as _;
use mycelium_sdk::house::seat::Call as _;
use mycelium_sdk::house::seat::MailRead as _;
use mycelium_sdk::house::seat::Outcome as _;
use mycelium_sdk::house::seat::Percept as _;
use mycelium_sdk::house::seat::Post as _;
use mycelium_sdk::house::seat::ReadMail as _;
use mycelium_sdk::house::seat::Record as _;
use mycelium_sdk::house::seat::Refusal as _;
use mycelium_sdk::house::seat::Role as _;
use mycelium_sdk::house::seat::Seat as _;
use mycelium_sdk::house::seat::SeatFn as _;
use mycelium_sdk::house::seat::Seats as _;
use mycelium_sdk::house::seat::SignedPost as _;
use mycelium_sdk::house::seat::Take as _;
use mycelium_sdk::house::seat::Taken as _;

// house: the atlas, a tier's fold, its challenges, a rows commitment
use mycelium_sdk::house::atlas::decode_ledger_reply as _;
use mycelium_sdk::house::atlas::decode_map_reply as _;
use mycelium_sdk::house::atlas::encode_ledger as _;
use mycelium_sdk::house::atlas::encode_map as _;
use mycelium_sdk::house::atlas::LedgerEntry as _;
use mycelium_sdk::house::atlas::Ledgered as _;
use mycelium_sdk::house::atlas::MapAct as _;
use mycelium_sdk::house::atlas::MapRecord as _;
use mycelium_sdk::house::atlas::MapRefusal as _;
use mycelium_sdk::house::atlas::Mapped as _;
use mycelium_sdk::house::challenge::line_head as _;
use mycelium_sdk::house::challenge::Challenge as _;
use mycelium_sdk::house::challenge::ChallengeRefusal as _;
use mycelium_sdk::house::challenge::Challenged as _;
use mycelium_sdk::house::challenge::Entry as _;
use mycelium_sdk::house::challenge::Finality as _;
use mycelium_sdk::house::challenge::FinalityReading as _;
use mycelium_sdk::house::challenge::LineHead as _;
use mycelium_sdk::house::challenge::SignedChallenge as _;
use mycelium_sdk::house::commitment::commitment as _;
use mycelium_sdk::house::uplink::Discontinuous as _;
use mycelium_sdk::house::uplink::FoldAt as _;
use mycelium_sdk::house::uplink::Net as _;
use mycelium_sdk::house::uplink::SignedFold as _;
use mycelium_sdk::house::uplink::TierFold as _;
use mycelium_sdk::house::uplink::Uplinked as _;

// house: handoff, traverse, the ledger and map, the challenge window's
// reads, the fold sent up, the equivocation proof
use mycelium_sdk::house::decode_handoff_reply as _;
use mycelium_sdk::house::decode_orbits_reply as _;
use mycelium_sdk::house::decode_read_reply as _;
use mycelium_sdk::house::decode_tube_reading as _;
use mycelium_sdk::house::encode_handoff as _;
use mycelium_sdk::house::encode_handoff_read as _;
use mycelium_sdk::house::encode_orbits_read as _;
use mycelium_sdk::house::encode_tube_read as _;
use mycelium_sdk::house::ConvertIn as _;
use mycelium_sdk::house::ConvertOut as _;
use mycelium_sdk::house::Handed as _;
use mycelium_sdk::house::Handoff as _;
use mycelium_sdk::house::HandoffOffer as _;
use mycelium_sdk::house::HandoffRefusal as _;
use mycelium_sdk::house::HandoffState as _;
use mycelium_sdk::house::MouthOrbit as _;
use mycelium_sdk::house::Mouth as _;
use mycelium_sdk::house::SignedHandoff as _;
use mycelium_sdk::house::Tariff as _;
use mycelium_sdk::house::Tube as _;
use mycelium_sdk::house::TubeReading as _;
use mycelium_sdk::house::decode_traverse_reply as _;
use mycelium_sdk::house::encode_traverse as _;
use mycelium_sdk::house::CrossRefusal as _;
use mycelium_sdk::house::Crossed as _;
use mycelium_sdk::house::Declaration as _;
use mycelium_sdk::house::Gas as _;
use mycelium_sdk::house::SignedTraverse as _;
use mycelium_sdk::house::Traverse as _;
use mycelium_sdk::house::decode_ledger_page as _;
use mycelium_sdk::house::decode_ledger_reply as _;
use mycelium_sdk::house::decode_map_reading as _;
use mycelium_sdk::house::decode_map_reply as _;
use mycelium_sdk::house::encode_ledger as _;
use mycelium_sdk::house::encode_ledger_read as _;
use mycelium_sdk::house::encode_map as _;
use mycelium_sdk::house::encode_map_read as _;
use mycelium_sdk::house::LedgerEntry as _;
use mycelium_sdk::house::LedgerPage as _;
use mycelium_sdk::house::LedgerRefusal as _;
use mycelium_sdk::house::Ledgered as _;
use mycelium_sdk::house::MapAct as _;
use mycelium_sdk::house::MapEntry as _;
use mycelium_sdk::house::MapRecord as _;
use mycelium_sdk::house::MapRefusal as _;
use mycelium_sdk::house::Mapped as _;
use mycelium_sdk::house::SignedLedger as _;
use mycelium_sdk::house::SignedMap as _;
use mycelium_sdk::house::decode_challenge_reply as _;
use mycelium_sdk::house::decode_finality_reading as _;
use mycelium_sdk::house::decode_finality_reply as _;
use mycelium_sdk::house::decode_line_head_reply as _;
use mycelium_sdk::house::decode_obstructions_reply as _;
use mycelium_sdk::house::encode_challenge as _;
use mycelium_sdk::house::encode_finality_request as _;
use mycelium_sdk::house::encode_line_head_request as _;
use mycelium_sdk::house::encode_obstructions_request as _;
use mycelium_sdk::house::Challenge as _;
use mycelium_sdk::house::ChallengeRefusal as _;
use mycelium_sdk::house::Challenged as _;
use mycelium_sdk::house::Finality as _;
use mycelium_sdk::house::FinalityReading as _;
use mycelium_sdk::house::FoldAt as _;
use mycelium_sdk::house::LineHead as _;
use mycelium_sdk::house::OpenObstruction as _;
use mycelium_sdk::house::SignedChallenge as _;
use mycelium_sdk::house::encode_uplink as _;
use mycelium_sdk::house::SignedFold as _;
use mycelium_sdk::house::TierFold as _;
use mycelium_sdk::house::decode_receipt_section as _;
use mycelium_sdk::house::decode_ruled as _;
use mycelium_sdk::house::encode_equivocation as _;
use mycelium_sdk::house::encode_receipt_section as _;
use mycelium_sdk::house::equivocation_proof as _;
use mycelium_sdk::house::ReceiptSection as _;
use mycelium_sdk::house::Ruled as _;
use mycelium_sdk::house::EQUIVOCATION_VERSION as _;

// house: framing and identity
use mycelium_sdk::house::payload_hash as _;
use mycelium_sdk::house::put_key_bytes as _;
use mycelium_sdk::house::put_key_count as _;
use mycelium_sdk::house::delimited_len as _;
use mycelium_sdk::house::write_act as _;
use mycelium_sdk::house::take_key_bytes as _;
use mycelium_sdk::house::take_key_count as _;
use mycelium_sdk::house::Delimited as _;
use mycelium_sdk::house::Expression as _;
use mycelium_sdk::house::BODY_BUDGET as _;
use mycelium_sdk::house::DELIMITED_MAX as _;
use mycelium_sdk::house::HEAD as _;

// forms
use mycelium_sdk::forms::crossed as _;
use mycelium_sdk::forms::put_address_only as _;
use mycelium_sdk::forms::verify_carried as _;
use mycelium_sdk::forms::Body as _;
use mycelium_sdk::forms::Clauses as _;
use mycelium_sdk::forms::Content as _;
use mycelium_sdk::forms::Crossing as _;
use mycelium_sdk::forms::CrossingRefusal as _;
use mycelium_sdk::forms::Motion as _;
use mycelium_sdk::forms::Place as _;
use mycelium_sdk::forms::Ring as _;
use mycelium_sdk::forms::TurnBody as _;
use mycelium_sdk::forms::shares::share_compounds as _;
use mycelium_sdk::forms::shares::share_compounds::Remaining as _;
use mycelium_sdk::forms::reference as _;
use mycelium_sdk::forms::passage as _;
use mycelium_sdk::forms::passage::pass_share as _;
use mycelium_sdk::forms::passage::Breach as _;
use mycelium_sdk::forms::passage::Crossing as _;
use mycelium_sdk::forms::passage::Frame as _;
use mycelium_sdk::forms::passage::PassageRefusal as _;
use mycelium_sdk::forms::passage::Place as _;
use mycelium_sdk::forms::passage::Pole as _;
use mycelium_sdk::forms::passage::Poles as _;
use mycelium_sdk::forms::passage::Throat as _;
use mycelium_sdk::forms::shares as _;
use mycelium_sdk::forms::LAW_IDS as _;
use mycelium_sdk::forms::Q as _;

// prelude
use mycelium_sdk::prelude::Crystal as _;
use mycelium_sdk::prelude::Element as _;
use mycelium_sdk::prelude::NoCrystal as _;
use mycelium_sdk::prelude::NoRule as _;
use mycelium_sdk::prelude::SeatName as _;
use mycelium_sdk::prelude::Session as _;
use mycelium_sdk::prelude::SigningKey as _;
use mycelium_sdk::prelude::Step as _;
use mycelium_sdk::prelude::Symmetry as _;
use mycelium_sdk::prelude::TPoint as _;
use mycelium_sdk::prelude::TaskFamily as _;
use mycelium_sdk::prelude::Voice as _;
use mycelium_sdk::prelude::VoiceKey as _;
use mycelium_sdk::prelude::SOLVE_BOUNTY as _;
use mycelium_sdk::prelude::SOLVE_AGAIN as _;
use mycelium_sdk::prelude::Mu as _;
use mycelium_sdk::prelude::Modes as _;
use mycelium_sdk::wallet::Mu as _;
use mycelium_sdk::wallet::MuRefusal as _;
use mycelium_sdk::prelude::SOLVE_NOTHING as _;
use mycelium_sdk::prelude::SOLVE_UNREAD as _;
use mycelium_sdk::prelude::WIRE_VERSION as _;

/// The surface compiled: every item above stands at its path.
#[test]
fn surface_imports_every_item() {
    assert!(!mycelium_sdk::ACT_TABLE.is_empty());
}

/// The associated functions a kernel calls stand on the types the surface
/// carries: a key loaded and never minted, a voice from its key's bytes, an
/// element read from its tape.
#[test]
fn surface_carries_associated_functions() {
    let load_only: fn(&str) -> Option<mycelium_sdk::voice::VoiceKey> = mycelium_sdk::voice::VoiceKey::load_only;
    let from_voice: fn([u8; 32]) -> mycelium_sdk::voice::Voice = mycelium_sdk::voice::Voice::from_voice;
    let from_bytes: fn(&[u8], &mut usize) -> Option<mycelium_sdk::rule::Element> = mycelium_sdk::rule::Element::from_bytes;
    assert!(load_only("").is_none());
    assert_eq!(from_voice([7; 32]).id(), [7; 32]);
    assert!(from_bytes(&[], &mut 0).is_none());
}
