// 78.145 — ACT_SEAT: the seat relay. A post is checked for its signer and
// held or filed (`Kernel::seat_post`), answered `[1][seq]` or the refusal; a
// listing is read; a reader's mail is read past what it acknowledges.
//
//! A relay, not a court act: nothing is sealed and nothing is persisted, so
//! the answer waits on no durability.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::seat::{decode_seat_request, encode_mail_reply, encode_mail_reply_standing, encode_post_reply, MailRead, ReadMail, Refusal, SeatRequest};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

/// A READ'S REPLY in the layout its sub-kind asked: v1.4's (sub-kind 8) with
/// the relay's epoch and last sequence, v1.3's (7) the pieces alone.
fn mail_reply(r: &ReadMail, read: Result<MailRead, Refusal>) -> Option<Vec<u8>> {
    match r.standing {
        Some(_) => encode_mail_reply_standing(read.as_ref().map_err(|why| *why)),
        None => encode_mail_reply(read.as_ref().map(|m| m.mail.as_slice()).map_err(|why| *why)),
    }
}

/// 78.146 — A MIND'S READ, SERVED WHERE THE DOOR WOULD DEFER IT: a full
/// worker pool answers every act with the door's deficit, and reading one's
/// own mail spends nothing — so a READ-MAIL is served off the relay's own
/// lock (no kernel), once per window of the reader's mail
/// (`SharedRelay::exempt_read`). `false` where the frame is no read, or the
/// read is not exempt: the caller defers it as before. A task read, a
/// wallet read and a seat post are served here too (2026-10-09).
pub(crate) fn serve_exempt(stream: &mut TcpStream, frame: &Frame, door: &Door) -> bool {
    // A TASK READ takes the registry's lock and never the kernel's, so a
    // full pool waiting on cognition has no reason to defer it: a seated
    // mind's fetch of the task before its seat was deferred 2026-10-08 on
    // every hypha and no call was made. Served here, as observe is.
    if mycelium::anastome::dialogue::act_tag(&frame.body) == Some(mycelium::anastome::dialogue::ACT_TASK) {
        return crate::door::serve_control(stream, frame, door);
    }
    // A wallet read and a seat listing are answered off the published
    // reading before this (`judge::serve_published`, 78.151).
    // A SEAT POST (TAKE, CALL) is a mind's one act at its seat: deferred
    // after every restart, minds read their percepts and never held their
    // seats again (2026-10-09). Served here the same way, in its turn.
    if let Some(Ok(SeatRequest::Post(_))) = decode_seat_request(&frame.body) {
        judge(stream, frame, door);
        return true;
    }
    let Some(Ok(SeatRequest::ReadMail(r))) = decode_seat_request(&frame.body) else { return false };
    let Some(mail) = door.seat_relay.exempt_read(&r) else { return false };
    let Some(body) = mail_reply(&r, mail) else { return false };
    answer(stream, Frame { lane: frame.lane, body });
    true
}

/// 78.151 — A SEAT LISTING, read on the relay's own lock: relay state, never
/// the court's, so no kernel is taken. `false` where the frame is no listing.
/// v1.4: at the sub-origin it names, where it names one.
pub(crate) fn serve_listing(stream: &mut TcpStream, frame: &Frame, door: &Door) -> bool {
    let Some(Ok(SeatRequest::ReadListing(domain, sub_origin))) = decode_seat_request(&frame.body) else { return false };
    // A domain with no listing is answered with none: an empty body.
    match door.seat_relay.listing(&domain, sub_origin.as_ref()) {
        Some(body) => answer(stream, Frame { lane: frame.lane, body }),
        None => answer(stream, Frame { lane: frame.lane, body: Vec::new() }),
    }
    true
}

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(request) = decode_seat_request(&frame.body) else { return };
    let body = match request {
        Err(why) => {
            // A frame that names no domain: counted on the relay's reading.
            door.seat_relay.note_unread();
            encode_post_reply(Err(why))
        }
        Ok(SeatRequest::ReadListing(..)) => {
            serve_listing(stream, frame, door);
            return;
        }
        Ok(SeatRequest::Post(s)) => match crate::held::held!(door.kernel) {
            Ok(mut h) => {
                let filed = h.run(|k, _| k.seat_post(&s));
                drop(h);
                // A post filed is an event of the court: the held reads are
                // answered (`hold`).
                if filed.is_ok() {
                    crate::hold::stir();
                }
                encode_post_reply(filed)
            }
            Err(_) => return,
        },
        Ok(SeatRequest::ReadMail(r)) => match crate::held::held!(door.kernel) {
            Ok(mut h) => {
                let mail = h.run(|k, _| k.seat_mail(&r));
                let Some(b) = mail_reply(&r, mail) else { return };
                b
            }
            Err(_) => return,
        },
    };
    answer(stream, Frame { lane: frame.lane, body });
}
