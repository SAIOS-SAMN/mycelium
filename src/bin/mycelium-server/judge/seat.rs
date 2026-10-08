// 78.145 — ACT_SEAT: the seat relay. A post is checked for its signer and
// held or filed (`Kernel::seat_post`), answered `[1][seq]` or the refusal; a
// listing is read; a reader's mail is read past what it acknowledges.
//
//! A relay, not a court act: nothing is sealed and nothing is persisted, so
//! the answer waits on no durability.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::seat::{decode_seat_request, encode_mail_reply, encode_post_reply, SeatRequest};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

/// 78.146 — A MIND'S READ, SERVED WHERE THE DOOR WOULD DEFER IT: a full
/// worker pool answers every act with the door's deficit, and reading one's
/// own mail spends nothing — so a READ-MAIL is served off the relay's own
/// lock (no kernel), once per window of the reader's mail
/// (`SharedRelay::exempt_read`). `false` where the frame is no read, or the
/// read is not exempt: the caller defers it as before. Posts keep the rule.
pub(crate) fn serve_exempt(stream: &mut TcpStream, frame: &Frame, door: &Door) -> bool {
    // A TASK READ takes the registry's lock and never the kernel's, so a
    // full pool waiting on cognition has no reason to defer it: a seated
    // mind's fetch of the task before its seat was deferred 2026-10-08 on
    // every hypha and no call was made. Served here, as observe is.
    if mycelium::anastome::dialogue::act_tag(&frame.body) == Some(mycelium::anastome::dialogue::ACT_TASK) {
        return crate::door::serve_control(stream, frame, door);
    }
    let Some(Ok(SeatRequest::ReadMail(r))) = decode_seat_request(&frame.body) else { return false };
    let Some(mail) = door.seat_relay.exempt_read(&r) else { return false };
    let Some(body) = encode_mail_reply(mail.as_deref().map_err(|why| *why)) else { return false };
    answer(stream, Frame { lane: frame.lane, body });
    true
}

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(request) = decode_seat_request(&frame.body) else { return };
    let body = match request {
        Err(why) => encode_post_reply(Err(why)),
        Ok(request) => match door.kernel.lock() {
            Ok(mut k) => match request {
                SeatRequest::Post(s) => encode_post_reply(k.seat_post(&s)),
                SeatRequest::ReadListing(domain) => match k.seat_listing(&domain) {
                    Some(b) => b,
                    None => Vec::new(),
                },
                SeatRequest::ReadMail(r) => {
                    let mail = k.seat_mail(&r);
                    let Some(b) = encode_mail_reply(mail.as_deref().map_err(|why| *why)) else { return };
                    b
                }
            },
            Err(_) => return,
        },
    };
    answer(stream, Frame { lane: frame.lane, body });
}
