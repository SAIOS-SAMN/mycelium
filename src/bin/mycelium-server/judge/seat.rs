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
                    let Some(b) = encode_mail_reply(mail.as_deref()) else { return };
                    b
                }
            },
            Err(_) => return,
        },
    };
    answer(stream, Frame { lane: frame.lane, body });
}
