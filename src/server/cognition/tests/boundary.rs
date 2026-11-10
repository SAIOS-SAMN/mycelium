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
use crate::physics::laws::clauses::{register, Choice, SignedClause, Signing};
use crate::physics::laws::{BodyState, Breach, Law, Transition};
use crate::anastome::dialogue::crossing_record::SignedStill;
use crate::server::cognition::Court;
use crate::transducer::cga::{membrane_at, rotator, Mv};
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

/// Every registered clause, each `vetoed` law vetoed and every other
/// enabled with each fixture reached as the hypha reaches it.
fn clauses(vetoed: &[&str]) -> Vec<SignedClause> {
    register()
        .expect("the register stands")
        .into_iter()
        .map(|r| {
            let choice = match vetoed.contains(&r.clause.id.law.as_str()) {
                true => Choice::Vetoed,
                false => Choice::Enabled { reached: r.reached },
            };
            SignedClause { id: r.clause.id, fixtures: r.clause.fixtures, choice }
        })
        .collect()
}

/// THE CONCORDANCE SIGNED BY `key` (`ACT_LAWS` mode 8), every clause
/// enabled and every fixture reached: the key converses on each clause of
/// the register served. The fixture for every test whose universe crosses
/// its universal core tube.
///
/// The court calibrated too ([`calibrate`]).
pub(super) fn signs(k: &mut Court, key: &SigningKey) {
    calibrate(k);
    assert!(k.hear_signing(&Signing::sign(key, clauses(&[])).to_bytes()).is_ok(), "signed by reconstruction");
}

/// THE COURT CALIBRATED AS A HYPHA SEALS ITS OWN (`Physics::calibrated`):
/// the tube reads the alignment one read per signing event it holds, on
/// one stroke of the court's metabolism, and a harness that never seals a
/// calibration strokes only the minimal stroke. Calibrated, the reading is
/// never cut short by the harness.
pub(super) fn calibrate(k: &mut Court) {
    use crate::physics::{Calibration, Memory, MemorySource, Physics};
    let one = std::num::NonZeroU64::MIN;
    let memory = std::num::NonZeroU64::new(1 << 24).expect("sixteen mebibytes");
    k.physics = Physics::calibrated(&Calibration::of(Memory::of(memory, MemorySource::CgroupMax), one, one).expect("a hypha's calibration"));
}

/// The domain founded naming its reserve and a line, the reserve
/// designated, and the Concordance signed by it.
fn founded() -> Court {
    let mut k = unsigned();
    signs(&mut k, &reserve_key());
    k
}

/// The domain founded and its reserve designated, the Concordance not
/// signed.
fn unsigned() -> Court {
    unsigned_as(&domain_key(), &reserve_key(), &line_key())
}

