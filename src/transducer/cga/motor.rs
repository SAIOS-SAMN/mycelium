// 78.41(3), MT1 — THE MOTOR: a transition is kinetic, and it is known by its
// ACTION — the exact linear map `x ↦ M̂ x M̃` on the whole algebra — never by
// a gate on its carrier.
//
//! A versor's action is the grade-preserving case of a motor's, and
//! `acts_as_versor` is a property READ off the action (the vectors carried
//! to vectors, the vector block of full rank, the pseudoscalar to its own
//! line): today's three clauses, read on the action. The ELEMENT's
//! `cga::is_versor` is these with its parity clause — two facts, two names.
//! A motor whose action loses rank CONTRACTS —
//! named `contracted r of n`, read, never refused.
//!
//! Refused by name: `⟨M M̃⟩₀` as an energy; a norm on a signal; a versor gate
//! that refuses a shear or a sum. The sandwich stays TRANSPORT's shape; the
//! opening stays the commutator (78.39).

use super::multivector::{base_axis, one, Mv, EM, EP};
use num_rational::BigRational;
use num_traits::Zero;

/// A MOTOR: any element of the algebra, even or odd, read by its action.
/// 78.41's noun — `element::Action` is the element's four-lane action, and
/// one word meaning two things is the collision refused before it is written.
#[derive(Clone, Debug)]
pub struct Motor(Mv);

/// WHAT A MOTOR DOES TO THE VECTORS AND THE PSEUDOSCALAR — the reading
/// `is_versor` is a property of, `n + 3` images.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorReading {
    /// Every generator `eᵢ` carried to a vector.
    pub vectors_to_vectors: bool,
    /// The rank of the images of the `n + 2` generators, by the exact-rank
    /// reader, and the dimension it is read against.
    pub rank: usize,
    pub dim: usize,
    /// The pseudoscalar carried to its own line, `M̂ I M̃ = λ I`.
    pub pseudo_fixed: bool,
}

impl VectorReading {
    /// THE ACTION OF SOME VERSOR, read off the action (78.28(1) as amended):
    /// vectors to vectors, independent, and `I` to its own line. A fact of
    /// the ACTION, named apart from `cga::is_versor`, a fact of the ELEMENT
    /// (a product of vectors, its parity clause kept): the two agree on
    /// every element of definite parity, and `1 + I`, whose action is `2x`,
    /// is the witness they differ (MT1, ruled 2026-09-25).
    pub fn acts_as_versor(&self) -> bool {
        self.vectors_to_vectors && self.rank == self.dim && self.pseudo_fixed
    }
}

/// THE ACTION OF A MOTOR ON THE WHOLE ALGEBRA: the image of every blade of
/// `Cl(n+1, 1)`, `2^(n+2)` of them, as the exact rational column over the
/// blades, with the readings made of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MotorAction {
    pub n: u8,
    /// The blades, as masks, in the order the columns and rows run.
    pub blades: Vec<usize>,
    /// Column `j`: the image of blade `j`, its coefficient on every blade.
    pub columns: Vec<Vec<BigRational>>,
    /// Every blade's image stands in the blade's own grade.
    pub grade_preserving: bool,
    /// The vectors and the pseudoscalar, read as `is_versor` reads them.
    pub vectors: VectorReading,
    /// The rank of the whole matrix, by the exact-rank reader.
    pub algebra_rank: usize,
}

impl MotorAction {
    /// The action of some versor: the property of its action on the vectors.
    pub fn acts_as_versor(&self) -> bool {
        self.vectors.acts_as_versor()
    }

    /// CONTRACTED r OF n: where the vector block has lost rank, the rank it
    /// kept and the dimension it acts on; `None` where it kept all of it.
    /// A reading, never a refusal.
    pub fn contracted(&self) -> Option<(usize, usize)> {
        (self.vectors.rank < self.vectors.dim).then_some((self.vectors.rank, self.vectors.dim))
    }

    /// The algebra's dimension, `2^(n+2)`.
    pub fn dim(&self) -> usize {
        self.blades.len()
    }
}

/// The generators of `Cl(n+1, 1)`: the base axes, then `e₊`, `e₋`.
fn generators(n: u8) -> Vec<usize> {
    (1..=usize::from(n)).map(base_axis).chain([EP, EM]).collect()
}

