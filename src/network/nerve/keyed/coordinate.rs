// A21, §4.4 — THE COORDINATE SYSTEM AS A TYPE: which coordinate an index
// answers, held where the compiler reads it.
//
//! `Keyed` already carried its coordinate system — as a FIELD, set by
//! `for_system`, checked by `read_exact` and `read_fiber`. Every production
//! carrier in the tree left it `Any` and called `insert`/`get`, which do not
//! check it. The declaration existed and nothing was declared: `by_class`,
//! whose doc comment says "chart key → the Word vertices on that chart",
//! was handed a member key, answered nothing, and `incident_chart` read
//! every idea as standing on no chart.
//!
//! A doc comment is not a gate, and a runtime check nobody calls is not a
//! gate either. This lifts the declaration into the TYPE: a carrier names
//! the system its keys belong to, a [`Key`] names the system it was written
//! from, and `insert` will not typecheck unless they agree. The byte check
//! stays underneath, because two systems can open with the same tag — a
//! section key is `chart ‖ carriage` and opens with `TAG_CHART` exactly as
//! a chart key does, so the tag separates neither and the TYPE separates
//! both.
//!
//! §4.4: where a type can hold the rule, it does, and the type is the gate.
//! Not as a preference over a lint — the lint is impossible. Nothing in the
//! source distinguishes a correct key from a wrong one when both are
//! `Vec<u8>`.

use super::*;
use std::marker::PhantomData;

/// A COORDINATE SYSTEM, AS A TYPE.
///
/// One implementor per system a key can be written from. `SYSTEM` is the
/// runtime declaration this refines — it is what the bytes are checked
/// against — and two distinct types may name the same `SYSTEM` when their
/// keys open with the same tag and differ in DEPTH (`Chart` and `Section`).
/// The type separates those; the tag cannot.
pub trait Coordinate: Copy + Clone + std::fmt::Debug + PartialEq + Eq + 'static {
    /// The runtime system these keys belong to, for the byte check.
    const SYSTEM: CoordinateSystem;
    /// The system's name, for a refusal that says which coordinate it is.
    const NAME: &'static str;

    /// The schema this system's keys are written from, where one is
    /// declared. `Some` makes a key's DEPTH checked as well as its tag —
    /// which is what separates a chart key from a section key written over
    /// it. `None` leaves the tag as the only byte check, and that is a
    /// statement about this system, not a default to leave standing.
    fn schema() -> Option<&'static Schema> {
        None
    }
}

/// THE COORDINATE SYSTEMS, one marker per A21 tag.
///
/// A carrier declares one of these and a key is written from one of these.
/// `Any` is the undeclared carrier — the state every carrier in the tree
/// was in — and it is the only one whose `insert` takes loose bytes.
pub mod coord {
    use super::{Coordinate, CoordinateSystem, Schema};

    macro_rules! coordinate {
        ($($name:ident => $system:ident, $label:literal;)*) => {$(
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
            pub struct $name;
            impl Coordinate for $name {
                const SYSTEM: CoordinateSystem = CoordinateSystem::$system;
                const NAME: &'static str = $label;
            }
        )*};
    }

    coordinate! {
        Any      => Any,      "any";
        Rotor    => Rotor,    "rotor";
        Versor   => Versor,   "versor";
        Word     => Word,     "word";
        Form     => Form,     "form";
        Chart    => Chart,    "chart";
        Cert     => Cert,     "cert";
        Listing  => Listing,  "listing";
        Sel      => Sel,      "sel";
        Hue      => Hue,      "hue";
        Assign   => Assign,   "assign";
        Gauge    => Gauge,    "gauge";
        Plane    => Plane,    "plane";
        Simplex  => Simplex,  "simplex";
        Cocycle  => Cocycle,  "cocycle";
        Task     => Task,     "task";
        Demand   => Demand,   "demand";
        DemandAt => DemandAt, "demand-at";
        Residual => Residual, "residual";
    }

