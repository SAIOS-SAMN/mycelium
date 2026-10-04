// 78.84 — INSTRUMENTATION ONLY (§8.1): the bridge. Every train pair of a
// corpus slice read as the perception carries it (`family::bridge`), each
// transition put through the universal law register (`physics::laws::verify`)
// — what the laws admit, what they refuse by law and by breach, and how much
// of the world's change is no transition the laws can be asked about. Data;
// nothing is decided.

use mycelium::physics::laws::LAWS;
use mycelium::transducer::family::bridge::{bridge_pair, Bridge};

const STROKE: usize = 1 << 20;

const BREACHES: [&str; mycelium::transducer::family::bridge::BREACH_KINDS] = [
    "not a versor",
    "undecomposed",
    "not carried",
    "momentum unread",
    "momentum not conserved",
    "non-rigid",
    "rigidity unmeasured",
    "height not advanced",
    "not unitary",
    "mass not conserved",
    "flux unplaced",
];

pub(crate) fn report(dir: &str, names: &[String]) {
    let mut total = Bridge::default();
    for name in names {
        let Ok(json) = std::fs::read_to_string(format!("{dir}/{name}")) else { continue };
        let Some(task) = mycelium::arc::parse_task(&json) else { continue };
        let mut physics = mycelium::physics::Physics::default();
        let mut task_read = Bridge::default();
        for (x, y) in &task.train {
            let mut budget = physics.stroke_n(STROKE);
            task_read.add(&bridge_pair(x, y, &mut budget));
        }
        println!(
            "  bridge {name}: bodies {} carried {} uncarried {} | transitions {} (several bodies {}) admitted {}",
            task_read.bodies, task_read.carried, task_read.uncarried, task_read.transitions, task_read.several, task_read.admitted
        );
        total.add(&task_read);
    }
    println!(
        "bridge: input bodies {} | carried onto an output body {} | no transition to ask of {} | transitions {} (several bodies {}) admitted {} | isometric {} dilating {} | unbuilt {}",
        total.bodies, total.carried, total.uncarried, total.transitions, total.several, total.admitted, total.isometric, total.dilating, total.unbuilt
    );
    let laws: Vec<String> = LAWS.iter().zip(total.by_law).map(|(l, n)| format!("{} {n}", l.name())).collect();
    println!("bridge transitions breaching each law: {}", laws.join(" | "));
    let kinds: Vec<String> = BREACHES.iter().zip(total.breaches).map(|(k, n)| format!("{k} {n}")).collect();
    println!("bridge breaches by kind: {}", kinds.join(" | "));
    println!(
        "bridge momentum refusals: on one body {} | on an odd motion (a reflection) {} | the motion dilates {} | a body's mass (sites) changed {} | conserved as a ray (LV2) {} with one ratio {}",
        total.momentum_single,
        total.momentum_odd,
        total.momentum_dilating,
        total.momentum_mass_changed,
        total.momentum_projective,
        total.momentum_one_ratio
    );
}
