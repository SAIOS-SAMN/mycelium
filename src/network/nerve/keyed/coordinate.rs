// A21, §4.4 — THE COORDINATE AN INDEX ANSWERS, AS A VALUE THAT DECODES.
//
//! A first cut of this made `Key<C>` a phantom type wrapped around an
//! opaque `Arc<[u8]>`. `Key<Chart>` and `Key<Section>` were then BYTE
//! IDENTICAL at runtime, the claim about which coordinate they carried
//! lived in a type parameter that `declared`, `as_bytes` and `into_vec`
//! discarded on contact, and `declared` accepted any bytes at all for a
//! system with no tag. That is a label beside bytes, not a coordinate:
//! rule 2 wants the key to BE the object's coordinates written out, so
//! that identity is content and a reader can get the object back.
//!
//! A coordinate here is a VALUE. It encodes to its canonical bytes and
//! decodes back to itself:
//!
//! ```text
//!     x ≡ decode(encode(x))
//! ```
//!
//! An index is `Keyed<V, C>` for that value type, and its reads and writes
//! take `&C` — never bytes. There is no escape: the byte form exists only
//! inside the trie, where the walk needs it, and every key handed back out
//! of a walk is decoded first. A key that cannot decode is a refusal and
//! says which coordinate it failed at (rule 11), never an empty answer.

use super::*;

/// THE CANONICAL BYTES OF ONE COORDINATE.
///
/// Sealed: it is produced by [`Coordinate::encode`] and consumed by the
/// trie, and nothing else can make one out of arbitrary bytes. That is
/// what stops the escape hatch from growing back — there is no
/// `CompactBytes::from(&[u8])` for a caller to reach for.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompactBytes(std::sync::Arc<[u8]>);

impl CompactBytes {
    /// Written by a coordinate's own `encode`, which is the only caller.
    pub fn written(buf: Vec<u8>) -> CompactBytes {
        CompactBytes(buf.into())
    }

    /// The object's one allocation, shared rather than copied (Z4c): a
    /// form's address is megabytes and every carrier naming it points at
    /// the same bytes.
    pub fn shared(bytes: std::sync::Arc<[u8]>) -> CompactBytes {
        CompactBytes(bytes)
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub(crate) fn arc(&self) -> std::sync::Arc<[u8]> {
        std::sync::Arc::clone(&self.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// WHY A COORDINATE DID NOT DECODE.
///
/// The empty answer and the refusal are different facts, and naming which
/// coordinate failed is what keeps them apart (rule 11).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoordinateError {
    /// The system the bytes were read against.
    pub system: &'static str,
    /// What the bytes did not spell.
    pub why: &'static str,
}

impl std::fmt::Display for CoordinateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} coordinate: {}", self.system, self.why)
    }
}

impl std::error::Error for CoordinateError {}

/// A COORDINATE: a value that is its own canonical bytes, both ways.
///
/// `decode(encode(x)) == x` for every `x`, which is the property that
/// makes the bytes a coordinate rather than a label. A type that cannot
/// satisfy it is not a coordinate and does not belong in a key.
pub trait Coordinate: Eq + Clone + std::fmt::Debug + Send + Sync + 'static {
    /// The system's name, for a refusal that says which coordinate it is.
    const NAME: &'static str;
    /// The runtime system these bytes belong to, where the A21 tag names
    /// one. `Any` is for a system whose keys are not tag-led; it is a
    /// statement about the layout, never a licence to skip the decode.
    const SYSTEM: CoordinateSystem;

    fn encode(&self) -> CompactBytes;
    fn decode(bytes: &[u8]) -> Result<Self, CoordinateError>
    where
        Self: Sized;

    /// The refusal this system raises, so implementors say it in one line.
    fn refuse(why: &'static str) -> CoordinateError {
        CoordinateError { system: Self::NAME, why }
    }
}

/// THE PREFIX A COARSE COORDINATE MAKES IN A FINER ONE.
///
/// A fibre read follows the prefix of a coarse coordinate — every section over one
/// chart, every member of one class — and that is only meaningful where
/// the coarse system's bytes are genuinely an opening segment of the fine
/// system's. Declaring it is what makes `fibre_over` a stratum read rather
/// than a byte-prefix coincidence.
pub trait Stratum<Fine: Coordinate>: Coordinate {
    /// THE OPENING BYTES OF A FINE KEY THAT THIS COARSE COORDINATE FIXES.
    ///
    /// Not always `encode()`: a [`coord::Section`] opens with the chart's
    /// key LENGTH-DELIMITED, so the chart's own bytes are not a byte
    /// prefix of it and reading the fibre at `encode()` would follow
    /// the wrong bits. Stating the opening separately is what keeps the
    /// two ends from drifting — the coarse reader and the fine writer are
    /// the same function here, rather than two written to agree.
    fn opening(&self) -> CompactBytes;
}

/// THE COORDINATE SYSTEMS, as values.
pub mod coord {
    use super::{CompactBytes, Coordinate, CoordinateError, CoordinateSystem, Stratum};
    use crate::transducer::address as a;

    /// Read the A21 tag and refuse a foreign one in a single line.
    fn tagged<C: Coordinate>(bytes: &[u8], want: u8) -> Result<usize, CoordinateError> {
        match a::tag_of(bytes) {
            Some(t) if t == want => Ok(1),
            Some(_) => Err(C::refuse("the key opens with another system's tag")),
            None => Err(C::refuse("an empty key spells no coordinate")),
        }
    }

    /// THE COORDINATE WHOSE STRUCTURE IS NOT YET DECLARED.
    ///
    /// Transitional, and the count of carriers still spelling it is the
    /// exact remaining work of §4.4 — not a permanent member of this
    /// module. It satisfies invertibility vacuously (the bytes are the
    /// value), which is precisely the statement that it asserts nothing:
    /// an `Any` carrier cannot tell a chart from a member key, and every
    /// one of them is a latent wrong-key defect until it is replaced by
    /// the coordinate it actually answers.
    ///
    /// It is NOT an escape hatch for a typed carrier: a `Keyed<V, Chart>`
    /// cannot be handed one of these, because `put` takes `&Chart`.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Any(pub Vec<u8>);

    impl Any {
        /// Bytes a site has not yet given a coordinate. Greppable on
        /// purpose: each one is a place the declaration is still missing.
        pub fn undeclared(bytes: impl AsRef<[u8]>) -> Any {
            Any(bytes.as_ref().to_vec())
        }
    }

