// 78.152b — ACT_TICK: a peer keeper's ring, pushed on its own act, heard at
// the door where the frame is read — before the work queue and on the
// overflow path — so TUniversal never waits on this court's load.
//
//! The door takes no kernel here: `tshelf::hear_tick` is handed the mesh and
//! the frame, never the kernel. Answered one byte.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::{TICK_HEARD, TICK_REFUSED};
use mycelium::network::wire::Frame;
use std::net::TcpStream;

/// A pushed ring heard and answered `TICK_HEARD`, or `TICK_REFUSED` where it
/// does not read, its keeper is no known peer court, no origin is adopted
/// here, or a reading was refused (counted by name in `hear_ring`).
pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let ack = match crate::tshelf::hear_tick(&door.mesh, &frame.body) {
        Ok(_) => TICK_HEARD,
        Err(_) => TICK_REFUSED,
    };
    answer(stream, Frame { lane: frame.lane, body: vec![ack] });
}
