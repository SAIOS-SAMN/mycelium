// 78.145 — THE SEAT RELAY (`Kernel::seat_post`, `seat_mail`): the house — its
// domain's sitting reserve — lists seats, a mind takes one, the house reads
// the take and answers, perceives, the mind calls and reads the outcome; each
// reader reads only its own mail, an acknowledged piece leaves, and a box
// never passes its bound.

use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::seat::*;
use crate::server::cognition::seat::mail_bound;
use crate::server::cognition::Kernel;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}
fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}
fn domain() -> SigningKey {
    key(4)
}
fn house() -> SigningKey {
    key(21)
}
fn tier() -> SigningKey {
    key(22)
}
fn mind() -> SigningKey {
    key(30)
}
fn other() -> SigningKey {
    key(31)
}

/// A domain founded with two tiers, `house` its sitting reserve.
fn founded() -> Kernel {
    let mut k = Kernel::new();
    let mut tiers = vec![vk(&house()), vk(&tier())];
    tiers.sort_unstable();
    let g = DomainGenesis { domain: vk(&domain()), tiers };
    assert!(matches!(k.admit_domain(&g.sign(&domain())), Founded::Sealed { .. }));
    let r = Reservation { domain: vk(&domain()), tier: vk(&house()), nonce: [1; 32] }.sign(&domain());
    assert!(matches!(k.designate_reserve(&r), Founded::Reserved { now: true, .. }));
    k
}

fn seats(epoch: u64) -> Seats {
    let seat = Seat { seat: 7, role: Role::Steward, on: [6; 32], beat: Arg::Q { num: 1, den: 60 }, fns: vec![SeatFn { id: 3, tags: vec![2] }] };
    Seats { domain: vk(&domain()), sub_origin: None, epoch, seats: vec![seat] }
}

fn read(k: &mut Kernel, who: &SigningKey, after: u64) -> Vec<Mail> {
    k.seat_mail(&ReadMail::sign(who, after, [0; 32])).expect("signed").mail
}

