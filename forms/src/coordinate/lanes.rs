// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// LAW-COORDINATE v4 (proposal 78.173 §1) — THE LANES: a lane is a vector of
// exact signed components, each keyed by its axis (H's by its generator, a
// turning plane of two frame units that square alike), and a lane set is a
// list of named lanes, the base S, G, T and H and any lane a ruled opening
// admits.

use crate::Q;
use std::cmp::Ordering;

/// WHY A KEY, A LANE OR A LANE SET IS REFUSED, by name and with what was
/// read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormRefusal {
    /// An axis whose name is not one ASCII capital and a positive index:
    /// the letter and the index as presented.
    AxisUnwritable { letter: char, index: u32 },
    /// A generator named by one axis twice: no plane. The axis.
    NotPlane { axis: Axis },
    /// A key held twice in one lane, or one plane named in both
    /// orientations: the second key read.
    KeyRepeated { key: Key },
    /// An H component keyed by one axis: H's components are keyed by
    /// generators. The key read.
    NotGenerator { key: Key },
    /// An H component keyed by a generator that holds no turning: a frame
    /// plane's own two units, a mixed pair of opposite squares (each a
    /// boost, which stands on S), or axes whose squares are not named
    /// (netstratum docs/117 §10.3). The key read and the plane it names.
    NotTurningPlane { key: Key, plane: Plane },
    /// A lane held twice in one set: the lane.
    LaneRepeated { lane: Lane },
    /// A base lane absent from a set: the lane.
    LaneMissing { lane: Lane },
    /// A lane beyond S, G, T and H that no ruled opening admits: its name
    /// and law version.
    LaneNotOpened { lane: Lane },
}

/// THE SIGN OF A FRAME UNIT'S SQUARE: plane `k`'s positive unit `e₊ₖ`
/// squares to +1 and its negative unit `e₋ₖ` to −1 (78.172 §1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Square {
    Positive,
    Negative,
}

impl Square {
    /// The sign's one writing in a key: `+` or `-`.
    fn sign(self) -> char {
        match self {
            Square::Positive => '+',
            Square::Negative => '-',
        }
    }
}

/// WHAT A GENERATOR'S PLANE HOLDS (netstratum docs/117 §10.3, agreed
/// 2026-11-05): a turning stands only on two units that square alike; a
/// plane of opposite squares holds a boost, which stands on S.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plane {
    /// Two frame units of one square, on two planes: a turning. Positive
    /// is the continuum's `e₊ₖe₊ₗ`, negative the vacuum's `e₋ₖe₋ₗ`, the
    /// inverted face.
    Turning(Square),
    /// One frame plane's own two units, `e₊ₖe₋ₖ`: its boost.
    Frame,
    /// Units of opposite squares on two planes, `e₊ₖe₋ₗ`: a boost.
    Mixed,
    /// An axis named by a letter: no square is named, so no turning is
    /// read.
    Unsigned,
}

/// AN AXIS, written out: named by one ASCII capital and a positive index
/// (`S1`, `G1`), the name the caller's and nothing read into the letter; or
/// a FRAME UNIT, plane `k`'s `e₊ₖ` or `e₋ₖ`, written `+k` or `-k`, whose
/// sign names its square.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Axis {
    letter: char,
    index: u32,
}

impl Axis {
    /// The axis `letter` `index`. Refused `AxisUnwritable` where the letter
    /// is not an ASCII capital or the index is 0, so every axis has one
    /// writing.
    pub fn of(letter: char, index: u32) -> Result<Axis, FormRefusal> {
        match (letter.is_ascii_uppercase(), index) {
            (true, 1..) => Ok(Axis { letter, index }),
            _ => Err(FormRefusal::AxisUnwritable { letter, index }),
        }
    }

    /// PLANE `k`'S UNIT OF SQUARE `square`: `e₊ₖ` (`+k`) or `e₋ₖ` (`-k`).
    /// Refused `AxisUnwritable`, the sign and the index, where `k` is 0.
    pub fn unit(square: Square, k: u32) -> Result<Axis, FormRefusal> {
        match k {
            0 => Err(FormRefusal::AxisUnwritable { letter: square.sign(), index: k }),
            _ => Ok(Axis { letter: square.sign(), index: k }),
        }
    }

    /// The square a frame unit names; `None` for an axis named by a letter.
    pub fn square(&self) -> Option<Square> {
        match self.letter {
            '+' => Some(Square::Positive),
            '-' => Some(Square::Negative),
            _ => None,
        }
    }

    /// The axis's letter: an ASCII capital, or a frame unit's sign.
    pub fn letter(&self) -> char {
        self.letter
    }

