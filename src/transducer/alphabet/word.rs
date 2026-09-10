// 1.3, 16.2 — THE WORD: a path through the alphabet, and its normal form.
//
//! A word is a sequence of letters with orientation. `run_reduce` carries a
//! tape to the normal form the relators define; the contact rides on the
//! word's key, so a rebased word is a different vertex.

use num_traits::{One, Zero};
use crate::transducer::alphabet::{
    complementary, letter, letter_inverse, plane_of, LetterId, PlaneForm, Rotor,
};

use super::*;

/// Boundary contact of a word form: apply until the image's ring distance
/// to `anchor` equals `ring`. On the form and in its key; not a letter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Contact {
    pub anchor: u16,
    pub ring: u8,
}

/// A WORD: an oriented tape of letters. `true` reads the letter backwards
/// (its carried `Φ⁻¹`). The stored tape is the geodesic normal form.
/// Equal words compare by rotor projective equality, never by a tape or a digest.
/// Contact, ρ, placement, and gauge live on the word *form* (`Vertex::Word`),
/// never in this equality.
#[derive(Debug)]
pub struct Word(Vec<(LetterId, bool)>, std::sync::OnceLock<Rotor>);

impl Default for Word {
    fn default() -> Self {
        Word(Vec::new(), std::sync::OnceLock::new())
    }
}

impl Clone for Word {
    fn clone(&self) -> Self {
        let w = Word(self.0.clone(), std::sync::OnceLock::new());
        if let Some(r) = self.1.get() {
            let _ = w.1.set(r.clone());
        }
        w
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        // M1: vertex equality is A1 of the rotor (λ ∈ ℚ×, either sign).
        self.rotor().projective_eq(&other.rotor())
    }
}
impl Eq for Word {}

impl Word {
    pub(crate) fn raw(tape: Vec<(LetterId, bool)>) -> Word {
        Word(tape, std::sync::OnceLock::new())
    }

    pub fn identity() -> Word {
        Word::raw(Vec::new())
    }

    pub fn from_letters(tape: &[(LetterId, bool)]) -> Word {
        Word::raw(tape.to_vec())
    }

    /// A word that carries an ACTION and no letters: the alphabet does
    /// not spell it, and the rotor is the whole of what it is. This is
    /// what an adjoined generator carries (37.7) — a letter named by
    /// content (law 6), with no escape byte and no place in the
    /// seventeen.
    pub fn formed(r: Rotor) -> Word {
        let w = Word::raw(Vec::new());
        let _ = w.1.set(r);
        w
    }

