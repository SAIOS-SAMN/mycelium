// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-HOSTING §5a.7, §5a.9 B4; ROADMAP track 24 — THE BOUNDARY READING:
// every refusal this court answered at a universe's universal core tube, by
// domain, by act and by the refusal's name (`NotThroughCore`, `NotSignatory`,
// `Bounded`, `Unaligned`, and a fold's `NotLawful`), served under
// `ACT_OBSERVE`. Written where the court refuses; read only at a question and
// never read back by the court, so it touches nothing in consensus.
//
//! A row is one domain's one act refused by one name: the last refusal
//! whole (the clause and the reserve's reading of it, or the laws a fold
//! broke), the court's height when it was answered, and how many times
//! since boot. Rows are kept only for domains this court holds (every site
//! that refuses at the tube has read the domain from its own registries
//! first), so the reading is bounded by the domains founded here, the seven
//! acts and the five names.

use crate::court::admission::{Boundary, UniverseAct};
use crate::network::nerve::Keyed;
use crate::physics::laws::{Law, LAWS};
use std::sync::{LazyLock, Mutex};

/// THE NAMES A CROSSING IS REFUSED BY at the tube, at the code each row
/// carries: the four of [`Boundary`] by its wire tag, then `NotLawful`.
pub const TUBE_REFUSALS: [&str; 5] = ["NotThroughCore", "NotSignatory", "Bounded", "Unaligned", "NotLawful"];

/// ONE REFUSAL AT THE UNIVERSAL CORE TUBE, whole.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AtTube {
    /// The act refused by the boundary law, its clause and reading kept.
    Refused(Boundary),
    /// TRACK 24 B3 — a fold whose stated transitions break the universal
    /// laws: the domain and each law broken, in the register's order.
    NotLawful { domain: [u8; 32], laws: Vec<Law> },
}

impl AtTube {
    /// The act refused.
    pub fn act(&self) -> UniverseAct {
        match self {
            AtTube::Refused(b) => b.act(),
            AtTube::NotLawful { .. } => UniverseAct::Fold,
        }
    }

    /// The domain whose act it was.
    pub fn domain(&self) -> [u8; 32] {
        match self {
            AtTube::Refused(b) => b.domain(),
            AtTube::NotLawful { domain, .. } => *domain,
        }
    }

    /// The refusal's code on [`TUBE_REFUSALS`].
    pub fn code(&self) -> u8 {
        match self {
            AtTube::Refused(Boundary::NotThroughCore { .. }) => 0,
            AtTube::Refused(Boundary::NotSignatory { .. }) => 1,
            AtTube::Refused(Boundary::Bounded { .. }) => 2,
            AtTube::Refused(Boundary::Unaligned { .. }) => 3,
            AtTube::NotLawful { .. } => 4,
        }
    }

    /// The refusal's name.
    pub fn name(&self) -> &'static str {
        match TUBE_REFUSALS.get(usize::from(self.code())) {
            Some(n) => n,
            None => "?",
        }
    }
}

/// ONE DOMAIN'S ONE ACT REFUSED BY ONE NAME: the last refusal whole, the
/// court's height when it was answered, and the count since boot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TubeRow {
    pub last: AtTube,
    pub height: u64,
    pub times: u64,
}

/// THE BOUNDARY READING: every refusal this court answered at a tube since
/// boot, one row per domain, act and name, in that order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BoundaryReading {
    pub rows: Vec<TubeRow>,
}

static TUBE: LazyLock<Mutex<Keyed<TubeRow>>> = LazyLock::new(|| Mutex::new(Keyed::new()));

/// ONE REFUSAL AT THE TUBE, answered at the court's `height`. A count that
/// cannot pass its carrier is not invented: the row keeps its last count
/// and takes the refusal (rule 11).
pub fn note_at_tube(refusal: &AtTube, height: u64) {
    let mut key = refusal.domain().to_vec();
    key.push(refusal.act().byte());
    key.push(refusal.code());
    let Ok(mut t) = TUBE.lock() else { return };
    let times = match t.get(&key) {
        Some(r) => match r.times.checked_add(1) {
            Some(n) => n,
            None => r.times,
        },
        None => 1,
    };
    t.insert(key, TubeRow { last: refusal.clone(), height, times });
}