    /// The axis's index.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// The writing: the letter or sign, then the index in decimal (`S1`,
    /// `+2`).
    pub fn written(&self) -> String {
        format!("{}{}", self.letter, self.index)
    }
}

/// A COMPONENT'S KEY: one axis (a component of S, G or T, or of an opened
/// lane) or a GENERATOR, the plane named by its two axes in the orientation
/// written (`e_+2+3` the continuum's turning about axis 1, `e_+1-1` frame
/// plane 1's boost). Written `e_` and the axes; keys order by that writing,
/// so the key's order routes and never measures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Key {
    axes: Vec<Axis>,
}

impl Key {
    /// The key of one axis.
    pub fn axis(axis: Axis) -> Key {
        Key { axes: vec![axis] }
    }

    /// THE GENERATOR OF THE PLANE `a ∧ b`, in that orientation. Refused
    /// `NotPlane` where both axes are one.
    pub fn generator(a: Axis, b: Axis) -> Result<Key, FormRefusal> {
        match a == b {
            true => Err(FormRefusal::NotPlane { axis: a }),
            false => Ok(Key { axes: vec![a, b] }),
        }
    }

    /// The axes the key names: one, or a generator's two.
    pub fn axes(&self) -> &[Axis] {
        &self.axes
    }

    /// WHAT THE GENERATOR'S PLANE HOLDS, read from its two units' squares:
    /// a turning where they square alike, the frame plane's boost or a
    /// mixed boost where they do not, unsigned where an axis is named by a
    /// letter. `None` for a key of one axis.
    pub fn plane(&self) -> Option<Plane> {
        let [a, b] = self.axes.as_slice() else { return None };
        Some(match (a.square(), b.square()) {
            (Some(x), Some(y)) if x == y => Plane::Turning(x),
            (Some(_), Some(_)) if a.index == b.index => Plane::Frame,
            (Some(_), Some(_)) => Plane::Mixed,
            _ => Plane::Unsigned,
        })
    }

    /// The writing: `e_` and each axis's writing in turn (`e_S1`,
    /// `e_+2+3`). Each axis opens with its letter or sign, so the writing
    /// reads back as one key.
    pub fn written(&self) -> String {
        let axes: String = self.axes.iter().map(Axis::written).collect();
        format!("e_{axes}")
    }

    /// The key's axes as written, ascending: one plane in either
    /// orientation reads the same.
    fn unoriented(&self) -> Vec<String> {
        let mut axes: Vec<String> = self.axes.iter().map(Axis::written).collect();
        axes.sort();
        axes
    }
}

impl Ord for Key {
    /// BY THE KEY WRITTEN OUT: the canonical order of components.
    fn cmp(&self, other: &Key) -> Ordering {
        self.written().cmp(&other.written())
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A LANE'S VECTOR: its exact signed components, each keyed by its axis,
/// in the canonical order of their keys written out, each key once.
/// Ordered by its components in that order, key then value. Components are
/// never summed into one another.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector {
    components: Vec<(Key, Q)>,
}

impl Vector {
    /// THE VECTOR OF `components`, put in the keys' canonical order.
    /// Refused `KeyRepeated` where a key, or one plane in both
    /// orientations, is held twice.
    pub fn of(mut components: Vec<(Key, Q)>) -> Result<Vector, FormRefusal> {
        components.sort();
        let mut planes: Vec<(Vec<String>, Key)> = components.iter().map(|(k, _)| (k.unoriented(), k.clone())).collect();
        planes.sort();
        let repeated = planes.windows(2).find_map(|w| match w {
            [a, b] if a.0 == b.0 => Some(b.1.clone()),
            _ => None,
        });
        match repeated {
            Some(key) => Err(FormRefusal::KeyRepeated { key }),
            None => Ok(Vector { components }),
        }
    }

    /// The vector of one component.
    pub fn one(key: Key, value: Q) -> Vector {
        Vector { components: vec![(key, value)] }
    }

    /// The components, in the keys' canonical order.
    pub fn components(&self) -> &[(Key, Q)] {
        &self.components
    }

    /// The keys, in canonical order: the lane's shape.
    pub fn keys(&self) -> Vec<Key> {
        self.components.iter().map(|(k, _)| k.clone()).collect()
    }

    /// The component keyed `key`, found by bisection on the keys.
    pub fn get(&self, key: &Key) -> Option<&Q> {
        let at = self.components.binary_search_by(|(k, _)| k.cmp(key)).ok()?;
        self.components.get(at).map(|(_, v)| v)
    }

