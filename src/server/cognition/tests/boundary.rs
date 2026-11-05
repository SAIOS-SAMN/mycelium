// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-HOSTING §5a.3–5a.6, CONCORDANCE 4.1, 6.2a, §0, ROADMAP track 24 B3 and
// B4 — THE BOUNDARY AT THE VERTEX: what crosses a universe's universal core
// tube is rebuilt under the universal laws and refused by the law it breaks;
// a universe's act that reaches the hypha other than from its sitting
// reserve is refused `NotThroughCore`; a domain whose reserve does not
// converse on every network clause has its crossings refused `NotSignatory`;
// and a key's own acts stand outside the tube.

use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::uplink::{Child, ChildNet, TierFold, Uplinked};
use crate::physics::equilibrium::{carried_body, BodyMoments};
use crate::physics::laws::{BodyState, Breach, Law, Transition};
use crate::server::cognition::Court;
use crate::transducer::cga::{rotator, sphere_at, Mv};
use ed25519_dalek::SigningKey;
use num_rational::BigRational as Q;

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

fn domain_key() -> SigningKey {
    SigningKey::from_bytes(&[4; 32])
}

/// The domain's sitting reserve: its universal core.
fn reserve_key() -> SigningKey {
    SigningKey::from_bytes(&[9; 32])
}

/// A line of the domain: a tier that is not its reserve.
fn line_key() -> SigningKey {
    SigningKey::from_bytes(&[11; 32])
}

/// The domain founded naming its reserve and a line, and the reserve
/// designated.
fn founded() -> Court {
    let mut k = Court::new();
    let mut tiers = vec![vk(&reserve_key()), vk(&line_key())];
    tiers.sort();
    let genesis = DomainGenesis { domain: vk(&domain_key()), tiers }.sign(&domain_key());
    assert!(matches!(k.hear_domain(&genesis), Founded::Sealed { .. }), "founded");
    let r = Reservation { domain: vk(&domain_key()), tier: vk(&reserve_key()), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&r.sign(&domain_key())), Founded::Reserved { now: true, .. }), "reserved");
    k
}

/// The (3,4,5) turn in (e₁, e₂), `2 + e₁₂`: a rational rotor, no angle.
fn turn() -> Mv {
    rotator(&q(2), &q(1))
}

/// THE TURN CARRYING TWO BODIES, heights 7 → 8: lawful under every law of
/// the register.
fn lawful() -> Transition {
    let bodies = [[1, 0, 0], [0, 2, 0]]
        .iter()
        .map(|c| {
            let body = BodyMoments {
                centre: c.iter().map(|x| q(*x)).collect(),
                mass: q(1),
                moments: vec![vec![q(1), q(0), q(0)], vec![q(0), q(0), q(0)], vec![q(0), q(0), q(0)]],
            };
            let nucleus = sphere_at(&body.centre, &q(1));
            let after = BodyState { nucleus: turn().sandwich(&nucleus), body: carried_body(&body, &turn()).expect("a rigid carriage") };
            (BodyState { nucleus, body }, after)
        })
        .collect();
    Transition { motion: turn(), heights: (7, 8), bodies, hues: Default::default(), fluxes: Vec::new() }
}

/// The same turn standing still at height 8: it breaks temporal
/// monotonicity alone.
fn still() -> Transition {
    Transition { heights: (8, 8), ..lawful() }
}

/// A fold of `tier` over `prefix`, a trade between two identities that
/// makes nothing, stating `transitions`.
fn fold(tier: &SigningKey, prefix: u8, transitions: Vec<Transition>) -> TierFold {
    TierFold {
        tier: vk(tier),
        prefix: [prefix; 32],
        rows: vec![([1; 32], [5, 0, 0, 0]), ([2; 32], [-5, 0, 0, 0])],
        declared: Vec::new(),
        children: Vec::new(),
        handoffs: Vec::new(),
        transitions,
    }
}

/// B3 — A CROSSING IS REFUSED BY THE LAW IT BREAKS: the reserve's fold
/// stating a transition whose height stands still is refused
/// `NotLawful`, naming the transition and `Law::Time` with its breach, and
/// nothing seals; a line carried in the reserve's fold is rebuilt under the
/// same laws, and its transition is named by the path to the line. A
/// transition the crossing states is never admitted on a name: the fold
/// names no law, and the hypha computes each law's verdict on the
/// transition's own coordinates.
#[test]
fn crossing_refused_by_the_law_it_breaks() {
    let mut k = founded();
    let head = k.chain.head_height();
    let refused = k.hear_uplink(&fold(&reserve_key(), 7, vec![lawful(), still()]).sign(&reserve_key()));
    let Uplinked::NotLawful { path, index, breaches } = refused else { panic!("refused by the law: {refused:?}") };
    assert_eq!((path, index), (Vec::new(), 1), "the second transition of the fold itself");
    assert_eq!(breaches, vec![(Law::Time, Breach::HeightNotAdvanced { before: 8, after: 8 })], "the law it breaks, with its breach");
    assert_eq!(k.chain.head_height(), head, "nothing sealed");
    let line = fold(&line_key(), 3, vec![still()]).sign(&line_key());
    let part = ChildNet {
        child: Child::Carried(Box::new(line)),
        rows: Vec::new(),
        declared: Vec::new(),
        included_rows: Vec::new(),
        included_declared: Vec::new(),
        rate: [(1, 1); 4],
    };
    let carrier = TierFold { children: vec![part], ..fold(&reserve_key(), 8, vec![lawful()]) };
    let refused = k.hear_uplink(&carrier.sign(&reserve_key()));
    assert!(
        matches!(&refused, Uplinked::NotLawful { path, index: 0, breaches } if *path == vec![0] && breaches.iter().all(|(l, _)| *l == Law::Time)),
        "the carried line's transition, by its path: {refused:?}"
    );
    assert_eq!(k.chain.head_height(), head, "nothing sealed");
}