    pub fn letters(&self) -> &[(LetterId, bool)] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, id: LetterId, inverse: bool) {
        self.0.push((id, inverse));
        self.1.take();
    }

    /// M1 key: A1 of the rotor. Mixed-plane spellings of one rotor share
    /// this key. Contact is not here.
    pub fn key(&self) -> crate::transducer::address::Address {
        self.rotor().key()
    }

    /// Group identity: the rotor is projectively 1.
    pub fn is_identity(&self) -> bool {
        self.rotor().projective_eq(&Rotor::identity())
    }

    /// `by · self · by⁻¹`, then the normal form.
    pub fn rebase(&self, by: &Word) -> Word {
        by.compose(&self.compose(&by.inverse())).normal_form()
    }

    /// Same conjugacy class: rotor invariants first (15.3). The plane form
    /// is a sensor section, not the identity of the class.
    pub fn same_class(&self, other: &Word) -> bool {
        match (self.rotor().class_invariants(), other.rotor().class_invariants()) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    /// Conjugacy coordinates written out. Rotor class first; the plane
    /// form is not the key. None when the rotor is not a versor: a
    /// non-versor is never a class.
    pub fn class_key(&self) -> Option<crate::transducer::address::Address> {
        self.rotor().class_key()
    }

    /// Shortest tape for this vertex: unit exponents mod 4, primes as
    /// spelled, planes in slot order.
    pub fn geodesic(&self) -> Word {
        self.normal_form()
    }

    /// The formed product `γ` of the tape, built once and memoised.
    pub fn rotor(&self) -> Rotor {
        self.1
            .get_or_init(|| {
                self.0.iter().fold(Rotor::identity(), |acc, &(id, inv)| {
                    acc.geo(&if inv { letter_inverse(id) } else { letter(id) })
                })
            })
            .clone()
    }

    /// The word read backwards: the inverse transport.
    pub fn inverse(&self) -> Word {
        Word::raw(self.0.iter().rev().map(|&(id, inv)| (id, !inv)).collect())
    }

    /// Concatenate tapes: `self` then `other` (last letter of `other` acts first).
    pub fn compose(&self, other: &Word) -> Word {
        let mut t = self.0.clone();
        t.extend_from_slice(&other.0);
        Word::raw(t)
    }

    /// On the sensor plane (its letters and the sheet's, 1.2): the A3 tape.
    /// Anywhere else: A2 run reduction. The key is A1 of the rotor either
    /// way; a mixed-plane word is never keyed as 1. A word on another plane
    /// keeps the sensor plane closed and its sensor form is the identity,
    /// so the A3 path would write it as the empty tape; the letters decide
    /// the path, never the sensor's shadow (found by L2, 50.4).
    pub fn normal_form(&self) -> Word {
        let sensor = (PLANE_ROW, PLANE_COL);
        let sheet = (PLANE_COL, 2);
        let on_sensor = self.0.iter().all(|(id, _)| plane_of(*id).is_none_or(|p| p == sensor || p == sheet));
        if on_sensor {
            if let Some(f) = self.form() {
                return f.to_word();
            }
        }
        run_reduce(self.letters())
    }

    /// The plane group is rotations ⋊ orientation, not a commuting tape.
    /// `None` when the word does not preserve the sensor plane.
    pub fn form(&self) -> Option<PlaneForm> {
        let b = self.plane_block()?;
        let det = &b[0][0] * &b[1][1] - &b[0][1] * &b[1][0];
        let rho2 = &b[0][0] * &b[0][0] + &b[1][0] * &b[1][0];
        if rho2.is_zero() {
            return None;
        }
        let ori = det / &rho2;
        let orientation = if ori.is_one() {
            false
        } else if (-&ori).is_one() {
            true
        } else {
            return None;
        };
        let mut r = b;
        if orientation {
            r[0][1] = -&r[0][1];
            r[1][1] = -&r[1][1];
        }
        let sp = spell(&r)?;
        let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
        for &(id, inv) in sp.word.letters() {
            rot[id.raw() as usize] += if inv { -1 } else { 1 };
        }
        let mut f = PlaneForm { orientation, rot };
        reduce_unit_exponents(&mut f);
        Some(f)
    }

    /// Two words are the same vertex: rotor first. A missing plane form is
    /// not an exception when the rotors agree.
    pub fn same_form(&self, other: &Word) -> bool {
        if !self.rotor().projective_eq(&other.rotor()) {
            return false;
        }
        match (self.form(), other.form()) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        }
    }

    /// The plane element is the identity: even sheet, trivial rotation.
    pub fn abelian_identity(&self) -> bool {
        self.form().is_some_and(|f| f.is_identity())
    }

    /// THE ORDER OF THE WORD'S ROTATION, read on the rotor (15.3, ruled
    /// 2026-09-09) and never on the sensor plane's shadow: the period of
    /// the rotor's two commuting quaternion factors (`Rotor::order`). Read
    /// on the plane form, a word off the sensor plane — which keeps that
    /// plane closed and reads the identity there — answered 1 or an
    /// assumed 4, and two quarter turns on planes sharing an axis, which
    /// close in three, were never a cycle. A word is a product of letters,
    /// each a versor of positive weight, so its rotor has a weight; an even
    /// element with none has `|q₊| ≠ |q₋|` and no power that is a scalar,
    /// which is infinite order by the same arithmetic — not a default.
    pub fn order(&self) -> Order {
        match self.rotor().order() {
            Some(order) => order,
            None => Order::Infinite,
        }
    }

    /// The exact 4×4 matrix of the word on the chart, columns the images
    /// of `e0..e3`.
    pub fn matrix(&self) -> Option<[[Q; 4]; 4]> {
        let r = self.rotor();
        let mut cols: Vec<[Q; 4]> = Vec::with_capacity(4);
        for k in 0..4 {
            let e: [Q; 4] = std::array::from_fn(|i| if i == k { Q::one() } else { Q::zero() });
            cols.push(r.carry(&e)?);
        }
        Some(std::array::from_fn(|row| std::array::from_fn(|col| cols[col][row].clone())))
    }

    /// The word's action on the sensor's plane `(e0, e1)` as a 2×2 block —
    /// only when the word preserves that plane (no third axis mixed in).
    pub fn plane_block(&self) -> Option<[[Q; 2]; 2]> {
        self.block_on((PLANE_ROW, PLANE_COL))
    }

    /// The word's action on the pair plane `(i, j)` as a 2×2 block, columns
    /// the images of `e_i` and `e_j` — only when the word keeps that plane
    /// closed (50.4, A22).
    pub fn block_on(&self, plane: (u8, u8)) -> Option<[[Q; 2]; 2]> {
        rotor_block_on(&self.rotor(), plane)
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.0.len() as u16).to_le_bytes());
        for &(id, inv) in &self.0 {
            out.push(id.raw() | if inv { 0x80 } else { 0 });
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Word> {
        let n = u16::from_le_bytes(b.get(*pos..*pos + 2)?.try_into().ok()?) as usize;
        *pos += 2;
        let mut tape = Vec::with_capacity(n);
        for _ in 0..n {
            let byte = *b.get(*pos)?;
            *pos += 1;
            tape.push((LetterId::try_new(byte & 0x7f)?, byte & 0x80 != 0));
        }
        Some(Word::raw(tape))
    }
}

