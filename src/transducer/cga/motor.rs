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

use super::multivector::{one, Mv, EM, EP};

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
/// `Cl(n+1, 1)`, `2^(n+2)` of them, each an element of the algebra — never a
/// column of a matrix — with the readings made of them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MotorAction {
    pub n: u8,
    /// The blades, as masks, in the order their images stand.
    pub blades: Vec<usize>,
    /// The image of each blade, `M̂ b M̃`.
    pub images: Vec<Mv>,
    /// Every blade's image stands in the blade's own grade.
    pub grade_preserving: bool,
    /// The vectors and the pseudoscalar, read as `is_versor` reads them.
    pub vectors: VectorReading,
    /// The dimension the images span: the grade of their span.
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
    super::multivector::axes(n).chain([EP, EM]).collect()
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

/// THE DIMENSION ELEMENTS OF THE ALGEBRA SPAN: the grade of their span
/// (`core::span`), each element read as the ray it is.
fn span_grade(elements: &[Mv]) -> usize {
    crate::core::span::grade_of(&elements.iter().map(super::linear::ray_of).collect::<Vec<_>>())
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
        let vector_parts: Vec<Mv> = images.iter().map(|x| x.grade(1)).collect();
        // Every value's `n` is within the algebra's masks, so its
        // pseudoscalar is one of them.
        let pseudo_fixed = super::multivector::pseudo(n).is_some_and(|i| {
            super::group::projective_ratio(&self.image(i), &Mv::blade(i, one())).is_some()
        });
        VectorReading { vectors_to_vectors, rank: span_grade(&vector_parts), dim: gens.len(), pseudo_fixed }
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
        MotorAction {
            n,
            algebra_rank: span_grade(&images),
            images,
            grade_preserving,
            vectors: self.vector_reading(),
            blades,
        }
    }
}
