// 45.5 — THE GENESIS: the mesh's origin, the corpora a court is bonded
// against, and the recut that returns a diverged node to one mesh.
//
//! H_genesis is the chain hash of the mesh's FIRST RECEIPT, written once and
//! provisioned to every other node. A corpus enters only behind a manifest a
//! genesis key signed, and the cover it measures is the bond the court stakes.

use mycelium::court::identity::{parse_hex32, parse_hex64};
use mycelium::cognition::Kernel;
use mycelium::court::identity::HyphaKey;
use mycelium::network::mesh::MeshRecord;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub(crate) struct Genesis {
    pub(crate) hypha: Vec<([u8; 32], [u64; 4])>,
    pub(crate) voice: Vec<([u8; 32], [u64; 4])>,
}

pub(crate) fn corpus_hash(dir: &Path) -> Option<[u8; 32]> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    let mut chunks: Vec<Vec<u8>> = Vec::new();
    for f in &files {
        // A file outside the corpus directory is named by its whole path,
        // which is the name it has.
        let rel = match f.strip_prefix(dir) {
            Ok(r) => r,
            Err(_) => f.as_path(),
        };
        chunks.push(rel.to_string_lossy().as_bytes().to_vec());
        chunks.push(std::fs::read(f).ok()?);
    }
    Some(mycelium::network::wire::integrity_hash(chunks.iter().map(|c| c.as_slice())))
}

pub(crate) fn parse_manifest(path: &Path) -> Option<(PathBuf, [u8; 32], [u8; 64])> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut lines = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'));
    let dir = PathBuf::from(lines.next()?);
    let hash = parse_hex32(lines.next()?)?;
    let sig = parse_hex64(lines.next()?)?;
    Some((dir, hash, sig))
}

/// A RECUT drops this node's origin so it takes the mesh's again (45.5).
/// It is an operation, not a file edit: the mesh record is signed, so a
/// genesis cannot be cleared from outside the process that holds the key,
/// and a fleet that has diverged has no other way back to one mesh.
///
/// It DROPS AND EXITS. As a serving mode it would re-cut on every restart
/// -- which is what a unit drop-in carrying it did to this fleet, walking
/// it to epoch 3 one restart at a time. Exiting makes that impossible: a
/// unit that carries the flag flaps visibly instead of quietly
/// re-originating the mesh under everyone. `scripts/recut` runs it once.
pub(crate) fn recut(mesh_rec: &mut MeshRecord, home: &Path, hypha_key: &HyphaKey, tag: &str) {
    match mesh_rec.genesis {
        Some(_) => {
            mesh_rec.genesis = None;
            mesh_rec.epoch += 1;
            if let Err(e) = mesh_rec.save(home, hypha_key) {
                eprintln!("[{tag}] recut refused: {e}");
                std::process::exit(2);
            }
            eprintln!("[{tag}] recut: origin dropped, epoch {}", mesh_rec.epoch);
        }
        None => eprintln!("[{tag}] recut: no origin held, epoch {}", mesh_rec.epoch),
    }
}

/// What origin this node holds, said once at boot. A node that has adopted
/// none says so rather than printing a placeholder.
pub(crate) fn announce(mesh_rec: &MeshRecord, tag: &str) {
match mesh_rec.genesis {
    None => eprintln!("[{tag}] genesis unwritten epoch {}", mesh_rec.epoch),
    Some(g) => eprintln!("[{tag}] genesis {:02x}{:02x}… epoch {}", g[0], g[1], mesh_rec.epoch),
}
}

