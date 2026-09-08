// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 45.5 — THE MESH RECORD: one signed record at `home/mesh` carrying the
//! node's bind, its peers, its bootstrap, and the genesis and epoch it
//! speaks under.

use crate::court::identity::{self, HyphaKey};
use std::path::Path;

pub const MAGIC: &[u8; 4] = b"MSH1";
pub const HYPHA: u8 = 0;
pub const VOICE: u8 = 1;

#[derive(Clone, Debug, Default)]
pub struct MeshRecord {
    pub kind: u8,
    pub bind: String,
    pub peers: Vec<String>,
    pub bootstrap: String,
    pub hypha: Vec<([u8; 32], [u64; 4])>,
    pub voice: Vec<([u8; 32], [u64; 4])>,
    /// H_genesis: the chain hash of the mesh's FIRST receipt (45.5).
    /// `None` is UNMEASURED -- a node that has adopted no origin -- and is
    /// not the same as a genesis of all zeros, which is what the field held
    /// before and what every 47.2 origin check would have compared against.
    /// The wire still carries 32 bytes; all-zeros there IS the absence.
    pub genesis: Option<[u8; 32]>,
    pub epoch: u32,
}

fn put_str(out: &mut Vec<u8>, s: &str) {
    let b = s.as_bytes();
    let n = b.len().min(255);
    out.push(n as u8);
    out.extend_from_slice(&b[..n]);
}

fn take_str(b: &[u8], p: &mut usize) -> Option<String> {
    let n = *b.get(*p)? as usize;
    *p += 1;
    let s = b.get(*p..*p + n)?;
    *p += n;
    String::from_utf8(s.to_vec()).ok()
}

fn put_bond(out: &mut Vec<u8>, who: &[u8; 32], bond: &[u64; 4]) {
    out.extend_from_slice(who);
    for x in bond {
        out.extend_from_slice(&x.to_le_bytes());
    }
}

fn take_bond(b: &[u8], p: &mut usize) -> Option<([u8; 32], [u64; 4])> {
    let who: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
    *p += 32;
    let mut bond = [0u64; 4];
    for x in &mut bond {
        *x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
    }
    Some((who, bond))
}

impl MeshRecord {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::from(*MAGIC);
        out.push(self.kind);
        put_str(&mut out, &self.bind);
        out.push(self.peers.len().min(255) as u8);
        for p in self.peers.iter().take(255) {
            put_str(&mut out, p);
        }
        put_str(&mut out, &self.bootstrap);
        out.push(self.hypha.len().min(255) as u8);
        for (who, bond) in self.hypha.iter().take(255) {
            put_bond(&mut out, who, bond);
        }
        out.push(self.voice.len().min(255) as u8);
        for (who, bond) in self.voice.iter().take(255) {
            put_bond(&mut out, who, bond);
        }
        out.extend_from_slice(&self.genesis.unwrap_or([0u8; 32]));
        out.extend_from_slice(&self.epoch.to_le_bytes());
        out
    }

    pub fn decode(b: &[u8]) -> Option<(Self, usize)> {
        if b.get(0..4) != Some(&MAGIC[..]) {
            return None;
        }
        let mut p = 4usize;
        let kind = *b.get(p)?;
        p += 1;
        let bind = take_str(b, &mut p)?;
        let np = *b.get(p)? as usize;
        p += 1;
        let mut peers = Vec::with_capacity(np);
        for _ in 0..np {
            peers.push(take_str(b, &mut p)?);
        }
        let bootstrap = take_str(b, &mut p)?;
        let nh = *b.get(p)? as usize;
        p += 1;
        let mut hypha = Vec::with_capacity(nh);
        for _ in 0..nh {
            hypha.push(take_bond(b, &mut p)?);
        }
        let nv = *b.get(p)? as usize;
        p += 1;
        let mut voice = Vec::with_capacity(nv);
        for _ in 0..nv {
            voice.push(take_bond(b, &mut p)?);
        }
        let genesis: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
        let genesis = (genesis != [0u8; 32]).then_some(genesis);
        p += 32;
        let epoch = u32::from_le_bytes(b.get(p..p + 4)?.try_into().ok()?);
        p += 4;
        Some((
            MeshRecord {
                kind,
                bind,
                peers,
                bootstrap,
                hypha,
                voice,
                genesis,
                epoch,
            },
            p,
        ))
    }

    pub fn signed_bytes(&self, key: &HyphaKey) -> Vec<u8> {
        let mut b = self.encode();
        let sig = key.sign(&b);
        b.extend_from_slice(&sig);
        b
    }

    pub fn open(bytes: &[u8], vk: &[u8; 32]) -> Option<Self> {
        let (rec, p) = Self::decode(bytes)?;
        let sig: [u8; 64] = bytes.get(p..p + 64)?.try_into().ok()?;
        if !identity::verify(vk, &bytes[..p], &sig) {
            return None;
        }
        Some(rec)
    }

    pub fn save(&self, home: &Path, key: &HyphaKey) -> std::io::Result<()> {
        let path = home.join("mesh");
        let tmp = home.join("mesh.tmp");
        std::fs::write(&tmp, self.signed_bytes(key))?;
        std::fs::rename(&tmp, path)
    }

    pub fn load(home: &Path, vk: &[u8; 32]) -> Option<Self> {
        let bytes = std::fs::read(home.join("mesh")).ok()?;
        Self::open(&bytes, vk)
    }
}

