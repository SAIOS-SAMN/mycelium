// 16.3, 9.1 — THE PLANE FORM: the coefficient group of a word's planes,
// and the conjugacy class it belongs to.
//
//! Two words are conjugate exactly when their plane forms are, and the
//! conjugacy key writes those coefficients out. The order of a form is read
//! from the coefficients, never assumed and never capped.


use super::*;

/// The plane group element a word represents: orientation and the
/// rotation in (e0, e1). Rotation exponents are those of the unflipped
/// spelling; a half-turn conjugates a rotation to its inverse, so this is
/// not the commuting sum of the tape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaneForm {
    pub orientation: bool,
    pub rot: [i64; CAYLEY_ALPHABET_LEN],
}

impl Default for PlaneForm {
    fn default() -> Self {
        PlaneForm { orientation: false, rot: [0; CAYLEY_ALPHABET_LEN] }
    }
}

impl PlaneForm {
    pub fn identity() -> PlaneForm {
        PlaneForm::default()
    }

    pub fn is_identity(&self) -> bool {
        !self.orientation && self.rot.iter().all(|&x| x == 0)
    }

    /// Occupied plane coordinates in 12.6 slot order: per slot plane,
    /// (orientation on the sensor plane, unit in Z/4, prime-5, prime-13).
    /// Read from the conjugacy representative; orientation lives on slot 0.
    pub fn coordinates(&self) -> [(bool, u32, i64, i64); 6] {
        let rep = self.conjugacy_rep();
        let mut plane = [(false, 0u32, 0i64, 0i64); 6];
        for (p, slot) in plane.iter_mut().enumerate() {
            let unit = rep.rot[1 + p].rem_euclid(4) as u32;
            let m5 = rep.rot[7 + p];
            let m13 = rep.rot.get(13 + p).copied().filter(|_| p < 4).unwrap_or(0);
            *slot = (rep.orientation && p == 0, unit, m5, m13);
        }
        plane
    }

    /// Canonical conjugacy representative. Even sheet: `{r, −r}` with the
    /// first nonzero component positive. Odd sheet: each exponent mod 2
    /// in `{0,1}`. Compared as coefficients, never hashed.
    pub fn conjugacy_rep(&self) -> PlaneForm {
        if self.orientation {
            let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
            for (i, &n) in self.rot.iter().enumerate() {
                rot[i] = n.rem_euclid(2);
            }
            PlaneForm { orientation: true, rot }
        } else {
            let neg: [i64; CAYLEY_ALPHABET_LEN] = std::array::from_fn(|i| -self.rot[i]);
            let pick_neg = match self.rot.iter().zip(neg.iter()).find(|(&a, &b)| a != b) {
                Some((&a, _)) => a < 0,
                None => false,
            };
            PlaneForm { orientation: false, rot: if pick_neg { neg } else { self.rot } }
        }
    }

    /// Same conjugacy class: the representatives agree as coefficients.
    pub fn conjugacy_eq(&self, other: &PlaneForm) -> bool {
        self.conjugacy_rep() == other.conjugacy_rep()
    }

    /// Conjugacy representative written out. Class identity is [`conjugacy_eq`].
    pub fn conjugacy_key(&self) -> crate::transducer::a21::Address {
        let r = self.conjugacy_rep();
        let mut buf = vec![crate::transducer::a21::TAG_PLANE, r.orientation as u8];
        for &n in &r.rot {
            buf.extend_from_slice(&n.to_le_bytes());
        }
        crate::transducer::a21::Address::new(buf)
    }

    /// Plane-group product: rotations ⋊ orientation. A trailing half-turn
    /// conjugates the leading rotation to its inverse.
    pub fn compose(&self, other: &PlaneForm) -> PlaneForm {
        let orientation = self.orientation ^ other.orientation;
        let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
        if other.orientation {
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = -self.rot[i] + other.rot[i];
            }
        } else {
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = self.rot[i] + other.rot[i];
            }
        }
        PlaneForm { orientation, rot }
    }

    pub fn inverse(&self) -> PlaneForm {
        if self.orientation {
            *self
        } else {
            let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = -self.rot[i];
            }
            PlaneForm { orientation: false, rot }
        }
    }

    /// The half-turn acting on exponents: `S R S⁻¹ = R⁻¹`.
    pub fn conjugate_by(&self, by: &PlaneForm) -> PlaneForm {
        if !by.orientation {
            *self
        } else if self.orientation {
            let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = -self.rot[i];
            }
            PlaneForm { orientation: true, rot }
        } else {
            let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
            for i in 0..CAYLEY_ALPHABET_LEN {
                rot[i] = -self.rot[i];
            }
            PlaneForm { orientation: false, rot }
        }
    }

    /// Half-turn on (e1,e2): the column flip of the sensor plane, two
    /// quarter-turns. The orientation letter of the plane group; there is
    /// no reflection letter.
    pub fn sheet_word() -> Word {
        let id = LetterId::quarter((1, 2)).expect("plane (1,2) is a letter");
        Word::raw(vec![(id, false), (id, false)])
    }

    pub fn to_word(&self) -> Word {
        let mut w = if self.orientation {
            Self::sheet_word()
        } else {
            Word::identity()
        };
        for (i, &n) in self.rot.iter().enumerate() {
            if n == 0 {
                continue;
            }
            let Some(id) = LetterId::try_new(i as u8) else { continue };
            if id.raw() == 0 {
                continue;
            }
            let inv = n < 0;
            for _ in 0..n.unsigned_abs() {
                w.push(id, inv);
            }
        }
        w
    }

    pub fn key(&self) -> crate::transducer::a21::Address {
        let mut buf = vec![crate::transducer::a21::TAG_PLANE, self.orientation as u8];
        for &n in &self.rot {
            buf.extend_from_slice(&n.to_le_bytes());
        }
        crate::transducer::a21::Address::new(buf)
    }

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        out.push(self.orientation as u8);
        for &n in &self.rot {
            out.extend_from_slice(&n.to_le_bytes());
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<PlaneForm> {
        let orientation = *b.get(*pos)? != 0;
        *pos += 1;
        let mut rot = [0i64; CAYLEY_ALPHABET_LEN];
        for n in &mut rot {
            *n = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
            *pos += 8;
        }
        Some(PlaneForm { orientation, rot })
    }
}
