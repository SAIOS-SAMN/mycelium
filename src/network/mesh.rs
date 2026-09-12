// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 45.5 — THE MESH RECORD: one signed record at `home/mesh` carrying the
//! node's bind, its peers, its bootstrap, and the genesis and epoch it
//! speaks under.

use crate::court::identity::{self, HyphaKey};
use std::path::Path;

/// MSH2: the bind and the genesis each carry a PRESENCE BYTE. MSH1 wrote
/// both as values — an empty string and 32 zeros — so a node that had
/// adopted no origin and one whose origin hashed to zeros were the same
/// record, and so were a node bound nowhere and one bound at "". A record
/// written under the old magic does not open, and the migration path
/// writes the node's own again.
pub const MAGIC: &[u8; 4] = b"MSH2";
pub const HYPHA: u8 = 0;
pub const VOICE: u8 = 1;

#[derive(Clone, Debug, Default)]
pub struct MeshRecord {
    pub kind: u8,
    /// The address this node serves on. `None` is a node that has not bound
    /// one yet — it resolves an address at boot and writes it here — and is
    /// not the same as an empty address, which is what the field held
    /// before and what every reader had to tell apart by its length.
    pub bind: Option<String>,
    pub peers: Vec<String>,
    pub bootstrap: String,
    pub hypha: Vec<([u8; 32], [u64; 4])>,
    pub voice: Vec<([u8; 32], [u64; 4])>,
    /// H_genesis: the chain hash of the mesh's FIRST receipt (45.5).
    /// `None` is UNMEASURED -- a node that has adopted no origin -- and is
    /// not the same as a genesis of all zeros, which is what the field held
    /// before and what every 47.2 origin check would have compared against.
    /// MSH2 carries a presence byte, so the tape says which it is.
    pub genesis: Option<[u8; 32]>,
    pub epoch: u32,
}

/// A string behind a one-byte length. `None` where it does not fit: the
/// truncating form wrote the first 255 bytes of a longer bind or peer
/// address and signed the record over the short one, so the node
/// advertised a door nobody could dial.
fn put_str(out: &mut Vec<u8>, s: &str) -> Option<()> {
    crate::transducer::address::put_u8_bytes(out, s.as_bytes())
}

/// A string that may be ABSENT: one presence byte, then the string. A bind
/// nothing has written is not a bind of no characters, and the two are told
/// apart here rather than by every reader guessing from an empty one.
fn put_opt_str(out: &mut Vec<u8>, s: Option<&str>) -> Option<()> {
    match s {
        Some(s) => {
            out.push(1);
            put_str(out, s)
        }
        None => {
            out.push(0);
            Some(())
        }
    }
}