/// [`unsigned`] for the domain `domain`, its reserve `reserve` and its line
/// `line`.
fn unsigned_as(domain: &SigningKey, reserve: &SigningKey, line: &SigningKey) -> Court {
    let mut k = Court::new();
    let mut tiers = vec![vk(reserve), vk(line)];
    tiers.sort();
    let genesis = DomainGenesis { domain: vk(domain), tiers }.sign(domain);
    assert!(matches!(k.hear_domain(&genesis), Founded::Sealed { .. }), "founded");
    let r = Reservation { domain: vk(domain), tier: vk(reserve), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&r.sign(domain)), Founded::Reserved { now: true, .. }), "reserved");
    calibrate(&mut k);
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
            let nucleus = membrane_at(&body.centre, &q(1)).expect("a positive gauge");
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
    let refused = k.hear_uplink(&fold(&reserve_key(), 7, vec![lawful(), still()]).signed(&reserve_key()));
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
    let refused = k.hear_uplink(&carrier.signed(&reserve_key()));
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
    assert_eq!(k.hear_uplink(&fold(&line, 7, vec![lawful()]).signed(&line)), Uplinked::Boundary(off(UniverseAct::Fold)));
    let entry = LedgerEntry { domain, u: 1, commitment: [1; 32], events: [0; 32], point: fp(10), parent: [0; 32], applied: Vec::new(), nonce: [0; 32] };
    for by in [&line, &domain_key()] {
        assert_eq!(k.ledger(&entry.clone().signed(by), Some(fp(10))), Ledgered::Boundary(off(UniverseAct::Anchor)), "an anchor off the core");
    }
    let map = MapAct { domain, point: fp(10), record: MapRecord::Mouth { place: ([3; 32], 0), freshness: 1 }, nonce: [1; 32] };
    assert_eq!(k.map(&map.signed(&line), Some(fp(10))), Mapped::Boundary(off(UniverseAct::Map)));
    let statement = Statement { domain, stated: Stated::Geometry(Geometry::Curved((1, 1))) };
    assert_eq!(k.state(&statement.signed(&line)), Founded::Boundary(off(UniverseAct::Statement)));
    let manifest = Manifest { domain, nuclei: vec![(1, num_bigint::BigUint::from(1u8))], nonce: [2; 32] };
    assert_eq!(k.manifest(&manifest.signed(&line).expect("signs")), Granted::Boundary(off(UniverseAct::Manifest)));
    let out = ConvertOut { tier: vk(&line), holder: [50; 32], identity: [7; 32], g: 10, rate: [(1, 2), (0, 1), (0, 1), (0, 1)], nonce: [3; 32] };
    assert_eq!(k.handoff(&Handoff::ConvertOut(Box::new(out)).signed(&line)), Handed::Boundary(off(UniverseAct::ConversionOut)));
    assert_eq!(k.chain.head_height(), head, "nothing sealed");
}

/// B3 — A LAWFUL CROSSING IS ADMITTED: the reserve's fold stating the turn
/// that carries its bodies is rebuilt under every law of the register and
/// sealed; a transition changed after signing no longer reconstructs.
#[test]
fn lawful_crossing_admitted() {
    let mut k = founded();
    let signed = fold(&reserve_key(), 7, vec![lawful()]).signed(&reserve_key());
    let mut moved = signed.clone();
    moved.fold.transitions = vec![Transition { heights: (7, 9), ..lawful() }];
    assert_eq!(k.hear_uplink(&moved), Uplinked::Unsigned, "the transitions are signed, written out");
    assert!(matches!(k.hear_uplink(&signed), Uplinked::Admitted { .. }), "lawful, admitted");
}

/// X1 (#160) — A UNIVERSE ACT WHOSE CROSSING DOES NOT VERIFY FROM ITSELF IS
/// REFUSED AT THE BOUNDARY, naming the act, the domain and why: the
/// reserve's fold carrying no crossing (`Absent`), one stating no frame
/// (`NoFrame`), and one whose motion stated as turns does not carry its
/// body (`Turns`, the breach named). A crossing changed after signing no
/// longer verifies (`Unsigned`). Nothing seals; the record that verifies is
/// admitted.
#[test]
fn universe_act_with_unverified_crossing_refused_at_the_boundary() {
    use crate::anastome::dialogue::crossing_record::{Body, Crossing, CrossingRefusal, Motion, Place, TurnBody};
    use crate::court::admission::{Boundary, UniverseAct};
    use crate::physics::laws::TurnBreach;
    use crate::transducer::family::transduce::Turn;
    let mut k = founded();
    let domain = vk(&domain_key());
    let refused = |why| Uplinked::Boundary(Boundary::Crossing { act: UniverseAct::Fold, domain, why });
    let head = k.chain.head_height();
    let f = || fold(&reserve_key(), 7, vec![lawful()]);
    assert_eq!(k.hear_uplink(&f().sign(&reserve_key())), refused(CrossingRefusal::Absent), "no record");
    let base = Crossing::still(Place { at: fp(10), parents: Vec::new() }, Vec::new());
    assert_eq!(k.hear_uplink(&f().sign_crossed(base.clone(), &reserve_key()).expect("signs")), refused(CrossingRefusal::NoFrame), "no frame");
    let body = |origin| Body { origin, boundary: TurnBody { breakpoints: vec![vec![Turn::of(1, 4)]], extent: Q::new(1.into(), 4.into()) } };
    let mut turned = Crossing::still(Place { at: fp(10), parents: Vec::new() }, vec![4]);
    turned.content.motion = Motion::Turns(vec![Turn::of(1, 4)]);
    turned.content.before = vec![body(vec![Turn::of(0, 4)])];
    turned.content.after = vec![body(vec![Turn::of(2, 4)])];
    let uncarried = k.hear_uplink(&f().sign_crossed(turned.clone(), &reserve_key()).expect("signs"));
    assert!(matches!(&uncarried, Uplinked::Boundary(Boundary::Crossing { act: UniverseAct::Fold, why: CrossingRefusal::Turns(b), .. }) if matches!(b[..], [TurnBreach::NotCarried { body: 0, .. }])), "{uncarried:?}");
    let mut moved = f().signed(&reserve_key());
    moved.crossing = Some(turned.clone());
    assert_eq!(k.hear_uplink(&moved), Uplinked::Unsigned, "the crossing is signed, written out");
    assert_eq!(k.chain.head_height(), head, "nothing sealed");
    turned.content.after = vec![body(vec![Turn::of(1, 4)])];
    assert!(matches!(k.hear_uplink(&f().sign_crossed(turned, &reserve_key()).expect("signs")), Uplinked::Admitted { .. }), "carried, admitted");
}

