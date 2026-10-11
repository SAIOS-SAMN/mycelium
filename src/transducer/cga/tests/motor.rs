// 78.41(3), MT1 — a transition is known by its action on the whole algebra:
// a versor's reading unchanged, a contraction named by rank, a degenerate
// action read, never refused.

use super::common::q;
use crate::transducer::cga::at_three::translator;
use crate::transducer::cga::*;

/// A rotor and a translator are versors by their action: grade-preserving
/// on every blade, the whole matrix of full rank, nothing contracted — and
/// `is_versor` reads the same property.
#[test]
fn a_versor_is_read_off_its_action_unchanged() {
    let rotor = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1)));
    for m in [rotor, translator(&q(1, 1), &q(0, 1), &q(2, 1)), reflector()].map(|m| m.widened(3)) {
        let a = Motor::new(&m).action();
        assert!(a.acts_as_versor() && is_versor(&m), "a versor's action, and a versor as an element");
        assert!(a.grade_preserving, "every blade's image in its own grade");
        assert_eq!(a.dim(), 32, "Cl(4,1) has 2^5 blades");
        assert_eq!(a.algebra_rank, 32, "a versor's action is invertible");
        assert_eq!(a.contracted(), None);
    }
}

/// A null vector sends every generator onto one line: CONTRACTED 1 OF 5,
/// read, and no versor. A null bivector `e₁ e_∞` is even and loses rank on
/// the vectors and on the whole algebra — DEGENERATE, keyed by the rank it
/// kept.
#[test]
fn a_contraction_is_named_by_rank_never_refused() {
    let e_inf = e_inf().widened(3);
    let null = Motor::new(&e_inf).action();
    assert!(null.vectors.vectors_to_vectors);
    assert_eq!(null.contracted(), Some((1, 5)), "every generator onto the null line");
    assert!(!null.acts_as_versor() && !is_versor(&e_inf));
    assert!(null.algebra_rank < null.dim());

    let null_plane = Mv::blade(E1, q(1, 1)).widened(3).geo(&e_inf);
    let a = Motor::new(&null_plane).action();
    assert!(matches!(a.contracted(), Some((r, 5)) if r < 5), "the vectors contracted");
    assert!(a.algebra_rank < a.dim(), "the whole action degenerate");
    assert!(!a.acts_as_versor() && !is_versor(&null_plane));
}

/// `1 + I` is of no one parity, so no versor as an element; but `I` is
/// central in `Cl(4,1)` and its twisted action is `(1 − I) x (1 + I) =
/// (1 − I²) x = 2x` — the identity's action up to scale. It ACTS AS a
/// versor; it IS not one — two facts under two names (MT1, ruled).
#[test]
fn a_mixed_parity_element_acts_as_the_identity() {
    let one_plus_i = Mv::scalar(q(1, 1)).widened(3).add(&Mv::blade(pseudo(3).expect("the pseudoscalar at n = 3"), q(1, 1)).widened(3));
    let a = Motor::new(&one_plus_i).action();
    assert!(a.acts_as_versor() && a.grade_preserving && a.algebra_rank == a.dim());
    assert!(!is_versor(&one_plus_i), "the element has no parity");
}

/// The motor tally files each transition under its kind by what its action
/// is — a degenerate one by the contraction `contracted()` answers, the
/// algebra rank beside it — the lanes it carries as a distribution, a
/// frame-axis step's translation along the frame as its PARABOLIC plane,
/// never as a turn, and a DISAGREEING step's planes off both its motors.
#[test]
fn the_tally_types_each_transition_by_its_action() {
    let _ = take_motor_tally();
    let along = translator(&q(0, 1), &q(0, 1), &q(1, 1));
    let across = translator(&q(1, 1), &q(0, 1), &q(0, 1));
    note_motor(TransitionKind::FrameAxis, &along, Some([1, 0, 2, 0]));
    note_motor(TransitionKind::FrameAxis, &e_inf().widened(3), None);
    note_unread(TransitionKind::Face);
    note_step(StepClass::Agree, &[&along]);
    note_step(StepClass::Disagree, &[&along, &across]);
    note_step(StepClass::Neither, &[]);
    let t = take_motor_tally();
    assert_eq!((t.frame_axis.read, t.frame_axis.versors), (2, 1));
    let degenerate: Vec<Vec<u8>> = t.frame_axis.degenerate.scoped().map(|(k, _)| k).collect();
    assert_eq!(degenerate.len(), 1, "the null vector degenerate under one key");
    assert!(t.line().contains("contracted 1 of 5 algebra "), "keyed by its contraction, the algebra rank beside it");
    assert_eq!((t.face.read, t.face.unread, t.in_out.read), (0, 1, 0));
    assert_eq!((t.steps.agree, t.steps.disagree, t.steps.neither, t.steps.one_sided), (1, 1, 1, 0));
    assert_eq!(t.steps.planes.scoped().count(), 0, "a translation turns no plane");
    assert_eq!(t.steps.parabolic.scoped().count(), 2, "e3∞ from both steps, e1∞ from the disagreement's second motor");
    assert_eq!(t.steps.parabolic.scoped().map(|(_, &c)| c).sum::<u64>(), 3, "along twice, across once");
    assert_eq!(t.lanes[0].get(&1u64.to_be_bytes()), Some(&1));
    assert_eq!(t.lanes[2].get(&2u64.to_be_bytes()), Some(&1));
    assert!(t.line().contains("frame-axis read 2 versors 1"));
    assert_eq!(take_motor_tally().frame_axis.read, 0, "the take clears");
}
