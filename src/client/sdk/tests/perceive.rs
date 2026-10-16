// MU-CLIENT §3.1, §4.12, §7.1 R1/R3/R4 — A SCENE IS PERCEPTION, AND ITS
// ESCAPING THOUGHT IS BOUND: a scene taken in is perceived off the field, on
// the perception carrier, and lands under its citation; the slide between
// its hours escapes onto the docket, and the one demand taker binds it — a
// close of a thought of that scene, which K moves by.

use super::common::*;
use crate::client::sdk::entity::perceive::{Reading, Scene};
use crate::client::sdk::*;

/// THE SCENE, PERCEIVED OFF THE FIELD, LANDS UNDER ITS CITATION; ITS ESCAPING
/// THOUGHT IS BOUND WHERE IT LANDS. Two hours of a slide are taken in as one
/// scene; the perception carrier reads them (R1) and the winding admits the
/// reading: the hours are held under the scene's citation and meet there, and
/// the field lets them go as a slide's motion is let go (MU-CLIENT §4.13) —
/// while the motion between them, which never returns, stands docketed under
/// that citation, and the demand taker binds it (R3), by construction where
/// the language holds no word for it. The binding is a return of a thought of
/// that scene: K moves by it.
#[test]
fn a_scene_perceived_off_the_field_lands_and_its_escaping_thought_is_bound() {
    let id = [9u8; 32];
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("perceiving"));
    let [first, second] = two_hours_of_a_slide();
    let mut field = std::mem::take(&mut mind.field);
    let (bound, returned, k) = field.think(|| {
        mind.perceive_scene(Scene { id: id.to_vec(), train: vec![first, second], tests: Vec::new(), reading: Reading::Task });
        mind.settle_perception();
        let mut b = budget();
        let bound = mind.take_demands(&mut b);
        let returned = mind.act_on_returns(&mut b, bound.clone(), true);
        (bound, returned, mind.own_k())
    });
    mind.field = field;
    assert!(bound.contains(&id.to_vec()), "the slide between its hours escaped under its citation and was bound: a close of a thought of this scene");
    assert!(returned > 0, "the binding is a return");
    assert_eq!(k, returned, "K moved by exactly the returns");
}
