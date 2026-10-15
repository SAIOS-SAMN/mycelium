// MU-CLIENT §5.1 — THE HYPHA CARRIER: the mind's link to its court, on its
// own thread, so a court slow to answer holds only this carrier and never the
// field.
//
//! THE FIELD ASKS AND GOES ON. An exchange is posted whole — the wire half
//! run on the carrier against the one session this mind holds to the nerve,
//! the field half held beside the field — and the field takes each answer at
//! its next admission, in the order the asks were posted (the carrier runs
//! them in turn). What the answer moves is the field's: an answer that asks
//! for thought (a quote's listings, a task) is laid on the board for the tick
//! to think; one that only files (a verdict, an adjudication, a probe's
//! closure) files where it lands.
//!
//! THE WIRE'S READINGS ARE THE CARRIER'S: the final point the doors carried,
//! the impedance, the round trip. The carrier publishes them after every
//! exchange; the field reads the last published.
//!
//! The carrier wakes the field when an answer lands (`Hypha::rest`), so a
//! resting mind takes its court's word as it arrives, never a park later.

use crate::client::sdk::*;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};

/// One exchange's wire half, run on the carrier against its session.
type Job = Box<dyn FnOnce(&mut Session) + Send>;

/// One exchange's field half: `true` once its answer landed and was taken.
type Landing = Box<dyn FnMut(&mut Entity) -> bool>;

/// WHAT THE WIRE READ, as the carrier last published it.
#[derive(Clone, Copy)]
struct View {
    point: Option<crate::chronometry::tuniversal::TPoint>,
    aperture: TransportAperture,
    rtt: Option<u64>,
    courts: usize,
}

impl View {
    fn of(s: &Session) -> View {
        View { point: s.tuniversal(), aperture: s.transport(), rtt: s.rtt(), courts: s.peers.len() }
    }
}

/// THE FIELD'S END OF THE CARRIER: where asks are posted, the answers not
/// yet taken, the wire's last readings, and the kernel this mind speaks as.
pub(crate) struct Hypha {
    to: Sender<Job>,
    landings: VecDeque<Landing>,
    view: Arc<Mutex<View>>,
    woken: Receiver<()>,
    /// What wakes the resting field: every carrier that lands something for
    /// it (this one, the perception carrier) holds a copy.
    wake: Sender<()>,
    kernel: Kernel,
    addr: String,
}

impl Hypha {
    /// THE CARRIER OPENED on `session`, on its own thread, where it
    /// discovers the nerve before it carries the first ask: a court still
    /// waking holds the carrier, never the mind's boot.
    pub(crate) fn open(mut session: Session) -> Hypha {
        let kernel = session.kernel().clone();
        let addr = session.addr.clone();
        let view = Arc::new(Mutex::new(View::of(&session)));
        let (to, jobs) = channel::<Job>();
        let (wake, woken) = channel::<()>();
        let waker = wake.clone();
        let published = Arc::clone(&view);
        // A carrier that cannot be spawned leaves every ask unanswered: the
        // field goes on with its court silent, as on an occluded wire.
        let _ = std::thread::Builder::new().name("hypha".into()).spawn(move || {
            session.discover();
            if let Ok(mut v) = published.lock() {
                *v = View::of(&session);
            }
            for job in jobs {
                job(&mut session);
                if let Ok(mut v) = published.lock() {
                    *v = View::of(&session);
                }
                let _ = wake.send(());
            }
        });
        Hypha { to, landings: VecDeque::new(), view, woken, wake: waker, kernel, addr }
    }

    pub(crate) fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// A copy of what wakes the resting field, for another carrier that
    /// lands something for it.
    pub(crate) fn waker(&self) -> Sender<()> {
        self.wake.clone()
    }

    /// The hypha this mind was pointed at.
    pub(crate) fn addr(&self) -> &str {
        &self.addr
    }

    fn view(&self) -> Option<View> {
        self.view.lock().ok().map(|v| *v)
    }

    /// The last final point the doors carried (78.152 TU-7).
    pub(crate) fn tuniversal(&self) -> Option<crate::chronometry::tuniversal::TPoint> {
        self.view().and_then(|v| v.point)
    }

