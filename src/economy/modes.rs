// 78.162 — μ AS MODES (the inverse-Sisyphus protocol): the exact motions a
// verified work moved, each on the lane its kind names, each with an exact
// amount, keyed by its canonical coordinates. Nothing is counted.
//
//! A MODE is one exact motion: on S a boost or dilation (its generator's ray,
//! its rational point and side); on G a hue cycle or move; on T a null flux
//! (its ray, and the side it runs); on H a turn (its plane's ray). An AMOUNT
//! is an exact positive rational: on T the count of the flux along its ray,
//! on H the fraction of 360° turned (t/ℓ), on S and G the number of times
//! the mode was moved. Two modes are one where their
//! canonical coordinates are one (A21, law 2): their amounts add. A draw takes
//! modes out, or is refused naming the mode it lacks and both amounts.

use crate::network::nerve::Keyed;
use crate::transducer::cga::Mv;
use crate::transducer::sentence::presentation::GaugeMode;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// THE LANES, by their place (law 5): S, G, T, H.
pub const S: u8 = 0;
pub const G: u8 = 1;
pub const T: u8 = 2;
pub const H: u8 = 3;

/// ONE EXACT MODE, on the lane its kind names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    /// S — a boost or dilation: its generator's canonical ray, its rational
    /// point `c` and its side (the sign of λ against the ray).
    Boost { ray: Mv, c: Q, forward: bool },
    /// G — a hue cycle (its hues from the least, in the order carried) or a
    /// move.
    Hue(GaugeMode),
    /// T — a null flux: its canonical ray and the side it runs.
    Flux { ray: Mv, forward: bool },
    /// H — a turn: its plane's canonical ray. Its amount is the exact
    /// fraction of 360° turned (t/ℓ per turn), so two quarter turns and one
    /// half turn are one holding.
    Turn { ray: Mv },
}

impl Mode {
    /// The lane the mode stands on.
    pub fn lane(&self) -> u8 {
        match self {
            Mode::Boost { .. } => S,
            Mode::Hue(_) => G,
            Mode::Flux { .. } => T,
            Mode::Turn { .. } => H,
        }
    }

    /// THE MODE'S COORDINATES WRITTEN OUT (A21): the lane, then what the mode
    /// is, each length-delimited by its own codec. `None` where a ray has no
    /// canonical writing.
    pub fn key(&self) -> Option<Vec<u8>> {
        let mut k = vec![self.lane()];
        let ray = |k: &mut Vec<u8>, r: &Mv| -> Option<()> {
            r.canonical()?.put_canonical(k);
            Some(())
        };
        match self {
            Mode::Boost { ray: r, c, forward } => {
                ray(&mut k, r)?;
                crate::transducer::alphabet::put_q(&mut k, c);
                k.push(u8::from(*forward));
            }
            Mode::Hue(GaugeMode::Cycle(hues)) => {
                k.push(1);
                k.push(u8::try_from(hues.len()).ok()?);
                k.extend_from_slice(hues);
            }
            Mode::Hue(GaugeMode::Move(a, b)) => {
                k.push(0);
                k.push(*a);
                k.push(*b);
            }
            Mode::Flux { ray: r, forward } => {
                ray(&mut k, r)?;
                k.push(u8::from(*forward));
            }
            Mode::Turn { ray: r } => {
                ray(&mut k, r)?;
            }
        }
        Some(k)
    }