/// The register's network clause, by its content's name.
fn network_clause() -> crate::physics::laws::clauses::ConcordanceClauseId {
    use crate::physics::laws::clauses::Standing;
    let held = register().expect("the register stands");
    held.into_iter().find(|r| r.clause.standing == Standing::Network).expect("a network clause").clause.id
}

/// B4 — AN UNSIGNED RESERVE'S CROSSING IS REFUSED `NotSignatory`, naming
/// the network clause and where the reserve stands on it, read from the
/// ledger: unread before any signing; colliding, with the residue, after a
/// signing that did not reach a fixture; and admitted once a signing reaches
/// every fixture, the latest event read. Nothing seals while it is refused.
#[test]
fn unsigned_reserve_crossing_refused_not_signatory() {
    use crate::court::admission::{Boundary, UniverseAct};
    use crate::physics::laws::clauses::ClauseReading;
    let mut k = unsigned();
    let domain = vk(&domain_key());
    let crossing = fold(&reserve_key(), 7, vec![lawful()]).signed(&reserve_key());
    let head = k.chain.head_height();
    let Uplinked::Boundary(Boundary::NotSignatory { act, domain: named, clause, reading }) = k.hear_uplink(&crossing) else { panic!("not signatory") };
    assert_eq!((act, named, clause, *reading), (UniverseAct::Fold, domain, network_clause(), ClauseReading::Unread), "unread, named");
    assert_eq!(k.chain.head_height(), head, "nothing sealed");
    let mut wrong = clauses(&[]);
    let Some(Choice::Enabled { reached }) = wrong.iter_mut().find(|c| c.id == network_clause()).map(|c| &mut c.choice) else { panic!("enabled") };
    let first = reached[0].clone();
    if let Some(last) = reached.iter_mut().rev().find(|o| **o != first) {
        *last = first;
    }
    assert!(k.hear_signing(&Signing::sign(&reserve_key(), wrong).to_bytes()).is_err(), "a fixture not reached");
    let refused = k.hear_uplink(&crossing);
    assert!(
        matches!(&refused, Uplinked::Boundary(Boundary::NotSignatory { reading, .. }) if matches!(**reading, ClauseReading::Collides { .. })),
        "colliding, with its residue: {refused:?}"
    );
    signs(&mut k, &reserve_key());
    assert!(matches!(k.hear_uplink(&crossing), Uplinked::Admitted { .. }), "the latest signing converses");
}