/// A final point of the mesh at tick `h`.
fn fp(h: u64) -> crate::chronometry::tuniversal::TPoint {
    use crate::chronometry::tuniversal::{Origin, TPoint};
    TPoint { origin: Origin { genesis: [9; 32], epoch: 0 }, h, lanes: [h, 0, 0, 0].into(), k: h }
}

/// B4 — A UNIVERSE'S ACT OFF ITS CORE IS REFUSED `NotThroughCore`, naming
/// the act and the domain: a fold, an anchor, a map, a statement, a
/// manifest and a conversion out, each signed by a line of the domain (and
/// the anchor by its founder) rather than its sitting reserve, reach the
/// hypha other than at the universe's vertex. Nothing seals.
#[test]
fn universe_act_off_the_core_refused_not_through_core() {
    use crate::anastome::dialogue::atlas::{LedgerEntry, Ledgered, MapAct, MapRecord, Mapped};
    use crate::anastome::dialogue::domain::BigBangRecord;
    use crate::anastome::dialogue::endowment::{Granted, Manifest};
    use crate::anastome::dialogue::handoff::{ConvertOut, Handed, Handoff};
    use crate::anastome::dialogue::universe::{Geometry, Stated, Statement};
    use crate::court::admission::{Boundary, UniverseAct};
    let mut k = founded();
    let domain = vk(&domain_key());
    let off = |act| Boundary::NotThroughCore { act, domain };
    let origin = BigBangRecord { domain, kind: 1, version: 3, commitment: [1; 32], universe: None };
    assert!(matches!(k.record_origin(&origin.sign(&domain_key())), Founded::Originated { .. }), "a universe");
    let head = k.chain.head_height();
    let line = line_key();
    assert_eq!(k.hear_uplink(&fold(&line, 7, vec![lawful()]).sign(&line)), Uplinked::Boundary(off(UniverseAct::Fold)));
    let entry = LedgerEntry { domain, u: 1, commitment: [1; 32], events: [0; 32], point: fp(10), parent: [0; 32], applied: Vec::new(), nonce: [0; 32] };
    for by in [&line, &domain_key()] {
        assert_eq!(k.ledger(&entry.clone().sign(by), Some(fp(10))), Ledgered::Boundary(off(UniverseAct::Anchor)), "an anchor off the core");
    }
    let map = MapAct { domain, point: fp(10), record: MapRecord::Mouth { place: ([3; 32], 0), freshness: 1 }, nonce: [1; 32] };
    assert_eq!(k.map(&map.sign(&line), Some(fp(10))), Mapped::Boundary(off(UniverseAct::Map)));
    let statement = Statement { domain, stated: Stated::Geometry(Geometry::Curved((1, 1))) };
    assert_eq!(k.state(&statement.sign(&line)), Founded::Boundary(off(UniverseAct::Statement)));
    let manifest = Manifest { domain, nuclei: vec![(1, num_bigint::BigUint::from(1u8))], nonce: [2; 32] };
    assert_eq!(k.manifest(&manifest.sign(&line).expect("signs")), Granted::Boundary(off(UniverseAct::Manifest)));
    let out = ConvertOut { tier: vk(&line), holder: [50; 32], identity: [7; 32], g: 10, rate: [(1, 2), (0, 1), (0, 1), (0, 1)], nonce: [3; 32] };
    assert_eq!(k.handoff(&Handoff::ConvertOut(Box::new(out)).sign(&line)), Handed::Boundary(off(UniverseAct::ConversionOut)));
    assert_eq!(k.chain.head_height(), head, "nothing sealed");
}

/// B3 — A LAWFUL CROSSING IS ADMITTED: the reserve's fold stating the turn
/// that carries its bodies is rebuilt under every law of the register and
/// sealed; a transition changed after signing no longer reconstructs.
#[test]
fn lawful_crossing_admitted() {
    let mut k = founded();
    let signed = fold(&reserve_key(), 7, vec![lawful()]).sign(&reserve_key());
    let mut moved = signed.clone();
    moved.fold.transitions = vec![Transition { heights: (7, 9), ..lawful() }];
    assert_eq!(k.hear_uplink(&moved), Uplinked::Unsigned, "the transitions are signed, written out");
    assert!(matches!(k.hear_uplink(&signed), Uplinked::Admitted { .. }), "lawful, admitted");
}
