// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.125 (law version 5, ruled 2026-10-31; MU-DERIVATIONS D1.1) — THE WINDOW
// CLOSE ON THE COURT: every `until` of the challenge windows read off a
// sealed reach and return of every other keeper, never a count of heights.
//
//! A window opened at height `h₀` (a fold's seal, a demand, a withdrawal, a
//! toll's acceptance, a mouth or handoff offer, a crossing's claim) closes at
//! the height of the first `Cite::Window` receipt whose `through` is at or
//! above `h₀`: every other keeper had then verified a reading of this court
//! at or above `h₀` and read again after. Reach only rises, so one sealed
//! close settles every window opened at or below it, and one seal per stroke
//! closes every window the stroke settles. A court with no other keeper
//! closes nothing (`WindowRefusal::NoOtherKeeper`, TU-3). A window opened
//! under law version 4 (at or below `v4_through`) closes at `h₀ + 1024`, by
//! the version it was opened under.

use super::ledger::Standing;
use super::Court;
use crate::anastome::dialogue::handoff::State;
use crate::chronometry::tuniversal::{Reach, WindowRefusal};
use crate::chronometry::{ActKind, Cite, KStepReceipt};
use crate::physics::laws::CHALLENGE_WINDOW;

/// The keys a trie holds, read before its values are changed in place.
fn keys<T>(k: &crate::network::nerve::Keyed<T>) -> Vec<Vec<u8>> {
    k.scoped().map(|(k, _)| k).collect()
}

/// THE WINDOW BOOK: the last height this court opened windows under law
/// version 4, where it ever did, and every close it sealed, rising.
/// Persisted (`LEAF_WINDOWS`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct WindowBook {
    /// Windows opened at or below this height were opened under law version
    /// 4; `None` on a court that began under version 5.
    pub(crate) v4_through: Option<u64>,
    /// Each sealed close: the greatest opening it closes, and the height of
    /// its receipt. Both rise.
    pub(crate) closes: Vec<(u64, u64)>,
    /// Why the last reach handed in closed nothing new: volatile, a reading
    /// for the refusal a still-open window names.
    pub(crate) refusal: Option<WindowRefusal>,
}

impl WindowBook {
    /// The book of a home written under law version 4: every window it holds
    /// was opened at or below the head it was read at, under version 4.
    pub(crate) fn from_v4_home(head: u64) -> WindowBook {
        WindowBook { v4_through: Some(head), closes: Vec::new(), refusal: None }
    }

    /// Whether a window opened at `opened` was opened under law version 4.
    fn is_v4(&self, opened: u64) -> bool {
        self.v4_through.is_some_and(|t| opened <= t)
    }

    /// `[0 | 1 v4_through u64][count u32][(through u64, at u64)…]`.
    pub(crate) fn put(&self) -> Vec<u8> {
        let mut v = Vec::new();
        match self.v4_through {
            None => v.push(0),
            Some(t) => {
                v.push(1);
                v.extend_from_slice(&t.to_le_bytes());
            }
        }
        crate::transducer::address::put_key_count(&mut v, self.closes.len());
        for (through, at) in &self.closes {
            v.extend_from_slice(&through.to_le_bytes());
            v.extend_from_slice(&at.to_le_bytes());
        }
        v
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<WindowBook> {
        let u64_at = |p: &mut usize| -> Option<u64> {
            let x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(x)
        };
        let flag = *b.get(*p)?;
        *p += 1;
        let v4_through = match flag {
            0 => None,
            1 => Some(u64_at(p)?),
            _ => return None,
        };
        let n = crate::transducer::address::take_key_count(b, p)?;
        let closes = (0..n).map(|_| Some((u64_at(p)?, u64_at(p)?))).collect::<Option<Vec<_>>>()?;
        Some(WindowBook { v4_through, closes, refusal: None })
    }
}

impl Court {
    /// THE HEIGHT A WINDOW OPENED AT `opened` CLOSES: by law version 4 where
    /// it was opened under version 4; otherwise the receipt height of the
    /// first sealed close at or above it. Refused by name while it stays
    /// open, with what it waits on.
    pub fn window_close(&self, opened: u64) -> Result<u64, WindowRefusal> {
        if self.windows.is_v4(opened) {
            return opened.checked_add(CHALLENGE_WINDOW).ok_or(WindowRefusal::PastCarrier);
        }
        let i = self.windows.closes.partition_point(|(through, _)| *through < opened);
        if let Some((_, at)) = self.windows.closes.get(i) {
            return Ok(*at);
        }
        match (self.windows.refusal, self.windows.closes.iter().next_back()) {
            (Some(why), _) => Err(why),
            (None, Some((through, _))) => Err(WindowRefusal::Unreached { reach: *through }),
            (None, None) => Err(WindowRefusal::NoOtherKeeper),
        }
    }

    /// Whether the window opened at `opened` has closed at `head`.
    pub(crate) fn window_closed(&self, opened: u64, head: u64) -> bool {
        matches!(self.window_close(opened), Ok(at) if head >= at)
    }