/// The refusals this court answered at its tubes.
pub fn boundary_reading() -> BoundaryReading {
    match TUBE.lock() {
        Ok(t) => BoundaryReading { rows: t.scoped().map(|(_, r)| r.clone()).collect() },
        Err(_) => BoundaryReading::default(),
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

impl BoundaryReading {
    /// `[rows u32]`, then per row `[height u64][times u64]` and the refusal:
    /// `[0]` and the boundary as its reply writes it, or `[1][domain 32]
    /// [laws u8][law index]…`.
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        crate::transducer::address::put_u32_count(out, self.rows.len())?;
        for r in &self.rows {
            out.extend_from_slice(&r.height.to_le_bytes());
            out.extend_from_slice(&r.times.to_le_bytes());
            match &r.last {
                AtTube::Refused(b) => {
                    out.push(0);
                    b.put(out);
                }
                AtTube::NotLawful { domain, laws } => {
                    out.push(1);
                    out.extend_from_slice(domain);
                    out.push(u8::try_from(laws.len()).ok()?);
                    out.extend(laws.iter().map(|l| l.index()));
                }
            }
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<BoundaryReading> {
        let n = usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
        *p += 4;
        // Each row is at least a height, a count, a form byte and a domain.
        if n > b.len().checked_sub(*p)? / 49 {
            return None;
        }
        let mut rows = Vec::with_capacity(n);
        for _ in 0..n {
            let height = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            let times = u64::from_le_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
            *p += 16;
            let form = *b.get(*p)?;
            *p += 1;
            let last = match form {
                0 => AtTube::Refused(Boundary::take(b, p)?),
                1 => {
                    let domain: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                    let k = usize::from(*b.get(*p + 32)?);
                    *p += 33;
                    let laws = b.get(*p..*p + k)?.iter().map(|i| LAWS.get(usize::from(*i)).copied()).collect::<Option<Vec<_>>>()?;
                    *p += k;
                    AtTube::NotLawful { domain, laws }
                }
                _ => return None,
            };
            rows.push(TubeRow { last, height, times });
        }
        Some(BoundaryReading { rows })
    }

    /// The reading as the console prints it: the count per name, then one
    /// line per domain, act and name, the domain whole and what decided it.
    pub fn lines(&self) -> Vec<String> {
        let mut per = [0u64; 5];
        for r in &self.rows {
            if let Some(s) = per.get_mut(usize::from(r.last.code())) {
                if let Some(v) = s.checked_add(r.times) {
                    *s = v;
                }
            }
        }
        let counts = TUBE_REFUSALS.iter().zip(per).map(|(n, c)| format!("{n} {c}")).collect::<Vec<_>>().join(" · ");
        let mut out = vec![format!("refused at the tube {} (domain, act, name) · {counts}", self.rows.len())];
        out.extend(self.rows.iter().map(|r| {
            let why = match &r.last {
                AtTube::Refused(Boundary::NotSignatory { clause, reading, .. }) => format!(" · {} v{} {reading:?}", clause.law, clause.version),
                AtTube::Refused(Boundary::Bounded { clause, .. }) => format!(" · vetoed {} v{}", clause.law, clause.version),
                AtTube::Refused(Boundary::Unaligned { why, .. }) => format!(" · {why:?}"),
                AtTube::Refused(Boundary::NotThroughCore { .. }) => String::new(),
                AtTube::NotLawful { laws, .. } => format!(" · broke {}", laws.iter().map(|l| l.name()).collect::<Vec<_>>().join(", ")),
            };
            format!("  domain {} · {:?} · {} ×{} · last at height {}{why}", hex(&r.last.domain()), r.last.act(), r.last.name(), r.times, r.height)
        }));
        out
    }
}