/// Reduce quarter-turn exponents mod 4; three quarters is one inverse.
pub(crate) fn reduce_unit_exponents(f: &mut PlaneForm) {
    for i in 1..=6 {
        let mut n = f.rot[i as usize].rem_euclid(4);
        if n == 3 {
            n = -1;
        }
        f.rot[i as usize] = n;
    }
}

/// The canonical tape of a run on one plane (A2 step 1): the sensor plane
/// through its plane form, any other pair plane through its own peel
/// (50.4). A run read through the sensor's form alone was erased when its
/// plane was not the sensor's — its sensor block is the identity — which
/// turned a word on `(2,3)` into the empty word (found by L2's A22 test).
pub(crate) fn plane_nf(w: &Word) -> Word {
    let Some(plane) = w.letters().first().and_then(|(id, _)| plane_of(*id)) else {
        return w.clone();
    };
    if plane == (PLANE_ROW, PLANE_COL) {
        return match w.form() {
            Some(f) => f.to_word(),
            None => w.clone(),
        };
    }
    let Some(block) = w.block_on(plane) else {
        return w.clone();
    };
    match spell_peel_on(plane, &block) {
        Some((crate::transducer::peel::PeelNest::Closed { cores }, rho)) if rho.is_one() => {
            let mut tape = Word::identity();
            for syllable in cores.iter().rev() {
                for &(id, inv) in syllable {
                    tape.push(id, inv);
                }
            }
            if tape.block_on(plane).as_ref() == Some(&block) {
                tape
            } else {
                w.clone()
            }
        }
        _ => w.clone(),
    }
}

/// A2 step 1: split into per-plane runs, reduce each, commute complementary
/// planes until a same-plane merge, never rewrite as the identity.
pub(crate) fn run_reduce(tape: &[(LetterId, bool)]) -> Word {
    let tape: Vec<(LetterId, bool)> = tape.iter().copied().filter(|(id, _)| id.raw() != 0).collect();
    if tape.is_empty() {
        return Word::identity();
    }
    let mut runs: Vec<Word> = Vec::new();
    let mut cur: Vec<(LetterId, bool)> = Vec::new();
    let mut cur_plane: Option<(u8, u8)> = None;
    for &ltr in &tape {
        let p = plane_of(ltr.0);
        if cur.is_empty() || p == cur_plane {
            cur.push(ltr);
            cur_plane = p;
        } else {
            runs.push(plane_nf(&Word::from_letters(&cur)));
            cur = vec![ltr];
            cur_plane = p;
        }
    }
    if !cur.is_empty() {
        runs.push(plane_nf(&Word::from_letters(&cur)));
    }
    runs.retain(|w| !w.is_empty());
    loop {
        let n = runs.len();
        let mut merged = false;
        let mut i = 1usize;
        while i < runs.len() {
            let pi = runs[i].letters().first().and_then(|(id, _)| plane_of(*id));
            let Some(pi) = pi else {
                i += 1;
                continue;
            };
            let mut j = i;
            let mut gap_ok = true;
            while j > 0 {
                j -= 1;
                let pj = runs[j].letters().first().and_then(|(id, _)| plane_of(*id));
                let Some(pj) = pj else {
                    gap_ok = false;
                    break;
                };
                if pj == pi {
                    break;
                }
                if !complementary(pj, pi) {
                    gap_ok = false;
                    break;
                }
                if j == 0 && pj != pi {
                    gap_ok = false;
                }
            }
            if gap_ok && j < i && runs[j].letters().first().and_then(|(id, _)| plane_of(*id)) == Some(pi) {
                let piece = runs.remove(i);
                let joined = runs[j].compose(&piece);
                runs[j] = plane_nf(&joined);
                if runs[j].is_empty() {
                    runs.remove(j);
                }
                merged = true;
                break;
            }
            i += 1;
        }
        if !merged || runs.len() >= n {
            break;
        }
        runs.retain(|w| !w.is_empty());
    }
    runs.into_iter().fold(Word::identity(), |a, b| a.compose(&b))
}

/// The order of a rotation: the period of its rotor's class (15.3), or
/// infinite when a factor is not a root of unity. Finite sorts before
/// Infinite; among finite, smaller ℓ first. Wire: 0 = infinite, else ℓ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Finite(u32),
    Infinite,
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Order::Finite(a), Order::Finite(b)) => a.cmp(b),
            (Order::Finite(_), Order::Infinite) => std::cmp::Ordering::Less,
            (Order::Infinite, Order::Finite(_)) => std::cmp::Ordering::Greater,
            (Order::Infinite, Order::Infinite) => std::cmp::Ordering::Equal,
        }
    }
}

impl Order {
    /// Wire encoding: 0 = infinite, else the finite period.
    pub fn to_u32(self) -> u32 {
        match self {
            Order::Finite(ell) => ell,
            Order::Infinite => 0,
        }
    }

    pub fn from_u32(v: u32) -> Self {
        if v == 0 {
            Order::Infinite
        } else {
            Order::Finite(v)
        }
    }
}

pub(crate) fn gcd_u32(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}