/// THE WHOLE SURFACE ON ONE COURT: only the house lists; a mind takes; the
/// house reads the take and answers it, perceives with counsel; the mind
/// reads only its own mail and calls the counsel; the house reads the call
/// and answers its outcome by the call's reference; what a reader
/// acknowledges leaves.
#[test]
fn seat_post_relays_between_house_and_mind() {
    let mut k = founded();
    let d = vk(&domain());
    let m = vk(&mind());
    // Listing: only the sitting reserve; an unfounded domain is unknown.
    assert_eq!(k.seat_post(&Post::Seats(seats(1)).sign(&tier()).expect("writes")), Err(Refusal::NotTheHouse), "a tier not sitting is not the house");
    assert_eq!(k.seat_post(&Post::Seats(seats(1)).sign(&mind()).expect("writes")), Err(Refusal::NotTheHouse));
    let stranger = Seats { domain: [77; 32], ..seats(1) };
    assert_eq!(k.seat_post(&Post::Seats(stranger).sign(&house()).expect("writes")), Err(Refusal::UnknownDomain));
    assert_eq!(k.seat_listing(&d, None), None);
    let listed = Post::Seats(seats(2)).sign(&house()).expect("writes");
    assert!(k.seat_post(&listed).is_ok());
    assert_eq!(decode_listing_reply(&k.seat_listing(&d, None).expect("held"), None), Some(Some(listed.clone())));
    assert!(k.seat_post(&Post::Seats(seats(1)).sign(&house()).expect("writes")).is_ok());
    assert_eq!(decode_listing_reply(&k.seat_listing(&d, None).expect("held"), None), Some(Some(listed)), "an older epoch does not replace");
    // A take, signed by the mind it names — not by another.
    let take = Take { domain: d, sub_origin: None, seat: 7, mind: m, nonce: [2; 32] };
    assert_eq!(k.seat_post(&Post::Take(take.clone()).sign(&other()).expect("writes")), Err(Refusal::Unsigned));
    let take_seq = k.seat_post(&Post::Take(take.clone()).sign(&mind()).expect("writes")).expect("relayed");
    // The mind's own box holds nothing yet; the house reads the take.
    assert!(read(&mut k, &mind(), 0).is_empty(), "a take is the house's mail");
    assert!(read(&mut k, &tier(), 0).is_empty(), "a tier not sitting reads no house mail");
    let mail = read(&mut k, &house(), 0);
    assert_eq!(mail.len(), 1);
    assert_eq!((mail[0].seq, mail[0].post().map(|s| s.post)), (take_seq, Some(Post::Take(take))));
    // The house answers and perceives; a mind may not post for the house.
    let taken = Taken { domain: d, sub_origin: None, seat: 7, mind: m, ok: true, reason: 0 };
    assert_eq!(k.seat_post(&Post::Taken(taken.clone()).sign(&mind()).expect("writes")), Err(Refusal::NotTheHouse));
    k.seat_post(&Post::Taken(taken.clone()).sign(&house()).expect("writes")).expect("relayed");
    let counsel = Record::Counsel { fn_id: 3, args: vec![Arg::Count(2)], worth: Arg::Q { num: 1, den: 2 }, cost: Arg::Q { num: 1, den: 3 } };
    let percept = Percept { domain: d, sub_origin: None, seat: 7, mind: m, local_time: Arg::Q { num: 12, den: 1 }, seq: 1, records: vec![counsel], point: None };
    k.seat_post(&Post::Percept(percept.clone()).sign(&house()).expect("writes")).expect("relayed");
    // Another mind's percept is not this mind's mail.
    let other_percept = Percept { mind: vk(&other()), ..percept.clone() };
    k.seat_post(&Post::Percept(other_percept).sign(&house()).expect("writes")).expect("relayed");
    let mail = read(&mut k, &mind(), 0);
    let posts: Vec<Post> = mail.iter().filter_map(|x| x.post().map(|s| s.post)).collect();
    assert_eq!(posts, vec![Post::Taken(taken), Post::Percept(percept)], "the mind reads only its own mail, in order");
    assert_eq!(read(&mut k, &other(), 0).len(), 1);
    // Read past: acknowledged pieces leave; unacknowledged stay.
    let last = mail[1].seq;
    assert_eq!(read(&mut k, &mind(), mail[0].seq).len(), 1);
    assert!(read(&mut k, &mind(), last).is_empty());
    assert_eq!(k.seats.held_bytes(false, &m), 0, "the box is gone once read past");
    // The call goes to the house; its outcome back to the mind, naming it.
    let call = Post::Call(Call { domain: d, sub_origin: None, seat: 7, mind: m, nonce: [3; 32], fn_id: 3, args: vec![Arg::Count(2)] }).sign(&mind()).expect("writes");
    let reference = call.reference().expect("writes");
    k.seat_post(&call).expect("relayed");
    let house_mail = read(&mut k, &house(), take_seq);
    assert_eq!(house_mail.len(), 1, "the acknowledged take left");
    let got = house_mail[0].post().expect("reads");
    assert_eq!(got.reference(), Some(reference), "the house reads the reference the mind holds");
    let outcome = Outcome { domain: d, sub_origin: None, seat: 7, mind: m, call: reference, applied: true, reason: 0, deed: [8; 32] };
    k.seat_post(&Post::Outcome(outcome.clone()).sign(&house()).expect("writes")).expect("relayed");
    let back = read(&mut k, &mind(), last);
    assert_eq!(back.len(), 1);
    assert_eq!(back[0].post().map(|s| s.post), Some(Post::Outcome(outcome)));
    // An unsigned read is refused.
    let mut forged = ReadMail::sign(&mind(), 0, [0; 32]);
    forged.reader = vk(&other());
    assert_eq!(k.seat_mail(&forged), Err(Refusal::Unsigned));
}

