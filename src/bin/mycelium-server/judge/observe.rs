// §8.1 — ACT_OBSERVE: a voice's readings held as they stand, and every
// reading this hypha holds served to whoever asks.

use crate::door::Door;
use crate::transport::answer;
use mycelium::network::wire::Frame;
use mycelium::observe::{self, Court, Memory, ObserveAct, Reading, Source};
use std::net::TcpStream;

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(act) = observe::decode_act(&frame.body) else { return };
    match act {
        // A VOICE'S TOTALS, held under the key it names. The readings are
        // an instrument and no cognition reads them, so the key is taken as
        // spoken; a voice that names another's key misreports the console
        // and nothing else.
        ObserveAct::Push(vk, readings) => {
            let held = readings.into_iter().all(|r| observe::replace(Source::Voice(vk), r));
            answer(stream, Frame { lane: frame.lane, body: vec![u8::from(held)] });
        }
        ObserveAct::Query(prefix) => {
            // THE COURT'S OWN STANDING, read at the question: what it holds
            // now, and the allocator at its own K.
            let own = Source::Court(door.id);
            // LD1 — and the bytes its awards, its receipt deltas and its
            // frames carry, measured whole at the question.
            // ACT1: the origin this court adopted, read before the kernel's
            // lock and never under it.
            let genesis = door.mesh.origin.lock().ok().and_then(|o| *o);
            let standing = door.kernel.lock().ok().map(|k| {
                let mut carriage = observe::Carriage::default();
                k.treasury().carriage(&mut carriage);
                carriage.of_chain(k.chain());
                carriage.of_frames();
                // RM1 — its remainders over its complexes, beside what the
                // torsion filing kept of them.
                let (remainders, torsion_words) = k.remainders();
                let court = Court {
                    k: k.chain().head_k(),
                    basis: k.basis_len() as u64,
                    treasury: k.treasury_entry_count() as u64,
                    served: observe::served(),
                    reopened: k.reopened(),
                    remainders,
                    torsion_words,
                };
                // ACT1 — THIS COURT ON THE CLOCK, with its three counts.
                // EC1 — and its payments by proof of work, off its journal.
                (court, carriage, k.clock(genesis), k.treasury().economy())
            });
            let standing = standing.map(|(court, carriage, clock, economy)| {
                observe::replace(own, Reading::Carriage(carriage));
                observe::replace(own, Reading::Clock(observe::Clock::Court(Box::new(clock))));
                if let Some(e) = economy {
                    observe::replace(own, Reading::Economy(e));
                }
                court
            });
            // PH5 — and its phase, the last window it wrote.
            if let Some(p) = observe::last_phase() {
                observe::replace(own, Reading::Phase(p));
            }
            if let Some(court) = standing {
                if let Some(m) = Memory::now(court.k) {
                    observe::publish(own, Reading::Memory(m));
                }
                observe::replace(own, Reading::Court(court));
            }
            // CV5: what this court did with each signer's receipts, and its
            // verdicts by kind, read at the question from the counters the
            // exchange, the back-fill and the adjudication keep.
            observe::replace(own, Reading::Replication(observe::replication()));
            observe::replace(own, Reading::Adjudication(observe::adjudication()));
            // The body's share of the frame, exact, as `rows` reads it.
            let Some(budget) = mycelium::network::wire::frame_budget().checked_sub(64) else {
                return;
            };
            let body = observe::encode_reply(&observe::readings(&prefix), budget);
            answer(stream, Frame { lane: frame.lane, body });
        }
    }
}
