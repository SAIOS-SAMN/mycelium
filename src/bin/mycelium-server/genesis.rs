// 45.5 — THE GENESIS: the mesh's origin, the corpora a court is bonded
// against, and the recut that returns a diverged node to one mesh.
//
//! H_genesis is the chain hash of the mesh's FIRST RECEIPT, written once by
//! the first hypha started on a fresh state and PROVISIONED to every other
//! node the way the bootstrap peer list is. A corpus enters only behind a
//! manifest a genesis key signed, and the cover it measures is the bond the
//! court stakes.

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
        let rel = f.strip_prefix(dir).unwrap_or(f.as_path());
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
        let mut files: Vec<_> = std::fs::read_dir(&corpus_dir)
            .map(|rd| {
                rd.filter_map(|e| e.ok().map(|e| e.path()))
                    .filter(|p| p.extension().is_some_and(|x| x == "json"))
                    .collect()
            })
            .unwrap_or_default();
        files.sort();
        let mut tasked = 0usize;
        for f in &files {
            let Ok(text) = std::fs::read_to_string(f) else { continue };
            let Some(task) = mycelium::arc::parse_task(&text) else { continue };
            if let Some(fam) = mycelium::arc::family_cover(&task.train) {
                cover_measured = true;
                mycelium::arc::cover_join(&mut cover, &fam);
            }
            registry.lock().unwrap().register(task);
            tasked += 1;
        }
        let restored = std::fs::read(home.join("corpora").join("registry"))
            .or_else(|_| std::fs::read(format!("{state_path}.registry")))
            .map(|b| registry.lock().unwrap().overlay(&b))
            .unwrap_or(0);
        eprintln!(
            "[{tag}] corpus registered: {tasked} tasks from {} ({} unsolved, {restored} settled states restored; intake continues under stroke budget)",
            corpus_dir.display(),
            registry.lock().unwrap().unsolved(),
        );
    }
}
{
    let mut k = kernel.lock().unwrap();
    if cover_measured {
        k.set_genesis_cover(cover);
        if k.treasury().bond_of(&vk).is_none() {
            let staked = k.enroll(vk, cover);
            eprintln!("[{tag}] genesis bond {cover:?}; staked {staked:?}");
        }
        for (who, _) in &genesis.voice {
            if k.treasury().bond_of(who).is_none() {
                k.enroll(*who, cover);
            }
        }
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
