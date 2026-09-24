// 19, A9 — THE SELECTION TERM'S CODEC (TN1, 78.34: "tape" names a word's
// letter tape and nothing else; this module was `anchor/tape.rs`): the
// selection term as bytes, and back.

use super::*;

impl Sel {
    /// SERIALIZE the term — prefix walk, one opcode byte per node, operands
    /// inline. Opcodes are the algebra's operators (wire form of the same
    /// constructors), never semantic labels.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        match self {
            Sel::Hue(h) => {
                out.push(0);
                out.push(*h);
            }
            Sel::Comp(e, k) => {
                out.push(1);
                out.push(*k);
                e.to_bytes(out);
            }
            Sel::Delta(e) => {
                out.push(2);
                e.to_bytes(out);
            }
            Sel::Star(e) => {
                out.push(3);
                e.to_bytes(out);
            }
            Sel::Ext(e, a) => {
                out.push(4);
                out.push(*a);
                e.to_bytes(out);
            }
            Sel::Inter(a, b) => {
                out.push(5);
                a.to_bytes(out);
                b.to_bytes(out);
            }
            Sel::Minus(a, b) => {
                out.push(6);
                a.to_bytes(out);
                b.to_bytes(out);
            }
            Sel::Flood(a, b) => {
                out.push(7);
                a.to_bytes(out);
                b.to_bytes(out);
            }
            Sel::ArgExt(e, c) => {
                out.push(8);
                out.push(*c);
                e.to_bytes(out);
            }
            Sel::Transport(a, b) => {
                out.push(9);
                a.to_bytes(out);
                b.to_bytes(out);
            }
        }
    }

    /// Parse a term (untrusted input: depth-capped at 16, every read guarded —
    /// a corrupt term is None, never a crash or an allocation bomb).
    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Sel> {
        Sel::parse(b, pos, 0)
    }

    fn parse(b: &[u8], pos: &mut usize, depth: u8) -> Option<Sel> {
        if depth > 16 {
            return None;
        }
        let op = *b.get(*pos)?;
        *pos += 1;
        Some(match op {
            0 => {
                let h = *b.get(*pos)?;
                *pos += 1;
                Sel::Hue(h)
            }
            1 => {
                let k = *b.get(*pos)?;
                *pos += 1;
                Sel::Comp(Box::new(Sel::parse(b, pos, depth + 1)?), k)
            }
            2 => Sel::Delta(Box::new(Sel::parse(b, pos, depth + 1)?)),
            3 => Sel::Star(Box::new(Sel::parse(b, pos, depth + 1)?)),
            4 => {
                let a = *b.get(*pos)?;
                *pos += 1;
                Sel::Ext(Box::new(Sel::parse(b, pos, depth + 1)?), a)
            }
            5 => Sel::Inter(
                Box::new(Sel::parse(b, pos, depth + 1)?),
                Box::new(Sel::parse(b, pos, depth + 1)?),
            ),
            6 => Sel::Minus(
                Box::new(Sel::parse(b, pos, depth + 1)?),
                Box::new(Sel::parse(b, pos, depth + 1)?),
            ),
            7 => Sel::Flood(
                Box::new(Sel::parse(b, pos, depth + 1)?),
                Box::new(Sel::parse(b, pos, depth + 1)?),
            ),
            8 => {
                let c = *b.get(*pos)?;
                *pos += 1;
                Sel::ArgExt(Box::new(Sel::parse(b, pos, depth + 1)?), c)
            }
            9 => Sel::Transport(
                Box::new(Sel::parse(b, pos, depth + 1)?),
                Box::new(Sel::parse(b, pos, depth + 1)?),
            ),
            _ => return None,
        })
    }
}