    impl Coordinate for Any {
        const NAME: &'static str = "undeclared";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            CompactBytes::written(self.0.clone())
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            Ok(Any(bytes.to_vec()))
        }
    }

    /// A CHART — the invariant that names it (18.2). Decodes through
    /// `Invariant::from_key`, which is total or refuses: a key ending
    /// mid-coordinate decodes to nothing rather than to the coordinates
    /// before it, which would be a different chart read as this one.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Chart(pub crate::transducer::stalk::Invariant);

    impl Coordinate for Chart {
        const NAME: &'static str = "chart";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Chart;

        fn encode(&self) -> CompactBytes {
            CompactBytes::shared(self.0.key().shared())
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            tagged::<Self>(bytes, a::TAG_CHART)?;
            crate::transducer::stalk::Invariant::from_key(bytes)
                .map(Chart)
                .ok_or_else(|| Self::refuse("the bytes do not spell a chart's axes"))
        }
    }

    /// A HOLONOMY CLASS — the two Casimirs of the bivector generator and
    /// the trichotomy byte, `(I₁, I₂, type)` read in `Cl(4,1)`.
    ///
    /// The type is a READING of the motion and `Trivial` is not
    /// `Parabolic`: `B = 0` is the identity, which turns nothing, while a
    /// genuine parabolic has `B ≠ 0` and `B² = 0` — a null shear that
    /// displaces along its own null direction. Collapsing them put every
    /// identity orbit in the same class as every dislocation.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Holonomy {
        pub i1: crate::transducer::alphabet::Q,
        pub i2: crate::transducer::alphabet::Q,
        pub kind: BivectorKind,
    }

    /// THE FOUR READINGS OF A BIVECTOR GENERATOR.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum BivectorKind {
        /// `B = 0` — the identity. Zero holonomy, not a null shear.
        Trivial = 0,
        /// `⟨B²⟩₀ < 0` — a spatial rotation, `eᵢ∧eⱼ` or `eᵢ∧e₊`. Six of the
        /// ten bivectors of `⋀²ℚ^{4,1}`, and the only kind `Cl(4,0)` has.
        Elliptic = 1,
        /// `⟨B²⟩₀ > 0` — a geodesic transvection, `eₐ∧e₋` for any spacelike
        /// `a`. Four of the ten. The generator `Cl(4,0)` cannot represent
        /// at all, its metric being positive definite.
        Hyperbolic = 2,
        /// `B ≠ 0` and `⟨B²⟩₀ = 0` — a null shear.
        ///
        /// ITS GENERATOR IS `e_∞ ∧ v`, NOT `e₊ ∧ e₋`. A first cut named the
        /// latter, and the algebra says otherwise:
        ///
        /// ```text
        ///     (e₊e₋)² = e₊e₋e₊e₋ = −e₊e₊e₋e₋ = −(+1)(−1) = +1
        /// ```
        ///
        /// so `e₊∧e₋` is HYPERBOLIC — the fourth boost of ℍ⁴, which the same
        /// cut had also dropped from the generator list. The null bivectors
        /// are the ones built on `e_∞ = e₋ + e₊`, where `e_∞² = e₋² + e₊² =
        /// −1 + 1 = 0`: the conformal translator `T(d) = 1 − ½ d e_∞` is
        /// exactly this type, which is why it fixes the point at infinity
        /// and why it is the flatness R3 removes.
        Parabolic = 3,
    }

    impl BivectorKind {
        pub const fn as_u8(self) -> u8 {
            self as u8
        }

        pub const fn from_u8(b: u8) -> Option<BivectorKind> {
            match b {
                0 => Some(BivectorKind::Trivial),
                1 => Some(BivectorKind::Elliptic),
                2 => Some(BivectorKind::Hyperbolic),
                3 => Some(BivectorKind::Parabolic),
                _ => None,
            }
        }
    }

    impl Coordinate for Holonomy {
        const NAME: &'static str = "holonomy-class";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Class;

        fn encode(&self) -> CompactBytes {
            let mut buf = vec![a::TAG_CLASS];
            crate::transducer::alphabet::put_q(&mut buf, &self.i1);
            crate::transducer::alphabet::put_q(&mut buf, &self.i2);
            buf.push(self.kind.as_u8());
            CompactBytes::written(buf)
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let mut pos = tagged::<Self>(bytes, a::TAG_CLASS)?;
            let i1 = crate::transducer::alphabet::take_q(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the first invariant does not spell an exact ℚ"))?;
            let i2 = crate::transducer::alphabet::take_q(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the second invariant does not spell an exact ℚ"))?;
            let kind = bytes
                .get(pos)
                .copied()
                .and_then(BivectorKind::from_u8)
                .ok_or_else(|| Self::refuse("the type byte names no bivector reading"))?;
            if pos + 1 != bytes.len() {
                return Err(Self::refuse("the key carries bytes past its last coordinate"));
            }
            Ok(Holonomy { i1, i2, kind })
        }
    }

    /// A COMPARTMENT OF THE BASE — `(row, col, frame)`, the three SPATIAL
    /// coordinates the sensor writes, and a genuine coordinate of the
    /// manifold rather than an ordinal.
    ///
    /// THE THIRD AXIS IS THE FRAME, NOT THE HUE (§4.4 E8, corrected
    /// 2026-09-22). It was named `hue` and every caller fed it
    /// `support_of`'s third component, which is `Compartment`'s `k` — the
    /// corpus's own `e₃`, 51.3's sixth adjacency direction, the axis
    /// `at_frame_zero` recentres. The colour never reached this coordinate
    /// and could not: a cover piece is of ONE hue by construction and
    /// carries it on [`Piece`], which is why the base a cover is read over
    /// is three spatial axes and the palette is a coordinate of the chart.
    /// 71.2(1), BQ1 — the compartment IS a site of the base, `n` coordinates with
    /// `n` written first ([`Site::put`](crate::transducer::base::Site::put)):
    /// row, column and frame at the corpus's `n = 3`, and whatever the base
    /// has at any other.
    ///
    /// It exists so the nerve's overlaps can be derived by READING THE
    /// BASE. A 2-simplex of a cover is certified by a WITNESS: a compartment
    /// standing in three charts at once, `p ∈ U_i ∩ U_j ∩ U_k`. Keyed by
    /// this coordinate, the support of the cover is an inverted reading
    /// `p ↦ charts(p)` built in one linear pass, and any compartment whose
    /// reading names three or more charts hands back the simplex directly.
    ///
    /// THE ALTERNATIVE IS FALSE. Taking 3-cliques of the pairwise meet
    /// graph proves only `U_i ∩ U_j ≠ ∅` three times over, and three
    /// charts can ring a pore they do not cover — filling that triangle
    /// would kill an authentic `H₁` class by fiat. It is also a
    /// combinatorial scan over triples, which is a search (rule 3); the
    /// base read is `O(compartments · valence)` and derives the simplex from the
    /// cover the engine already built.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Compartment {
        pub site: crate::transducer::base::Site,
    }

    impl Coordinate for Compartment {
        const NAME: &'static str = "lamina-cell";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            let mut buf = vec![7u8];
            self.site.put(&mut buf);
            CompactBytes::written(buf)
        }

        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — the tag byte at position 0 is the wire's discriminant, not a member chosen"
        )]
        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let refuse = || Self::refuse("a lamina cell is a tag byte and a site, n first");
            match bytes.first() {
                Some(7u8) => {
                    let mut pos = 1usize;
                    let site = crate::transducer::base::Site::take(bytes, &mut pos).ok_or_else(refuse)?;
                    match pos == bytes.len() {
                        true => Ok(Compartment { site }),
                        false => Err(refuse()),
                    }
                }
                _ => Err(refuse()),
            }
        }
    }

    // `VertexIx` — a vertex's position in its own form — STOOD HERE AND IS
    // DELETED WITH THE SLOTS (AT1): a vertex is reached by its coordinate,
    // [`VertexAt`], and nothing names it by where it was pushed.

    /// AN UNFOLDED VERTEX'S MEASURE. The form's own namespace, not A21's:
    /// it opens with `4`, which is `TAG_FORM` in the other namespace and
    /// means nothing here — two systems over one byte, which is the case
    /// only a type can decide.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Unfolded(pub crate::transducer::sentence::Measure);

    impl Coordinate for Unfolded {
        const NAME: &'static str = "unfolded-measure";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            use crate::transducer::sentence::Measure;
            let mut k = vec![4u8];
            match &self.0 {
                Measure::Occupied => k.push(0),
                Measure::Remainder => k.push(1),
                // AT1 — the ring's anchor by the vertex's own coordinate.
                Measure::Ring { anchor } => {
                    k.push(2);
                    a::put_key_bytes(&mut k, anchor.encode().as_bytes());
                }
            }
            CompactBytes::written(k)
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            use crate::transducer::sentence::Measure;
            match bytes {
                [4u8, 0] => Ok(Unfolded(Measure::Occupied)),
                [4u8, 1] => Ok(Unfolded(Measure::Remainder)),
                [4u8, 2, ..] => {
                    let mut pos = 2usize;
                    let ab = a::take_key_bytes(bytes, &mut pos)
                        .ok_or_else(|| Self::refuse("the ring's anchor is not length-delimited"))?;
                    if pos != bytes.len() {
                        return Err(Self::refuse("the key carries bytes past its last coordinate"));
                    }
                    Ok(Unfolded(Measure::Ring { anchor: Box::new(VertexAt::decode(&ab)?) }))
                }
                _ => Err(Self::refuse("the bytes spell no unfolded measure")),
            }
        }
    }

    /// A DEMAND VERTEX OF ONE FORM — `(kind, chart)` (50.5), each end
    /// length-delimited so the pair reads back apart.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct DemandVertex {
        pub kind: crate::transducer::sentence::Demand,
        pub chart: crate::transducer::stalk::Invariant,
    }

    impl Coordinate for DemandVertex {
        const NAME: &'static str = "demand-vertex";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            let mut buf = Vec::new();
            a::put_key_bytes(&mut buf, self.kind.key().as_bytes());
            a::put_key_bytes(&mut buf, self.chart.key().as_bytes());
            CompactBytes::written(buf)
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let mut pos = 0usize;
            let kb = a::take_key_bytes(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the demand kind is not length-delimited"))?;
            let cb = a::take_key_bytes(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the chart is not length-delimited"))?;
            if pos != bytes.len() {
                return Err(Self::refuse("the key carries bytes past its last coordinate"));
            }
            let mut kp = tagged::<Self>(&kb, a::TAG_DEMAND)?;
            let kind = crate::transducer::sentence::Demand::from_bytes(&kb, &mut kp)
                .ok_or_else(|| Self::refuse("the bytes spell no demand kind"))?;
            if kp != kb.len() {
                return Err(Self::refuse("the demand kind carries trailing bytes"));
            }
            let chart = crate::transducer::stalk::Invariant::from_key(&cb)
                .ok_or_else(|| Self::refuse("the bytes spell no chart"))?;
            Ok(DemandVertex { kind, chart })
        }
    }

    /// A SECTION OF A CHART — `chart ‖ carriage`, the coordinate that
    /// separates two sheets of one fibre (18.2, 21 step 1).
    ///
    /// It opens with `TAG_CHART` exactly as [`Chart`] does, which is why
    /// the tag separated neither and eight readers asked a chart index
    /// with one of these and got nothing. It is a strictly finer
    /// coordinate, and a [`Stratum`] of it.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Section {
        pub chart: crate::transducer::stalk::Invariant,
        pub carriage: crate::transducer::element::Versor,
    }

    impl Coordinate for Section {
        const NAME: &'static str = "section";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Chart;

        fn encode(&self) -> CompactBytes {
            let mut k = Vec::new();
            a::put_key_bytes(&mut k, self.chart.key().as_bytes());
            self.carriage.put_canonical(&mut k);
            CompactBytes::written(k)
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let mut pos = 0usize;
            let cb = a::take_key_bytes(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the chart is not length-delimited"))?;
            let chart = crate::transducer::stalk::Invariant::from_key(&cb)
                .ok_or_else(|| Self::refuse("the bytes spell no chart"))?;
            let carriage = crate::transducer::cga::Mv::from_bytes(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the carriage does not spell an exact versor"))?;
            if pos != bytes.len() {
                return Err(Self::refuse("the key carries bytes past its last coordinate"));
            }
            Ok(Section { chart, carriage })
        }
    }

    /// A TRANSPORT CLASS — shell kind and the conjugacy of the word
    /// (21 step 1, 15.4). Never the hue map and never the body's moduli,
    /// which are section coordinates.
    ///
    /// It opens with the shell kind, not an A21 tag, so `SYSTEM` is `Any`
    /// and the decode is the whole check — which is the honest spelling of
    /// a system whose keys are not tag-led.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Transport {
        pub kind: crate::transducer::peel::ShellKind,
        pub class: Holonomy,
    }

    impl Coordinate for Transport {
        const NAME: &'static str = "transport-class";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            let mut buf = vec![self.kind.as_u8()];
            a::put_key_bytes(&mut buf, self.class.encode().as_bytes());
            CompactBytes::written(buf)
        }

        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — the tag byte at position 0 is the wire's discriminant, not a member chosen"
        )]
        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let kind = bytes
                .first()
                .copied()
                .and_then(crate::transducer::peel::ShellKind::from_u8)
                .ok_or_else(|| Self::refuse("the leading byte names no shell kind"))?;
            let mut pos = 1usize;
            let cb = a::take_key_bytes(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the word class is not length-delimited"))?;
            if pos != bytes.len() {
                return Err(Self::refuse("the key carries bytes past its last coordinate"));
            }
            Ok(Transport { kind, class: Holonomy::decode(&cb)? })
        }
    }

    /// A MEMBER'S SECTION — `transport class ‖ chart ‖ address ‖ carriage`
    /// (18.2, 21 step 1, 52.5 item 3).
    ///
    /// `Form.sections` is read at BOTH depths — by transport class and by
    /// member section — and that is one system at two strata rather than
    /// two carriers sharing a name, because this opens with exactly the
    /// transport class's bytes. [`Transport`] declares that relation.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MemberSection {
        pub class: Transport,
        pub chart: crate::transducer::stalk::Invariant,
        pub address: crate::transducer::address::Address,
        pub carriage: crate::transducer::element::Versor,
    }

    impl Coordinate for MemberSection {
        const NAME: &'static str = "member-section";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            let mut k = self.class.encode().as_bytes().to_vec();
            a::put_key_bytes(&mut k, self.chart.key().as_bytes());
            a::put_key_bytes(&mut k, self.address.as_bytes());
            let mut carriage = Vec::new();
            self.carriage.put_canonical(&mut carriage);
            a::put_key_bytes(&mut k, &carriage);
            CompactBytes::written(k)
        }

        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — the tag byte at position 0 is the wire's discriminant, not a member chosen"
        )]
        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            // The class is itself length-framed at its tail, so the member
            // coordinates are parsed after it rather than at a remembered
            // offset (A21 §2).
            let kind = bytes
                .first()
                .copied()
                .and_then(crate::transducer::peel::ShellKind::from_u8)
                .ok_or_else(|| Self::refuse("the leading byte names no shell kind"))?;
            let mut pos = 1usize;
            let cb = a::take_key_bytes(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the word class is not length-delimited"))?;
            let class = Transport { kind, class: Holonomy::decode(&cb)? };
            let chb = a::take_key_bytes(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the chart is not length-delimited"))?;
            let chart = crate::transducer::stalk::Invariant::from_key(&chb)
                .ok_or_else(|| Self::refuse("the bytes spell no chart"))?;
            let ab = a::take_key_bytes(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the address is not length-delimited"))?;
            let vb = a::take_key_bytes(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the carriage is not length-delimited"))?;
            let mut vp = 0usize;
            let carriage = crate::transducer::cga::Mv::from_bytes(&vb, &mut vp)
                .ok_or_else(|| Self::refuse("the carriage does not spell an exact versor"))?;
            if pos != bytes.len() {
                return Err(Self::refuse("the key carries bytes past its last coordinate"));
            }
            Ok(MemberSection {
                class,
                chart,
                address: crate::transducer::address::Address::from_bytes(&ab),
                carriage,
            })
        }
    }

    /// THE CLASS DEPTH of the section system: a transport class is the
    /// coarse stratum of every member section written over it, and this is
    /// the declaration that makes the two-depth read of `Form.sections` a
    /// stratum read rather than a coincidence.
    impl Stratum<MemberSection> for Transport {
        fn opening(&self) -> CompactBytes {
            self.encode()
        }
    }

    /// WHAT `Form.sections` IS KEYED BY — one system at two depths.
    ///
    /// 54.7 — the carrier is written at the CLASS depth by `extract`'s
    /// class union and at the MEMBER depth by its member loop, and read at
    /// both by the court. Its doc comment named only the first. The type
    /// says both now, and the two are unambiguous because a
    /// [`MemberSection`] must consume its bytes to the last coordinate:
    /// a class-length key fails that and decodes as [`Transport`].
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum SectionAt {
        /// Every member of this class is claimed on these frames.
        Class(Transport),
        /// This member, on this chart, by this carriage. Boxed: the member
        /// carries a whole carriage, and the class depth should not carry
        /// that footprint.
        Member(Box<MemberSection>),
    }

    impl Coordinate for SectionAt {
        const NAME: &'static str = "section";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            match self {
                SectionAt::Class(t) => t.encode(),
                SectionAt::Member(m) => m.encode(),
            }
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            match MemberSection::decode(bytes) {
                Ok(m) => Ok(SectionAt::Member(Box::new(m))),
                Err(_) => Transport::decode(bytes)
                    .map(SectionAt::Class)
                    .map_err(|_| Self::refuse("the bytes spell neither depth of a section")),
            }
        }
    }

    /// THE CLASS DEPTH IS A STRATUM OF THE CARRIER, so "every section of
    /// this class" is a fibre read and not a whole-carrier read.
    impl Stratum<SectionAt> for Transport {
        fn opening(&self) -> CompactBytes {
            self.encode()
        }
    }

    /// A WORD VERTEX'S ADDRESS — the coordinate `by_address` and
    /// `by_rebased` answer, as `word_vertex_address` writes it under
    /// `TAG_FORM`.
    ///
    /// NOT YET STRUCTURAL, AND SAYING SO IS THE POINT. The value is
    /// `(Derivation, sections, Contact)` and the tree has `put_word_vertex`
    /// with no `take_word_vertex` beside it, so `decode` here validates the
    /// tag and the form-vertex discriminant and carries the bytes. That is
    /// weaker than `x ≡ decode(encode(x))` promises and it is counted as
    /// outstanding, not quietly accepted: writing the vertex parser is what
    /// closes it.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Member(pub crate::transducer::address::Address);

    impl Coordinate for Member {
        const NAME: &'static str = "member";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Form;

        fn encode(&self) -> CompactBytes {
            CompactBytes::shared(self.0.shared())
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            tagged::<Self>(bytes, a::TAG_FORM)?;
            match bytes.get(1) {
                Some(1) => Ok(Member(crate::transducer::address::Address::from_bytes(bytes))),
                _ => Err(Self::refuse("the record is not a word vertex")),
            }
        }
    }

    /// A SITE OF THE COVER — where a citation stands, as distinct from what
    /// it cites (54.7 B5a): the Glued vertex's own boundary record as
    /// `put_vertex` writes it, in the form's local encoding.
    ///
    /// NOT YET STRUCTURAL, for the same reason [`Member`] is not: the
    /// vertex parser does not exist. `decode` checks the record's
    /// discriminant and carries the bytes, and that gap is counted.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Site(pub Vec<u8>);

    impl Coordinate for Site {
        const NAME: &'static str = "cover-site";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            CompactBytes::written(self.0.clone())
        }

        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — the tag byte at position 0 is the wire's discriminant, not a member chosen"
        )]
        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            // `put_vertex` writes the kind discriminant first, and a
            // citation is kind 2.
            match bytes.first() {
                Some(2) => Ok(Site(bytes.to_vec())),
                _ => Err(Self::refuse("the record is not a citation's boundary")),
            }
        }
    }

    /// A PLACEMENT — `vertex transport ‖ chart` (18.2). Two carriages on
    /// one transport class are two placements because the chart separates
    /// them; keyed by the class alone they were one entry conflicting with
    /// itself.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Placement {
        pub vertex_transport: crate::transducer::address::Address,
        pub chart: crate::transducer::stalk::Invariant,
    }

    impl Coordinate for Placement {
        const NAME: &'static str = "placement";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Word;

        fn encode(&self) -> CompactBytes {
            let mut k = self.vertex_transport.as_bytes().to_vec();
            a::put_key_bytes(&mut k, self.chart.key().as_bytes());
            CompactBytes::written(k)
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            tagged::<Self>(bytes, a::TAG_WORD)?;
            // The chart is the last length-delimited run; the transport
            // prefix is what precedes it, parsed by finding that frame.
            let n = bytes.len();
            let mut best: Option<(usize, crate::transducer::stalk::Invariant)> = None;
            // index: the one framed tail this layout writes — scanned from
            // each position a key-component length can start such that it
            // spans exactly to the end (A21 self-delimiting tail).
            for start in 0..n {
                let mut pos = start;
                let Some(cb) = a::take_key_bytes(bytes, &mut pos) else { continue };
                if pos != n {
                    continue;
                }
                if let Some(inv) = crate::transducer::stalk::Invariant::from_key(&cb) {
                    best = Some((start, inv));
                    break;
                }
            }
            let (start, chart) =
                best.ok_or_else(|| Self::refuse("no framed chart closes the placement"))?;
            Ok(Placement {
                vertex_transport: crate::transducer::address::Address::from_bytes(
                    &bytes[..start],
                ),
                chart,
            })
        }
    }

    /// THE CHART A SECTION STANDS OVER is the section's own opening
    /// segment, so every section over one chart is a fibre read at it —
    /// following the LENGTH-DELIMITED chart key, which is what
    /// `Section::encode` writes first and what the bare chart key is not.
    impl Stratum<Section> for Chart {
        fn opening(&self) -> CompactBytes {
            let mut k = Vec::new();
            a::put_key_bytes(&mut k, self.0.key().as_bytes());
            CompactBytes::written(k)
        }
    }

    /// §4.4 E8 — THE THREE COORDINATES OF A COVER (ruled 2026-09-22).
    ///
    /// `family::pieces` has always written these three addresses out of
    /// the pieces' own content — a piece by its shell kind and its support written
    /// out; a carriage by its domain and codomain in that order; a meet by
    /// its two ends unordered — and every carrier
    /// holding one was `Keyed<V, Any>`. The bytes were a coordinate and the
    /// type said they were not, so a carriage key could be handed to the
    /// piece index and the trie would answer.
    ///
    /// Declaring them closes three of E8's carriers and, in doing so, forced
    /// one genuine defect into the open: `assembly_key` wrote a ONE-PIECE
    /// assembly as that piece's bare address and a many-piece assembly as a
    /// run of length-prefixed ones, so the bytes did not say which they
    /// were. A carriage now names a list of pieces on each side, always
    /// length-delimited, and the two passes agree because both sides are
    /// lists — not because one collapses to the other's shape.
    ///
    /// A compartment of the base carried into a support is a site with its `n` —
    /// `(row, col, frame)` at the corpus's three, FRAME the sensor's third axis. It is not the hue, and the hue
    /// is not on this address either: a piece is a HUE-BLIND component
    /// (70.4), its colours are the gauge its compartments carry, and nothing reads
    /// a colour back off a piece's key, so none is written there (law 12).
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Piece {
        pub kind: crate::transducer::peel::ShellKind,
        pub support: Vec<crate::transducer::base::Site>,
    }

    /// A support written and read back in one place, so its layout has ONE
    /// producer and cannot drift between the coordinates that share it (law
    /// 12): the count, then each site with its `n` (71.2(1)).
    fn put_support(buf: &mut Vec<u8>, support: &[crate::transducer::base::Site]) {
        crate::transducer::address::put_key_count(buf, support.len());
        for s in support {
            s.put(buf);
        }
    }

    fn take_support(bytes: &[u8], pos: &mut usize) -> Option<Vec<crate::transducer::base::Site>> {
        let n = crate::transducer::address::take_key_count(bytes, pos)?;
        let mut out = Vec::with_capacity(n.min(bytes.len()));
        for _ in 0..n {
            out.push(crate::transducer::base::Site::take(bytes, pos)?);
        }
        Some(out)
    }

    impl Piece {
        fn put(&self, buf: &mut Vec<u8>) {
            buf.push(a::TAG_PIECE);
            buf.push(self.kind.as_u8());
            put_support(buf, &self.support);
        }

        /// The one reader of a piece's layout; a key holding several reads
        /// each through this, the next starting where the last ended.
        pub(crate) fn take(bytes: &[u8], pos: &mut usize) -> Option<Piece> {
            match bytes.get(*pos) {
                Some(&t) if t == a::TAG_PIECE => *pos += 1,
                _ => return None,
            }
            let kind = crate::transducer::peel::ShellKind::from_u8(*bytes.get(*pos)?)?;
            *pos += 1;
            Some(Piece { kind, support: take_support(bytes, pos)? })
        }
    }

    impl Coordinate for Piece {
        const NAME: &'static str = "cover-piece";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Piece;

        fn encode(&self) -> CompactBytes {
            let mut buf = Vec::new();
            self.put(&mut buf);
            CompactBytes::written(buf)
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let mut pos = 0usize;
            let p = Piece::take(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the bytes do not spell a kind and a support"))?;
            match pos == bytes.len() {
                true => Ok(p),
                false => Err(Self::refuse("the key carries bytes past its last coordinate")),
            }
        }
    }

    /// A COVER PIECE'S ADDRESS, WHICH IS A CARRIAGE AND NOT A PIECE OF A
    /// BODY: the ordered pair of the pieces its domain is assembled from and
    /// the pieces its codomain is assembled from.
    ///
    /// ORDERED, unlike a meet, because a carriage has a direction. A LIST on
    /// BOTH sides, because both passes assemble and in opposite directions:
    /// the pullback takes `Φ⁻¹(V)` over the input pieces under an output
    /// piece's sites, the propagation takes `Φ(U)` over the output pieces
    /// under a carried piece's image, and neither requires the two peels to
    /// agree on granularity.
    ///
    /// Keyed by the domain alone this would assert that `Φ` is injective on
    /// the input, and by the codomain alone that it is injective on the
    /// image. Neither holds: one domain carried by ten versors is
    /// replication, many domains reaching one codomain is merging, and under
    /// either injectivity condition a family with one input piece and ten
    /// output pieces places exactly ONE carriage — the other nine refused by
    /// bookkeeping and not by geometry — leaving no second node in the nerve
    /// and no transition to have an order at all.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Carriage {
        pub domain: Vec<Piece>,
        pub codomain: Vec<Piece>,
    }

    impl Carriage {
        fn put(&self, buf: &mut Vec<u8>) {
            buf.push(a::TAG_CARRIAGE);
            for side in [&self.domain, &self.codomain] {
                crate::transducer::address::put_key_count(buf, side.len());
                for p in side {
                    p.put(buf);
                }
            }
        }

        fn take(bytes: &[u8], pos: &mut usize) -> Option<Carriage> {
            match bytes.get(*pos) {
                Some(&t) if t == a::TAG_CARRIAGE => *pos += 1,
                _ => return None,
            }
            let mut side = || -> Option<Vec<Piece>> {
                let n = crate::transducer::address::take_key_count(bytes, pos)?;
                let mut out = Vec::with_capacity(n.min(bytes.len()));
                for _ in 0..n {
                    out.push(Piece::take(bytes, pos)?);
                }
                Some(out)
            };
            let domain = side()?;
            let codomain = side()?;
            Some(Carriage { domain, codomain })
        }
    }

    impl Coordinate for Carriage {
        const NAME: &'static str = "cover-carriage";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Carriage;

        fn encode(&self) -> CompactBytes {
            let mut buf = Vec::new();
            self.put(&mut buf);
            CompactBytes::written(buf)
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let mut pos = 0usize;
            let c = Carriage::take(bytes, &mut pos)
                .ok_or_else(|| Self::refuse("the bytes do not spell a domain and a codomain"))?;
            match pos == bytes.len() {
                true => Ok(c),
                false => Err(Self::refuse("the key carries bytes past its last coordinate")),
            }
        }
    }

    /// A 1-SIMPLEX OF THE NERVE: two carriages that overlap, named by the
    /// two ends and never by the carriages' positions.
    ///
    /// UNORDERED, because a seam has no direction. The pair is put in the
    /// canonical order at construction, so the two ends read back the way
    /// they were stored and the nerve keeps nothing beside the key (law 12).
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Meet {
        lo: Carriage,
        hi: Carriage,
    }

    impl Meet {
        /// The seam between two carriages, in the order the key stores.
        pub fn of(a: Carriage, b: Carriage) -> Meet {
            match a <= b {
                true => Meet { lo: a, hi: b },
                false => Meet { lo: b, hi: a },
            }
        }

        /// The two ends, read back out of the coordinate itself.
        pub fn ends(&self) -> (&Carriage, &Carriage) {
            (&self.lo, &self.hi)
        }
    }

    impl Coordinate for Meet {
        const NAME: &'static str = "cover-meet";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Meet;

        fn encode(&self) -> CompactBytes {
            let mut buf = vec![a::TAG_MEET];
            self.lo.put(&mut buf);
            self.hi.put(&mut buf);
            CompactBytes::written(buf)
        }

        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — the tag byte at position 0 is the wire's discriminant, not a member chosen"
        )]
        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let mut pos = match bytes.first() {
                Some(&t) if t == a::TAG_MEET => 1usize,
                Some(_) => return Err(Self::refuse("the key opens with another system's tag")),
                None => return Err(Self::refuse("an empty key spells no coordinate")),
            };
            let refuse = || Self::refuse("the bytes do not spell two carriages");
            let lo = Carriage::take(bytes, &mut pos).ok_or_else(refuse)?;
            let hi = Carriage::take(bytes, &mut pos).ok_or_else(refuse)?;
            if pos != bytes.len() {
                return Err(Self::refuse("the key carries bytes past its last coordinate"));
            }
            // THE ORDER IS PART OF THE COORDINATE. A key written with its
            // ends the other way round is a second address for one seam, and
            // accepting it would break `decode(encode(x)) == x` for the value
            // it decodes to (rule 11).
            match lo <= hi {
                true => Ok(Meet { lo, hi }),
                false => Err(Self::refuse("a seam's two ends are not in canonical order")),
            }
        }
    }

    /// AT1 Q4 — THE FRAME'S SPELLING: a length byte, then the minimal
    /// big-endian bytes. Unbounded, and ordered by (length, bytes), which is
    /// the numeric order the carrier reads the succession by (law 2) — not
    /// LEB128, whose bytes order by the low bits, and not a u16 wall.
    pub fn put_frame(k: u64, out: &mut Vec<u8>) {
        let be = k.to_be_bytes();
        let body: Vec<u8> = be.iter().copied().skip_while(|&b| b == 0).collect();
        #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — a constant table: the minimal big-endian bytes of a u64, at most eight")]
        out.push(body.len() as u8);
        out.extend_from_slice(&body);
    }

    /// A u64 COUNT BY ITS OWN SPELLING, LEB128 (ruling (a)): a lane's
    /// measure and a piece count are values, never fixed widths.
    fn put_count(mut n: u64, out: &mut Vec<u8>) {
        loop {
            let low = (n & 0x7f) as u8;
            n >>= 7;
            match n {
                0 => {
                    out.push(low);
                    return;
                }
                _ => out.push(low | 0x80),
            }
        }
    }

    /// The count read back; `None` past ten bytes or on a non-minimal
    /// spelling (a trailing zero group is a second spelling of one value).
    fn take_count(bytes: &[u8], pos: &mut usize) -> Option<u64> {
        let mut n = 0u64;
        for shift in (0..70).step_by(7) {
            let b = *bytes.get(*pos)?;
            *pos += 1;
            n |= u64::from(b & 0x7f).checked_shl(shift)?;
            if b & 0x80 == 0 {
                return (b != 0 || shift == 0).then_some(n);
            }
        }
        None
    }

    /// The frame read back; `None` for a length past eight or a body with a
    /// leading zero, which would be a second spelling of one frame.
    #[expect(clippy::disallowed_methods, reason = "rule 15 — the length byte at the cursor is the spelling's own discriminant, not a member chosen")]
    pub fn take_frame(bytes: &[u8], pos: &mut usize) -> Option<u64> {
        let n = usize::from(*bytes.get(*pos)?);
        let body = bytes.get(*pos + 1..*pos + 1 + n)?;
        if n > 8 || body.first() == Some(&0) {
            return None;
        }
        *pos += 1 + n;
        let mut be = [0u8; 8];
        be.get_mut(8 - n..)?.copy_from_slice(body);
        Some(u64::from_be_bytes(be))
    }

    /// AT1 (i), Q1 — A VERTEX OF THE ATLAS BY ITS COORDINATE, never by its
    /// position in a list: its kind's tag first, then what that kind is
    /// named by. A Word is its TRANSPORT CLASS (one vertex per class, 21
    /// step 1); the Frame is its tag alone; an Unfolded vertex its measure;
    /// a Demand its kind, its chart, its chain's measure per lane and its
    /// piece count — the bodies are evidence and ride the record (54.5 Road
    /// (d)); a Glued vertex its site, the boundary record it holds. A
    /// Rebased vertex is its BASE's class then its own former class (ruled):
    /// the base first, so every vertex rebased on one base is a fibre read
    /// under it; its own class is held on the vertex, the overwrite having
    /// made it underivable.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum VertexAt {
        Frame,
        Word(Transport),
        Rebased { base: Transport, own: Transport },
        Unfolded(Unfolded),
        Demand {
            kind: crate::transducer::sentence::Demand,
            chart: crate::transducer::stalk::Invariant,
            measure: [u64; 4],
            pieces: u64,
        },
        Glued(Site),
    }

    impl VertexAt {
        pub const FRAME: u8 = 0;
        pub const WORD: u8 = 1;
        pub const REBASED: u8 = 2;
        pub const UNFOLDED: u8 = 3;
        pub const DEMAND: u8 = 4;
        pub const GLUED: u8 = 5;
    }

    impl Coordinate for VertexAt {
        const NAME: &'static str = "atlas-vertex";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            let mut buf = Vec::new();
            match self {
                VertexAt::Frame => buf.push(Self::FRAME),
                VertexAt::Word(t) => {
                    buf.push(Self::WORD);
                    buf.extend_from_slice(t.encode().as_bytes());
                }
                VertexAt::Rebased { base, own } => {
                    buf.push(Self::REBASED);
                    a::put_key_bytes(&mut buf, base.encode().as_bytes());
                    a::put_key_bytes(&mut buf, own.encode().as_bytes());
                }
                VertexAt::Unfolded(u) => {
                    buf.push(Self::UNFOLDED);
                    buf.extend_from_slice(u.encode().as_bytes());
                }
                VertexAt::Demand { kind, chart, measure, pieces } => {
                    buf.push(Self::DEMAND);
                    a::put_key_bytes(&mut buf, kind.key().as_bytes());
                    a::put_key_bytes(&mut buf, chart.key().as_bytes());
                    for lane in measure {
                        put_count(*lane, &mut buf);
                    }
                    put_count(*pieces, &mut buf);
                }
                VertexAt::Glued(s) => {
                    buf.push(Self::GLUED);
                    buf.extend_from_slice(&s.0);
                }
            }
            CompactBytes::written(buf)
        }

        #[expect(clippy::disallowed_methods, reason = "rule 15 — the tag byte at position 0 is the wire's discriminant, not a member chosen")]
        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let (Some(&tag), Some(body)) = (bytes.first(), bytes.get(1..)) else {
                return Err(Self::refuse("an empty key spells no vertex"));
            };
            match tag {
                Self::FRAME if body.is_empty() => Ok(VertexAt::Frame),
                Self::WORD => Ok(VertexAt::Word(Transport::decode(body)?)),
                Self::UNFOLDED => Ok(VertexAt::Unfolded(Unfolded::decode(body)?)),
                Self::GLUED => Ok(VertexAt::Glued(Site::decode(body)?)),
                Self::REBASED => {
                    let mut pos = 1usize;
                    let bb = a::take_key_bytes(bytes, &mut pos)
                        .ok_or_else(|| Self::refuse("the base's class is not length-delimited"))?;
                    let ob = a::take_key_bytes(bytes, &mut pos)
                        .ok_or_else(|| Self::refuse("the vertex's own class is not length-delimited"))?;
                    if pos != bytes.len() {
                        return Err(Self::refuse("the key carries bytes past its last coordinate"));
                    }
                    Ok(VertexAt::Rebased { base: Transport::decode(&bb)?, own: Transport::decode(&ob)? })
                }
                Self::DEMAND => {
                    let mut pos = 1usize;
                    let kb = a::take_key_bytes(bytes, &mut pos)
                        .ok_or_else(|| Self::refuse("the demand kind is not length-delimited"))?;
                    let cb = a::take_key_bytes(bytes, &mut pos)
                        .ok_or_else(|| Self::refuse("the chart is not length-delimited"))?;
                    let mut measure = [0u64; 4];
                    for lane in measure.iter_mut() {
                        *lane = take_count(bytes, &mut pos)
                            .ok_or_else(|| Self::refuse("a lane's measure is not a count"))?;
                    }
                    let pieces = take_count(bytes, &mut pos)
                        .ok_or_else(|| Self::refuse("the piece count is not a count"))?;
                    if pos != bytes.len() {
                        return Err(Self::refuse("the key carries bytes past its last coordinate"));
                    }
                    let mut kp = tagged::<Self>(&kb, a::TAG_DEMAND)?;
                    let kind = crate::transducer::sentence::Demand::from_bytes(&kb, &mut kp)
                        .ok_or_else(|| Self::refuse("the bytes spell no demand kind"))?;
                    if kp != kb.len() {
                        return Err(Self::refuse("the demand kind carries trailing bytes"));
                    }
                    let chart = crate::transducer::stalk::Invariant::from_key(&cb)
                        .ok_or_else(|| Self::refuse("the bytes spell no chart"))?;
                    Ok(VertexAt::Demand { kind, chart, measure, pieces })
                }
                _ => Err(Self::refuse("the tag names no vertex kind")),
            }
        }
    }

    /// AT1 (i), Q1–Q3 — A CHART OF THE ATLAS: a vertex's member at one
    /// frame, `vertex ‖ Invariant ‖ frame`, each length-delimited but the
    /// frame, whose spelling delimits itself. The Frame's chart is its tag
    /// alone: it stands on no Invariant and at no frame.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum ChartAt {
        Frame,
        /// Boxed: a member at a frame carries a vertex coordinate and an
        /// Invariant, and the Frame's chart should not carry that footprint.
        At(Box<MemberAt>),
    }

    /// One member of a vertex at one frame, and — for a section's piece
    /// only — the piece it names (AT1, ruled (a) on the chords).
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MemberAt {
        pub vertex: VertexAt,
        pub chart: crate::transducer::stalk::Invariant,
        pub frame: u64,
        pub piece: Option<PieceAt>,
    }

    /// A PIECE BY ITS FIXED-SIZE COORDINATES (ruled): its nucleus — the
    /// body's sphere, canonical — and its shape's moments, as the cover
    /// matches pieces by them. NEVER its support sites: a footprint in a key
    /// is sensor content copied into the address (LD0, R6).
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct PieceAt {
        pub nucleus: crate::transducer::cga::Mv,
        pub moments: Vec<u8>,
    }

    impl ChartAt {
        /// A member's chart at a frame.
        pub fn at(vertex: VertexAt, chart: crate::transducer::stalk::Invariant, frame: u64) -> ChartAt {
            ChartAt::At(Box::new(MemberAt { vertex, chart, frame, piece: None }))
        }

        /// A section's piece within a member's chart at a frame: the end of a
        /// meet chord (AT1, ruled (a)).
        pub fn piece(
            vertex: VertexAt,
            chart: crate::transducer::stalk::Invariant,
            frame: u64,
            piece: PieceAt,
        ) -> ChartAt {
            ChartAt::At(Box::new(MemberAt { vertex, chart, frame, piece: Some(piece) }))
        }

        fn put(&self, buf: &mut Vec<u8>) {
            match self {
                ChartAt::Frame => buf.push(0),
                ChartAt::At(m) => {
                    buf.push(1);
                    a::put_key_bytes(buf, m.vertex.encode().as_bytes());
                    a::put_key_bytes(buf, m.chart.key().as_bytes());
                    put_frame(m.frame, buf);
                    match &m.piece {
                        None => buf.push(0),
                        Some(p) => {
                            buf.push(1);
                            let mut nucleus = Vec::new();
                            p.nucleus.put_canonical(&mut nucleus);
                            a::put_key_bytes(buf, &nucleus);
                            a::put_key_bytes(buf, &p.moments);
                        }
                    }
                }
            }
        }

        fn take(bytes: &[u8], pos: &mut usize) -> Option<ChartAt> {
            let tag = *bytes.get(*pos)?;
            *pos += 1;
            match tag {
                0 => Some(ChartAt::Frame),
                1 => {
                    let vertex = VertexAt::decode(&a::take_key_bytes(bytes, pos)?).ok()?;
                    let chart = crate::transducer::stalk::Invariant::from_key(&a::take_key_bytes(bytes, pos)?)?;
                    let frame = take_frame(bytes, pos)?;
                    let flag = *bytes.get(*pos)?;
                    *pos += 1;
                    let piece = match flag {
                        0 => None,
                        1 => {
                            let nb = a::take_key_bytes(bytes, pos)?;
                            let mut np = 0usize;
                            let nucleus = crate::transducer::cga::Mv::from_bytes(&nb, &mut np)?;
                            (np == nb.len()).then_some(())?;
                            Some(PieceAt { nucleus, moments: a::take_key_bytes(bytes, pos)? })
                        }
                        _ => return None,
                    };
                    Some(ChartAt::At(Box::new(MemberAt { vertex, chart, frame, piece })))
                }
                _ => None,
            }
        }
    }

    impl Coordinate for ChartAt {
        const NAME: &'static str = "atlas-chart";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            let mut buf = Vec::new();
            self.put(&mut buf);
            CompactBytes::written(buf)
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let mut pos = 0usize;
            let c = ChartAt::take(bytes, &mut pos).ok_or_else(|| Self::refuse("the bytes spell no chart of the atlas"))?;
            match pos == bytes.len() {
                true => Ok(c),
                false => Err(Self::refuse("the key carries bytes past its last coordinate")),
            }
        }
    }

    /// AT1 (i), Q2 — A FACE IS AN OVERLAP OF TWO CHARTS, keyed by the two:
    /// the order is the orientation. It carries its place (the transition
    /// and what rides it); its word is derived from its two ends' vertices,
    /// never a coordinate of the key.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Overlap {
        pub from: ChartAt,
        pub to: ChartAt,
    }

    impl Coordinate for Overlap {
        const NAME: &'static str = "atlas-overlap";
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;

        fn encode(&self) -> CompactBytes {
            let mut buf = Vec::new();
            self.from.put(&mut buf);
            self.to.put(&mut buf);
            CompactBytes::written(buf)
        }

        fn decode(bytes: &[u8]) -> Result<Self, CoordinateError> {
            let mut pos = 0usize;
            let refuse = || Self::refuse("the bytes do not spell two charts");
            let from = ChartAt::take(bytes, &mut pos).ok_or_else(refuse)?;
            let to = ChartAt::take(bytes, &mut pos).ok_or_else(refuse)?;
            match pos == bytes.len() {
                true => Ok(Overlap { from, to }),
                false => Err(Self::refuse("the key carries bytes past its last coordinate")),
            }
        }
    }

    /// Every overlap FROM one chart is a fibre read at the chart's own
    /// bytes: the key opens with them.
    impl Stratum<Overlap> for ChartAt {
        fn opening(&self) -> CompactBytes {
            self.encode()
        }
    }
}
