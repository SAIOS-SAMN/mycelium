// 78.152 TU-7 — THIS DOOR'S TUNIVERSAL SHELF: one per process, beside the
// 78.151 published reading and outside the court. The keepers' quorum raises
// its final point; every expression read at the door is heard on it; every
// reply the door writes carries its final point inside the expression
// (`express`, `hear`), never on the envelope.
//
//! THE KEEPERS (TU-1..4) are held here too, under the keepers' own lock, never
//! nested with the court's: the court's tick is read under the court by the
//! court loop (78.152b) and the digest's signer and handed in after; a peer's
//! ring — on its digest or pushed as `ACT_TICK` — is admitted with no court
//! at all.

use crate::Mesh;
use mycelium::chronometry::tuniversal::{
    record, Admitted, KeeperRefusal, Keepers, LastSigned, Origin, Reach, Recorded, Residue, TPoint, TShelf, Tick, TickReading, WindowRefusal,
};
use mycelium::observe;
use std::sync::{LazyLock, Mutex};

static SHELF: LazyLock<TShelf> = LazyLock::new(TShelf::new);

/// The keepers as this court holds them, at the origin it adopted; the last
/// residue written to the journal.
static KEEPERS: Mutex<Option<(Keepers, Residue)>> = Mutex::new(None);

/// THE KEEPERS AS THE OBSERVER READS THEM: each keeper and the residue,
/// copied whenever the keepers are touched. A leaf lock, held only for the
/// copy, so the court stroke reads it under the court without waiting on a
/// ring's verification under the keepers' own lock.
static SEEN: Mutex<(Vec<observe::KeeperSeen>, Residue)> =
    Mutex::new((Vec::new(), Residue { edges: Vec::new(), holonomy: None }));

/// 78.125 (law version 5) — THIS COURT'S REACH AS LAST READ BY THE KEEPERS,
/// with the count of its changes: copied where the keepers' reading moved,
/// so the court loop hands the court a reach only when it moved (a leaf
/// lock, never nested with the court's).
static REACH: Mutex<(u64, Option<Result<Reach, WindowRefusal>>)> = Mutex::new((0, None));

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

/// The final point every reply of this door carries.
pub(crate) fn final_point() -> Option<TPoint> {
    SHELF.final_point()
}

/// THIS DOOR'S EXPRESSION OF `act` (TU-7; MU-ENGINEERING §9.1): the act and
/// the door's final point, `None` before it holds one. Every reply the door
/// writes and every act a court asks a peer is written so. `None` where the
/// act passes the wire's budget.
pub(crate) fn express(act: &[u8]) -> Option<Vec<u8>> {
    mycelium::anastome::dialogue::write_act(act, final_point().as_ref())
}

/// THE ACT AN EXPRESSION READ AT THIS DOOR CARRIES, its speaker's point
/// heard on the shelf (held apart as heard, never as final); or why it is
/// not heard: no expression, or one at a wire version this door does not
/// speak (#235), refused by name before its act is read.
pub(crate) fn hear(expression: Vec<u8>) -> Result<Vec<u8>, mycelium::anastome::dialogue::Unheard> {
    let e = mycelium::anastome::dialogue::Expression::heard(expression)?;
    SHELF.heard_at_door(e.point);
    Ok(e.act)
}

/// The origin this court adopted, as a point's origin; `None` before it.
fn origin(mesh: &Mesh) -> Option<Origin> {
    let (genesis, epoch) = mesh.origin.lock().ok().and_then(|o| *o)?;
    Some(Origin { genesis, epoch })
}

/// The keepers at `origin`, under the keepers' lock: begun where none are held or
/// they stand at another origin (a recut re-originates them).
fn with_keepers<R>(mesh: &Mesh, origin: Origin, f: impl FnOnce(&mut Keepers, &mut Residue) -> R) -> Option<R> {
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
        *g = Some((k, Residue::default()));
    }
    let (k, r) = g.as_mut()?;
    let out = f(k, r);
    let (k, _) = g.as_ref()?;
    if let Some(p) = k.final_point() {
        SHELF.raise_final(p);
    }
    let seen = (observe::keepers_seen(k), k.residue());
    match SEEN.lock() {
        Ok(mut s) => *s = seen,
        Err(p) => *p.into_inner() = seen,
    }
    let mut r = match REACH.lock() {
        Ok(r) => r,
        Err(p) => p.into_inner(),
    };
    if r.1.as_ref() != Some(k.reach()) {
        r.0 = r.0.wrapping_add(1);
        r.1 = Some(k.reach().clone());
    }
    Some(out)
}

