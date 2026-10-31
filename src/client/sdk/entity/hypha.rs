// MU-CLIENT §5.1 — THE MIND'S CARRIERS: each on its own thread, holding its
// own state, so what is slow on one holds only that carrier and never the
// field. The hypha carrier holds the mind's link to its court; the
// perception carrier (§7.1 R1) holds the extraction memo and perceives off
// the field; the seat's holds its link to the house.
//
//! THE FIELD ASKS AND GOES ON. An exchange is posted whole — the far half
//! run on the carrier against the state it holds, the field half held
//! beside the field — and the field takes each answer at its next admission,
//! in the order the asks were posted (the carrier runs them in turn). What
//! the answer moves is the field's: an answer that asks for thought (a
//! quote's listings, a task, a scene read) is laid on the board or admitted;
//! one that only files (a verdict, an adjudication, a probe's closure) files
//! where it lands.
//!
//! THE WIRE'S READINGS ARE THE HYPHA CARRIER'S: the final point the doors
//! carried, the impedance, the round trip. The carrier publishes them after
//! every exchange; the field reads the last published.
//!
//! Every carrier wakes the field when an answer lands (`Hypha::rest`), so a
//! resting mind takes what landed as it arrives, never a park later.

use crate::client::sdk::*;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};

/// One exchange's far half, run on the carrier against the state it holds.
type Job<S> = Box<dyn FnOnce(&mut S) + Send>;

/// One exchange's field half: `true` once its answer landed and was taken.
type Landing = Box<dyn FnMut(&mut Mind) -> bool>;

/// ONE CARRIER: where its asks are posted, and the answers not yet taken,
/// each with the path of the field half that takes it (a reading).
pub(crate) struct Carrier<S> {
    to: Sender<Job<S>>,
    landings: VecDeque<(&'static str, Landing)>,
}

impl<S: Send + 'static> Carrier<S> {
    /// THE CARRIER OPENED on its own thread `name`, holding `state`: `start`
    /// runs there first, `after` after each exchange, and `wake` is told
    /// whenever an answer lands. A carrier that cannot be spawned leaves
    /// every ask unanswered, as an occluded wire does.
    pub(crate) fn open(
        name: &str,
        mut state: S,
        wake: Sender<()>,
        start: impl FnOnce(&mut S) + Send + 'static,
        mut after: impl FnMut(&mut S) + Send + 'static,
    ) -> Carrier<S> {
        let (to, jobs) = channel::<Job<S>>();
        let _ = std::thread::Builder::new().name(name.into()).spawn(move || {
            start(&mut state);
            for job in jobs {
                job(&mut state);
                after(&mut state);
                let _ = wake.send(());
            }
        });
        Carrier { to, landings: VecDeque::new() }
    }

    /// Answers posted and not yet taken.
    pub(crate) fn in_flight(&self) -> usize {
        self.landings.len()
    }

    /// THE ANSWERS NOT YET TAKEN, by the path that takes them, each with its
    /// count, in the order each path first stands in the queue.
    pub(crate) fn waiting_by_path(&self) -> Vec<(&'static str, usize)> {
        let mut out: Vec<(&'static str, usize)> = Vec::new();
        for (name, _) in &self.landings {
            match out.iter_mut().find(|(n, _)| n == name) {
                Some((_, k)) => *k += 1,
                None => out.push((name, 1)),
            }
        }
        out
    }
}

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

/// THE FIELD'S END OF THE HYPHA CARRIER: the carrier over the one session
/// this mind holds to the nerve, the wire's last readings, what wakes the
/// resting field, and the voice this mind speaks as.
pub(crate) struct Hypha {
    carrier: Carrier<Session>,
    view: Arc<Mutex<View>>,
    woken: Receiver<()>,
    /// What wakes the resting field: every carrier that lands something for
    /// it holds a copy.
    wake: Sender<()>,
    voice: Voice,
}

impl Hypha {
    /// THE CARRIER OPENED on `session`, on its own thread, where it
    /// discovers the nerve before it carries the first ask: a court still
    /// waking holds the carrier, never the mind's boot.
    pub(crate) fn open(session: Session) -> Hypha {
        Hypha::open_waking(session, None)
    }

    /// A CARRIER BESIDE THE MIND'S, whose landings wake the mind's own rest
    /// (`mind`, the main hypha's waker): the seat's carrier is one — the
    /// mind blocks on its main hypha alone, and a seat's answer woke a
    /// channel no one waited on, so a percept the house filed sat unread
    /// until something else woke the mind (2026-10-23: n4, 31 minutes).
    pub(crate) fn open_beside(session: Session, mind: Sender<()>) -> Hypha {
        Hypha::open_waking(session, Some(mind))
    }

    fn open_waking(session: Session, mind: Option<Sender<()>>) -> Hypha {
        let voice = session.voice().clone();
        let view = Arc::new(Mutex::new(View::of(&session)));
        let (own, woken) = channel::<()>();
        let wake = match mind {
            Some(w) => w,
            None => own,
        };
        let (first, every) = (Arc::clone(&view), Arc::clone(&view));
        let publish = |v: &Arc<Mutex<View>>, s: &Session| {
            if let Ok(mut held) = v.lock() {
                *held = View::of(s);
            }
        };
        let carrier = Carrier::open(
            "hypha",
            session,
            wake.clone(),
            move |s: &mut Session| {
                s.discover();
                publish(&first, s);
            },
            move |s: &mut Session| publish(&every, s),
        );
        Hypha { carrier, view, woken, wake, voice }
    }

    pub(crate) fn voice(&self) -> &Voice {
        &self.voice
    }

