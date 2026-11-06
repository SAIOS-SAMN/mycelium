// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// ROADMAP track 25 X1; MU-HOSTING §5a.10; issue #160 (ruled 2026-11-06) —
// THE CROSSING'S REFUSAL WRITTEN, as a universe act's reply carries it
// (`Boundary::Crossing`): each refusal by its name, a breach of the
// conformal law with what it breaks it by, every turn exact.

use super::crossing_record::CrossingRefusal;
use crate::physics::laws::TurnBreach;
use crate::transducer::family::transduce::Turn;

fn put_word(out: &mut Vec<u8>, n: usize) -> Option<()> {
    out.extend_from_slice(&u64::try_from(n).ok()?.to_le_bytes());
    Some(())
}

fn take_word(b: &[u8], p: &mut usize) -> Option<usize> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    usize::try_from(v).ok()
}

fn byte(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

/// A BREACH WRITTEN: `[tag]` then, by tag, `0` Rank `[stated][phases]`;
/// `1` LeavesPeriods `[phase][turn][period]`; `2` PlaceRank
/// `[body][stated]`; `3` NotCarried `[body][count][turn …]`; `4` Unread
/// `[phase]`. Each count a u64 LE, each turn as [`Turn::put`] writes it.
fn put_breach(out: &mut Vec<u8>, b: &TurnBreach) -> Option<()> {
    match b {
        TurnBreach::Rank { stated, phases } => {
            out.push(0);
            put_word(out, *stated)?;
            put_word(out, *phases)
        }
        TurnBreach::LeavesPeriods { phase, turn, period } => {
            out.push(1);
            put_word(out, *phase)?;
            turn.put(out)?;
            put_word(out, *period)
        }
        TurnBreach::PlaceRank { body, stated } => {
            out.push(2);
            put_word(out, *body)?;
            put_word(out, *stated)
        }
        TurnBreach::NotCarried { body, residue } => {
            out.push(3);
            put_word(out, *body)?;
            put_word(out, residue.len())?;
            residue.iter().try_for_each(|t| t.put(out))
        }
        TurnBreach::Unread { phase } => {
            out.push(4);
            put_word(out, *phase)
        }
    }
}

fn take_breach(b: &[u8], p: &mut usize) -> Option<TurnBreach> {
    Some(match byte(b, p)? {
        0 => TurnBreach::Rank { stated: take_word(b, p)?, phases: take_word(b, p)? },
        1 => TurnBreach::LeavesPeriods { phase: take_word(b, p)?, turn: Turn::take(b, p)?, period: take_word(b, p)? },
        2 => TurnBreach::PlaceRank { body: take_word(b, p)?, stated: take_word(b, p)? },
        3 => {
            let body = take_word(b, p)?;
            let residue = (0..take_word(b, p)?).map(|_| Turn::take(b, p)).collect::<Option<Vec<Turn>>>()?;
            TurnBreach::NotCarried { body, residue }
        }
        4 => TurnBreach::Unread { phase: take_word(b, p)? },
        _ => return None,
    })
}

impl CrossingRefusal {
    /// THE REFUSAL WRITTEN: `[tag]` then, by tag, `0` AddressOnly, `1`
    /// RingStart, `2` Malformed, `3` NoFrame (nothing after each); `4`
    /// NoPlace `[body]`; `5` Unpaired `[before][after]`; `6` Turns
    /// `[count][breach …]`; `7` Absent. `None` where a turn cannot be
    /// written.
    pub fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        match self {
            CrossingRefusal::AddressOnly => out.push(0),
            CrossingRefusal::RingStart => out.push(1),
            CrossingRefusal::Malformed => out.push(2),
            CrossingRefusal::NoFrame => out.push(3),
            CrossingRefusal::NoPlace { body } => {
                out.push(4);
                put_word(out, *body)?;
            }
            CrossingRefusal::Unpaired { before, after } => {
                out.push(5);
                put_word(out, *before)?;
                put_word(out, *after)?;
            }
            CrossingRefusal::Turns(breaches) => {
                out.push(6);
                put_word(out, breaches.len())?;
                breaches.iter().try_for_each(|x| put_breach(out, x))?;
            }
            CrossingRefusal::Absent => out.push(7),
        }
        Some(())
    }

    /// The refusal read back from `p` on, or none.
    pub fn take(b: &[u8], p: &mut usize) -> Option<CrossingRefusal> {
        Some(match byte(b, p)? {
            0 => CrossingRefusal::AddressOnly,
            1 => CrossingRefusal::RingStart,
            2 => CrossingRefusal::Malformed,
            3 => CrossingRefusal::NoFrame,
            4 => CrossingRefusal::NoPlace { body: take_word(b, p)? },
            5 => CrossingRefusal::Unpaired { before: take_word(b, p)?, after: take_word(b, p)? },
            6 => CrossingRefusal::Turns((0..take_word(b, p)?).map(|_| take_breach(b, p)).collect::<Option<Vec<_>>>()?),
            7 => CrossingRefusal::Absent,
            _ => return None,
        })
    }
}
