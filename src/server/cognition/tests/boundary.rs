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
