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
/// record, and so were a node bound nowhere and one bound at "".
pub const MAGIC: &[u8; 4] = b"MSH2";

/// The magic a node's record was provisioned under before the presence
/// bytes. `decode` reads it under its own rules and `save` writes MSH2, so
/// a deployment keeps the wiring §10 says a recut keeps.
pub const MAGIC_V1: &[u8; 4] = b"MSH1";
pub const HYPHA: u8 = 0;
pub const VOICE: u8 = 1;

/// WHICH LAYOUT A RECORD ON DISK WAS READ UNDER. A bool here would name
/// the question and neither of its answers (F3ah); the migration turns on
/// it, and so does the deletion of the arm that reads the old one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    /// MSH2, with the presence bytes.
    Current,
    /// MSH1, read under its own rules. `load_or_migrate_hypha` and
    /// `load_or_migrate_voice` re-save such a record ONCE, at the boot
    /// that read it, so the arm that reads MSH1 has an end: when every
    /// node has booted once, no MSH1 record is left and the arm goes.
    Legacy,
}

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

fn put_bond(out: &mut Vec<u8>, party: &[u8; 32], bond: &[u64; 4]) {
    out.extend_from_slice(party);
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

    /// MSH2, or the MSH1 record a node provisioned before the presence
    /// bytes existed. THE PROVISIONING IS CONFIGURATION AND A RECUT KEEPS
    /// IT (§10): the bind, the peers and the bootstrap are how a node is
    /// wired into the mesh, and a magic it cannot read would throw them
    /// away. MSH1 is read under its own rules — an empty bind IS "bound
    /// nowhere" there, and 32 zeros ARE "no genesis", which is exactly the
    /// conflation MSH2 exists to end — and the next `save` writes MSH2.
    pub fn decode(b: &[u8]) -> Option<(Self, usize)> {
        Self::decode_layout(b).map(|(rec, p, _)| (rec, p))
    }

    /// The same reading, with the LAYOUT it was read under. One reader for
    /// both (law 12): a second function that looked at the magic on its own
    /// could disagree with the arm that parsed the body.
    pub fn decode_layout(b: &[u8]) -> Option<(Self, usize, Layout)> {
        let layout = match b.get(0..4) {
            Some(m) if m == MAGIC => Layout::Current,
            Some(m) if m == MAGIC_V1 => Layout::Legacy,
            _ => return None,
        };
        let legacy = layout == Layout::Legacy;
        let mut p = 4usize;
        let kind = *b.get(p)?;
        p += 1;
        let bind = if legacy {
            let s = take_str(b, &mut p)?;
            (!s.is_empty()).then_some(s)
        } else {
            take_opt_str(b, &mut p)?
        };
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
        let genesis = if legacy {
            let g: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            (g != [0u8; 32]).then_some(g)
        } else {
            match *b.get(p)? {
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
            }
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
            layout,
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

    pub fn open(bytes: &[u8], key: &[u8; 32]) -> Option<Self> {
        Self::open_layout(bytes, key).map(|(rec, _)| rec)
    }

    /// The record and the layout it was read under, both verified under the
    /// node's own key.
    pub fn open_layout(bytes: &[u8], key: &[u8; 32]) -> Option<(Self, Layout)> {
        let (rec, p, layout) = Self::decode_layout(bytes)?;
        let sig: [u8; 64] = bytes.get(p..p + 64)?.try_into().ok()?;
        if !identity::verify(key, &bytes[..p], &sig) {
            return None;
        }
        Some((rec, layout))
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

    pub fn load(home: &Path, key: &[u8; 32]) -> Option<Self> {
        let bytes = std::fs::read(home.join("mesh")).ok()?;
        Self::open(&bytes, key)
    }

    /// The record at `home/mesh`, told apart from its absence: `Ok(None)`
    /// is a home that has none and migrates, `Err` is a home that HAS one
    /// this key cannot open. Reading the second as the first wrote a fresh
    /// record over a node's peers, genesis and epoch — a configuration
    /// refusal spelled as a first boot.
    pub fn open_at(home: &Path, key: &[u8; 32]) -> Result<Option<(Self, Layout)>, String> {
        let path = home.join("mesh");
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("{} unreadable: {e}", path.display())),
        };
        match Self::open_layout(&bytes, key) {
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

/// X7 — A RECORD READ UNDER THE OLD MAGIC IS WRITTEN BACK ONCE, at the
/// boot that read it, so the read arm for MSH1 has an END: when every node
/// has booted once there is no MSH1 record left in the mesh and the arm
/// is deleted.
///
/// Without this the arm is permanent. fc75a59 added it because the magic
/// bump refused all eight provisioned records at once and the mesh could
/// not boot, but a read arm that nothing retires is two layouts the tree
/// carries forever, and MSH1's conflations (an empty bind for absence, 32
/// zeros for no genesis) are exactly what MSH2 exists to end.
///
/// The re-save is a REFUSAL when it fails, not a warning: a node that
/// cannot write its own record has a configuration fault, and continuing
/// would read MSH1 again at the next boot while reporting nothing.
fn resave_legacy(
    home: &Path,
    key: &HyphaKey,
    rec: &MeshRecord,
    layout: Layout,
) -> Result<(), String> {
    if layout == Layout::Current {
        return Ok(());
    }
    match rec.save(home, key) {
        Ok(()) => {
            eprintln!("  mesh record read as MSH1 and written back as MSH2: {}", home.display());
            Ok(())
        }
        Err(e) => Err(format!("{}/mesh was read as MSH1 and cannot be written back: {e}", home.display())),
    }
}

/// Load `home/mesh`, or write it once from the old text files and remove
/// them. `Err` is a CONFIGURATION REFUSAL with its reason: a home that
/// holds a record this key cannot open is not a home that holds none.
pub fn load_or_migrate_hypha(home: &Path, key: &HyphaKey) -> Result<MeshRecord, String> {
    let vk = key.verifying_bytes();
    if let Some((rec, layout)) = MeshRecord::open_at(home, &vk)? {
        resave_legacy(home, key, &rec, layout)?;
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
    let vk = key.verifying_bytes();
    if let Some((rec, layout)) = MeshRecord::open_at(home, &vk)? {
        resave_legacy(home, key, &rec, layout)?;
        drop_loose_voice(home);
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
    drop_loose_voice(home);
    migrate_genome(home, &vk);
    Ok(rec)
}

/// A VOICE'S LOOSE OLD FILES, dropped: and `feed` only where it is a loose
/// old FILE — the folder `home/feed` is the mind's live feed boundary (78.164,
/// MU-CLIENT §7.1), and dropping it at every boot erased the ARC-AGI-2
/// corpus staged there before a restart (2026-10-18 09:25).
fn drop_loose_voice(home: &Path) {
    drop_loose(home, &["bootstrap", "tuniversal", "thoughts", "conductor"]);
    let _ = std::fs::remove_file(home.join("feed"));
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

fn migrate_genome(home: &Path, key: &[u8; 32]) {
    let dest = home.join("genome");
    let named = home.join(format!("{}.genome", identity::hex32(key)));
    if !dest.exists() && named.exists() {
        let _ = std::fs::rename(&named, &dest);
    }
    let _ = std::fs::remove_file(&named);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A signed mesh record opens under its node's key with its bind, peers,
    /// bonds, genesis and epoch intact.
    #[test]
    fn mesh_record_round_trips_and_verifies() {
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
        let back = MeshRecord::open(&bytes, &key.verifying_bytes()).expect("verify");
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
    fn genesis_none_reads_back_absent_not_zero() {
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
        let back = MeshRecord::open(&bytes, &key.verifying_bytes()).expect("verify");
        assert_eq!(back.genesis, None, "unwritten stays unwritten");

        let written = MeshRecord { genesis: Some([9; 32]), ..rec };
        let back = MeshRecord::open(&written.signed_bytes(&key).expect("fits"), &key.verifying_bytes()).expect("verify");
        assert_eq!(back.genesis, Some([9; 32]), "a written one survives");

        // The sentinel it replaces: a genesis that IS all zeros is a
        // genesis, and reads back as one. MSH1 could not say this.
        let zeros = MeshRecord { genesis: Some([0; 32]), ..written };
        let back = MeshRecord::open(&zeros.signed_bytes(&key).expect("fits"), &key.verifying_bytes()).expect("verify");
        assert_eq!(back.genesis, Some([0; 32]), "a zero origin is an origin");
    }

    /// A bind nothing has written is ABSENT, and an address that is the
    /// empty string is a bind of no characters. Two different records.
    #[test]
    fn bind_none_reads_back_unbound_not_empty() {
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
        let back = MeshRecord::open(&rec.signed_bytes(&key).expect("fits"), &key.verifying_bytes()).expect("verify");
        assert_eq!(back.bind, None, "unbound stays unbound");

        let empty = MeshRecord { bind: Some(String::new()), ..rec };
        let back = MeshRecord::open(&empty.signed_bytes(&key).expect("fits"), &key.verifying_bytes()).expect("verify");
        assert_eq!(back.bind, Some(String::new()), "an empty address is an address");
    }

    /// §10 — A RECUT KEEPS THE PROVISIONING. The bind, the peers and the
    /// bootstrap are how a node is wired into the mesh, and they live in
    /// the record a magic bump would make unreadable. MSH1 opens under its
    /// own rules and the next save writes MSH2.
    ///
    /// Measured before this stood: all eight homes of the mesh held MSH1,
    /// `open_at` refused every one of them, and the nodes — including the
    /// one-shot `--recut` runs — exited 2 at boot.
    #[test]
    fn open_reads_msh1_record_under_its_rules() {
        let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-v1.key");
        // The MSH1 layout, written out: magic, kind, bind as a plain
        // length-prefixed string, peers, bootstrap, bonds, 32 genesis
        // bytes, epoch.
        let mut v1 = Vec::from(*MAGIC_V1);
        v1.push(HYPHA);
        put_str(&mut v1, "127.0.0.1:7500").expect("fits");
        crate::transducer::address::put_u8_count(&mut v1,1).expect("fits");
        put_str(&mut v1, "127.0.0.1:7501").expect("fits");
        put_str(&mut v1, "").expect("fits");
        crate::transducer::address::put_u8_count(&mut v1,1).expect("fits");
        put_bond(&mut v1, &[5; 32], &[1, 2, 3, 4]);
        crate::transducer::address::put_u8_count(&mut v1,0).expect("fits");
        v1.extend_from_slice(&[9u8; 32]);
        v1.extend_from_slice(&7u32.to_le_bytes());
        let sig = key.sign(&v1);
        let mut signed = v1.clone();
        signed.extend_from_slice(&sig);

        let back = MeshRecord::open(&signed, &key.verifying_bytes()).expect("the old record opens");
        assert_eq!(back.bind, Some("127.0.0.1:7500".into()), "the bind survives");
        assert_eq!(back.peers, vec!["127.0.0.1:7501".to_string()], "and the peers");
        assert_eq!(back.hypha, vec![([5; 32], [1, 2, 3, 4])], "and the genesis bonds");
        assert_eq!(back.genesis, Some([9; 32]), "and the origin it stood under");
        assert_eq!(back.epoch, 7);
        // MSH1's own conflations, read as MSH1 wrote them: an empty bind is
        // "bound nowhere" and 32 zeros are "no genesis". That is what MSH2
        // exists to end, and it ends for records written from here on.
        let mut bare = Vec::from(*MAGIC_V1);
        bare.push(VOICE);
        put_str(&mut bare, "").expect("fits");
        crate::transducer::address::put_u8_count(&mut bare,0).expect("fits");
        put_str(&mut bare, "").expect("fits");
        crate::transducer::address::put_u8_count(&mut bare,0).expect("fits");
        crate::transducer::address::put_u8_count(&mut bare,0).expect("fits");
        bare.extend_from_slice(&[0u8; 32]);
        bare.extend_from_slice(&0u32.to_le_bytes());
        let (rec, _, layout) = MeshRecord::decode_layout(&bare).expect("the bare old record decodes");
        assert_eq!(layout, Layout::Legacy, "and the reading names the layout it read");
        assert_eq!(rec.bind, None, "an empty MSH1 bind is absence");
        assert_eq!(rec.genesis, None, "and 32 zeros are no genesis");

        // AND THE NEXT WRITE IS MSH2: the record re-signs under the new
        // magic and reads back with the presence bytes.
        let re = back.signed_bytes(&key).expect("fits");
        assert_eq!(re.get(0..4), Some(&MAGIC[..]), "saved under the current magic");
        let again = MeshRecord::open(&re, &key.verifying_bytes()).expect("and opens");
        assert_eq!(again.bind, back.bind);
        assert_eq!(again.genesis, back.genesis);
        assert_eq!(again.peers, back.peers);
    }

    /// X7 — THE OLD ARM HAS AN END. A boot that reads an MSH1 record writes
    /// it back as MSH2 there and then, so the mesh retires the layout by
    /// booting rather than by a flag, and the arm that reads MSH1 can be
    /// deleted once every node has. Read here on the migration door
    /// itself, since that is where the boot enters: the record on disk is
    /// MSH1 before the call and MSH2 after it, with the wiring §10 says a
    /// recut keeps — bind, peers, bootstrap, bonds, origin, epoch — intact
    /// across the rewrite, and the second call writes nothing new.
    #[test]
    fn load_or_migrate_hypha_resaves_msh1_as_msh2_once() {
        let home = std::path::PathBuf::from("/tmp/mycelium-mesh-x7");
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&home).expect("a home to boot in");
        let key = HyphaKey::load_or_mint("/tmp/mycelium-mesh-x7.key");

        let mut v1 = Vec::from(*MAGIC_V1);
        v1.push(HYPHA);
        put_str(&mut v1, "127.0.0.1:7500").expect("fits");
        crate::transducer::address::put_u8_count(&mut v1, 1).expect("fits");
        put_str(&mut v1, "127.0.0.1:7501").expect("fits");
        put_str(&mut v1, "").expect("fits");
        crate::transducer::address::put_u8_count(&mut v1, 1).expect("fits");
        put_bond(&mut v1, &[5; 32], &[1, 2, 3, 4]);
        crate::transducer::address::put_u8_count(&mut v1, 0).expect("fits");
        v1.extend_from_slice(&[9u8; 32]);
        v1.extend_from_slice(&7u32.to_le_bytes());
        let mut signed = v1.clone();
        signed.extend_from_slice(&key.sign(&v1));
        std::fs::write(home.join("mesh"), &signed).expect("the provisioned record");
        assert_eq!(
            std::fs::read(home.join("mesh")).expect("on disk").get(0..4),
            Some(&MAGIC_V1[..]),
            "provisioned under the old magic"
        );

        let rec = load_or_migrate_hypha(&home, &key).expect("the record opens");
        assert_eq!(rec.bind, Some("127.0.0.1:7500".into()));
        assert_eq!(rec.peers, vec!["127.0.0.1:7501".to_string()]);
        assert_eq!(rec.hypha, vec![([5; 32], [1, 2, 3, 4])]);
        assert_eq!(rec.genesis, Some([9; 32]));
        assert_eq!(rec.epoch, 7);
        let now = std::fs::read(home.join("mesh")).expect("still on disk");
        assert_eq!(now.get(0..4), Some(&MAGIC[..]), "and the boot wrote it back as MSH2");
        assert_eq!(
            MeshRecord::open_layout(&now, &key.verifying_bytes()).map(|(_, l)| l),
            Some(Layout::Current),
            "read as the current layout from here on"
        );

        // ONCE: the second boot reads MSH2 and has nothing to write.
        let again = load_or_migrate_hypha(&home, &key).expect("the record opens");
        assert_eq!(std::fs::read(home.join("mesh")).expect("on disk"), now, "byte for byte");
        assert_eq!(again.bind, rec.bind);
        let _ = std::fs::remove_dir_all(&home);
    }
}