/// 78.146 — A MIND'S OWN READ IS EXEMPT, ONCE PER WINDOW: a mind holding no
/// μ at all reads its mail through the relay's own lock (no kernel, no
/// reserve); a second read before new mail is not exempt (the door's ordinary
/// rule answers it); new mail opens the window again; a forged read is
/// refused; a reader with no box has nothing exempt.
#[test]
fn exempt_read_serves_once_per_window() {
    let mut k = founded();
    let d = vk(&domain());
    let m = vk(&mind());
    let relay = k.seat_relay();
    let percept = |seq| Percept { domain: d, sub_origin: None, seat: 7, mind: m, local_time: Arg::Q { num: 1, den: 1 }, seq, records: Vec::new(), point: None };
    assert_eq!(relay.exempt_read(&ReadMail::sign(&mind(), 0, [0; 32])), None, "no box, nothing exempt");
    let first = k.seat_post(&Post::Percept(percept(1)).sign(&house()).expect("writes")).expect("relayed");
    let served = relay.exempt_read(&ReadMail::sign(&mind(), 0, [1; 32])).expect("exempt").expect("signed").mail;
    assert_eq!(served.iter().map(|x| x.seq).collect::<Vec<_>>(), vec![first], "the zero-μ mind is served its mail");
    for n in 0..5u8 {
        assert_eq!(relay.exempt_read(&ReadMail::sign(&mind(), 0, [n; 32])), None, "a flood within the window is not exempt");
    }
    let second = k.seat_post(&Post::Percept(percept(2)).sign(&house()).expect("writes")).expect("relayed");
    let served = relay.exempt_read(&ReadMail::sign(&mind(), first, [9; 32])).expect("new mail opens the window").expect("signed").mail;
    assert_eq!(served.iter().map(|x| x.seq).collect::<Vec<_>>(), vec![second], "what `after` acknowledged left");
    let mut forged = ReadMail::sign(&mind(), 0, [7; 32]);
    forged.reader = vk(&other());
    assert_eq!(relay.exempt_read(&forged), Some(Err(Refusal::Unsigned)));
    // The court's own read still reads the same box, through the same lock.
    assert_eq!(read(&mut k, &mind(), first).len(), 1);
}

/// 78.146 amended, MU-CLIENT §7.1 — A READ WAITS WHERE NOTHING WAS FILED
/// PAST IT (`filed_past`, the court's held read): a read acknowledging the
/// relay's last sequence, at its epoch, has nothing past it and waits for
/// the court's next event; a post filed anywhere puts something past it; a
/// read under another epoch acknowledges nothing, so anything filed is past
/// it.
#[test]
fn filed_past_holds_read_with_nothing_new() {
    let mut k = founded();
    let d = vk(&domain());
    let m = vk(&mind());
    let relay = k.seat_relay();
    let percept = |seq| Percept { domain: d, sub_origin: None, seat: 7, mind: m, local_time: Arg::Q { num: 1, den: 1 }, seq, records: Vec::new(), point: None };
    let first = k.seat_post(&Post::Percept(percept(1)).sign(&house()).expect("writes")).expect("relayed");
    let (epoch, seq) = relay.standing();
    assert_eq!(seq, first, "the relay's last sequence is the post just filed");
    assert!(!relay.filed_past(&ReadMail::sign_standing(&mind(), seq, epoch, [1; 32])), "acknowledging the last sequence: nothing past it, it waits");
    assert!(relay.filed_past(&ReadMail::sign_standing(&mind(), 0, epoch, [2; 32])), "acknowledging nothing: what was filed is past it");
    assert!(relay.filed_past(&ReadMail::sign_standing(&mind(), seq, epoch.wrapping_add(1), [3; 32])), "under another epoch it acknowledges nothing");
    k.seat_post(&Post::Percept(percept(2)).sign(&house()).expect("writes")).expect("relayed");
    assert!(relay.filed_past(&ReadMail::sign_standing(&mind(), seq, epoch, [4; 32])), "a post filed since: it is answered now");
}

