// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-ENGINEERING §9.1 (Composition), 78.152 TU-7 — THE EXPRESSION: one act
// of anastome (its tag, its content and its signature) and the TUniversal
// point its speaker last read, stood beside it. The transport moves an
// expression as opaque bytes and never reads it; the point is no envelope
// flag and no envelope trailer (the v0.5.0 recut).
//
//! The writing is `[act][point][point count u16 LE][wire version u16 LE]`:
//! the act's bytes as its own codec writes them, then the point's wire bytes
//! (`TPoint::put`), then their count, then the wire version the expression is
//! written at (`WIRE_VERSION`, #235). A speaker that has read no point writes
//! the count `0` and no point bytes; a point is never shorter than
//! `TPoint::MIN_LEN`, so a count of zero names no point and nothing else. The
//! act's tag stays at offset zero (`act_tag`). No writing is legacy: an
//! expression without its count and version is refused whole.
//!
//! THE VERSION IS READ FIRST, before the point or the act (#235): a reader
//! hears an expression of its own major version, at any minor, and refuses
//! another major by name (`Unheard::Version`). By the wire version rule
//! (MU-ENGINEERING §4) a minor step is additive, a new act, mode or trailing
//! field an older reader ignores, and a step that moves an existing act's
//! layout, or this writing, is a major one.

use super::WIRE_VERSION;
use crate::chronometry::tuniversal::TPoint;
use crate::network::wire::BODY_BUDGET;

/// WHY AN EXPRESSION IS NOT HEARD (#235).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unheard {
    /// Bytes that do not read as an expression: no version or count, a count
    /// past the bytes, a point that does not read whole, or an act past
    /// `BODY_BUDGET`.
    Malformed,
    /// An expression written at another major wire version than this
    /// reader's.
    Version { spoken: u16, served: u16 },
}

/// Whether a reader at `served` hears an expression written at `spoken`:
/// the same major, at any minor (a minor step is additive).
pub fn speaks(spoken: u16, served: u16) -> bool {
    spoken >> 8 == served >> 8
}

/// One expression: the act's bytes and the point its speaker read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    /// The act: its tag at offset zero, its content and its signature.
    pub act: Vec<u8>,
    /// The last final TUniversal point the speaker read; `None` before it
    /// has read one.
    pub point: Option<TPoint>,
}

impl Expression {
    /// The expression of `act` at `point`.
    pub fn of(act: Vec<u8>, point: Option<TPoint>) -> Expression {
        Expression { act, point }
    }

    /// The expression's bytes. `None` where the act passes the wire's
    /// `BODY_BUDGET` or the point passes its u16 count.
    pub fn write(&self) -> Option<Vec<u8>> {
        write_act(&self.act, self.point.as_ref())
    }

    /// The expression `bytes` writes, where this reader hears it; `None`
    /// where [`Expression::heard`] refuses it.
    pub fn read(bytes: Vec<u8>) -> Option<Expression> {
        Expression::heard(bytes).ok()
    }

    /// THE EXPRESSION HEARD, OR WHY NOT (#235): its wire version read first
    /// and refused by name where this reader does not speak it; then its
    /// point and its act, refused `Malformed` where the count is missing,
    /// passes the bytes, or does not count one point read whole, or the act
    /// passes `BODY_BUDGET`.
    pub fn heard(mut bytes: Vec<u8>) -> Result<Expression, Unheard> {
        let at = bytes.len().checked_sub(2).ok_or(Unheard::Malformed)?;
        let spoken = bytes.get(at..).and_then(|v| <[u8; 2]>::try_from(v).ok()).map(u16::from_le_bytes).ok_or(Unheard::Malformed)?;
        if !speaks(spoken, WIRE_VERSION) {
            return Err(Unheard::Version { spoken, served: WIRE_VERSION });
        }
        bytes.truncate(at);
        Expression::point_and_act(bytes).ok_or(Unheard::Malformed)
    }

    fn point_and_act(mut bytes: Vec<u8>) -> Option<Expression> {
        let end = bytes.len().checked_sub(2)?;
        let n = usize::from(u16::from_le_bytes(bytes.get(end..)?.try_into().ok()?));
        let point = match n {
            0 => {
                bytes.truncate(end);
                None
            }
            _ => {
                let (point, at) = TPoint::take_trailer(&bytes)?;
                bytes.truncate(at);
                Some(point)
            }
        };
        (bytes.len() <= BODY_BUDGET).then_some(Expression { act: bytes, point })
    }
}

/// `act` written as an expression at `point`, without a copy of the act
/// held apart. `None` past `BODY_BUDGET` or past the point's u16 count.
pub fn write_act(act: &[u8], point: Option<&TPoint>) -> Option<Vec<u8>> {
    if act.len() > BODY_BUDGET {
        return None;
    }
    let mut out = Vec::with_capacity(act.len() + 4 + point.map_or(0, |_| TPoint::MIN_LEN));
    out.extend_from_slice(act);
    match point {
        Some(p) => p.put_trailer(&mut out)?,
        None => out.extend_from_slice(&0u16.to_le_bytes()),
    }
    out.extend_from_slice(&WIRE_VERSION.to_le_bytes());
    Some(out)
}
