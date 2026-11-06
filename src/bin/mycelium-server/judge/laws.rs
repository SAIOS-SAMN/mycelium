// 78.62 steps 2 and 3 — ACT_LAWS: the universal law register served to any
// kernel that asks, and a transition a kernel composed verified against it.
//
//! The court does not simulate: it reconstructs the transition against L0
//! (`physics::laws::verify`) and answers every breach with its law. Nothing is
//! admitted, no receipt is written and no book moves — the verdict is a
//! reading the kernel acts on. The verifier runs under a stroke of its own,
//! as the observer's census does: the court's metabolism is the acts it
//! admits, and this admits none.
//!
//! Track 24 B2 — two modes take the court: a signing act (mode 8) is
//! recorded on the court's line, as a stake act on `ACT_WALLET` is a write
//! (`Court::hear_signing`), answered once durable; and the alignment reading
//! (mode 9) is derived from the signing events the court's ledger holds,
//! under the hold's stroke (`Court::alignment`).

use crate::context::Context;
use crate::transport::answer;
use mycelium::network::wire::WireFrame;
use mycelium::physics::laws::codec::{decode_laws_request, encode_register_reply, LawsRequest};
use mycelium::network::channel::Channel;

pub(crate) fn judge(stream: &mut Channel, frame: &WireFrame, context: &Context) {
    let Some(request) = decode_laws_request(&frame.body) else { return };
    let body = match request {
        LawsRequest::Register => encode_register_reply(),
        // 78.124 — the core clock: this court's head stamp, read at the ask.
        // 78.151 — the core clock as the court stroke last published it: the
        // stamp is its own height.
        LawsRequest::Clock => {
            mycelium::observe::note_serve(mycelium::observe::ServeAxis::ReadsPublished, 1);
            mycelium::physics::laws::codec::encode_clock_reply(context.published.take().clock())
        }
        // 78.125 — the register's parameters (the challenge window), at its version.
        LawsRequest::Parameters => mycelium::physics::laws::codec::encode_parameters_reply(),
        // 78.122 — the shared lexicon, at its version.
        LawsRequest::Lexicon => mycelium::physics::laws::codec::encode_lexicon_reply(),
        // MU-PRECURSORS §4 P5 — the client conformance statement this tree
        // publishes, the same object the SDK holds.
        LawsRequest::Statement => mycelium::anastome::dialogue::conformance::encode_statement_reply(),
        // Concordance §0 — the clause register.
        LawsRequest::Concordance => mycelium::physics::laws::codec::encode_concordance_reply(&mycelium::physics::laws::clauses::register()),
        // Concordance §0, track 24 B2 — a kernel's signing act admitted only
        // on its reconstruction of each clause's fixtures, admitted or
        // refused by name, and the verdict sealed on the court's line; the
        // reply waits until it is durable (78.137).
        LawsRequest::Signature(b) => {
            let verdict = match crate::court_lock::held!(context.court) {
                Ok(mut h) => h.run(|k, _| k.hear_signing(&b)),
                Err(_) => return,
            };
            if !crate::persist::durable(&context.court, &context.state_path) {
                return;
            }
            mycelium::physics::laws::codec::encode_signature_reply(&verdict)
        }
        // Track 24 B2 — the alignment reading, from the signing events the
        // court's ledger holds.
        LawsRequest::Alignment(only) => match crate::court_lock::held!(context.court) {
            Ok(mut h) => mycelium::physics::laws::codec::encode_alignment_reply(&h.run(|k, reads| k.alignment(only, reads))),
            Err(_) => return,
        },
        LawsRequest::Verify(t) => {
            // 78.116 — under the stroke the motion's modes need, never the
            // court's default: a verdict of the transition alone.
            mycelium::physics::laws::codec::encode_verify_reply(&mycelium::physics::laws::verify_sized(&t))
        }
        // 78.118 — each transition of the batch, in the order sent.
        LawsRequest::Batch(ts) => {
            let verdicts: Vec<_> = ts.iter().map(mycelium::physics::laws::verify_sized).collect();
            mycelium::physics::laws::codec::encode_batch_reply(&verdicts)
        }
    };
    answer(stream, WireFrame { lane: frame.lane, body });
}