    /// THE CLASS COORDINATE, the one system whose key layout is parsed
    /// rather than trusted: `ClassVector::class_schema` reads its five
    /// segments, so a class key is checked to its last coordinate and a
    /// key carrying bytes past it is refused by name.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Class;
    impl Coordinate for Class {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Class;
        const NAME: &'static str = "class";
        fn schema() -> Option<&'static Schema> {
            Some(crate::transducer::sentence::ClassVector::class_schema())
        }
    }

    /// A SECTION OF A CHART — `chart ‖ carriage`, which opens with
    /// `TAG_CHART` exactly as a chart key does and is NOT a chart key.
    ///
    /// This is R1's defect written as a type. `by_class` answers [`Chart`];
    /// `acts` is keyed by [`Section`]; they share a tag and share nothing
    /// else, and for eight sites the difference lived only in prose.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Section;
    impl Coordinate for Section {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Chart;
        const NAME: &'static str = "section";
    }

    /// A MEMBER OF A FORM — the vertex key a form's member is written
    /// under. Opens with `TAG_FORM` and is a strictly finer coordinate
    /// than the form's own address.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Member;
    impl Coordinate for Member {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Form;
        const NAME: &'static str = "member";
    }

    /// A TRANSPORT CLASS — `kind ‖ word class` (21 step 1).
    ///
    /// `TransportClass::key` opens with the shell kind, not an A21 tag, so
    /// there is no tag to check and `SYSTEM` is `Any`: the byte gate says
    /// nothing here and the TYPE says everything. That is the honest
    /// spelling — a system whose keys are not tag-led cannot pretend to a
    /// check it does not have.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Transport;
    impl Coordinate for Transport {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;
        const NAME: &'static str = "transport-class";
    }

    /// A VERTEX'S POSITION IN ITS OWN FORM — `vertex_ix_key`, an ordinal
    /// and not a coordinate (rule 15). Typed so that it cannot be handed
    /// to a carrier that answers a coordinate, which is the whole of the
    /// distinction rule 15 makes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct VertexIx;
    impl Coordinate for VertexIx {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;
        const NAME: &'static str = "vertex-index";
    }

    /// A SITE OF THE COVER — where a citation stands, as distinct from what
    /// it cites (54.7 B5a). It is the Glued vertex's own record as
    /// `put_vertex` writes it: the host's statement of its own boundary, in
    /// the form's local encoding and under no A21 tag. `NerveEdge` is keyed
    /// `site ‖ target`; the site alone is this.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Site;
    impl Coordinate for Site {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;
        const NAME: &'static str = "cover-site";
    }

    /// A HOLONOMY CLASS — `Rotor::class_key`, which opens `TAG_CLASS` and
    /// is NOT a `ClassVector` key even though that opens with the same
    /// byte. A class vector spells five coordinates and this spells one, so
    /// [`Class`]'s schema refuses it by name — two systems over one tag,
    /// separated by the type and by nothing else.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Holonomy;
    impl Coordinate for Holonomy {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Class;
        const NAME: &'static str = "holonomy-class";
    }

    /// THE FORM-LOCAL SYSTEMS. Each opens with its own discriminant byte in
    /// the form's private namespace, NOT an A21 tag — `unfolded_key` opens
    /// with `4`, which is `TAG_FORM` in the other namespace and means
    /// nothing here. They are separate systems that happen to share a byte,
    /// which is precisely the case a tag check cannot decide and a type can.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Unfolded;
    impl Coordinate for Unfolded {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;
        const NAME: &'static str = "unfolded-measure";
    }

    /// ONE DEMAND VERTEX OF ONE FORM — `(kind, chart)` (50.5), each end
    /// length-delimited, opening with neither's tag.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct DemandVertex;
    impl Coordinate for DemandVertex {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;
        const NAME: &'static str = "demand-vertex";
    }

    /// A MEMBER'S SECTION — `transport class ‖ chart ‖ address ‖ carriage`
    /// (18.2, 21 step 1, 52.5 item 3). It opens with the shell kind, like
    /// the transport class it extends, and is a strictly finer coordinate:
    /// two members of one class on one chart are two sections, and reading
    /// one carrier at the other's key is the defect this separates.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct MemberSection;
    impl Coordinate for MemberSection {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Any;
        const NAME: &'static str = "member-section";
    }

    /// A PLACEMENT — `vertex transport ‖ chart` (18.2). Two carriages on one
    /// transport class are two placements because the chart separates them;
    /// keyed by the transport class alone they were one entry conflicting
    /// with itself, and `place_conflict` dropped both.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct Placement;
    impl Coordinate for Placement {
        const SYSTEM: CoordinateSystem = CoordinateSystem::Word;
        const NAME: &'static str = "placement";
    }
}

/// A KEY WRITTEN FROM A DECLARED COORDINATE SYSTEM.
///
/// The bytes are the object's coordinates written out (law 2); what this
/// adds is the system they were written from, carried in the type so that
/// handing them to a carrier of another system does not compile.
///
/// A key OWNS ITS BYTES ONCE AND SHARES THEM (Z4c): a form's address is
/// megabytes, and every carrier naming it points at the same allocation.
pub struct Key<C: Coordinate> {
    bytes: std::sync::Arc<[u8]>,
    _coord: PhantomData<C>,
}