/// Every blade of `Cl(n+1, 1)`, as the masks the subsets of its generators
/// make, the scalar first.
fn all_blades(n: u8) -> Vec<usize> {
    let gens = generators(n);
    let mut out = vec![0usize];
    for g in gens {
        let with: Vec<usize> = out.iter().map(|b| b | g).collect();
        out.extend(with);
    }
    out.sort_unstable();
    out
}

/// THE RANK OF RATIONAL ROWS by the exact-rank reader (`core::lattice`):
/// each row cleared of its denominators to the integers it is proportional
/// to, which leaves its rank as it was.
pub(crate) fn rational_rank(rows: &[Vec<BigRational>]) -> usize {
    use num_integer::Integer;
    let cleared: Vec<crate::core::lattice::SparseRow> = rows
        .iter()
        .map(|row| {
            let lcm = row.iter().fold(num_bigint::BigInt::from(1), |l, q| l.lcm(q.denom()));
            row.iter()
                .enumerate()
                .filter(|(_, q)| !q.is_zero())
                .map(|(j, q)| (j, q.numer() * (&lcm / q.denom())))
                .collect()
        })
        .collect();
    crate::core::lattice::sparse_rank(&cleared)
}

// MT1 — THE READING: every transition the percept holds, read by its
// action. One counter per kind, the frame-axis step's two readings typed by
// how they agree, the planes a frame-axis transition turns, and the frame
// each transition carries per lane as a distribution — never a total.

/// Which transition a motor read is: the percept's in→out carriage within a
/// frame, a face's carriage, or the FRAME-AXIS transition — one member's
/// body carried from one occupied frame to the next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionKind {
    InOut,
    Face,
    FrameAxis,
}

/// One kind's transitions by what their actions are: versors; of full
/// algebra rank and no versor; DEGENERATE, keyed by the contraction
/// `contracted()` answers — `r of n` on the vector block, or none — with the
/// algebra rank kept a second coordinate beside it; SUPERPOSED — a sum of rotors, which nothing produces today and the
/// arm exists to count; and unread where the stroke ended first.
#[derive(Clone, Debug, Default)]
pub struct KindReading {
    pub read: u64,
    pub versors: u64,
    pub full_rank: u64,
    pub degenerate: crate::network::nerve::Keyed<u64>,
    pub superposed: u64,
    pub unread: u64,
}

/// HOW A FRAME-AXIS STEP'S TWO READINGS STAND — the member's input body
/// carried to its next occupied frame and its output body likewise, typed
/// by class and never averaged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepClass {
    /// Both solved, one ray.
    Agree,
    /// Both solved, two rays.
    Disagree,
    /// One side solved.
    OneSided,
    /// Neither side solved.
    Neither,
}

/// The frame-axis steps by class, and the planes their actions turn: a
/// plane of the bivector part, a null pair counted as ONE parabolic plane
/// (`occupancy`, D1's reader) — so a turn in a harmonic plane `eᵢ∧e₃`
/// stands beside the grid's `e₁∧e₂`, and the frame's translation reads as
/// its parabolic plane, never as a turn.
#[derive(Clone, Debug, Default)]
pub struct StepReading {
    pub agree: u64,
    pub disagree: u64,
    pub one_sided: u64,
    pub neither: u64,
    pub planes: crate::network::nerve::Keyed<u64>,
    pub parabolic: crate::network::nerve::Keyed<u64>,
}

/// THE MOTOR CENSUS OF A PERCEPT (MT1).
#[derive(Clone, Debug, Default)]
pub struct MotorTally {
    pub in_out: KindReading,
    pub face: KindReading,
    pub frame_axis: KindReading,
    pub steps: StepReading,
    /// The frame each transition carries, per lane — `measure_lanes` of the
    /// chain it carries — as the count of transitions at each value.
    pub lanes: [crate::network::nerve::Keyed<u64>; 4],
}

thread_local! {
    static MOTORS: std::cell::RefCell<MotorTally> = std::cell::RefCell::new(MotorTally::default());
}

fn count_key(v: u64) -> [u8; 8] {
    v.to_be_bytes()
}

/// A degenerate action's key: its contraction first — a flag, then `r` and
/// `n` where the vector block lost rank — and the algebra rank after it.
fn degenerate_key(action: &MotorAction) -> Vec<u8> {
    let mut k = Vec::with_capacity(25);
    match action.contracted() {
        Some((r, n)) => {
            k.push(1);
            k.extend_from_slice(&count_key(r as u64));
            k.extend_from_slice(&count_key(n as u64));
        }
        None => k.push(0),
    }
    k.extend_from_slice(&count_key(action.algebra_rank as u64));
    k
}