fn read_line(path: &Path) -> Option<String> {
    let s = std::fs::read_to_string(path).ok()?;
    s.lines()
        .map(|l| l.trim())
        .find(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
}

fn read_lines(path: &Path) -> Vec<String> {
    std::fs::read_to_string(path)
        .map(|s| {
            s.lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

/// THE GENESIS ALLOCATIONS of one kind: a key and the μ⃗ seeded to it. §6
/// keeps seeded value separable from earned value, so these never enter as
/// mints.
type Allocations = Vec<([u8; 32], [u64; 4])>;

fn parse_genesis_text(path: &Path) -> (Allocations, Allocations) {
    let mut hypha = Vec::new();
    let mut voice = Vec::new();
    for line in read_lines(path) {
        let mut it = line.split_whitespace();
        let Some(kind) = it.next() else { continue };
        let Some(hex) = it.next() else { continue };
        let Some(vk) = identity::parse_hex32(hex) else { continue };
        let n = |s: Option<&str>| s.and_then(|x| x.parse().ok()).unwrap_or(0u64);
        let bond = [n(it.next()), n(it.next()), n(it.next()), n(it.next())];
        match kind {
            "hypha" => hypha.push((vk, bond)),
            "voice" => voice.push((vk, bond)),
            _ => {}
        }
    }
    (hypha, voice)
}

fn drop_loose(home: &Path, names: &[&str]) {
    for n in names {
        let p = home.join(n);
        let _ = std::fs::remove_file(&p);
        let _ = std::fs::remove_dir_all(&p);
    }
}

/// Load `home/mesh`, or write it once from the old text files and remove them.
pub fn load_or_migrate_hypha(home: &Path, key: &HyphaKey) -> MeshRecord {
    let vk = key.vk_bytes();
    if let Some(rec) = MeshRecord::load(home, &vk) {
        drop_loose(
            home,
            &[
                "bind",
                "peers",
                "genesis",
                "tuniversal",
                "court.federation",
                "feed",
            ],
        );
        migrate_registry(home);
        return rec;
    }
    let bind = read_line(&home.join("bind")).unwrap_or_default();
    let peers = read_lines(&home.join("peers"));
    let (hypha, voice) = parse_genesis_text(&home.join("genesis"));
    let rec = MeshRecord {
        kind: HYPHA,
        bind,
        peers,
        bootstrap: String::new(),
        hypha,
        voice,
        genesis: None,
        epoch: 0,
    };
    let _ = rec.save(home, key);
    drop_loose(
        home,
        &[
            "bind",
            "peers",
            "genesis",
            "tuniversal",
            "court.federation",
            "feed",
        ],
    );
    migrate_registry(home);
    rec
}

/// Load `home/mesh` for a voice, or write it once from bootstrap/tuniversal.
pub fn load_or_migrate_voice(home: &Path, key: &HyphaKey) -> MeshRecord {
    let vk = key.vk_bytes();
    if let Some(rec) = MeshRecord::load(home, &vk) {
        drop_loose(home, &["bootstrap", "tuniversal", "thoughts", "feed", "conductor"]);
        migrate_genome(home, &vk);
        return rec;
    }
    let bootstrap = read_line(&home.join("bootstrap")).unwrap_or_else(|| "127.0.0.1:7500".into());
    let rec = MeshRecord {
        kind: VOICE,
        bind: String::new(),
        peers: Vec::new(),
        bootstrap,
        hypha: Vec::new(),
        voice: Vec::new(),
        genesis: None,
        epoch: 0,
    };
    let _ = rec.save(home, key);
    drop_loose(home, &["bootstrap", "tuniversal", "thoughts", "feed", "conductor"]);
    migrate_genome(home, &vk);
    rec
}

fn migrate_registry(home: &Path) {
    let old = home.join("ledger.registry");
    let new = home.join("corpora").join("registry");
    if old.exists() && !new.exists() {
        let _ = std::fs::create_dir_all(home.join("corpora"));
        let _ = std::fs::rename(&old, &new);
    }
    let _ = std::fs::remove_file(&old);
}

fn migrate_genome(home: &Path, vk: &[u8; 32]) {
    let dest = home.join("genome");
    let named = home.join(format!("{}.genome", identity::hex32(vk)));
    if !dest.exists() && named.exists() {
        let _ = std::fs::rename(&named, &dest);
    }
    let _ = std::fs::remove_file(&named);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_round_trips_and_verifies() {
        let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-test.key");
        let rec = MeshRecord {
            kind: HYPHA,
            bind: "127.0.0.1:7500".into(),
            peers: vec!["127.0.0.1:7501".into(), "127.0.0.1:7502".into()],
            bootstrap: String::new(),
            hypha: vec![([1; 32], [8, 8, 1, 8])],
            voice: vec![([2; 32], [8, 8, 1, 8])],
            genesis: Some([7; 32]),
            epoch: 2,
        };
        let bytes = rec.signed_bytes(&key);
        let back = MeshRecord::open(&bytes, &key.vk_bytes()).expect("verify");
        assert_eq!(back.bind, rec.bind);
        assert_eq!(back.peers, rec.peers);
        assert_eq!(back.hypha[0].1, [8, 8, 1, 8]);
        assert_eq!(back.genesis, rec.genesis);
        assert_eq!(back.epoch, rec.epoch);
    }

    /// 45.5: a genesis of all zeros is not a genesis, it is the ABSENCE of
    /// one. The field held `[0u8; 32]` and every reader compared against
    /// that sentinel, so a node with no origin and a node whose origin
    /// happened to be zero were the same value. The wire still carries 32
    /// bytes; the type no longer lets the sentinel reach 47.2's origin
    /// check, which would otherwise have admitted under a genesis nobody
    /// wrote.
    #[test]
    fn an_unwritten_genesis_reads_back_as_absent_not_as_zero() {
        let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-nogen.key");
        let rec = MeshRecord {
            kind: HYPHA,
            bind: "127.0.0.1:7500".into(),
            peers: vec![],
            bootstrap: String::new(),
            hypha: vec![],
            voice: vec![],
            genesis: None,
            epoch: 2,
        };
        let bytes = rec.signed_bytes(&key);
        let back = MeshRecord::open(&bytes, &key.vk_bytes()).expect("verify");
        assert_eq!(back.genesis, None, "unwritten stays unwritten");

        let written = MeshRecord { genesis: Some([9; 32]), ..rec };
        let back = MeshRecord::open(&written.signed_bytes(&key), &key.vk_bytes()).expect("verify");
        assert_eq!(back.genesis, Some([9; 32]), "a written one survives");
    }
}