fn take_opt_str(b: &[u8], p: &mut usize) -> Option<Option<String>> {
    match *b.get(*p)? {
        0 => {
            *p += 1;
            Some(None)
        }
        1 => {
            *p += 1;
            Some(Some(take_str(b, p)?))
        }
        _ => None,
    }
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
    /// `None` where a field does not fit the length the record writes for
    /// it — a peer list past 255, a bind or bootstrap string past 255
    /// bytes. The truncating form wrote the short one and SIGNED IT, so
    /// the node stood by a record it did not hold (47.1, 7.1).
    pub fn encode(&self) -> Option<Vec<u8>> {
        let mut out = Vec::from(*MAGIC);
        out.push(self.kind);
        put_opt_str(&mut out, self.bind.as_deref())?;
        crate::transducer::address::put_u8_count(&mut out, self.peers.len())?;
        for p in &self.peers {
            put_str(&mut out, p)?;
        }
        put_str(&mut out, &self.bootstrap)?;
        crate::transducer::address::put_u8_count(&mut out, self.hypha.len())?;
        for (who, bond) in &self.hypha {
            put_bond(&mut out, who, bond);
        }
        crate::transducer::address::put_u8_count(&mut out, self.voice.len())?;
        for (who, bond) in &self.voice {
            put_bond(&mut out, who, bond);
        }
        // ABSENCE IS A BYTE, NOT A VALUE. A mesh with no genesis wrote 32
        // zeros, and the reader turned 32 zeros back into "no genesis" —
        // so a genesis of all zeros and a node that has adopted none were
        // one record on the tape, and 5.2f's own round-trip could only pass
        // because no other spelling existed to tell them apart.
        match self.genesis {
            Some(g) => {
                out.push(1);
                out.extend_from_slice(&g);
            }
            None => out.push(0),
        }
        out.extend_from_slice(&self.epoch.to_le_bytes());
        Some(out)
    }

    pub fn decode(b: &[u8]) -> Option<(Self, usize)> {
        if b.get(0..4) != Some(&MAGIC[..]) {
            return None;
        }
        let mut p = 4usize;
        let kind = *b.get(p)?;
        p += 1;
        let bind = take_opt_str(b, &mut p)?;
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
        let genesis = match *b.get(p)? {
            0 => {
                p += 1;
                None
            }
            1 => {
                p += 1;
                let g: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
                p += 32;
                Some(g)
            }
            _ => return None,
        };
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

    /// `None` where the record does not encode: a record that cannot be
    /// written whole is never signed.
    pub fn signed_bytes(&self, key: &HyphaKey) -> Option<Vec<u8>> {
        let mut b = self.encode()?;
        let sig = key.sign(&b);
        b.extend_from_slice(&sig);
        Some(b)
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
        let Some(bytes) = self.signed_bytes(key) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "mesh record does not fit the lengths it writes",
            ));
        };
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, path)
    }

    pub fn load(home: &Path, vk: &[u8; 32]) -> Option<Self> {
        let bytes = std::fs::read(home.join("mesh")).ok()?;
        Self::open(&bytes, vk)
    }

    /// The record at `home/mesh`, told apart from its absence: `Ok(None)`
    /// is a home that has none and migrates, `Err` is a home that HAS one
    /// this key cannot open. Reading the second as the first wrote a fresh
    /// record over a node's peers, genesis and epoch — a configuration
    /// refusal spelled as a first boot.
    pub fn open_at(home: &Path, vk: &[u8; 32]) -> Result<Option<Self>, String> {
        let path = home.join("mesh");
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("{} unreadable: {e}", path.display())),
        };
        match Self::open(&bytes, vk) {
            Some(rec) => Ok(Some(rec)),
            None => Err(format!(
                "{} does not open under this node's key: it is another node's record, \
                 a record of another version, or a damaged one",
                path.display()
            )),
        }
    }
}

