// AI-GENERATED CONTENT — see DISCLAIMER.md.
//! One signed record at `home/mesh`: bind, peers, bootstrap, genesis, origin.
//! The binary writes it; a home of loose text files is migrated once.

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
    pub origin_unix: u64,
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
        out.extend_from_slice(&self.origin_unix.to_le_bytes());
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
        let origin_unix = u64::from_le_bytes(b.get(p..p + 8)?.try_into().ok()?);
        p += 8;
        Some((
            MeshRecord {
                kind,
                bind,
                peers,
                bootstrap,
                hypha,
                voice,
                origin_unix,
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

fn parse_genesis_text(path: &Path) -> (Vec<([u8; 32], [u64; 4])>, Vec<([u8; 32], [u64; 4])>) {
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

fn origin_of(home: &Path) -> u64 {
    if let Ok(bytes) = std::fs::read(home.join("tuniversal")) {
        if let Ok(raw) = <[u8; 8]>::try_from(bytes.as_slice()) {
            return u64::from_le_bytes(raw);
        }
    }
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
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
        origin_unix: origin_of(home),
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
        origin_unix: origin_of(home),
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
            origin_unix: 1788326481,
        };
        let bytes = rec.signed_bytes(&key);
        let back = MeshRecord::open(&bytes, &key.vk_bytes()).expect("verify");
        assert_eq!(back.bind, rec.bind);
        assert_eq!(back.peers, rec.peers);
        assert_eq!(back.hypha[0].1, [8, 8, 1, 8]);
        assert_eq!(back.origin_unix, rec.origin_unix);
    }
}
