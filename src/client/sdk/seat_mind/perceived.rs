use crate::anastome::dialogue::seat::Arg;
use num_bigint::BigInt;
use num_rational::BigRational as Q;

/// A COUNSEL the house gave at the latest beat: the call its in-game mind
/// would make, with the worth and cost it states.
#[derive(Clone, Debug, PartialEq)]
pub struct Counsel {
    pub fn_id: u16,
    pub args: Vec<Arg>,
    pub worth: Arg,
    pub cost: Arg,
}

/// WHAT ONE PERCEPT CAME TO.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Perceived {
    /// The house's percept sequence: the seat's beat.
    pub beat: u64,
    /// Records and counsel read.
    pub records: usize,
    pub counsel: usize,
    /// Series held, of them standing still, and returning with a period.
    pub series: usize,
    pub fixed: usize,
    pub orbits: usize,
    /// The distinct periods the returning series hold.
    pub periods: Vec<u64>,
    /// A call whose outcome this percept's records were read against: its
    /// function, and whether the house applied it.
    pub learned: Option<(u16, bool)>,
    /// A call whose outcome never came within its beat.
    pub abandoned: Option<u16>,
    /// 78.156 — the sub-instance's experienced dilation since the percept
    /// before, where both were located (`SeatMind::located`).
    pub dilation: Option<Dilation>,
}

/// 78.156 — WHERE THE MIND'S SUB-INSTANCE STOOD AT A PERCEPT: the house's
/// final TUniversal point (when the mind acted) and the world's axis
/// coordinate `u`, the percept's `local_time` (when in the world it acted).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Located {
    pub point: crate::chronometry::tuniversal::TPoint,
    pub u: Q,
}

/// 78.156 — THE SUB-INSTANCE'S EXPERIENCED DILATION between two located
/// percepts of one origin: the world's advance `du`, the final point's tick
/// advance `dh`, and the exact ratio `du/dh` — none where the final point
/// stood still (`dh` zero: the world moved while TUniversal did not). A
/// reading, never an error: the world keeps its own pace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dilation {
    pub du: Q,
    pub dh: u64,
    pub ratio: Option<Q>,
}

impl Dilation {
    /// The dilation from `before` to `now`; `None` across two origins (the
    /// universe re-originated: no common clock) or where the final point
    /// went back.
    pub fn between(before: &Located, now: &Located) -> Option<Dilation> {
        if before.point.origin != now.point.origin {
            return None;
        }
        let dh = now.point.h.checked_sub(before.point.h)?;
        let du = &now.u - &before.u;
        let ratio = match dh {
            0 => None,
            n => Some(&du / Q::from_integer(BigInt::from(n))),
        };
        Some(Dilation { du, dh, ratio })
    }
}

impl std::fmt::Display for Dilation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "du {} dh {}", self.du, self.dh)?;
        match &self.ratio {
            Some(r) => write!(f, " du/dh {r}"),
            None => write!(f, " du/dh unread (the final point stood)"),
        }
    }
}

impl std::fmt::Display for Perceived {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "beat {} records {} counsel {} | series {} fixed {} orbits {} periods {:?}",
            self.beat, self.records, self.counsel, self.series, self.fixed, self.orbits, self.periods
        )?;
        if let Some((fn_id, applied)) = self.learned {
            write!(f, " | learned fn {fn_id} (applied {applied})")?;
        }
        if let Some(fn_id) = self.abandoned {
            write!(f, " | fn {fn_id} unanswered within its beat")?;
        }
        if let Some(d) = &self.dilation {
            write!(f, " | dilation {d}")?;
        }
        Ok(())
    }
}