    /// The mode written on the wire: the lane, then the mode's own fields
    /// (rays whole, by `Mv::to_bytes`).
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.push(self.lane());
        match self {
            Mode::Boost { ray, c, forward } => {
                ray.to_bytes(out);
                crate::transducer::alphabet::put_q(out, c);
                out.push(u8::from(*forward));
            }
            Mode::Hue(GaugeMode::Cycle(hues)) => {
                out.push(1);
                out.push(u8::try_from(hues.len()).ok()?);
                out.extend_from_slice(hues);
            }
            Mode::Hue(GaugeMode::Move(a, b)) => {
                out.extend_from_slice(&[0, *a, *b]);
            }
            Mode::Flux { ray, forward } => {
                ray.to_bytes(out);
                out.push(u8::from(*forward));
            }
            Mode::Turn { ray } => {
                ray.to_bytes(out);
            }
        }
        Some(())
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Mode> {
        let byte = |p: &mut usize| -> Option<u8> {
            let x = *b.get(*p)?;
            *p += 1;
            Some(x)
        };
        let side = |p: &mut usize| -> Option<bool> {
            match byte(p)? {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        };
        Some(match byte(p)? {
            S => {
                let ray = Mv::from_bytes(b, p)?;
                let c = crate::transducer::alphabet::take_q(b, p)?;
                Mode::Boost { ray, c, forward: side(p)? }
            }
            G => match byte(p)? {
                1 => {
                    let n = usize::from(byte(p)?);
                    let hues = b.get(*p..*p + n)?.to_vec();
                    *p += n;
                    Mode::Hue(GaugeMode::Cycle(hues))
                }
                0 => Mode::Hue(GaugeMode::Move(byte(p)?, byte(p)?)),
                _ => return None,
            },
            T => {
                let ray = Mv::from_bytes(b, p)?;
                Mode::Flux { ray, forward: side(p)? }
            }
            H => Mode::Turn { ray: Mv::from_bytes(b, p)? },
            _ => return None,
        })
    }
}

/// WHAT A DRAW LACKED: the mode, what was held of it, what was asked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Short {
    pub mode: Mode,
    pub held: Q,
    pub asked: Q,
}

/// μ AS MODES: each mode held once, by its coordinates, with its exact
/// positive amount.
#[derive(Clone, Debug, Default)]
pub struct Modes {
    held: Keyed<(Mode, Q)>,
}

impl PartialEq for Modes {
    /// Two holdings are one where they hold the same modes in the same
    /// amounts — read mode by mode at each coordinate.
    fn eq(&self, other: &Modes) -> bool {
        self.held.len() == other.held.len()
            && self.held.scoped().all(|(k, (_, a))| other.held.get(&k).is_some_and(|(_, b)| a == b))
    }
}

impl Eq for Modes {}

impl Modes {
    pub fn new() -> Modes {
        Modes::default()
    }

    /// Nothing held.
    pub fn is_empty(&self) -> bool {
        self.held.is_empty()
    }

    /// The modes held, each with its amount, in coordinate order.
    pub fn modes(&self) -> impl Iterator<Item = (&Mode, &Q)> {
        self.held.scoped().map(|(_, (m, a))| (m, a))
    }

    /// The modes held on one lane, each with its amount.
    pub fn on_lane(&self, lane: u8) -> impl Iterator<Item = (&Mode, &Q)> {
        self.modes().filter(move |(m, _)| m.lane() == lane)
    }

    /// The amount held of one mode; `None` where it is not held.
    pub fn amount_of(&self, mode: &Mode) -> Option<&Q> {
        self.held.get(&mode.key()?).map(|(_, a)| a)
    }

    /// ADD AN AMOUNT OF ONE MODE: composed with what is held at its
    /// coordinates. A zero or negative amount, or a mode with no coordinates,
    /// adds nothing and says so.
    pub fn add_mode(&mut self, mode: Mode, amount: Q) -> bool {
        if !amount.is_positive() {
            return false;
        }
        let Some(k) = mode.key() else { return false };
        let total = match self.held.get(&k) {
            Some((_, held)) => held + &amount,
            None => amount,
        };
        self.held.insert(&k, (mode, total));
        true
    }

    /// ADD EVERY MODE OF ANOTHER HOLDING.
    pub fn add(&mut self, other: &Modes) {
        for (m, a) in other.modes() {
            self.add_mode(m.clone(), a.clone());
        }
    }

    /// TAKE ANOTHER HOLDING OUT, WHOLE OR NOT AT ALL: every mode asked is held
    /// in at least the amount asked, or nothing is taken and the first mode
    /// short — in coordinate order — is named with both amounts.
    pub fn take(&mut self, asked: &Modes) -> Result<(), Box<Short>> {
        for (k, (m, want)) in asked.held.scoped() {
            let have = match self.held.get(&k) {
                Some((_, a)) => a.clone(),
                None => Q::zero(),
            };
            if &have < want {
                return Err(Box::new(Short { mode: m.clone(), held: have, asked: want.clone() }));
            }
        }
        for (k, (_, want)) in asked.held.scoped() {
            let Some((m, have)) = self.held.get(&k).cloned() else { continue };
            let left = have - want;
            match left.is_zero() {
                true => {
                    self.held.remove(&k);
                }
                false => {
                    self.held.insert(&k, (m, left));
                }
            }
        }
        Ok(())
    }

