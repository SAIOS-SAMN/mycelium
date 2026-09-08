// 26 — ACT_SPEECH: a voice stands sentences at their home court.
//
//! The speaker's words are decoded, the book perceives them, and each one
//! that stood is named in the reply by its own code (SPEECH_STOOD and its
//! siblings), never by a count.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::network::wire::Frame;
use std::net::TcpStream;


/// A voice speaks sentences to the court that homes them (26).
pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let kernel = &door.kernel;

    let Some(dialogue::SpeechAct { voice: svoice, task, sentences: blobs, .. }) =
        dialogue::decode_speech(&frame.body)
    else {
        return;
    };
    let mut sentences = Vec::new();
    for b in &blobs {
        let mut p = 0usize;
        if let Some(e) = mycelium::transducer::element::Element::from_bytes(b, &mut p) {
            sentences.push(e);
        }
    }
    let codes = match kernel.lock() {
        Ok(mut k) => k.perceive_speech(svoice, &task, &sentences),
        Err(_) => return,
    };
    for (i, e) in sentences.iter().enumerate() {
        if codes.get(i).copied() == Some(dialogue::SPEECH_STOOD) {
            eprintln!(
                "  speech stood {:02x}{:02x}… sectioned at home",
                e.address()[0],
                e.address()[1]
            );
        }
    }
    let body = dialogue::encode_speech_reply(&codes);
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
}