impl<C: Coordinate> Clone for Key<C> {
    fn clone(&self) -> Self {
        Key { bytes: std::sync::Arc::clone(&self.bytes), _coord: PhantomData }
    }
}

impl<C: Coordinate> std::fmt::Debug for Key<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key<{}>({} bytes)", C::NAME, self.bytes.len())
    }
}

impl<C: Coordinate> PartialEq for Key<C> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl<C: Coordinate> Eq for Key<C> {}

impl<C: Coordinate> std::ops::Deref for Key<C> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<C: Coordinate> AsRef<[u8]> for Key<C> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<C: Coordinate> Key<C> {
    /// WRITE A KEY OF THIS SYSTEM, checking the bytes spell it.
    ///
    /// The tag is checked against the system; where the system declares a
    /// schema, every coordinate is parsed and a key carrying bytes past its
    /// last one is refused — which is the check that separates a chart key
    /// from a section key written over it, since the tag does not.
    pub fn write(bytes: impl AsRef<[u8]>) -> Result<Key<C>, SchemaRefusal> {
        let bytes = bytes.as_ref();
        if let Some(expected) = C::SYSTEM.tag() {
            match crate::transducer::address::tag_of(bytes) {
                Some(tag) if tag == expected => {}
                Some(tag) => {
                    return Err(SchemaRefusal::IncompatibleSystem {
                        expected: C::SYSTEM,
                        actual_tag: tag,
                    })
                }
                // An empty key spells no coordinate at all. `Prefix` refuses
                // it on the read side for the same reason (rule 11).
                None => {
                    return Err(SchemaRefusal::IncompatibleSystem {
                        expected: C::SYSTEM,
                        actual_tag: 0,
                    })
                }
            }
        }
        if let Some(schema) = C::schema() {
            schema.whole(bytes).map_err(SchemaRefusal::Stratum)?;
        }
        Ok(Key { bytes: std::sync::Arc::from(bytes), _coord: PhantomData })
    }

    /// THE OBJECT'S OWN COPY, filed without a second allocation (Z4c).
    ///
    /// Same checks as [`Key::write`]; what differs is that the caller hands
    /// the one allocation the object already holds instead of a scratch
    /// buffer, so every carrier naming a megabyte address points at it.
    pub fn from_shared(bytes: std::sync::Arc<[u8]>) -> Result<Key<C>, SchemaRefusal> {
        Key::<C>::write(&bytes[..])?;
        Ok(Key { bytes, _coord: PhantomData })
    }

    /// THE ADDRESS AS A KEY OF THIS SYSTEM, sharing its bytes.
    pub fn from_address(a: &crate::transducer::address::Address) -> Result<Key<C>, SchemaRefusal> {
        Key::<C>::from_shared(a.shared())
    }

    /// A KEY THIS SITE ASSERTS IT WROTE, checked on neither tag nor depth.
    ///
    /// The escape hatch, named so it is legible at its site and countable
    /// across the tree. It exists for the systems whose layout has no
    /// schema and whose tag is written by a producer this module cannot
    /// reach; every use is a place where the gate is a comment again, and
    /// the count of them is the remaining work.
    pub fn declared(bytes: impl AsRef<[u8]>) -> Key<C> {
        Key { bytes: std::sync::Arc::from(bytes.as_ref()), _coord: PhantomData }
    }

    /// As [`Key::declared`], taking the object's own allocation (Z4c).
    pub fn declared_shared(bytes: std::sync::Arc<[u8]>) -> Key<C> {
        Key { bytes, _coord: PhantomData }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// THE BYTES, AS THE OPENING OF A FINER COORDINATE.
    ///
    /// A key of system `C` is the coarse stratum of the keys written over
    /// it — a transport class opening a member section, a chart opening a
    /// section. Extending one is a real operation and this is where the
    /// coordinate is deliberately left behind; the result is not a key of
    /// any system until whoever finishes it says which.
    pub fn into_vec(self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    /// The one allocation, to file under without copying (Z4c).
    pub fn shared(&self) -> std::sync::Arc<[u8]> {
        std::sync::Arc::clone(&self.bytes)
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// The coordinate this key descends on, or `None` where it is empty —
    /// an empty key names the whole carrier and that is not a coordinate.
    pub fn prefix(&self) -> Option<Prefix<'_>> {
        Prefix::new(&self.bytes)
    }
}