/// 78.125 (law version 5) — THIS COURT'S REACH, where it moved since the
/// change `seen` names (`seen` then names this one). `None` where it has not
/// moved or no keepers are held yet.
pub(crate) fn reach_moved(seen: &mut u64) -> Option<Result<Reach, WindowRefusal>> {
    let r = match REACH.lock() {
        Ok(r) => r,
        Err(p) => p.into_inner(),
    };
    if r.0 == *seen {
        return None;
    }
    *seen = r.0;
    r.1.clone()
}

/// 78.152 — THE COURT'S TUNIVERSAL STANDING for the observer: the shelf's
/// final point and its advances, and the keepers and residue as last
/// copied. Nothing held before an origin is adopted.
pub(crate) fn standing() -> observe::TStanding {
    let (keepers, residue) = match SEEN.lock() {
        Ok(s) => s.clone(),
        Err(p) => p.into_inner().clone(),
    };
    observe::TStanding { final_point: SHELF.final_point(), advances: SHELF.advances(), keepers, residue }
}

/// THIS COURT'S RING FOR ITS DIGEST (TU-1, TU-2): its tick read (where the
/// court gave one), a reading signed where the measure moved, and the
/// newest readings, oldest first. Empty before an origin is adopted.
pub(crate) fn own_ring(mesh: &Mesh, tick: Option<Tick>) -> Vec<TickReading> {
    let Some(origin) = origin(mesh) else { return Vec::new() };
    // No keepers held is no ring: nothing is read off it.
    with_keepers(mesh, origin, |k, _| {
        if let Some(t) = tick {
            if k.read_own_kept(&mesh.sk, t, keep).is_none() {
                eprintln!("  court tuniversal reading refused: its cites pass the cites' count");
            }
        }
        k.own_ring()
    })
    .into_iter()
    .flatten()
    .collect()
}

/// 78.152b — THE COURT LOOP SIGNS ITS OWN READING: the tick read once per
/// stroke under the court the loop already holds, handed in here after the
/// court is dropped, and a reading signed wherever it is due — whether or
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
            eprintln!("  court tuniversal reading refused: its cites pass the cites' count");
            None
        }
    })
    .flatten()
}

/// 78.152b — WHY A PUSHED TICK WAS NOT HEARD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TickRefusal {
    /// The envelope does not read whole as `ACT_TICK`.
    Unreadable,
    /// Its keeper is no peer court this mesh knows (one whose signed digest
    /// named its key and door), or is this court itself.
    Stranger,
    /// This court has adopted no origin: no reading verifies here yet.
    NoOrigin,
    /// The ring did not admit (TU-2), by name.
    Ring(KeeperRefusal),
}

/// 78.152b — A KEEPER'S RING PUSHED AS `ACT_TICK`, HEARD AT THE DOOR WITH NO
/// COURT: read whole, its keeper a known peer court, then admitted exactly
/// as a digest's ring is (`hear_ring`). It takes the mesh's peer table, the
/// origin and the keepers' own lock, one at a time, and never the court —
/// it is not handed one.
pub(crate) fn hear_tick(mesh: &Mesh, body: &[u8]) -> Result<Admitted, TickRefusal> {
    let (vk, ring) = mycelium::anastome::dialogue::decode_tick(body).ok_or(TickRefusal::Unreadable)?;
    let known = vk != mesh.vk && mesh.addrs.lock().is_ok_and(|a| a.contains(&vk));
    if !known {
        return Err(TickRefusal::Stranger);
    }
    match hear_ring(mesh, vk, ring) {
        None => Err(TickRefusal::NoOrigin),
        Some(r) => r.map_err(TickRefusal::Ring),
    }
}

/// A PEER'S RING HEARD — on its digest or pushed as `ACT_TICK` (TU-2):
/// admitted, a refusal counted by name, the final point raised (TU-3), and
/// the residue (TU-4) written to the journal where it moved. `None` before
/// an origin is adopted.
pub(crate) fn hear_ring(mesh: &Mesh, keeper: [u8; 32], ring: Vec<TickReading>) -> Option<Result<Admitted, KeeperRefusal>> {
    let origin = origin(mesh)?;
    with_keepers(mesh, origin, |k, last| {
        let out = k.admit(keeper, ring);
        match out {
            Ok(Admitted::Anchored) => eprintln!("  court tuniversal keeper {:02x}{:02x}… anchored", keeper[0], keeper[1]),
            Ok(Admitted::Extended(_) | Admitted::Stale) => {}
            Err(why) => {
                let n = REFUSED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                if n.is_power_of_two() {
                    eprintln!("  court tuniversal ring refused ({n}) from {:02x}{:02x}…: {why:?}", keeper[0], keeper[1]);
                }
            }
        }
        let now = k.residue();
        if now != *last {
            eprintln!(
                "  court tuniversal final {} | residue holonomy {:?} edges {}",
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