    /// THE EXACT WRITING: a one-component vector bare, `n/d`; any other
    /// `(n/d,n/d,…)` in key order, each in lowest terms in lowercase hex, a
    /// negative numerator signed `-`. The keys are the lane's shape and
    /// are not written.
    pub fn written(&self) -> String {
        match self.components.as_slice() {
            [(_, v)] => component_text(v),
            all => {
                let parts: Vec<String> = all.iter().map(|(_, v)| component_text(v)).collect();
                format!("({})", parts.join(","))
            }
        }
    }

    /// The two vectors' components joined key by key by `f`; `None` where
    /// their keys differ.
    pub(crate) fn zip(&self, other: &Vector, f: impl Fn(&Q, &Q) -> Q) -> Option<Vector> {
        match self.keys() == other.keys() {
            true => Some(Vector { components: self.components.iter().zip(&other.components).map(|((k, a), (_, b))| (k.clone(), f(a, b))).collect() }),
            false => None,
        }
    }

    /// The vector with the component keyed `key` replaced by `value`;
    /// `None` where no component is keyed `key`.
    pub(crate) fn with(&self, key: &Key, value: Q) -> Option<Vector> {
        let at = self.components.binary_search_by(|(k, _)| k.cmp(key)).ok()?;
        let mut components = self.components.clone();
        let slot = components.get_mut(at)?;
        slot.1 = value;
        Some(Vector { components })
    }
}

/// A component's one writing: `n/d` in lowercase hex, lowest terms, a
/// negative numerator signed `-`.
fn component_text(q: &Q) -> String {
    format!("{}/{}", q.numer().to_str_radix(16), q.denom().to_str_radix(16))
}

/// A LANE: one of the four base lanes, in the network's meanings, or a lane
/// a ruled opening admits, named with its law version. Ordered S, G, T, H,
/// then the opened lanes by name and version.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lane {
    /// Spatial: boost and dilation, the reach; its keys may name boost
    /// planes (a frame plane's `e₊ₖe₋ₖ`, a mixed pair `e₊ₖe₋ₗ`).
    S,
    /// Gauge: change of kind.
    G,
    /// Temporal: null flux, the beat and the passages.
    T,
    /// Holonomic: the turnings between planes, in degrees, each component
    /// keyed by its generator, a turning plane (`e₊ₖe₊ₗ` or `e₋ₖe₋ₗ`).
    H,
    /// A lane beyond the base, admitted only by a ruled opening
    /// ([`Openings`]): its name and the law version that opened it.
    Opened { name: String, version: u16 },
}

impl Lane {
    /// The base lanes, in the canonical order.
    pub const BASE: [Lane; 4] = [Lane::S, Lane::G, Lane::T, Lane::H];
}

/// THE RULED OPENINGS: each lane beyond S, G, T and H that a ruled act
/// agreed between kernels has opened, `(name, law version)`. None is ruled
/// yet; the act itself is the next step (78.173 §7). A lane is never opened
/// by convention: only an entry here admits one.
const RULED: &[(&str, u16)] = &[];

/// THE LANES A SET MAY HOLD BEYOND THE BASE: read from the ruled openings,
/// never stated by a caller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Openings {
    opened: Vec<Lane>,
}

impl Openings {
    /// The openings the network has ruled.
    pub fn ruled() -> Openings {
        Openings { opened: RULED.iter().map(|(name, version)| Lane::Opened { name: (*name).to_string(), version: *version }).collect() }
    }

    /// The lanes opened, in the order ruled.
    pub fn opened(&self) -> &[Lane] {
        &self.opened
    }

    /// Openings stated for a claim of the form: the shape a ruled opening
    /// takes, read in this crate's tests only.
    #[cfg(test)]
    pub(crate) fn stated(opened: Vec<Lane>) -> Openings {
        Openings { opened }
    }

    /// Admitted where `lane` is a base lane or opened here; refused
    /// `LaneNotOpened` naming it otherwise.
    fn admit(&self, lane: &Lane) -> Result<(), FormRefusal> {
        match (lane, self.opened.contains(lane)) {
            (Lane::Opened { .. }, false) => Err(FormRefusal::LaneNotOpened { lane: lane.clone() }),
            _ => Ok(()),
        }
    }
}

/// A LANE SET: named lanes, each with its vector, in the canonical order
/// (S, G, T, H, then the opened lanes), every base lane held once and an
/// opened lane only where a ruled opening admits it. Ordered by the lanes
/// in turn, each by its vector.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lanes {
    held: Vec<(Lane, Vector)>,
}