    /// THE MODES OF ONE MOTION, at `mass` times its amount: each mode of its
    /// action read exactly (`Coordinates::of`) and set on the lane its kind
    /// names — a turn by its plane, order and side with `t` turns; a null
    /// flux by its ray and side with its count; a boost by its ray, point
    /// and side once. `None` where the motion's modes do not read (rule 11);
    /// the identity is no mode at all.
    pub fn of_motion(motion: &Mv, mass: &Q, budget: &mut crate::physics::Budget) -> Option<Modes> {
        use crate::transducer::sentence::coordinates::{Coordinates, Winding};
        let mut out = Modes::new();
        if crate::transducer::sentence::presentation::is_still(motion) {
            return Some(out);
        }
        let coords = Coordinates::of(motion, budget).ok()?;
        for h in coords.holes {
            let ray = h.generator.clone();
            let (mode, amount) = match h.winding {
                Winding::Turn { t, ell } => (Mode::Turn { ray }, Q::new(t.into(), ell.into())),
                Winding::Count(l) => (Mode::Flux { ray, forward: !l.is_negative() }, l.abs()),
                Winding::Named { c, lambda } => (Mode::Boost { ray, c, forward: !lambda.is_negative() }, Q::from_integer(1.into())),
            };
            out.add_mode(mode, amount * mass);
        }
        Some(out)
    }

    /// THE MODES OF ONE HUE MAP, at `mass` times: each cycle and each move
    /// once. `None` where the listing is no map.
    pub fn of_gauge(gauge: &[(u8, u8)], mass: &Q) -> Option<Modes> {
        let mut out = Modes::new();
        for m in crate::transducer::sentence::presentation::gauge_modes(gauge)? {
            out.add_mode(Mode::Hue(m), mass.clone());
        }
        Some(out)
    }

    /// THE HOLDING ON THE WIRE: the count of modes, then each mode and its
    /// amount, in coordinate order. `None` where a field passes its carrier.
    pub fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.extend_from_slice(&u32::try_from(self.held.len()).ok()?.to_le_bytes());
        for (_, (m, a)) in self.held.scoped() {
            m.put(out)?;
            crate::transducer::alphabet::put_q(out, a);
        }
        Some(())
    }

    /// The holding read back, whole or refused: every amount positive, every
    /// mode once.
    pub fn take_from(b: &[u8], p: &mut usize) -> Option<Modes> {
        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
        *p += 4;
        let mut out = Modes::new();
        for _ in 0..n {
            let m = Mode::take(b, p)?;
            let a = crate::transducer::alphabet::take_q(b, p)?;
            let k = m.key()?;
            if !a.is_positive() || out.held.get(&k).is_some() {
                return None;
            }
            out.held.insert(&k, (m, a));
        }
        Some(out)
    }

    /// THE HOLDING AS A PERSON READS IT, mode by mode, never a count: each
    /// lane's modes with their exact amounts — a turn as its t/ℓ of 360°.
    pub fn print(&self) -> String {
        let name = |m: &Mode, a: &Q| match m {
            Mode::Boost { c, forward, .. } => format!("S boost c={c} {} ×{a}", if *forward { "+" } else { "−" }),
            Mode::Hue(GaugeMode::Cycle(h)) => format!("G cycle {h:?} ×{a}"),
            Mode::Hue(GaugeMode::Move(x, y)) => format!("G move {x}→{y} ×{a}"),
            Mode::Flux { forward, .. } => format!("T flux {} {a}", if *forward { "+" } else { "−" }),
            Mode::Turn { .. } => format!("H turn {a} of 360°"),
        };
        match self.is_empty() {
            true => "no modes".to_string(),
            false => self.modes().map(|(m, a)| name(m, a)).collect::<Vec<_>>().join(" · "),
        }
    }
}
