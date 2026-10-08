// 78.139 — A UNIVERSE ON THE COURT: its curvature, basket and join stated
// once each by its sitting reserve; its origin of version 2 stating its
// curvature and parent face; and the CORE TUBE that joins a parent's core to
// the core of the universe it bounced into, kept by both reserves.

use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Origin, Reservation};
use crate::anastome::dialogue::handoff::{ConvertOut, Handed, Handoff, HandoffRefusal, Mouth, State, Tube};
use crate::anastome::dialogue::universe::{Genesis, Parent, Statement, Stated};
use crate::chronometry::Cite;
use crate::server::cognition::Kernel;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

/// Domain `d`'s key, its reserve, and a line of it.
fn domain(d: u8) -> SigningKey {
    key(d)
}
fn reserve(d: u8) -> SigningKey {
    key(d + 20)
}
fn line(d: u8) -> SigningKey {
    key(d + 40)
}

/// Domains 4 (the parent) and 5 (the child), each with its reserve and a
/// line, the reserves designated; nothing originated.
fn founded() -> Kernel {
    let mut k = Kernel::new();
    for d in [4u8, 5] {
        let mut tiers = vec![vk(&reserve(d)), vk(&line(d))];
        tiers.sort_unstable();
        let g = DomainGenesis { domain: vk(&domain(d)), tiers };
        assert!(matches!(k.admit_domain(&g.sign(&domain(d))), Founded::Sealed { .. }));
        let r = Reservation { domain: vk(&domain(d)), tier: vk(&reserve(d)), nonce: [0; 32] };
        assert!(matches!(k.designate_reserve(&r.sign(&domain(d))), Founded::Reserved { .. }));
        let anchor = k.anchor();
        k.treasury.genesis(vk(&reserve(d)), [10, 0, 0, 0], anchor);
    }
    k
}

fn origin(d: u8, universe: Option<Genesis>) -> Origin {
    let version = match universe {
        Some(_) => 2,
        None => 1,
    };
    Origin { domain: vk(&domain(d)), kind: 1, version, commitment: [d; 32], universe }
}

fn state(k: &mut Kernel, d: u8, stated: Stated, by: &SigningKey) -> Founded {
    k.state(&Statement { domain: vk(&domain(d)), stated }.sign(by))
}

/// The core tube from domain 4's core to domain 5's: unit mass, the near
/// end's `R` 1, the far end's `R` `to_r`.
fn core_tube(to_r: (u64, u64)) -> Mouth {
    let one = crate::transducer::element::identity_versor();
    Mouth {
        from_tier: vk(&reserve(4)),
        to_tier: vk(&reserve(5)),
        keeper: vk(&reserve(4)),
        from_frame: one.clone(),
        to_frame: one,
        toll: [0; 4],
        nonce: [4; 32],
        tariff: None,
        tube: Some(Tube { mass: (1, 1), from_r: (1, 1), to_r, supersedes: None, core: true }),
    }
}

fn open(k: &mut Kernel, m: Mouth) -> [u8; 32] {
    let id = m.id();
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(m)).sign(&reserve(4))), Handed::Sealed { .. }), "offered");
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&reserve(5))), Handed::Sealed { state: State::Accepted, .. }), "accepted");
    id
}