    /// The transport aperture the wire last read.
    pub(crate) fn transport(&self) -> TransportAperture {
        match self.view() {
            Some(v) => v.aperture,
            None => TransportAperture::Open,
        }
    }

    pub(crate) fn rtt(&self) -> Option<u64> {
        self.view().and_then(|v| v.rtt)
    }

    /// How many hyphae the nerve has shown this mind.
    pub(crate) fn courts(&self) -> usize {
        match self.view() {
            Some(v) => v.courts,
            None => 0,
        }
    }

    /// Answers posted and not yet taken.
    pub(crate) fn in_flight(&self) -> usize {
        self.landings.len()
    }

    /// REST until the carrier lands an answer or the idle park over the
    /// measured round trip (`rtt`, in strokes) passes, whichever is first.
    pub(crate) fn rest(&self, rtt: Option<u64>) {
        let _ = self.woken.recv_timeout(crate::physics::idle_park(rtt));
        while self.woken.try_recv().is_ok() {}
    }
}

/// AN EXCHANGE POSTED ON A CARRIER: `wire` runs on the carrier's thread
/// against its session, and `then` takes its answer on the field where it
/// lands.
fn post<R: Send + 'static>(carrier: &mut Hypha, wire: impl FnOnce(&mut Session) -> R + Send + 'static, then: impl FnOnce(&mut Entity, R) + 'static) {
    let (tx, rx) = channel::<R>();
    let job: Job = Box::new(move |s| {
        let _ = tx.send(wire(s));
    });
    if carrier.to.send(job).is_err() {
        // The carrier is gone: nothing will answer this ask.
        return;
    }
    let mut then = Some(then);
    carrier.landings.push_back(Box::new(move |me| match rx.try_recv() {
        Ok(r) => {
            if let Some(t) = then.take() {
                t(me, r);
            }
            true
        }
        Err(TryRecvError::Empty) => false,
        Err(TryRecvError::Disconnected) => true,
    }));
}

/// A CARRIER'S ANSWERS TAKEN, in the order they were asked: each that landed
/// is filed; the first not landed, and all after it, wait. An answer's filing
/// may ask again; those asks wait behind.
fn take_from(me: &mut Entity, carrier: fn(&mut Entity) -> &mut Hypha) {
    let mut waiting = std::mem::take(&mut carrier(me).landings);
    while let Some(mut landing) = waiting.pop_front() {
        if !landing(me) {
            waiting.push_front(landing);
            break;
        }
    }
    waiting.append(&mut carrier(me).landings);
    carrier(me).landings = waiting;
}

impl Entity {
    /// ASK THE COURT: `wire` runs on the hypha's carrier against the session,
    /// and `then` takes its answer on the field at the next admission.
    pub(crate) fn ask<R: Send + 'static>(
        &mut self,
        wire: impl FnOnce(&mut Session) -> R + Send + 'static,
        then: impl FnOnce(&mut Entity, R) + 'static,
    ) {
        post(&mut self.hypha, wire, then);
    }

    /// MU-CLIENT §6 — ASK THE HOUSE A SEAT STANDS IN: posted on the SEAT'S
    /// OWN CARRIER, so a slow house holds that carrier and nothing else.
    pub(crate) fn ask_seat<R: Send + 'static>(
        &mut self,
        wire: impl FnOnce(&mut Session) -> R + Send + 'static,
        then: impl FnOnce(&mut Entity, R) + 'static,
    ) {
        post(&mut self.seat, wire, then);
    }

    /// THE COURT'S ANSWERS TAKEN, in the order they were asked.
    pub(crate) fn take_answers(&mut self) {
        take_from(self, |e| &mut e.hypha);
    }

    /// THE HOUSE'S ANSWERS TAKEN off the seat's carrier, in the order asked.
    pub(crate) fn take_seat_answers(&mut self) {
        take_from(self, |e| &mut e.seat);
    }

    /// Every answer asked for, taken: the field waits on its court. A guard's
    /// only — nothing the mind does waits on a counterparty (MU-CLIENT §5.1).
    #[cfg(test)]
    pub(crate) fn settle(&mut self) {
        while self.hypha.in_flight() > 0 {
            let before = self.hypha.in_flight();
            self.take_answers();
            if self.hypha.in_flight() == before {
                let _ = self.hypha.woken.recv();
            }
        }
    }
}