    /// A copy of what wakes the resting field, for another carrier that
    /// lands something for it.
    pub(crate) fn waker(&self) -> Sender<()> {
        self.wake.clone()
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
        self.carrier.in_flight()
    }

    /// The answers not yet taken, by the path that takes them.
    pub(crate) fn waiting_by_path(&self) -> Vec<(&'static str, usize)> {
        self.carrier.waiting_by_path()
    }

    /// REST until a carrier lands an answer or the idle park over the
    /// measured round trip (`rtt`, in strokes) passes, whichever is first.
    pub(crate) fn rest(&self, rtt: Option<u64>) {
        let _ = self.woken.recv_timeout(crate::physics::idle_park(rtt));
        while self.woken.try_recv().is_ok() {}
    }

    /// MU-CLIENT §7.1 — SOVEREIGN REST: blocked until a carrier lands
    /// something — a court's answer, a held seat read the court answered at
    /// its next event, a hearing, an observer's call, the stop — and no clock
    /// in between.
    pub(crate) fn block(&self) {
        let _ = self.woken.recv();
        while self.woken.try_recv().is_ok() {}
    }

    /// Wait for the next landing on any carrier: a guard's only.
    #[cfg(test)]
    pub(crate) fn wait_landing(&self) {
        let _ = self.woken.recv();
    }
}

/// AN EXCHANGE POSTED ON A CARRIER: `far` runs on the carrier's thread
/// against its state, and `then` takes its answer on the field where it
/// lands.
pub(crate) fn post<S: Send + 'static, R: Send + 'static>(
    carrier: &mut Carrier<S>,
    far: impl FnOnce(&mut S) -> R + Send + 'static,
    then: impl FnOnce(&mut Mind, R) + 'static,
) {
    let (tx, rx) = channel::<R>();
    let job: Job<S> = Box::new(move |s| {
        let _ = tx.send(far(s));
    });
    if carrier.to.send(job).is_err() {
        // The carrier is gone: nothing will answer this ask.
        return;
    }
    // WHERE THE ANSWER IS TAKEN, named for the observer: the path of the
    // field half that takes it, so a landing that holds the winding names
    // itself (`observe::live`) — a reading, never read back by cognition.
    let named = std::any::type_name_of_val(&then);
    let mut then = Some(then);
    carrier.landings.push_back((named, Box::new(move |entity| match rx.try_recv() {
        Ok(r) => {
            if let Some(t) = then.take() {
                crate::observe::live::enter(named);
                t(entity, r);
            }
            true
        }
        Err(TryRecvError::Empty) => false,
        Err(TryRecvError::Disconnected) => true,
    })));
}

/// A CARRIER'S ANSWERS TAKEN, in the order they were asked: each that landed
/// is filed; the first not landed, and all after it, wait. An answer's filing
/// may ask again; those asks wait behind. A carrier not opened lands nothing.
pub(crate) fn take_from<S>(entity: &mut Mind, carrier: fn(&mut Mind) -> Option<&mut Carrier<S>>) {
    let Some(c) = carrier(entity) else { return };
    let mut waiting = std::mem::take(&mut c.landings);
    while let Some((name, mut landing)) = waiting.pop_front() {
        if !landing(entity) {
            waiting.push_front((name, landing));
            break;
        }
    }
    if let Some(c) = carrier(entity) {
        waiting.append(&mut c.landings);
        c.landings = waiting;
    }
}

impl Mind {
    /// ASK THE COURT: `wire` runs on the hypha's carrier against the session,
    /// and `then` takes its answer on the field at the next admission.
    pub(crate) fn ask<R: Send + 'static>(
        &mut self,
        wire: impl FnOnce(&mut Session) -> R + Send + 'static,
        then: impl FnOnce(&mut Mind, R) + 'static,
    ) {
        post(&mut self.hypha.carrier, wire, then);
    }

    /// MU-CLIENT §6 — ASK THE HOUSE A SEAT STANDS IN: posted on the SEAT'S
    /// OWN CARRIER, so a slow house holds that carrier and nothing else.
    pub(crate) fn ask_seat<R: Send + 'static>(
        &mut self,
        wire: impl FnOnce(&mut Session) -> R + Send + 'static,
        then: impl FnOnce(&mut Mind, R) + 'static,
    ) {
        post(&mut self.seat.carrier, wire, then);
    }

    /// THE COURT'S ANSWERS TAKEN, in the order they were asked.
    pub(crate) fn drain_answers(&mut self) {
        take_from(self, |e| Some(&mut e.hypha.carrier));
    }

    /// THE HELD MAIL READ, posted on the perceiving carrier alone
    /// (`Mind::held`), never in front of the seat's own acts.
    pub(crate) fn ask_held<R: Send + 'static>(
        &mut self,
        wire: impl FnOnce(&mut Session) -> R + Send + 'static,
        then: impl FnOnce(&mut Mind, R) + 'static,
    ) {
        post(&mut self.held.carrier, wire, then);
    }

    /// THE HOUSE'S ANSWERS TAKEN off the seat's carrier and the perceiving
    /// carrier, each in the order asked.
    pub(crate) fn drain_seat_answers(&mut self) {
        take_from(self, |e| Some(&mut e.seat.carrier));
        take_from(self, |e| Some(&mut e.held.carrier));
    }

    /// Every answer asked for, taken: the field waits on its court. A guard's
    /// only — nothing the mind does waits on a counterparty (MU-CLIENT §5.1).
    #[cfg(test)]
    pub(crate) fn settle(&mut self) {
        while self.hypha.in_flight() > 0 {
            let before = self.hypha.in_flight();
            self.drain_answers();
            if self.hypha.in_flight() == before {
                self.hypha.wait_landing();
            }
        }
    }
}