/// THE SITTING RESERVE STATES, ONCE EACH: the domain key, a line, an
/// unfounded domain — refused; a curvature not positive and baskets empty,
/// unordered or massless — malformed; stated, a second is never an edit. A
/// conversion at the reserve waits on the basket.
#[test]
fn a_universe_states_its_curvature_and_basket_once_by_its_sitting_reserve() {
    let mut k = founded();
    assert_eq!(state(&mut k, 4, Stated::Curvature((1, 1)), &domain(4)), Founded::Unsigned, "the founder's key no longer speaks");
    assert_eq!(state(&mut k, 4, Stated::Curvature((1, 1)), &line(4)), Founded::Unsigned);
    assert_eq!(k.state(&Statement { domain: [9; 32], stated: Stated::Curvature((1, 1)) }.sign(&reserve(4))), Founded::NotFounded);
    assert_eq!(state(&mut k, 4, Stated::Curvature((0, 1)), &reserve(4)), Founded::Malformed);
    assert!(matches!(state(&mut k, 4, Stated::Curvature((3, 2)), &reserve(4)), Founded::Stated { .. }));
    assert_eq!(k.chain.head().map(|h| h.cites.clone()), Some(vec![Cite::Stated { domain: vk(&domain(4)), kind: 8, body: Stated::Curvature((3, 2)).hash() }]));
    assert_eq!(state(&mut k, 4, Stated::Curvature((5, 2)), &reserve(4)), Founded::AlreadyStated, "never an edit");
    // The basket gates conversions at the reserve.
    let out = ConvertOut { tier: vk(&reserve(4)), holder: [7; 32], identity: [8; 32], g: 4, rate: [(1, 2), (0, 1), (0, 1), (0, 1)], nonce: [1; 32] };
    let convert = |k: &mut Kernel, nonce: u8| k.handoff(&Handoff::ConvertOut(Box::new(ConvertOut { nonce: [nonce; 32], ..out.clone() })).sign(&reserve(4)));
    assert_eq!(convert(&mut k, 1), Handed::Refused(HandoffRefusal::NoBasket), "the reserve unit is the ancients' basket");
    for bad in [Vec::new(), vec![(8, (3, 1)), (1, (2, 1))], vec![(1, (0, 1))]] {
        assert_eq!(state(&mut k, 4, Stated::Basket(bad), &reserve(4)), Founded::Malformed);
    }
    // Ammonia stated as its elements' masses, with water and iron.
    let basket = vec![(1, (21, 1)), (7, (14, 1)), (8, (160, 1)), (26, (1, 10))];
    assert!(matches!(state(&mut k, 4, Stated::Basket(basket.clone()), &reserve(4)), Founded::Stated { .. }));
    assert_eq!(state(&mut k, 4, Stated::Basket(vec![(1, (1, 1))]), &reserve(4)), Founded::AlreadyStated);
    assert!(matches!(convert(&mut k, 2), Handed::Sealed { .. }), "converted once its basket stands");
    let read = k.universe_reading(&vk(&domain(4)));
    assert_eq!((read.originated, read.curvature, read.parent, read.basket.clone()), (false, Some((3, 2)), None, basket.clone()));
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.universe_reading(&vk(&domain(4))), read, "held across a restart");
}

/// A ROOT ORIGINATES UNWITNESSED, A CHILD THROUGH ITS CORE TUBE: version 2
/// carries its curvature and parent face exactly; the parent's core tube is
/// offered by its reserve, kept by both; the child's origin names it and
/// states the far end's `R`. A core tube from an unoriginated domain, by a
/// line, or on a curvature its domain did not state is refused; an origin
/// naming a tube that is no core into it, or disagreeing with its far `R`,
/// is refused.
#[test]
fn a_child_universe_originates_through_its_parents_core_tube() {
    let mut k = founded();
    assert_eq!(k.handoff(&Handoff::Mouth(Box::new(core_tube((2, 1)))).sign(&reserve(4))), Handed::Refused(HandoffRefusal::NotAUniverse), "the parent is no universe yet");
    let root = Genesis { curvature: (1, 1), parent: Parent::Unwitnessed };
    assert_eq!(k.record_origin(&Origin { version: 1, ..origin(4, Some(root)) }.sign(&domain(4))), Founded::Malformed, "version and fields agree");
    assert!(matches!(k.record_origin(&origin(4, Some(root)).sign(&domain(4))), Founded::Originated { .. }));
    let by_line = Mouth { from_tier: vk(&line(4)), ..core_tube((2, 1)) };
    assert_eq!(k.handoff(&Handoff::Mouth(Box::new(by_line)).sign(&line(4))), Handed::Refused(HandoffRefusal::NotReserve), "only the ancients offer a core");
    let bent = Mouth { tube: Some(Tube { from_r: (3, 1), ..core_tube((2, 1)).tube.expect("a tube") }), ..core_tube((2, 1)) };
    assert_eq!(k.handoff(&Handoff::Mouth(Box::new(bent)).sign(&reserve(4))), Handed::Refused(HandoffRefusal::NoCurvature));
    let tube = open(&mut k, core_tube((2, 1)));
    let child = |r: (u64, u64), parent: Parent| origin(5, Some(Genesis { curvature: r, parent }));
    assert_eq!(k.record_origin(&child((2, 1), Parent::Core([9; 32])).sign(&domain(5))), Founded::NotACore);
    assert_eq!(k.record_origin(&child((5, 1), Parent::Core(tube)).sign(&domain(5))), Founded::Malformed, "the far end's R is the child's");
    assert!(matches!(k.record_origin(&child((2, 1), Parent::Core(tube)).sign(&domain(5))), Founded::Originated { .. }));
    let read = k.universe_reading(&vk(&domain(5)));
    assert_eq!((read.originated, read.curvature, read.parent), (true, Some((2, 1)), Some(Parent::Core(tube))), "every horizon has its far face");
    // Kept by both reserves: the child's feeds the throat.
    let fund = k.anchor();
    k.treasury.genesis(vk(&reserve(5)), [3, 0, 0, 0], fund);
    assert!(matches!(k.handoff(&Handoff::Feed { mouth: tube, mu_s: 3, seq: 0 }.sign(&reserve(5))), Handed::Sealed { .. }), "the second keeper");
    assert!(k.tube_of(&tube).is_some_and(|t| t.mass == (4, 1)));
}

