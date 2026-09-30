// §8.1 — ACT_OBSERVE: a voice's readings held as they stand, and every
// reading this hypha holds served to whoever asks.

use crate::door::Door;
use crate::transport::answer;
use mycelium::network::wire::Frame;
use mycelium::observe::{self, Court, Memory, ObserveAct, Reading, Source};
use std::net::TcpStream;

/// A question the observer could not answer because the kernel was held:
/// the court stroke takes it (`answer_standing`).
static STANDING_ASKED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The court stroke's side: where the observer left a question, the
/// standing is published under the lock the stroke already holds.
pub(crate) fn answer_standing(k: &mycelium::cognition::Kernel, mesh: &crate::exchange::Mesh, id: [u8; 32]) {
    if STANDING_ASKED.swap(false, std::sync::atomic::Ordering::Relaxed) {
        publish_standing(k, mesh, Source::Court(id));
    }
}

/// THE COURT'S OWN STANDING, read off the kernel: what it holds, the
/// allocator at its own K, its clock, economy, acts and carriage.
fn publish_standing(k: &mycelium::cognition::Kernel, mesh: &crate::exchange::Mesh, own: Source) {
    // ACT1: the origin this court adopted.
    let genesis = mesh.origin.lock().ok().and_then(|o| *o);
    // LD1 — the bytes its awards, its receipt deltas and its frames carry.
    let mut carriage = observe::Carriage::default();
    k.treasury().carriage(&mut carriage);
    carriage.of_chain(k.chain());
    carriage.of_frames();
    // RM1 — its remainders over its complexes, beside what the torsion
    // filing kept of them.
    let (remainders, torsion_words) = k.remainders();
    let court = Court {
        k: k.chain().head_k(),
        basis: k.basis_len() as u64,
        treasury: k.treasury_entry_count() as u64,
        served: observe::served(),
        reopened: k.reopened(),
        remainders,
        torsion_words,
        room_broken: k.room_refusals().0,
        room_overflowed: k.room_refusals().1,
        overflowed: k.overflowed(),
        folds: observe::folds(),
        catchup: observe::catchups(),
    };
    // ACT1 — this court on the clock; EC1 — its payments by proof of work;
    // and its acts and metabolism.
    observe::replace(own, Reading::Acts(Box::new(k.acts_reading())));
    observe::replace(own, Reading::Carriage(carriage));
    observe::replace(own, Reading::Clock(observe::Clock::Court(Box::new(k.clock(genesis)))));
    if let Some(e) = k.economy() {
        observe::replace(own, Reading::Economy(Box::new(e)));
    }
    if let Some(m) = Memory::now(court.k) {
        observe::publish(own, Reading::Memory(m));
    }
    observe::replace(own, Reading::Court(court));
}

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
            // THE OBSERVER NEVER WAITS ON COGNITION. The standing is read
            // here where the kernel is free; where a stroke holds it, the
            // question is LEFT for the stroke, which publishes the standing
            // at its end under the lock it already holds (Phase 1 item 2).
            // A busy court read "0 readings" for whole boots before, because
            // the try below never found the kernel free.
            match door.kernel.try_lock() {
                Ok(k) => publish_standing(&k, &door.mesh, own),
                Err(_) => STANDING_ASKED.store(true, std::sync::atomic::Ordering::Relaxed),
            }
            // PH5 — and its phase, the last window it wrote.
            if let Some(p) = observe::last_phase() {
                observe::replace(own, Reading::Phase(p));
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
            let Some(body) = observe::encode_reply(&observe::readings(&prefix), budget) else {
                eprintln!("  observe reply refused: more readings held than four bytes count");
                return;
            };
            answer(stream, Frame { lane: frame.lane, body });
        }
    }
}