/// SEAT-WIRE v1.4 ON THE RELAY: listings held per (domain, sub-origin), the
/// domain's own untouched by a sub-universe's; a scoped percept filed to the
/// mind it names; a read at the relay's standing answered with its epoch and
/// last sequence; an `after` read under another epoch — a relay restarted
/// under the reader — acknowledges nothing and the box is read from the
/// start; two relays are two epochs.
#[test]
fn seat_listing_keys_by_sub_origin_with_epoch() {
    let mut k = founded();
    let d = vk(&domain());
    let m = vk(&mind());
    let sub = [0x5c; 32];
    let own = Post::Seats(seats(1)).sign(&house()).expect("writes");
    let scoped = Post::Seats(Seats { sub_origin: Some(sub), ..seats(1) }).sign(&house()).expect("writes");
    k.seat_post(&own).expect("listed");
    k.seat_post(&scoped).expect("listed");
    assert_eq!(decode_listing_reply(&k.seat_listing(&d, None).expect("held"), None), Some(Some(own.clone())));
    assert_eq!(decode_listing_reply(&k.seat_listing(&d, Some(&sub)).expect("held"), Some(&sub)), Some(Some(scoped)));
    assert_eq!(k.seat_listing(&d, Some(&[1; 32])), None, "another sub-origin holds no listing");
    // A later epoch at the sub-origin replaces only there.
    let later = Post::Seats(Seats { sub_origin: Some(sub), ..seats(5) }).sign(&house()).expect("writes");
    k.seat_post(&later).expect("listed");
    assert_eq!(decode_listing_reply(&k.seat_listing(&d, Some(&sub)).expect("held"), Some(&sub)), Some(Some(later)));
    assert_eq!(decode_listing_reply(&k.seat_listing(&d, None).expect("held"), None), Some(Some(own)));
    // A scoped percept reaches its mind, the sub-origin on it.
    let p = Percept { domain: d, sub_origin: Some(sub), seat: 7, mind: m, local_time: Arg::Q { num: 9, den: 4 }, seq: 1, records: Vec::new(), point: None };
    let s1 = k.seat_post(&Post::Percept(p.clone()).sign(&house()).expect("writes")).expect("relayed");
    let (epoch, seq) = k.seat_relay().standing();
    assert_ne!(epoch, 0, "the relay epoch is never the none-held 0");
    assert_eq!(seq, s1);
    let read = k.seat_mail(&ReadMail::sign_standing(&mind(), 0, 0, [1; 32])).expect("signed");
    assert_eq!((read.epoch, read.seq), (epoch, s1), "the reply carries the relay's standing");
    assert_eq!(read.mail.iter().map(|x| x.post().map(|s| s.post)).collect::<Vec<_>>(), vec![Some(Post::Percept(p.clone()))]);
    assert_eq!(read.mail.iter().map(|x| x.sub).collect::<Vec<_>>(), vec![SUB_PERCEPT | SCOPED], "filed under its scoped sub-kind");
    // A second percept; a cursor read under ANOTHER epoch acknowledges
    // nothing: the whole box from the start.
    let s2 = k.seat_post(&Post::Percept(Percept { seq: 2, ..p.clone() }).sign(&house()).expect("writes")).expect("relayed");
    let stale = k.seat_mail(&ReadMail::sign_standing(&mind(), s1, epoch ^ 1, [2; 32])).expect("signed");
    assert_eq!(stale.mail.iter().map(|x| x.seq).collect::<Vec<_>>(), vec![s1, s2], "another epoch's cursor is no acknowledgement");
    assert_eq!(stale.cursor(Some(epoch ^ 1), s1), 0, "the reader knows the restart");
    // Under this epoch, the cursor acknowledges.
    let fresh = k.seat_mail(&ReadMail::sign_standing(&mind(), s1, epoch, [3; 32])).expect("signed");
    assert_eq!(fresh.mail.iter().map(|x| x.seq).collect::<Vec<_>>(), vec![s2]);
    assert_eq!(fresh.cursor(Some(epoch), s1), s1, "progress, not a restart");
    assert_eq!(fresh.cursor(Some(epoch), s2 + 7), 0, "a cursor past the relay's seq is a restart");
    // A forged standing read is refused; a v1.3 read signed for sub-kind 7
    // does not verify as sub-kind 8.
    let mut forged = ReadMail::sign_standing(&mind(), 0, epoch, [4; 32]);
    forged.reader = vk(&other());
    assert_eq!(k.seat_mail(&forged), Err(Refusal::Unsigned));
    let mut moved = ReadMail::sign(&mind(), 0, [5; 32]);
    moved.standing = Some(epoch);
    assert!(!moved.verified(), "the sub-kind is signed");
    // Two relays, two epochs: a restart is told from progress.
    assert_ne!(founded().seat_relay().standing().0, epoch);
}

