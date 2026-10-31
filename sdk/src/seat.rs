// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.145, seat wire v1.4 — THE SEAT SURFACE: a mind lists a domain's seats,
// takes one, calls, and reads its mail through the hypha's signed relay.

//! Seats in a house.
//!
//! A house (a domain's sitting reserve) lists its seats; a mind takes one,
//! signed by its key, calls the seat's functions, and reads its mail — the
//! house's answer to its take, the percepts, the outcomes of its calls. The
//! hypha holds and forwards each post, judging nothing in-game. The methods
//! are [`Session`](crate::session::Session)'s:
//!
//! - `list_seats(domain, sub_origin)`: the listing, signed by the house;
//! - `take(at, key)`: a mind takes the seat [`SeatName`] names;
//! - `call(at, key, fn_id, args)`: a seated mind calls; answered with the
//!   relay's sequence and the call's reference, which the outcome names;
//! - `mail(key, after, epoch)`: the mind's mail typed ([`MindRead`]);
//! - `read_mail(key, after, epoch)`: the raw mail ([`MailRead`]);
//! - `seat_post(post)`: one signed post, answered by its sequence.

/// A seat's name: its domain, the sub-universe it stands in, and its id.
pub use mycelium::sdk::seat::SeatName;

/// What a mind reads in its mail, typed, and the mail at the relay's
/// standing (epoch, last sequence).
pub use mycelium::sdk::seat::{MindMail, MindRead};

/// What a house reads in its mail: a take and a call.
pub use mycelium::sdk::seat::HouseMail;

/// The posts of the seat wire and the signed form of each.
pub use mycelium::anastome::dialogue::seat::{Call, Outcome, Percept, Post, Record, Seat, SeatFn, Seats, SignedPost, Take, Taken};

/// A call's argument, a seat's role, and who signed a post.
pub use mycelium::anastome::dialogue::seat::{Arg, Role, Signer};

/// The relay's refusal by name, and the raw mail at its standing.
pub use mycelium::anastome::dialogue::seat::{Mail, MailRead, Refusal};

/// The reasons an outcome refuses a call.
pub use mycelium::anastome::dialogue::seat::{REASON_BAD_ARGUMENTS, REASON_NOT_APPLIED, REASON_NOT_THE_HOLDER, REASON_NO_SUCH_FUNCTION};
