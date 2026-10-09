// 78.152 TU-7 — THIS DOOR'S TUNIVERSAL SHELF: one per process, beside the
// 78.151 published reading and outside the kernel. The keepers' quorum raises
// its final point; every frame read at the door is heard on it; every reply
// the door writes carries its final point.
//
//! THE KEEPERS (TU-1..4) are held here too, under their own lock, never
//! nested with the kernel's: the court's tick is read under the kernel by the
//! digest's signer and handed in after; a peer's ring is admitted with no
//! kernel at all.

use crate::Mesh;
use mycelium::chronometry::tuniversal::{Admitted, Keepers, Origin, Residual, TPoint, TShelf, Tick, TickReading};
use mycelium::observe;
use std::sync::{LazyLock, Mutex};

static SHELF: LazyLock<TShelf> = LazyLock::new(TShelf::new);

/// The keepers as this court holds them, at the origin it adopted; the last
/// residual written to the journal.
static KEEPERS: Mutex<Option<(Keepers, Residual)>> = Mutex::new(None);

/// THE KEEPERS AS THE OBSERVER READS THEM: each keeper and the residual,
/// copied whenever the keepers are touched. A leaf lock, held only for the
/// copy, so the court stroke reads it under the kernel without waiting on a
/// ring's verification under the keepers' own lock.
static SEEN: Mutex<(Vec<observe::KeeperSeen>, Residual)> =
    Mutex::new((Vec::new(), Residual { edges: Vec::new(), holonomy: None }));

/// Refusals of peers' rings, counted, said at powers of two.
static REFUSED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// The door's shelf.
pub(crate) fn shelf() -> &'static TShelf {
    &SHELF
}

/// A frame read at the door, with the point its speaker carried.
pub(crate) fn heard_at_door(point: Option<TPoint>) {
    SHELF.heard_at_door(point);
}

/// The final point every reply of this door carries.
pub(crate) fn final_point() -> Option<TPoint> {
    SHELF.final_point()
}

/// The origin this court adopted, as a point's origin; `None` before it.
fn origin(mesh: &Mesh) -> Option<Origin> {
    let (genesis, epoch) = mesh.origin.lock().ok().and_then(|o| *o)?;
    Some(Origin { genesis, epoch })
}

/// The keepers at `origin`, under their lock: begun where none are held or
/// they stand at another origin (a recut re-originates them).
fn with_keepers<R>(mesh: &Mesh, origin: Origin, f: impl FnOnce(&mut Keepers, &mut Residual) -> R) -> Option<R> {
    let mut g = match KEEPERS.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    if g.as_ref().is_none_or(|(k, _)| k.origin() != origin) {
        *g = Some((Keepers::new(origin, mesh.vk), Residual::default()));
    }
    let (k, r) = g.as_mut()?;
    let out = f(k, r);
    let (k, _) = g.as_ref()?;
    if let Some(p) = k.final_point() {
        SHELF.raise_final(p);
    }
    let seen = (observe::keepers_seen(k), k.residual());
    match SEEN.lock() {
        Ok(mut s) => *s = seen,
        Err(p) => *p.into_inner() = seen,
    }
    Some(out)
}

/// 78.152 — THE COURT'S TUNIVERSAL STANDING for the observer: the shelf's
/// final point and its advances, and the keepers and residual as last
/// copied. Nothing held before an origin is adopted.
pub(crate) fn standing() -> observe::TStanding {
    let (keepers, residual) = match SEEN.lock() {
        Ok(s) => s.clone(),
        Err(p) => p.into_inner().clone(),
    };
    observe::TStanding { final_point: SHELF.final_point(), advances: SHELF.advances(), keepers, residual }
}

/// THIS COURT'S RING FOR ITS DIGEST (TU-1, TU-2): its tick read (where the
/// kernel gave one), a reading signed where the measure moved, and the
/// newest readings, oldest first. Empty before an origin is adopted.
pub(crate) fn own_ring(mesh: &Mesh, tick: Option<Tick>) -> Vec<TickReading> {
    let Some(origin) = origin(mesh) else { return Vec::new() };
    // No keepers held is no ring: nothing is read off it.
    with_keepers(mesh, origin, |k, _| {
        if let Some(t) = tick {
            if k.read_own(&mesh.sk, t).is_none() {
                eprintln!("  court tuniversal reading refused: its cites pass their count");
            }
        }
        k.own_ring()
    })
    .into_iter()
    .flatten()
    .collect()
}

/// A PEER'S RING HEARD ON ITS DIGEST (TU-2): admitted, a refusal counted by
/// name, the final point raised (TU-3), and the residual (TU-4) written to
/// the journal where it moved.
pub(crate) fn hear_ring(mesh: &Mesh, vk: [u8; 32], ring: Vec<TickReading>) {
    let Some(origin) = origin(mesh) else { return };
    let _ = with_keepers(mesh, origin, |k, last| {
        match k.admit(vk, ring) {
            Ok(Admitted::Anchored) => eprintln!("  court tuniversal keeper {:02x}{:02x}… anchored", vk[0], vk[1]),
            Ok(Admitted::Extended(_) | Admitted::Stale) => {}
            Err(why) => {
                let n = REFUSED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                if n.is_power_of_two() {
                    eprintln!("  court tuniversal ring refused ({n}) from {:02x}{:02x}…: {why:?}", vk[0], vk[1]);
                }
            }
        }
        let now = k.residual();
        if now != *last {
            eprintln!(
                "  court tuniversal final {} | residual holonomy {:?} edges {}",
                match k.final_point() {
                    Some(p) => p.local(),
                    None => "none".to_string(),
                },
                now.holonomy,
                now.edges.iter().map(|e| format!("h {} {:?}", e.h, e.delta)).collect::<Vec<_>>().join(" ")
            );
            *last = now;
        }
    });
}