/// The degenerate key read back for the line: `r of n` or `uncontracted`,
/// then the algebra rank.
fn degenerate_label(k: &[u8]) -> Option<String> {
    let word = |b: &[u8], at: usize| -> Option<u64> { Some(u64::from_be_bytes(b.get(at..at + 8)?.try_into().ok()?)) };
    match k {
        [1, rest @ ..] => Some(format!("contracted {} of {} algebra {}", word(rest, 0)?, word(rest, 8)?, word(rest, 16)?)),
        [0, rest @ ..] => Some(format!("uncontracted algebra {}", word(rest, 0)?)),
        _ => None,
    }
}

fn bump(carrier: &mut crate::network::nerve::Keyed<u64>, k: &[u8]) {
    let now = match carrier.get(k) {
        Some(&v) => v + 1,
        None => 1,
    };
    carrier.insert(k, now);
}

impl MotorTally {
    fn kind_mut(&mut self, kind: TransitionKind) -> &mut KindReading {
        match kind {
            TransitionKind::InOut => &mut self.in_out,
            TransitionKind::Face => &mut self.face,
            TransitionKind::FrameAxis => &mut self.frame_axis,
        }
    }

    /// THE MOTOR LINE: each kind by what its actions are, the frame-axis
    /// steps by class, the planes they turn by name, the lanes' values.
    pub fn line(&self) -> String {
        let kind = |name: &str, k: &KindReading| {
            let degenerate: Vec<String> =
                k.degenerate.scoped().filter_map(|(key, c)| Some(format!("{}: {c}", degenerate_label(&key)?))).collect();
            format!(
                "{name} read {} versors {} full-rank {} degenerate [{}] superposed {} unread {}",
                k.read,
                k.versors,
                k.full_rank,
                degenerate.join(" "),
                k.superposed,
                k.unread
            )
        };
        let planes: Vec<String> = self
            .steps
            .planes
            .scoped()
            .filter_map(|(b, c)| {
                let blade = usize::try_from(u64::from_be_bytes(b.as_slice().try_into().ok()?)).ok()?;
                Some(format!("{} {c}", super::plane_name(blade)))
            })
            .collect();
        let parabolic: Vec<String> = self
            .steps
            .parabolic
            .scoped()
            .filter_map(|(k, c)| {
                let axis = usize::try_from(u64::from_be_bytes(k.get(..8)?.try_into().ok()?)).ok()?;
                let pole = super::Pole::from_u8(*k.get(8)?)?;
                Some(format!("{} {c}", super::parabolic_name(axis, pole)))
            })
            .collect();
        let lanes: Vec<String> = ["S", "G", "T", "H"]
            .iter()
            .zip(&self.lanes)
            .map(|(l, d)| {
                let vals: Vec<String> = d
                    .scoped()
                    .filter_map(|(v, c)| Some(format!("{}:{c}", u64::from_be_bytes(v.as_slice().try_into().ok()?))))
                    .collect();
                format!("{l}{{{}}}", vals.join(" "))
            })
            .collect();
        format!(
            "motors: {} | {} | {} | steps agree {} disagree {} one-sided {} neither {} turning [{}] parabolic [{}] | lanes {}",
            kind("in→out", &self.in_out),
            kind("face", &self.face),
            kind("frame-axis", &self.frame_axis),
            self.steps.agree,
            self.steps.disagree,
            self.steps.one_sided,
            self.steps.neither,
            planes.join(" "),
            parabolic.join(" "),
            lanes.join(" ")
        )
    }
}

/// READ ONE TRANSITION BY ITS ACTION and file it under its kind, with the
/// frame it carries per lane where it carries a chain.
pub fn note_motor(kind: TransitionKind, m: &Mv, lanes: Option<[u64; 4]>) {
    let action = Motor::new(m).action();
    MOTORS.with(|t| {
        let mut t = t.borrow_mut();
        let k = t.kind_mut(kind);
        k.read += 1;
        match (action.acts_as_versor(), action.algebra_rank == action.dim()) {
            (true, _) => k.versors += 1,
            (false, true) => k.full_rank += 1,
            (false, false) => bump(&mut k.degenerate, &degenerate_key(&action)),
        }
        if let Some(lanes) = lanes {
            for (d, v) in t.lanes.iter_mut().zip(lanes) {
                bump(d, &count_key(v));
            }
        }
    });
}