/// B4 — A KEY'S OWN ACT IS UNBOUND BY THE TUBE (6.2a, keys for a key's own
/// acts): the unsigned domain's reserve key stakes μ of its own and is
/// served, while the same key's fold, the universe's act, is refused
/// `NotSignatory`. The test is whose act it is, never who signs it.
#[test]
fn key_own_act_unbound_by_the_tube() {
    use crate::anastome::dialogue::wallet::StakeAct;
    use crate::court::admission::Boundary;
    let mut k = unsigned();
    let anchor = k.anchor();
    k.treasury.genesis(vk(&reserve_key()), [4, 0, 0, 0], anchor);
    let stake = StakeAct { who: vk(&reserve_key()), unstake: false, mu: [2, 0, 0, 0].into(), nonce: [1; 32] }.sign(&reserve_key());
    assert!(k.stake(&stake).is_ok(), "the key's own stake, served");
    let refused = k.hear_uplink(&fold(&reserve_key(), 7, vec![lawful()]).signed(&reserve_key()));
    assert!(matches!(refused, Uplinked::Boundary(Boundary::NotSignatory { .. })), "the universe's fold, refused: {refused:?}");
}

/// B4 — A VETOED KERNEL CLAUSE BOUNDS ONLY ITS CLAUSE: the reserve enables
/// every network clause and vetoes the kernel-standing coordinate clause.
/// Its fold and its statement cross, governed by no vetoed clause; its
/// anchor and its map, which state TUniversal coordinates, are refused
/// `Bounded`, naming the clause. The tube itself stands.
#[test]
fn kernel_clause_veto_bounds_only_its_clause() {
    use crate::anastome::dialogue::atlas::{LedgerEntry, Ledgered, MapAct, MapRecord, Mapped};
    use crate::anastome::dialogue::universe::{Geometry, Stated, Statement};
    use crate::court::admission::{Boundary, UniverseAct};
    use crate::physics::laws::clauses::ConcordanceClauseId;
    let mut k = unsigned();
    let domain = vk(&domain_key());
    assert!(k.hear_signing(&Signing::sign(&reserve_key(), clauses(&[forms::coordinate::LAW_ID])).to_bytes()).is_ok(), "a kernel clause vetoed");
    assert!(matches!(k.hear_uplink(&fold(&reserve_key(), 7, vec![lawful()]).signed(&reserve_key())), Uplinked::Admitted { .. }), "the fold crosses");
    let statement = Statement { domain, stated: Stated::Geometry(Geometry::Curved((1, 1))) };
    assert!(matches!(k.state(&statement.signed(&reserve_key())), Founded::Stated { .. }), "the statement crosses");
    let coordinate = ConcordanceClauseId::of(forms::coordinate::LAW_ID, forms::coordinate::LAW_VERSION);
    let bound = |act| Boundary::Bounded { act, domain, clause: coordinate.clone() };
    let entry = LedgerEntry { domain, u: 1, commitment: [1; 32], events: [0; 32], point: fp(10), parent: [0; 32], applied: Vec::new(), nonce: [0; 32] };
    assert_eq!(k.ledger(&entry.signed(&reserve_key()), Some(fp(10))), Ledgered::Boundary(bound(UniverseAct::Anchor)));
    let map = MapAct { domain, point: fp(10), record: MapRecord::Mouth { place: ([3; 32], 0), freshness: 1 }, nonce: [1; 32] };
    assert_eq!(k.map(&map.signed(&reserve_key()), Some(fp(10))), Mapped::Boundary(bound(UniverseAct::Map)));
}

