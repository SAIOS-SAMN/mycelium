// 78.152 TU-7 — THIS DOOR'S TUNIVERSAL SHELF: one per process, beside the
// 78.151 published reading and outside the kernel. The keepers' quorum raises
// its final point; every frame read at the door is heard on it; every reply
// the door writes carries its final point.
//
//! THE KEEPERS (TU-1..4) are held here too, under their own lock, never
//! nested with the kernel's: the court's tick is read under the kernel by the
//! court loop (78.152b) and the digest's signer and handed in after; a peer's
//! ring — on its digest or pushed as `ACT_TICK` — is admitted with no kernel
//! at all.

use crate::Mesh;
use mycelium::chronometry::tuniversal::{record, Admitted, Keepers, LastSigned, Origin, Recorded, Refused, Residual, TPoint, TShelf, Tick, TickReading};
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

/// 78.152 (persisted) — THE HOME'S KEEPER RECORD: its path, and the last
/// reading this court signed as read at boot, resumed when the keepers are
/// begun at its origin. Unbound (no home), readings are signed unkept.
static RECORD: Mutex<Option<(std::path::PathBuf, Option<LastSigned>)>> = Mutex::new(None);

/// The keeper record's file in a home, beside the ledger.
pub(crate) fn record_path(home: &std::path::Path) -> std::path::PathBuf {
    home.join("keeper")
}

/// 78.152 (persisted) — THE BOOT READS THE KEEPER RECORD: none is a keeper
/// that never signed here; a record that does not read is refused, for a
/// floor this court cannot read is one it could sign under (an
/// equivocation). Answers the record read.
pub(crate) fn bind_home(home: &std::path::Path) -> Result<Option<LastSigned>, String> {
    let path = record_path(home);
    let last = match record::read(&path) {
        Recorded::None => None,
        Recorded::Last(r) => Some(*r),
        Recorded::Unreadable(why) => return Err(format!("the keeper record at {} is unreadable: {why}", path.display())),
    };
    match RECORD.lock() {
        Ok(mut g) => *g = Some((path, last.clone())),
        Err(p) => *p.into_inner() = Some((path, last.clone())),
    }
    Ok(last)
}

/// The record read at boot, where it stands at `origin`.
fn recorded_at(origin: Origin) -> Option<LastSigned> {
    let g = match RECORD.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    g.as_ref()?.1.clone().filter(|r| r.origin == origin)
}

/// A new reading kept in the home's record before it is held: whether it
/// landed. No home bound keeps nothing and refuses nothing.
fn keep(rec: &LastSigned) -> bool {
    let mut g = match RECORD.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    let Some((path, last)) = g.as_mut() else { return true };
    let landed = record::write(path, rec);
    match landed {
        true => *last = Some(rec.clone()),
        false => eprintln!("  court tuniversal reading at h {} not kept: the keeper record did not land; it is not signed", rec.reading.tick.h),
    }
    landed
}

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
        let mut k = Keepers::new(origin, mesh.vk);
        // 78.152 (persisted): the last reading signed before a restart is
        // this line's newest, so nothing is signed at or below it.
        if let Some(rec) = recorded_at(origin) {
            match k.resume(&rec) {
                true => eprintln!("  court tuniversal keeper resumed above h {}", rec.reading.tick.h),
                false => eprintln!("  court tuniversal keeper record at h {} not resumed: not this court's at this origin", rec.reading.tick.h),
            }
        }
        *g = Some((k, Residual::default()));
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
            if k.read_own_kept(&mesh.sk, t, keep).is_none() {
                eprintln!("  court tuniversal reading refused: its cites pass their count");
            }
        }
        k.own_ring()
    })
    .into_iter()
    .flatten()
    .collect()
}

/// 78.152b — THE COURT LOOP SIGNS ITS OWN READING: the tick read once per
/// stroke under the kernel the loop already holds, handed in here after the
/// kernel is dropped, and a reading signed wherever it is due — whether or
/// not any peer asked for a digest. Answers this court's ring where a
/// reading was newly signed (the ring the nerve pushes as `ACT_TICK`), and
/// `None` where nothing was due, no origin is adopted, or no tick was read.
pub(crate) fn sign_tick(mesh: &Mesh, tick: Option<Tick>) -> Option<Vec<TickReading>> {
    let t = tick?;
    let origin = origin(mesh)?;
    with_keepers(mesh, origin, |k, _| match k.read_own_kept(&mesh.sk, t, keep) {
        Some(true) => Some(k.own_ring()),
        Some(false) => None,
        None => {
            eprintln!("  court tuniversal reading refused: its cites pass their count");
            None
        }
    })
    .flatten()
}

/// 78.152b — WHY A PUSHED TICK WAS NOT HEARD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TickRefused {
    /// The frame does not read whole as `ACT_TICK`.
    Unreadable,
    /// Its keeper is no peer court this mesh knows (one whose signed digest
    /// named its key and door), or is this court itself.
    Stranger,
    /// This court has adopted no origin: no reading verifies here yet.
    NoOrigin,
    /// The ring did not admit (TU-2), by name.
    Ring(Refused),
}

/// 78.152b — A KEEPER'S RING PUSHED AS `ACT_TICK`, HEARD AT THE DOOR WITH NO
/// KERNEL: read whole, its keeper a known peer court, then admitted exactly
/// as a digest's ring is (`hear_ring`). It takes the mesh's peer table, the
/// origin and the keepers' own lock, one at a time, and never the kernel —
/// it is not handed one.
pub(crate) fn hear_tick(mesh: &Mesh, body: &[u8]) -> Result<Admitted, TickRefused> {
    let (vk, ring) = mycelium::anastome::dialogue::decode_tick(body).ok_or(TickRefused::Unreadable)?;
    let known = vk != mesh.vk && mesh.addrs.lock().is_ok_and(|a| a.contains(&vk));
    if !known {
        return Err(TickRefused::Stranger);
    }
    match hear_ring(mesh, vk, ring) {
        None => Err(TickRefused::NoOrigin),
        Some(r) => r.map_err(TickRefused::Ring),
    }
}

/// A PEER'S RING HEARD — on its digest or pushed as `ACT_TICK` (TU-2):
/// admitted, a refusal counted by name, the final point raised (TU-3), and
/// the residual (TU-4) written to the journal where it moved. `None` before
/// an origin is adopted.
pub(crate) fn hear_ring(mesh: &Mesh, vk: [u8; 32], ring: Vec<TickReading>) -> Option<Result<Admitted, Refused>> {
    let origin = origin(mesh)?;
    with_keepers(mesh, origin, |k, last| {
        let out = k.admit(vk, ring);
        match out {
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
        out
    })
}