/// A transition the stroke ended before reading, counted under its kind.
pub fn note_unread(kind: TransitionKind) {
    MOTORS.with(|t| {
        let mut t = t.borrow_mut();
        let k = t.kind_mut(kind);
        k.unread += 1;
    });
}

/// ONE FRAME-AXIS STEP by class, and the planes its actions turn — read off
/// every motor the caller names: the one that solved, the input's where
/// both agree, and BOTH where they disagree.
pub fn note_step(class: StepClass, turned: &[&Mv]) {
    let occupied: Vec<super::Occupancy> = turned.iter().map(|m| super::occupancy(m, m.n())).collect();
    MOTORS.with(|t| {
        let mut t = t.borrow_mut();
        let s = &mut t.steps;
        match class {
            StepClass::Agree => s.agree += 1,
            StepClass::Disagree => s.disagree += 1,
            StepClass::OneSided => s.one_sided += 1,
            StepClass::Neither => s.neither += 1,
        }
        for o in occupied {
            for p in o.planes {
                bump(&mut s.planes, &count_key(p as u64));
            }
            for (axis, pole) in o.parabolic {
                let mut k = count_key(axis as u64).to_vec();
                k.push(pole.as_u8());
                bump(&mut s.parabolic, &k);
            }
        }
    });
}

/// The motor census, without clearing (the per-percept line).
pub fn motor_tally() -> MotorTally {
    MOTORS.with(|t| t.borrow().clone())
}

/// The motor census, cleared: the corpus's one reader.
pub fn take_motor_tally() -> MotorTally {
    MOTORS.with(|t| t.replace(MotorTally::default()))
}

impl Motor {
    pub fn new(m: &Mv) -> Motor {
        Motor(m.clone())
    }

    /// The element whose action this is.
    pub fn element(&self) -> &Mv {
        &self.0
    }

    /// The image of one blade, `M̂ x M̃` — the twisted adjoint, the sandwich
    /// for an even motor and its negation for an odd one (78.28(1)).
    fn image(&self, blade: usize) -> Mv {
        self.0.twisted(&Mv::blade(blade, one()))
    }

    /// THE ACTION ON THE VECTORS AND THE PSEUDOSCALAR: `n + 3` images, the
    /// reading `is_versor` is made of.
    pub fn vector_reading(&self) -> VectorReading {
        let n = self.0.n();
        let gens = generators(n);
        let images: Vec<Mv> = gens.iter().map(|&g| self.image(g)).collect();
        let vectors_to_vectors = images.iter().all(|x| x.sub(&x.grade(1)).is_zero());
        let rows: Vec<Vec<BigRational>> =
            images.iter().map(|x| gens.iter().map(|&g| x.coeff(g).clone()).collect()).collect();
        let pseudoscalar = Mv::blade(super::multivector::pseudo(n), one());
        VectorReading {
            vectors_to_vectors,
            rank: rational_rank(&rows),
            dim: gens.len(),
            pseudo_fixed: super::group::projective_ratio(&self.image(super::multivector::pseudo(n)), &pseudoscalar)
                .is_some(),
        }
    }

    /// The images of the named generators, each read on those generators as
    /// a column over them — the vector block a versor's reader takes.
    pub fn columns_on(&self, masks: &[usize]) -> Vec<Vec<BigRational>> {
        masks
            .iter()
            .map(|&m| {
                let x = self.image(m).grade(1);
                masks.iter().map(|&k| x.coeff(k).clone()).collect()
            })
            .collect()
    }

    /// THE WHOLE ACTION: one image per blade, `2^(n+2)` of them — the
    /// caller charges them to its stroke — and the ranks read off them.
    pub fn action(&self) -> MotorAction {
        let n = self.0.n();
        let blades = all_blades(n);
        let images: Vec<Mv> = blades.iter().map(|&b| self.image(b)).collect();
        let grade_preserving = blades
            .iter()
            .zip(&images)
            .all(|(&b, x)| x.sub(&x.grade(b.count_ones())).is_zero());
        let columns: Vec<Vec<BigRational>> =
            images.iter().map(|x| blades.iter().map(|&k| x.coeff(k).clone()).collect()).collect();
        MotorAction {
            n,
            algebra_rank: rational_rank(&columns),
            columns,
            grade_preserving,
            vectors: self.vector_reading(),
            blades,
        }
    }
}
