// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 12.6, A18 — THE AXES A CLASS OCCUPIES, and the rank audit's reading of
//! what a pivot did to them.

/// declaration order is the walk order and is not a preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Orientation(u8),
    Unit(u8),
    Prime5(u8),
    Prime13(u8),
    Hue,
    Placement,
    Dilation,
    OffPlane,
    Open,
}

impl Axis {
    /// One tape byte: the kind in the high bits, the slot plane in the low.
    pub(crate) fn to_byte(self) -> u8 {
        match self {
            Axis::Orientation(p) => p,
            Axis::Unit(p) => (1 << 3) | p,
            Axis::Prime5(p) => (2 << 3) | p,
            Axis::Prime13(p) => (3 << 3) | p,
            Axis::Hue => 4 << 3,
            Axis::Placement => 5 << 3,
            Axis::Dilation => 6 << 3,
            Axis::OffPlane => 7 << 3,
            Axis::Open => 8 << 3,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Axis::Orientation(_) => "orientation",
            Axis::Unit(_) => "unit",
            Axis::Prime5(_) => "prime5",
            Axis::Prime13(_) => "prime13",
            Axis::Hue => "hue",
            Axis::Placement => "placement",
            Axis::Dilation => "dilation",
            Axis::OffPlane => "offplane",
            Axis::Open => "open",
        }
    }

    /// THE LANE THIS AXIS CARRIES CONTENT ON — 42.6's table, which is
    /// 16.16's definition of the pressure applied to 12.6's coordinates:
    /// `𝒮` the laminae a word fails to speak, `𝒢` the hue transitions it
    /// fails to carry, `𝒯` the extent held in receipts, `𝒽` the shells it
    /// spans.
    ///
    /// - `Placement` is the translator and `Dilation` the dilator. Both say
    ///   where the support is and how large, which is the atlas: `𝒮`.
    /// - `Hue` is the gauge map, and the fibre over the word is the gauge
    ///   cover, so the fibre's sections and torsion cycles sit here too:
    ///   `𝒢`. The physics' own test is the proof — a two-lamina recolour
    ///   reads `[false, true, false, false]`.
    /// - The plane slots — orientation, unit, and the two prime exponents
    ///   on each of the six planes — and `OffPlane` are the ROTOR's own
    ///   coordinates (15.3). What the orbit turns by is the nerve: `𝒽`.
    /// - `Open` is an OPENED AXIS, and 42.6's table puts it on `𝒢` with
    ///   the fibre's sections and torsion cycles: the gauge cover is the
    ///   fibre over the word, and opening an axis widens that fibre. It is
    ///   not a coordinate of the class VECTOR — `occupies` returns false
    ///   for it and says why — but it is content on a lane, and the lane
    ///   is the one whose cover it moved.
    ///
    /// NO AXIS IS TEMPORAL, and that is not an omission. 16.16 reads `𝒯`
    /// as the extent held in RECEIPTS, and `measure_lanes` writes lane 2
    /// as zero with that reason at the line. It is why every `uncarried`
    /// vector the cover has read has the shape `[𝒮, 0, 0, 𝒽]`.
    pub fn lane(self) -> usize {
        match self {
            Axis::Placement | Axis::Dilation => 0,
            Axis::Hue | Axis::Open => 1,
            Axis::Orientation(_)
            | Axis::Unit(_)
            | Axis::Prime5(_)
            | Axis::Prime13(_)
            | Axis::OffPlane => 3,
        }
    }

    pub(crate) fn from_byte(b: u8) -> Option<Axis> {
        let p = b & 7;
        match (b >> 3, p) {
            (0, p) if p < 6 => Some(Axis::Orientation(p)),
            (1, p) if p < 6 => Some(Axis::Unit(p)),
            (2, p) if p < 6 => Some(Axis::Prime5(p)),
            (3, p) if p < 6 => Some(Axis::Prime13(p)),
            (4, 0) => Some(Axis::Hue),
            (5, 0) => Some(Axis::Placement),
            (6, 0) => Some(Axis::Dilation),
            (7, 0) => Some(Axis::OffPlane),
            (8, 0) => Some(Axis::Open),
            _ => None,
        }
    }
}