/// A ROOT IS JOINED WHEN ITS PARENT IS FOUND: a v1 origin with its curvature
/// stated, then the parent's core tube into it, then JOIN by its reserve —
/// once; before an origin it is no universe; a tube that is no core into it
/// is refused.
#[test]
fn a_root_joins_its_parents_core_once() {
    let mut k = founded();
    let root = Genesis { curvature: (1, 1), parent: Parent::Unwitnessed };
    assert!(matches!(k.record_origin(&origin(4, Some(root)).sign(&domain(4))), Founded::Originated { .. }));
    assert_eq!(state(&mut k, 5, Stated::Join([1; 32]), &reserve(5)), Founded::NotAUniverse, "originate first");
    assert!(matches!(k.record_origin(&origin(5, None).sign(&domain(5))), Founded::Originated { .. }), "a v1 root");
    assert!(matches!(state(&mut k, 5, Stated::Curvature((2, 1)), &reserve(5)), Founded::Stated { .. }), "v1 + its curvature");
    assert_eq!(state(&mut k, 5, Stated::Join([1; 32]), &reserve(5)), Founded::NotACore);
    let tube = open(&mut k, core_tube((2, 1)));
    assert!(matches!(state(&mut k, 5, Stated::Join(tube), &reserve(5)), Founded::Stated { .. }));
    assert_eq!(k.universe_reading(&vk(&domain(5))).parent, Some(Parent::Core(tube)), "joined");
    assert_eq!(state(&mut k, 5, Stated::Join(tube), &reserve(5)), Founded::AlreadyStated, "once");
}

/// THE WIRE: an origin of version 2, every statement, the universe read and
/// its reading, and a core tube read back whole; a byte left over is none.
#[test]
fn the_universe_acts_read_back_whole() {
    use crate::anastome::dialogue::domain::{decode_domain_reply, decode_domain_request, encode_domain_reply, encode_origin, DomainRequest};
    use crate::anastome::dialogue::handoff::{decode_handoff_request, encode_handoff, HandoffRequest};
    use crate::anastome::dialogue::universe::{decode_universe_reading, encode_statement, encode_universe_read, encode_universe_reading, UniverseReading};
    let signed = origin(5, Some(Genesis { curvature: (2, 3), parent: Parent::Core([6; 32]) })).sign(&domain(5));
    assert_eq!(decode_domain_request(&encode_origin(&signed)), Some(DomainRequest::Origin(signed)));
    for stated in [Stated::Curvature((7, 3)), Stated::Basket(vec![(1, (1, 2)), (26, (3, 1))]), Stated::Join([2; 32])] {
        let s = Statement { domain: vk(&domain(5)), stated }.sign(&reserve(5));
        let mut asked = encode_statement(&s);
        assert_eq!(decode_domain_request(&asked), Some(DomainRequest::State(s)));
        asked.push(0);
        assert_eq!(decode_domain_request(&asked), None);
    }
    assert_eq!(decode_domain_request(&encode_universe_read([3; 32])), Some(DomainRequest::ReadUniverse([3; 32])));
    let r = UniverseReading { originated: true, curvature: Some((1, 2)), parent: Some(Parent::Unwitnessed), basket: vec![(8, (1, 1))] };
    assert_eq!(decode_universe_reading(&encode_universe_reading(&r)), Some(r));
    for f in [Founded::AlreadyStated, Founded::NotACore, Founded::NotAUniverse, Founded::Malformed] {
        assert_eq!(decode_domain_reply(&encode_domain_reply(&f)), Some(f));
    }
    let core = Handoff::Mouth(Box::new(core_tube((2, 1)))).sign(&reserve(4));
    assert_eq!(decode_handoff_request(&encode_handoff(&core)), Some(HandoffRequest::Signed(core)));
    let flat = Mouth { tube: Some(Tube { core: false, ..core_tube((2, 1)).tube.expect("a tube") }), ..core_tube((2, 1)) };
    assert_ne!(flat.id(), core_tube((2, 1)).id(), "a core is its own throat");
}
