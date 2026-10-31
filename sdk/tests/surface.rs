// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-PRECURSORS §4 P2 — THE SDK'S SURFACE CLAIM: every public item of the
// curated surface, imported one `use` per item. The file compiles exactly
// where every item stands at its path; an item that disappears or moves
// fails the build by name. A new item enters the surface with its line here.

#![expect(unused_imports, reason = "each import is the claim: it resolves, and is never used")]

// The crate root.
use mycelium_sdk::forms as _;
use mycelium_sdk::ACT_TABLE as _;
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
use mycelium_sdk::session::Session as _;
use mycelium_sdk::session::TPoint as _;
use mycelium_sdk::session::TransportAperture as _;
use mycelium_sdk::session::Voice as _;

// task
use mycelium_sdk::task::decode_grids as _;
use mycelium_sdk::task::decode_solve_verdict as _;
use mycelium_sdk::task::decode_task_envelope as _;
use mycelium_sdk::task::encode_grids as _;
use mycelium_sdk::task::encode_solve as _;
use mycelium_sdk::task::encode_solve_with_rule as _;
use mycelium_sdk::task::encode_task_by_citation as _;
use mycelium_sdk::task::encode_task_request as _;
use mycelium_sdk::task::Modes as _;
use mycelium_sdk::task::SolveRefusal as _;
use mycelium_sdk::task::SolveVerdictRead as _;
use mycelium_sdk::task::TaskEnvelope as _;
use mycelium_sdk::task::SOLVE_BOUNTY as _;
use mycelium_sdk::task::SOLVE_FLOOR as _;
use mycelium_sdk::task::SOLVE_NOTHING as _;
use mycelium_sdk::task::SOLVE_UNREAD as _;

// rule
use mycelium_sdk::rule::extract_rule as _;
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
use mycelium_sdk::rule::Grid as _;
use mycelium_sdk::rule::Mv as _;
use mycelium_sdk::rule::NoCrystal as _;
use mycelium_sdk::rule::NoRule as _;
use mycelium_sdk::rule::ObstructionRefusal as _;
use mycelium_sdk::rule::Order as _;
use mycelium_sdk::rule::Paint as _;
use mycelium_sdk::rule::Physics as _;
use mycelium_sdk::rule::Pred as _;
use mycelium_sdk::rule::Region as _;
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
use mycelium_sdk::readings::decode_clock_reply as _;
use mycelium_sdk::readings::decode_lexicon_reply as _;
use mycelium_sdk::readings::decode_parameters_reply as _;
use mycelium_sdk::readings::decode_receipts_reply as _;
use mycelium_sdk::readings::decode_reply as _;
use mycelium_sdk::readings::decode_rows_reply as _;
use mycelium_sdk::readings::decode_why_reply as _;
use mycelium_sdk::readings::encode_clock_request as _;
use mycelium_sdk::readings::encode_lexicon_request as _;
use mycelium_sdk::readings::encode_parameters_request as _;
use mycelium_sdk::readings::encode_query as _;
use mycelium_sdk::readings::encode_receipts_request as _;
use mycelium_sdk::readings::encode_rows_request as _;
use mycelium_sdk::readings::encode_why as _;
use mycelium_sdk::readings::Held as _;
use mycelium_sdk::readings::Place as _;
use mycelium_sdk::readings::ProbeReading as _;
use mycelium_sdk::readings::Reading as _;
use mycelium_sdk::readings::ReserveReading as _;
use mycelium_sdk::readings::ServedTerm as _;
use mycelium_sdk::readings::Why as _;
use mycelium_sdk::readings::WhyAsk as _;

// mind
use mycelium_sdk::mind::GenomeReading as _;
use mycelium_sdk::mind::Mind as _;

// wire
use mycelium_sdk::wire::ACT_TABLE as _;
use mycelium_sdk::wire::WIRE_VERSION as _;

// laws
use mycelium_sdk::laws::decode_concordance_reply as _;
use mycelium_sdk::laws::decode_register_reply as _;
use mycelium_sdk::laws::decode_signature_reply as _;
use mycelium_sdk::laws::encode_concordance_request as _;
use mycelium_sdk::laws::encode_signature_request as _;
use mycelium_sdk::laws::verify_concordance_signature as _;
use mycelium_sdk::laws::ConcordanceRefusal as _;
use mycelium_sdk::laws::ConcordanceSignature as _;
use mycelium_sdk::laws::CONCORDANCE as _;
use mycelium_sdk::laws::CONCORDANCE_HASH as _;
use mycelium_sdk::laws::MODE_CONCORDANCE as _;
use mycelium_sdk::laws::MODE_SIGNATURE as _;
use mycelium_sdk::laws::SIGNATURE_LEN as _;
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
use mycelium_sdk::conformance::Need as _;
use mycelium_sdk::conformance::Report as _;
use mycelium_sdk::conformance::Statement as _;
use mycelium_sdk::conformance::StatementRead as _;
use mycelium_sdk::conformance::STATEMENT as _;

// forms
use mycelium_sdk::forms::frame as _;
use mycelium_sdk::forms::shares as _;
use mycelium_sdk::forms::LAW_IDS as _;
use mycelium_sdk::forms::Q as _;

// prelude
use mycelium_sdk::prelude::Crystal as _;
use mycelium_sdk::prelude::Element as _;
use mycelium_sdk::prelude::Grid as _;
use mycelium_sdk::prelude::NoCrystal as _;
use mycelium_sdk::prelude::NoRule as _;
use mycelium_sdk::prelude::SeatName as _;
use mycelium_sdk::prelude::Session as _;
use mycelium_sdk::prelude::SigningKey as _;
use mycelium_sdk::prelude::Step as _;
use mycelium_sdk::prelude::Symmetry as _;
use mycelium_sdk::prelude::TPoint as _;
use mycelium_sdk::prelude::TaskEnvelope as _;
use mycelium_sdk::prelude::Voice as _;
use mycelium_sdk::prelude::VoiceKey as _;
use mycelium_sdk::prelude::SOLVE_BOUNTY as _;
use mycelium_sdk::prelude::SOLVE_FLOOR as _;
use mycelium_sdk::prelude::SOLVE_NOTHING as _;
use mycelium_sdk::prelude::SOLVE_UNREAD as _;
use mycelium_sdk::prelude::WIRE_VERSION as _;

/// The surface compiled: every item above stands at its path.
#[test]
fn surface_imports_every_item() {
    assert!(!mycelium_sdk::ACT_TABLE.is_empty());
}