/// THE RELAY'S READING COUNTS EVERY POST: per domain with a reserve, filed
/// by kind and refused by reason; a domain with no reserve and a frame that
/// names none each counted apart; the standing — epoch, sequence, listings,
/// boxes, pieces and their bytes — as the relay holds it; and the reading
/// reads back whole through the observer's codec.
#[test]
fn relay_reading_counts_filed_and_refused_posts() {
    use crate::observe::{Reading, RelayPosts};
    let mut k = founded();
    let d = vk(&domain());
    let m = vk(&mind());
    let relay = k.seat_relay();
    for signer in [tier(), mind()] {
        assert_eq!(k.seat_post(&Post::Seats(seats(1)).sign(&signer).expect("writes")), Err(Refusal::NotTheHouse));
    }
    k.seat_post(&Post::Seats(seats(1)).sign(&house()).expect("writes")).expect("listed");
    let take = Take { domain: d, sub_origin: None, seat: 7, mind: m, nonce: [2; 32] };
    assert_eq!(k.seat_post(&Post::Take(take.clone()).sign(&other()).expect("writes")), Err(Refusal::Unsigned));
    k.seat_post(&Post::Take(take).sign(&mind()).expect("writes")).expect("relayed");
    let percept = Percept { domain: d, sub_origin: None, seat: 7, mind: m, local_time: Arg::Q { num: 1, den: 1 }, seq: 1, records: Vec::new(), point: None };
    let last = k.seat_post(&Post::Percept(percept).sign(&house()).expect("writes")).expect("relayed");
    let stranger = Seats { domain: [77; 32], ..seats(1) };
    assert_eq!(k.seat_post(&Post::Seats(stranger).sign(&house()).expect("writes")), Err(Refusal::UnknownDomain));
    relay.note_unread();
    let r = relay.reading();
    assert_eq!(r.domains, vec![RelayPosts { domain: d, filed: [1, 1, 0, 1, 0, 0], refused: [1, 2, 0] }], "seats, take, percept filed; unsigned 1, not-the-house 2");
    assert_eq!((r.unknown, r.unread), (1, 1), "the stranger and the unread frame, each apart");
    assert_eq!((r.epoch, r.seq), relay.standing());
    assert_eq!(r.seq, last, "three posts filed, three sequences");
    assert_eq!((r.listings, r.boxes, r.pieces, r.dropped), (1, 2, 2, 0), "the listing; the house's box (the take) and the mind's (the percept)");
    let bytes = k.seats.held_bytes(true, &d) + k.seats.held_bytes(false, &m);
    assert_eq!(r.bytes, bytes as u64);
    let mut out = Vec::new();
    let reading = Reading::Relay(Box::new(r));
    reading.put(&mut out).expect("writes");
    assert_eq!(Reading::take(&out, &mut 0), Some(reading), "the observer's codec reads it back whole");
}

/// THE BOUND HOLDS: a mind never reading its mail holds at most a frame's
/// worth of it, the oldest dropped and counted, and one read carries the
/// whole box.
#[test]
fn seat_post_keeps_mailbox_bound() {
    let mut k = founded();
    let d = vk(&domain());
    let m = vk(&mind());
    // A percept of ~64 KiB of records.
    let records: Vec<Record> = (0..250).map(|i| Record::Plain { kind: i, args: (0..30).map(|j| Arg::Key([j; 32])).collect() }).collect();
    let mut posted = 0u64;
    for seq in 0..20 {
        let p = Percept { domain: d, sub_origin: None, seat: 7, mind: m, local_time: Arg::Count(seq), seq, records: records.clone(), point: None };
        k.seat_post(&Post::Percept(p).sign(&house()).expect("writes")).expect("relayed");
        posted += 1;
        assert!(k.seats.held_bytes(false, &m) <= mail_bound(), "the box never passes its bound");
    }
    assert!(k.seats.dropped() > 0, "the oldest were dropped and counted");
    let mail = read(&mut k, &mind(), 0);
    assert_eq!(mail.len() as u64 + k.seats.dropped(), posted, "what is held and what was dropped are every piece");
    let reply = encode_mail_reply(Ok(&mail)).expect("writes");
    assert!(reply.len() <= crate::network::wire::frame_budget(), "one read carries the whole box");
    assert!(mail.windows(2).all(|w| w[0].seq < w[1].seq), "in sequence order");
}