fn read_line(path: &Path) -> Option<String> {
    let s = std::fs::read_to_string(path).ok()?;
    s.lines()
        .map(|l| l.trim())
        .find(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
}

/// A file that is not there lists nothing, which is the empty list read
/// out and not a list put in place of one.
fn read_lines(path: &Path) -> Vec<String> {
    let Ok(s) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    s.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
        .collect()
}

/// THE GENESIS ALLOCATIONS of one kind: a key and the μ⃗ seeded to it. §6
/// keeps seeded value separable from earned value, so these never enter as
/// mints.
type Allocations = Vec<([u8; 32], [u64; 4])>;

#[expect(
    clippy::manual_unwrap_or_default,
    reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a lane the genesis line does not name is seeded with nothing"
)]
fn parse_genesis_text(path: &Path) -> (Allocations, Allocations) {
    let mut hypha = Vec::new();
    let mut voice = Vec::new();
    for line in read_lines(path) {
        let mut it = line.split_whitespace();
        let Some(kind) = it.next() else { continue };
        let Some(hex) = it.next() else { continue };
        let Some(vk) = identity::parse_hex32(hex) else { continue };
        // A lane the genesis line does not name is seeded with nothing,
        // which is the absent lane read out.
        let n = |s: Option<&str>| match s.and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => 0u64,
        };
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

/// Load `home/mesh`, or write it once from the old text files and remove
/// them. `Err` is a CONFIGURATION REFUSAL with its reason: a home that
/// holds a record this key cannot open is not a home that holds none.
pub fn load_or_migrate_hypha(home: &Path, key: &HyphaKey) -> Result<MeshRecord, String> {
    let vk = key.vk_bytes();
    if let Some(rec) = MeshRecord::open_at(home, &vk)? {
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
        return Ok(rec);
    }
    // A home with no bind file has not bound an address. That is absence,
    // and it stays absence: the node resolves one at boot and writes it.
    let bind = read_line(&home.join("bind"));
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
    Ok(rec)
}

/// Load `home/mesh` for a voice, or write it once from bootstrap/tuniversal.
/// `Err` is a configuration refusal, as for a hypha.
pub fn load_or_migrate_voice(home: &Path, key: &HyphaKey) -> Result<MeshRecord, String> {
    let vk = key.vk_bytes();
    if let Some(rec) = MeshRecord::open_at(home, &vk)? {
        drop_loose(home, &["bootstrap", "tuniversal", "thoughts", "feed", "conductor"]);
        migrate_genome(home, &vk);
        return Ok(rec);
    }
    // A voice with no bootstrap file dials the loopback door, which is
    // where a voice on one box finds its court.
    let bootstrap = match read_line(&home.join("bootstrap")) {
        Some(b) => b,
        None => "127.0.0.1:7500".into(),
    };
    let rec = MeshRecord {
        kind: VOICE,
        bind: None,
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
    Ok(rec)
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
            bind: Some("127.0.0.1:7500".into()),
            peers: vec!["127.0.0.1:7501".into(), "127.0.0.1:7502".into()],
            bootstrap: String::new(),
            hypha: vec![([1; 32], [8, 8, 1, 8])],
            voice: vec![([2; 32], [8, 8, 1, 8])],
            genesis: Some([7; 32]),
            epoch: 2,
        };
        let bytes = rec.signed_bytes(&key).expect("the record fits its lengths");
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
    /// happened to be zero were the same value. MSH2 carries a PRESENCE
    /// BYTE, so the tape says which it is rather than a reader inferring
    /// it, and 47.2's origin check can never meet the sentinel.
    #[test]
    fn an_unwritten_genesis_reads_back_as_absent_not_as_zero() {
        let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-nogen.key");
        let rec = MeshRecord {
            kind: HYPHA,
            bind: Some("127.0.0.1:7500".into()),
            peers: vec![],
            bootstrap: String::new(),
            hypha: vec![],
            voice: vec![],
            genesis: None,
            epoch: 2,
        };
        let bytes = rec.signed_bytes(&key).expect("the record fits its lengths");
        let back = MeshRecord::open(&bytes, &key.vk_bytes()).expect("verify");
        assert_eq!(back.genesis, None, "unwritten stays unwritten");

        let written = MeshRecord { genesis: Some([9; 32]), ..rec };
        let back = MeshRecord::open(&written.signed_bytes(&key).expect("fits"), &key.vk_bytes()).expect("verify");
        assert_eq!(back.genesis, Some([9; 32]), "a written one survives");

        // The sentinel it replaces: a genesis that IS all zeros is a
        // genesis, and reads back as one. MSH1 could not say this.
        let zeros = MeshRecord { genesis: Some([0; 32]), ..written };
        let back = MeshRecord::open(&zeros.signed_bytes(&key).expect("fits"), &key.vk_bytes()).expect("verify");
        assert_eq!(back.genesis, Some([0; 32]), "a zero origin is an origin");
    }

    /// A bind nothing has written is ABSENT, and an address that is the
    /// empty string is a bind of no characters. Two different records.
    #[test]
    fn an_unbound_node_reads_back_as_unbound_not_as_the_empty_address() {
        let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-nobind.key");
        let rec = MeshRecord {
            kind: HYPHA,
            bind: None,
            peers: vec![],
            bootstrap: String::new(),
            hypha: vec![],
            voice: vec![],
            genesis: None,
            epoch: 0,
        };
        let back = MeshRecord::open(&rec.signed_bytes(&key).expect("fits"), &key.vk_bytes()).expect("verify");
        assert_eq!(back.bind, None, "unbound stays unbound");

        let empty = MeshRecord { bind: Some(String::new()), ..rec };
        let back = MeshRecord::open(&empty.signed_bytes(&key).expect("fits"), &key.vk_bytes()).expect("verify");
        assert_eq!(back.bind, Some(String::new()), "an empty address is an address");
    }
}