impl Lanes {
    /// THE SET OF `lanes`, put in canonical order. Refused, naming what was
    /// read: `LaneRepeated`, `LaneMissing` for an absent base lane,
    /// `LaneNotOpened` for a lane `openings` does not admit, `NotGenerator`
    /// for an H component keyed by one axis, and `NotTurningPlane` for an H
    /// component keyed by a plane that holds no turning, naming the plane.
    pub fn of(mut lanes: Vec<(Lane, Vector)>, openings: &Openings) -> Result<Lanes, FormRefusal> {
        lanes.sort();
        let repeated = lanes.windows(2).find_map(|w| match w {
            [a, b] if a.0 == b.0 => Some(b.0.clone()),
            _ => None,
        });
        if let Some(lane) = repeated {
            return Err(FormRefusal::LaneRepeated { lane });
        }
        if let Some(lane) = Lane::BASE.iter().find(|b| !lanes.iter().any(|(l, _)| l == *b)) {
            return Err(FormRefusal::LaneMissing { lane: lane.clone() });
        }
        for (lane, vector) in &lanes {
            openings.admit(lane)?;
            if let Lane::H = lane {
                vector.components().iter().try_for_each(|(key, _)| turning(key))?;
            }
        }
        Ok(Lanes { held: lanes })
    }

    /// THE BASE SET `S, G, T, H`, under the ruled openings.
    pub fn base(s: Vector, g: Vector, t: Vector, h: Vector) -> Result<Lanes, FormRefusal> {
        Lanes::of(vec![(Lane::S, s), (Lane::G, g), (Lane::T, t), (Lane::H, h)], &Openings::ruled())
    }

    /// The lanes and their vectors, in canonical order.
    pub fn held(&self) -> &[(Lane, Vector)] {
        &self.held
    }

    /// The vector on `lane`.
    pub fn get(&self, lane: &Lane) -> Option<&Vector> {
        self.held.iter().find(|(l, _)| l == lane).map(|(_, v)| v)
    }

    /// The place of `lane` in the set's canonical order.
    pub fn place(&self, lane: &Lane) -> Option<usize> {
        self.held.iter().position(|(l, _)| l == lane)
    }

    /// H's components, in key order.
    pub(crate) fn turnings(&self) -> impl Iterator<Item = &(Key, Q)> {
        self.held.iter().filter(|(l, _)| *l == Lane::H).flat_map(|(_, v)| v.components())
    }

    /// THE TWO SETS JOINED COMPONENT BY COMPONENT by `f`. Refused with the
    /// first lane whose shapes differ, and both shapes (a lane absent reads
    /// as no keys).
    pub(crate) fn zip(&self, other: &Lanes, f: impl Fn(&Q, &Q) -> Q) -> Result<Lanes, Shapes> {
        let shape = |s: &Lanes, lane: &Lane| s.get(lane).map(Vector::keys).into_iter().flatten().collect::<Vec<Key>>();
        let differ = |lane: &Lane| Shapes { lane: lane.clone(), held: shape(self, lane), asked: shape(other, lane) };
        if let Some((lane, _)) = other.held.iter().find(|(l, _)| self.get(l).is_none()) {
            return Err(differ(lane));
        }
        let held = self
            .held
            .iter()
            .map(|(lane, v)| other.get(lane).and_then(|w| v.zip(w, &f)).map(|z| (lane.clone(), z)).ok_or_else(|| differ(lane)))
            .collect::<Result<Vec<_>, Shapes>>()?;
        Ok(Lanes { held })
    }

    /// The set with component `key` of `lane` replaced by `value`; `None`
    /// where the set holds no such component.
    pub(crate) fn with(&self, lane: &Lane, key: &Key, value: Q) -> Option<Lanes> {
        let at = self.place(lane)?;
        let mut held = self.held.clone();
        let slot = held.get_mut(at)?;
        slot.1 = slot.1.with(key, value)?;
        Some(Lanes { held })
    }
}

/// AN H KEY READ AS A TURNING PLANE: admitted where its two units square
/// alike; refused `NotGenerator` for one axis and `NotTurningPlane`, naming
/// the plane, for a frame plane, a mixed pair or unsigned axes.
fn turning(key: &Key) -> Result<(), FormRefusal> {
    match key.plane() {
        None => Err(FormRefusal::NotGenerator { key: key.clone() }),
        Some(Plane::Turning(_)) => Ok(()),
        Some(plane) => Err(FormRefusal::NotTurningPlane { key: key.clone(), plane }),
    }
}

/// TWO SHAPES THAT DIFFER: the lane, and the keys each side holds on it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shapes {
    pub lane: Lane,
    pub held: Vec<Key>,
    pub asked: Vec<Key>,
}
