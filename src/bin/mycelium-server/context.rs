// 23.8, 26 — THE CONTEXT: what one hypha holds for the act it is serving.
//
//! `Context` is the whole of a server's state as one act sees it. The
//! control surface a client or an operator speaks to is `control`.

use crate::exchange::Mesh;
use crate::syllabus::Syllabus;
use mycelium::cognition::Court;
use mycelium::network::nerve::{Keyed, Router};
use std::sync::{Arc, Mutex};


#[derive(Clone)]
pub(crate) struct Context {
    /// 78.146 — the seat relay on its own lock (`judge::seat::serve_exempt`).
    pub(crate) seat_relay: mycelium::cognition::seat::SharedRelay,
    /// 78.151 — the court's published reading, which the server's reads
    /// answer off without the court (`judge::serve_published`).
    pub(crate) published: crate::published::Shelf,
    pub(crate) court: Arc<Mutex<Court>>,
    pub(crate) mesh: Mesh,
    pub(crate) router: Arc<Mutex<Router>>,
    pub(crate) id: [u8; 32],
    pub(crate) bind: String,
    pub(crate) join_peers: Arc<Vec<String>>,
    pub(crate) roots: Arc<Mutex<Keyed<Keyed<u64>>>>,
    pub(crate) registry: Arc<Mutex<mycelium::registry::TaskRegistry>>,
    pub(crate) syllabus: Arc<Mutex<Syllabus>>,
    /// The nerve, built from the seal (D2.5): a context exists only
    /// after the boot calibration stroke sealed it.
    pub(crate) nerve_tx: crate::nerve::Nerve,
    pub(crate) synth_tx: std::sync::mpsc::SyncSender<mycelium::synthesis::Deliberation>,
    pub(crate) state_path: String,
}
