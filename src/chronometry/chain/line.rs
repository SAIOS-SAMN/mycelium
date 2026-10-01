// 45.8, 78.24 — THE LINE ON DISK: the hot ledger written in its own order.
// What was filed since the last write is appended at the top, what a fold
// cut is named by its root, and the whole is written again only where the
// file carries as many nodes that left as nodes that stand.
//
//! The file is the line's magic and the key it signs as, then frames. A
//! frame is its byte length and its records, and is the one write that
//! carried it: a frame that runs past the file's end is a write that did
//! not land, and the line stands on the frames before it. A record is a
//! node filed (its receipt), a peer node that left (its stamp), or this
//! chain's own root (its root), which cuts its own line at or below it.

use super::{KStepReceipt, Keyed, Place, ReceiptChain, Root, Stamp};

/// The records a frame carries.
const NODE: u8 = 1;
const GONE: u8 = 2;
const ROOT: u8 = 3;

/// WHAT THE LINE ON DISK DOES NOT YET CARRY.
#[derive(Debug, Clone)]
pub(super) struct Journal {
    /// The stamps filed since the last write.
    filed: Keyed<()>,
    /// The stamps the file carries that left by a shed since the last
    /// write, each named by a `GONE` record.
    left: Keyed<()>,
    /// Whether this chain's own root moved since the last write.
    rooted: bool,
    /// The node records the file carries that no longer stand.
    dead: u64,
    /// Whether the next write is the whole line: none was written, or the
    /// last did not land.
    whole: bool,
}

impl Default for Journal {
    fn default() -> Self {
        Journal { filed: Keyed::new(), left: Keyed::new(), rooted: false, dead: 0, whole: true }
    }
}

impl Journal {
    /// A node filed.
    pub(super) fn filed(&mut self, key: &[u8]) {
        self.filed.insert(key, ());
    }

    /// A node shed: one the file never carried is simply not written.
    pub(super) fn left(&mut self, key: &[u8]) {
        if self.filed.remove(key).is_none() {
            self.left.insert(key, ());
        }
    }

    /// One of this chain's own nodes cut by its root: the root record names
    /// it, so only the file's count of what no longer stands moves.
    pub(super) fn cut(&mut self, key: &[u8]) {
        if self.filed.remove(key).is_none() {
            self.dead += 1;
        }
    }

    /// This chain's own root moved.
    pub(super) fn rooted(&mut self) {
        self.rooted = true;
    }

    fn unchanged(&self) -> bool {
        self.filed.is_empty() && self.left.is_empty() && !self.rooted
    }
}

/// ONE WRITE OF THE LINE: the whole, which replaces the file, or a frame
/// appended to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineWrite {
    pub whole: bool,
    pub bytes: Vec<u8>,
}

impl LineWrite {
    /// THE WRITE LANDED AT `path`: the whole through a sibling and a
    /// rename, so the file is the old line or the new one; a frame by an
    /// append. `false` where it did not land.
    pub fn land(&self, path: &std::path::Path) -> bool {
        use std::io::Write;
        match self.whole {
            true => {
                let tmp = path.with_extension("tmp");
                std::fs::write(&tmp, &self.bytes).is_ok() && std::fs::rename(&tmp, path).is_ok()
            }
            false => std::fs::OpenOptions::new()
                .append(true)
                .open(path)
                .and_then(|mut f| f.write_all(&self.bytes))
                .is_ok(),
        }
    }
}

/// A frame: its records' length, then the records.
fn frame(body: Vec<u8>) -> Vec<u8> {
    let mut out = (body.len() as u64).to_le_bytes().to_vec();
    out.extend_from_slice(&body);
    out
}

impl ReceiptChain {
    /// The line file's magic.
    pub const LINE_MAGIC: &'static [u8; 4] = b"MYL1";

    /// THE WHOLE LINE as the file carries it: the magic, the key, and one
    /// frame of the root this chain stands on and every node it holds, in
    /// stamp order. `None` where a node does not write (LD0).
    #[must_use = "a line that cannot be written is refused by name, never dropped (LD0)"]
    pub fn line_image(&self) -> Option<Vec<u8>> {
        let mut body = Vec::new();
        if let Some(root) = &self.root {
            body.push(ROOT);
            root.write(&mut body);
        }
        for r in self.nodes() {
            body.push(NODE);
            r.write(&mut body)?;
        }
        let mut out = Self::LINE_MAGIC.to_vec();
        out.extend_from_slice(&self.vk);
        out.extend_from_slice(&frame(body));
        Some(out)
    }