    /// EVERY WINDOW STILL HELD, by the height it opened at: each open fold's
    /// seal, demand and withdrawal; each toll, claim and offer; each
    /// settlement still on the docket (D2.5: it goes cold once its window
    /// closes).
    fn held_windows(&self) -> Vec<u64> {
        let mut out = Vec::new();
        for (key, r) in self.ledgers.records.scoped() {
            if r.standing != Standing::Open {
                continue;
            }
            out.push(r.sealed);
            out.extend(r.demanded);
            out.extend(self.ledgers.withdrawn.get(&key).copied());
        }
        out.extend(self.handoffs.tolls.scoped().map(|(_, t)| t.opened));
        out.extend(self.atlas.claims.scoped().map(|(_, c)| c.opened));
        let offered = |s: &State| match s {
            State::Offered { opened } => Some(*opened),
            State::Accepted | State::Returned => None,
        };
        out.extend(self.handoffs.mouths.scoped().filter_map(|(_, m)| offered(&m.state)));
        out.extend(self.handoffs.held.scoped().filter_map(|(_, h)| offered(&h.state)));
        out.extend(self.handoffs.conversions.scoped().filter_map(|(_, c)| offered(&c.state)));
        out.extend(self.settlements.iter().map(|s| s.at.h));
        out
    }

    /// 78.125 (law version 5) — THE CLOSE SEALED: where the keepers' reach
    /// has risen past the last close and a window held opened at or below
    /// it, every witness is checked (signatures, one keeper each, reach and
    /// return) and one `Cite::Window` receipt is sealed naming them; every
    /// window opened at or below its `through` reads closed at its height.
    /// A refusal is held as the reading an open window names. The receipt
    /// sealed, where one was.
    pub fn close_windows(&mut self, reach: &Result<Reach, WindowRefusal>) -> Option<KStepReceipt> {
        let reach = match reach {
            Ok(r) => r,
            Err(why) => {
                self.windows.refusal = Some(*why);
                return None;
            }
        };
        let last = self.windows.closes.iter().next_back().map(|(through, _)| *through);
        if last.is_some_and(|l| reach.through <= l) {
            self.windows.refusal = None;
            return None;
        }
        if reach.court != self.identity {
            self.windows.refusal = Some(WindowRefusal::Mismatched { keeper: reach.court });
            return None;
        }
        if let Err(why) = reach.verify() {
            self.windows.refusal = Some(why);
            return None;
        }
        let pending = self
            .held_windows()
            .into_iter()
            .any(|h| !self.windows.is_v4(h) && h <= reach.through && last.is_none_or(|l| h > l));
        self.windows.refusal = None;
        if !pending {
            return None;
        }
        let witnesses = reach.cited()?;
        let cite = Cite::Window { through: reach.through, witnesses };
        let r = self.seal_step(ActKind::Challenge, [0; 4], self.identity, |_| vec![cite.clone()])?;
        self.windows.closes.push((reach.through, r.stamp.h));
        self.mark_persist();
        Some(r)
    }

    /// The closes no held window reads any more, let go: every close below
    /// the one the lowest held window reads, and all but the last where none
    /// is held. The count let go.
    pub(crate) fn settle_closes(&mut self) -> usize {
        let mut lowest: Option<u64> = None;
        for h in self.held_windows() {
            if !self.windows.is_v4(h) && lowest.is_none_or(|l| h < l) {
                lowest = Some(h);
            }
        }
        let gone = match lowest {
            Some(h) => self.windows.closes.partition_point(|(through, _)| *through < h),
            None => match self.windows.closes.len().checked_sub(1) {
                Some(n) => n,
                None => return 0,
            },
        };
        self.windows.closes.drain(..gone);
        gone
    }

    /// A home written under law version 4 read: version 5 holds from its
    /// head, and every held window's stored end (`h₀ + 1024`) is read back
    /// to its opening, so it closes by the version it was opened under.
    pub(crate) fn read_v4_windows(&mut self) {
        let head = self.chain.head_height();
        self.windows = WindowBook::from_v4_home(head);
        let back = |end: &mut u64| {
            if let Some(o) = end.checked_sub(CHALLENGE_WINDOW) {
                *end = o;
            }
        };
        let reopen = |s: &mut State| {
            if let State::Offered { opened } = s {
                back(opened);
            }
        };
        for k in keys(&self.handoffs.tolls) {
            if let Some(t) = self.handoffs.tolls.get_mut(&k) {
                back(&mut t.opened);
            }
        }
        for k in keys(&self.atlas.claims) {
            if let Some(c) = self.atlas.claims.get_mut(&k) {
                back(&mut c.opened);
            }
        }
        for k in keys(&self.handoffs.mouths) {
            if let Some(m) = self.handoffs.mouths.get_mut(&k) {
                reopen(&mut m.state);
            }
        }
        for k in keys(&self.handoffs.held) {
            if let Some(h) = self.handoffs.held.get_mut(&k) {
                reopen(&mut h.state);
            }
        }
        for k in keys(&self.handoffs.conversions) {
            if let Some(c) = self.handoffs.conversions.get_mut(&k) {
                reopen(&mut c.state);
            }
        }
    }
}