/// A18: the dimension audit of a nerve or form, a vector never summed.
/// `torsion` counts the open finite-order orbits; `free` is the ℤ-rank of
/// the plane free parts alone; `offplane` is the number of distinct open
/// off-plane rotor classes; `axes` the chart axes opened by discharge.
///
/// Counts of distinct classes, so `u64` (STD1 3b's rule, the provenance
/// ruling: classes are the mesh's, never a width's).
#[derive(Clone, Copy, Debug, Default)]
pub struct Rank {
    pub torsion: u64,
    pub free: u64,
    pub offplane: u64,
    pub axes: u64,
}

/// ONE COORDINATE of the rank audit, by name. A refusal says which
/// coordinate refused it; a number in that sentence would say nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RankAxis {
    Torsion,
    Free,
    OffPlane,
    Axes,
}

/// WHAT A PIVOT DID TO THE RANK — three states and no fourth (S9.1, 12.6,
/// T21 (iii)).
///
/// The predicate this replaces was a four-coordinate monotone: it admitted
/// `(−3, −2, −1, +7)`, a move that emptied three coordinates and opened
/// seven axes, as readily as the one unit A18 describes. A pivot moves ONE
/// axis; a move that changes four coordinates at once is not a pivot that
/// went well, it is a different thing that the audit could not name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RankMove {
    /// The rank did not move. The pivot is certified by its closure alone
    /// — the coordinate it acted on is zero and no other rose — and pays
    /// for itself without touching the audit.
    Equal,
    /// ONE UNIT moved out of `from`: exactly one of torsion, free or
    /// off-plane fell by one and the other two are unchanged. This is A18's
    /// step and the only rank move a pivot makes.
    ///
    /// `axes` rises by that unit when the discharge OPENED a chart axis, and
    /// holds when it did not — a held word written onto a chord closes the
    /// orbit without widening the form's scope. Both are this arm: the unit
    /// left `from`, and where it went is the destination's business.
    ///
    /// FINDING (S9.1, 2026-09-08): `rank_of_rows` writes `axes: 0`
    /// unconditionally, so no rank the engine computes has ever carried an
    /// opened axis and no move can put a unit INTO axes. T21 (iii) reads
    /// "one unit from torsion, free or off-plane into axes"; the second half
    /// is unreachable until S9.17 gives `Axis::Open` a candidate and the
    /// reader a value to count. Requiring it here refuses every pivot in the
    /// tree — measured: five chapter-12 tests, every discharge.
    Discharged { from: RankAxis },
    /// Anything else, carried whole so a refusal can say what happened:
    /// `after − before` per coordinate, in the order torsion, free,
    /// off-plane, axes.
    Inadmissible { delta: [i64; 4] },
}

impl Rank {
    /// What the pivot did to the audit (12.6, R2, T21 (iii)). Reads the
    /// four coordinates and names the move; nothing here decides, and the
    /// trial is the site that refuses on `Inadmissible`.
    pub fn moved(before: Rank, after: Rank) -> RankMove {
        let delta = [
            after.torsion as i64 - before.torsion as i64,
            after.free as i64 - before.free as i64,
            after.offplane as i64 - before.offplane as i64,
            after.axes as i64 - before.axes as i64,
        ];
        if delta == [0; 4] {
            return RankMove::Equal;
        }
        // ONE unit out of ONE coordinate. `axes` rises by that unit if the
        // discharge opened a chart axis, or holds if it did not; nothing
        // else may move, and a fall of two is not a smaller fall of one.
        let from = match delta {
            [-1, 0, 0, 0 | 1] => Some(RankAxis::Torsion),
            [0, -1, 0, 0 | 1] => Some(RankAxis::Free),
            [0, 0, -1, 0 | 1] => Some(RankAxis::OffPlane),
            _ => None,
        };
        match from {
            Some(from) => RankMove::Discharged { from },
            None => RankMove::Inadmissible { delta },
        }
    }
}