/// B4 — A SIGNED RESERVE'S LAWFUL CROSSING IS ADMITTED: the reserve signs
/// the register by reconstruction, then its fold stating a lawful
/// transition, its statement and its anchor each cross the tube.
#[test]
fn signed_reserve_lawful_crossing_admitted() {
    use crate::anastome::dialogue::atlas::{LedgerEntry, Ledgered};
    use crate::anastome::dialogue::universe::{Geometry, Stated, Statement};
    let mut k = founded();
    let domain = vk(&domain_key());
    assert!(matches!(k.hear_uplink(&fold(&reserve_key(), 7, vec![lawful()]).signed(&reserve_key())), Uplinked::Admitted { .. }), "the fold");
    let statement = Statement { domain, stated: Stated::Geometry(Geometry::Curved((1, 1))) };
    assert!(matches!(k.state(&statement.signed(&reserve_key())), Founded::Stated { .. }), "the statement");
    let entry = LedgerEntry { domain, u: 1, commitment: [1; 32], events: [0; 32], point: fp(10), parent: [0; 32], applied: Vec::new(), nonce: [0; 32] };
    let anchored = k.ledger(&entry.signed(&reserve_key()), Some(fp(10)));
    assert!(!matches!(anchored, Ledgered::Boundary(_)), "the anchor reaches the court's own checks: {anchored:?}");
}

/// B4 — THE BOUNDARY READING (MU-HOSTING §5a.7): each refusal the court
/// answers at a tube stands on the reading under its domain, act and name,
/// the last one whole with its height and the count since boot. An unsigned
/// reserve's fold refused twice is one `NotSignatory` row counted twice, its
/// clause and reading kept; a line's fold is a `NotThroughCore` row; once
/// signed, a fold that breaks temporal monotonicity is a `NotLawful` row
/// naming the law; and a crossing admitted adds nothing. A domain of its own
/// keeps the rows apart from every other test's court.
#[test]
fn tube_refusals_read_by_domain_act_and_name() {
    use crate::court::admission::{Boundary, UniverseAct};
    use crate::observe::{boundary_reading, AtTube, TubeRow};
    use crate::physics::laws::clauses::ClauseReading;
    let (domain_k, reserve_k, line_k) = (SigningKey::from_bytes(&[41; 32]), SigningKey::from_bytes(&[42; 32]), SigningKey::from_bytes(&[43; 32]));
    let domain = vk(&domain_k);
    let rows = || -> Vec<TubeRow> { boundary_reading().rows.into_iter().filter(|r| r.last.domain() == domain).collect() };
    let mut k = unsigned_as(&domain_k, &reserve_k, &line_k);
    assert!(rows().is_empty(), "nothing refused yet");
    let crossing = fold(&reserve_k, 7, vec![lawful()]).sign(&reserve_k);
    let first = k.hear_uplink(&crossing);
    assert_eq!(k.hear_uplink(&crossing), first, "refused alike twice");
    let off = k.hear_uplink(&fold(&line_k, 7, vec![lawful()]).sign(&line_k));
    assert_eq!(off, Uplinked::Boundary(Boundary::NotThroughCore { act: UniverseAct::Fold, domain }));
    let held = rows();
    let unsigned = held.iter().find(|r| r.last.name() == "NotSignatory").expect("the unsigned reserve's row");
    let Uplinked::Boundary(refused) = first else { panic!("refused at the tube") };
    assert_eq!((unsigned.times, unsigned.height, &unsigned.last), (2, k.height(), &AtTube::Refused(refused)), "one row, counted twice, the last whole");
    assert!(matches!(&unsigned.last, AtTube::Refused(Boundary::NotSignatory { reading, .. }) if **reading == ClauseReading::Unread), "the reading kept");
    let core = held.iter().find(|r| r.last.name() == "NotThroughCore").expect("the line's row");
    assert_eq!((core.times, core.last.act()), (1, UniverseAct::Fold), "the line's fold, apart");
    assert_eq!(held.len(), 2, "one row per name");
    signs(&mut k, &reserve_k);
    assert!(matches!(k.hear_uplink(&fold(&reserve_k, 7, vec![lawful(), still()]).sign(&reserve_k)), Uplinked::NotLawful { .. }));
    let broke = rows().into_iter().find(|r| r.last.name() == "NotLawful").expect("the unlawful fold's row");
    assert_eq!((broke.times, broke.last), (1, AtTube::NotLawful { domain, laws: vec![Law::Time] }), "the law it broke, named");
    let before = rows();
    assert!(matches!(k.hear_uplink(&crossing), Uplinked::Admitted { .. }), "signed, it crosses");
    assert_eq!(rows(), before, "an admission is no refusal");
}