/// Every corpus behind a signed manifest enters the registry, and the cover
/// the corpus measures becomes this court's bond (46, E1). A manifest that
/// does not verify, a corpus whose hash does not match, and a corpus that
/// measures no cover are each said and skipped; none of them is guessed at.
pub(crate) fn register_corpora(
    home: &Path,
    state_path: &str,
    kernel: &Arc<Mutex<Kernel>>,
    registry: &Arc<Mutex<mycelium::registry::TaskRegistry>>,
    genesis: &Genesis,
    vk: [u8; 32],
    tag: &str,
) {
let genesis_vks: Vec<[u8; 32]> = genesis.hypha.iter().map(|(v, _)| *v).collect();
let mut cover = [0u64; 4];
let mut cover_measured = false;
if let Ok(rd) = std::fs::read_dir(home.join("corpora")) {
    let mut manifests: Vec<_> = rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| {
        p.extension().is_some_and(|x| x == "manifest")
    }).collect();
    manifests.sort();
    for man in manifests {
        let Some((corpus_dir, want, sig)) = parse_manifest(&man) else {
            eprintln!("[{tag}] corpus manifest {} unreadable", man.display());
            continue;
        };
        let Some(got) = corpus_hash(&corpus_dir) else {
            eprintln!("[{tag}] corpus {} unreadable", corpus_dir.display());
            continue;
        };
        if got != want {
            eprintln!("[{tag}] corpus {} hash mismatch", corpus_dir.display());
            continue;
        }
        let signers: Vec<[u8; 32]> = if genesis_vks.is_empty() { vec![vk] } else { genesis_vks.clone() };
        let signed = signers.iter().any(|s| mycelium::court::identity::verify(s, &want, &sig));
        if !signed {
            eprintln!("[{tag}] corpus {} signature refused", corpus_dir.display());
            continue;
        }
        // A directory that will not open lists no files.
        let mut files: Vec<_> = match std::fs::read_dir(&corpus_dir) {
            Ok(rd) => rd
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|x| x == "json"))
                .collect(),
            Err(_) => Vec::new(),
        };
        files.sort();
        let mut tasked = 0usize;
        for f in &files {
            let Ok(text) = std::fs::read_to_string(f) else { continue };
            let Some(task) = mycelium::arc::parse_task(&text) else { continue };
            if let Some(fam) = mycelium::arc::family_cover(&task.train) {
                cover_measured = true;
                mycelium::arc::cover_join(&mut cover, &fam);
            }
            let Ok(mut reg) = registry.lock() else { continue };
            reg.register(task);
            tasked += 1;
        }
        // No snapshot is no state restored (a count of zero is a reading);
        // a poisoned registry is a corpus not registered, and said (§2.4).
        let restored = std::fs::read(home.join("corpora").join("registry"))
            .or_else(|_| std::fs::read(format!("{state_path}.registry")))
            .ok()
            .and_then(|b| registry.lock().ok().map(|mut r| r.overlay(&b)))
            // No snapshot on disk, or a poisoned registry lock, restores
            // no tasks.
            .map_or(0, |n| n);
        let unsolved = match registry.lock() {
            Ok(reg) => reg.unsolved(),
            Err(_) => {
                eprintln!("[{tag}] corpus {} registry lock poisoned", corpus_dir.display());
                continue;
            }
        };
        eprintln!(
            "[{tag}] corpus registered: {tasked} tasks from {} ({unsolved} unsolved, {restored} settled states restored; intake continues under stroke budget)",
            corpus_dir.display(),
        );
    }
}
{
    let Ok(mut k) = kernel.lock() else {
        eprintln!("[{tag}] kernel lock poisoned; no bond staked");
        return;
    };
    if cover_measured {
        k.set_genesis_cover(cover);
        bond_genesis(&mut k, vk, cover, genesis, tag);
    } else if k.treasury().bond_of(&vk).is_none() {
        eprintln!("[{tag}] unbonded; the corpus did not measure a cover");
    }
    let room = k.lane_room();
    let bond = k.treasury().bond_of(&vk);
    eprintln!(
        "[{tag}] room {room:?} = bond {bond:?} − occupancy {:?}",
        k.occupancy()
    );
}
}

/// MU-PRODUCT §4 — "A GENESIS ALLOCATION FUNDS OUR CUSTOMER VOICE AND BONDS
/// OUR HYPHAE." This court, every voice of the genesis, and EVERY HYPHA of
/// the genesis are bonded here at the cover the corpus measured.
///
/// The hyphae were not. Each court enrolled itself and the voices and no
/// other court, so on every court `bond_of(peer hypha)` was `None`, and
/// `converge_witness`'s `independent` — a second vantage is "another organ,
/// bonded … with room on every lane" (47.3) — refused every hypha that
/// ever answered an offer: on 0fd41ca, 41 offers answered in five minutes,
/// `Inexact` 0, `Witnessed` 0, every one `AwaitingWitness`. The only
/// witnesses the fleet ever had were the voices, which are bonded, which
/// is the self-witnessing R46 removed. A hypha is bonded by the genesis,
/// on every court that holds the genesis, and the same cover is the same
/// bond on each — two courts reading one corpus stake one measurement.
///
/// An identity already bonded is not re-bonded: what is held is never
/// re-allocated (§6).
///
/// E3 — AND EACH IS FUNDED ONE COVER TO SPEND beside the cover it stakes.
/// The whole allocation went into the bond, so every balance stood at zero
/// and nothing could pay: a POWC use pays from the demander's balance
/// (`settle_use`), and on the fleet it paid nothing on every court (epoch
/// 106). The synthetic economy stands in for the contracts until they are
/// deployed; the allocation is the corpus's measured cover, never a number
/// chosen here.
pub(crate) fn bond_genesis(k: &mut Kernel, vk: [u8; 32], cover: [u64; 4], genesis: &Genesis, tag: &str) {
    if k.treasury().bond_of(&vk).is_none() {
        let staked = k.enroll(vk, cover);
        k.fund(vk, cover);
        eprintln!("[{tag}] genesis bond {cover:?}; staked {staked:?}");
    }
    for (who, _) in genesis.hypha.iter().chain(genesis.voice.iter()) {
        if k.treasury().bond_of(who).is_none() {
            k.enroll(*who, cover);
            k.fund(*who, cover);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every hypha of the genesis is bonded on this court at the cover,
    /// beside the voices and the court itself; a bond already held stands.
    #[test]
    fn the_genesis_bonds_every_hypha_and_every_voice_at_the_cover() {
        let mut k = Kernel::new();
        let (me, peer, other, voice) = ([1u8; 32], [2u8; 32], [3u8; 32], [9u8; 32]);
        let cover = [12u64, 3, 0, 20];
        let genesis = Genesis { hypha: vec![(me, cover), (peer, cover), (other, cover)], voice: vec![(voice, [8, 8, 1, 8])] };
        bond_genesis(&mut k, me, cover, &genesis, "t");
        for who in [me, peer, other, voice] {
            assert_eq!(k.treasury().bond_of(&who), Some(cover), "{who:?} is bonded at the cover");
            assert_eq!(k.treasury().balance_mu(&who), cover, "{who:?} is funded the cover to spend");
        }
        assert!(k.treasury().audit().is_ok(), "and the allocation conserves");
        // Bonded again with another cover: what is held stands (§6).
        bond_genesis(&mut k, me, [1, 1, 1, 1], &genesis, "t");
        assert_eq!(k.treasury().bond_of(&peer), Some(cover));
        assert_eq!(k.treasury().balance_mu(&peer), cover, "and funded once, never again");
    }
}