    /// WHAT THE LINE ON DISK DOES NOT YET CARRY, as the write that carries
    /// it: a frame of the nodes filed, the root, and the peer nodes shed
    /// since the last write — or the whole, where none was written, the
    /// last did not land, or the file would carry as many nodes that left
    /// as nodes that stand. `Ok(None)` where nothing moved; `Err` where a
    /// node does not write, and nothing is written then (LD0).
    pub fn line_write(&mut self) -> Result<Option<LineWrite>, &'static str> {
        const UNWRITABLE: &str = "a node of the line cannot be written";
        let j = &self.journal;
        if !j.whole && j.unchanged() {
            return Ok(None);
        }
        let dead = j.dead + j.left.len() as u64;
        let whole = j.whole || dead >= self.nodes.len() as u64;
        let bytes = match whole {
            true => self.line_image().ok_or(UNWRITABLE)?,
            false => {
                let mut body = Vec::new();
                for (key, ()) in j.filed.scoped() {
                    let Some(r) = self.nodes.get(&key) else { continue };
                    body.push(NODE);
                    r.write(&mut body).ok_or(UNWRITABLE)?;
                }
                if let (true, Some(root)) = (j.rooted, &self.root) {
                    body.push(ROOT);
                    root.write(&mut body);
                }
                for (key, ()) in j.left.scoped() {
                    body.push(GONE);
                    body.extend_from_slice(&key);
                }
                frame(body)
            }
        };
        self.journal = Journal { dead: if whole { 0 } else { dead }, whole: false, ..Journal::default() };
        Ok(Some(LineWrite { whole, bytes }))
    }

    /// The last write did not land: the next is the whole.
    pub fn line_lost(&mut self) {
        self.journal.whole = true;
    }

    /// THE LINE READ BACK from its file: every frame applied in order, and
    /// the held nodes settled as `from_nodes` settles them. A frame that
    /// runs past the end did not land: the line stands on the frames before
    /// it, the answer says so, and the next write is the whole. `None`
    /// refuses the file: not this layout, a record that does not read
    /// whole, a node twice, or a node named gone that is not held.
    pub fn from_line(b: &[u8]) -> Option<(ReceiptChain, bool)> {
        if b.get(0..4)? != Self::LINE_MAGIC {
            return None;
        }
        let vk: [u8; 32] = b.get(4..36)?.try_into().ok()?;
        let mut c = ReceiptChain::with_vk(vk);
        let (mut pos, mut written, mut torn) = (36usize, 0u64, false);
        while pos < b.len() {
            let body = b
                .get(pos..pos + 8)
                .and_then(|n| usize::try_from(u64::from_le_bytes(n.try_into().ok()?)).ok())
                .and_then(|n| b.get(pos + 8..(pos + 8).checked_add(n)?));
            let Some(body) = body else {
                torn = true;
                break;
            };
            pos += 8 + body.len();
            let mut p = 0usize;
            while p < body.len() {
                let tag = body[p];
                p += 1;
                match tag {
                    NODE => {
                        c.hold(KStepReceipt::read(body, &mut p)?)?;
                        written += 1;
                    }
                    GONE => c.release(&Stamp::take_canonical(body, &mut p)?)?,
                    ROOT => {
                        let root = Root::read(body, &mut p)?;
                        c.remove_own_upto(root.at);
                        c.stand_on(root);
                    }
                    _ => return None,
                }
            }
        }
        let dead = written.checked_sub(c.nodes.len() as u64)?;
        c.journal = Journal { dead, whole: torn, ..Journal::default() };
        c.settle().map(|c| (c, torn))
    }

    /// A NODE NAMED GONE leaves its line; `None` where it is not held.
    fn release(&mut self, at: &Stamp) -> Option<()> {
        let key = at.key();
        self.nodes.remove(&key)?;
        self.lines.remove(&Stamp::line_key(&at.vk, at.place()));
        self.placed.remove(&key);
        Some(())
    }

    /// THIS CHAIN'S OWN NODES AT OR BELOW `upto` leave, returned in line
    /// order with the root the last of them makes.
    pub(super) fn remove_own_upto(&mut self, upto: Place) -> (Vec<KStepReceipt>, Option<Root>) {
        let vk = self.vk;
        let places: Vec<Place> = self.line_between(&vk, None, upto, usize::MAX).iter().map(|r| r.stamp.place()).collect();
        let mut cold = Vec::new();
        let mut top = None;
        for at in places {
            let Some(h) = self.lines.remove(&Stamp::line_key(&vk, at)) else { continue };
            if let Some(r) = self.nodes.remove(&Stamp::new(h, vk, at).key()) {
                top = Some(Root { at: r.stamp.place(), hash: r.chain_hash, height: r.stamp.h });
                cold.push(r);
            }
        }
        (cold, top)
    }
}
