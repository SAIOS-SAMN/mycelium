// 45.8, 78.24 — THE LINE ON DISK: the hot ledger written in its own order.
// What was filed since the last write is appended at the top, what a fold
// cut is named by its root, and the whole is written again only where the
// file carries as many nodes that left as nodes that stand.
//
//! The file is the line's magic and the key it signs as, then frames. A
//! frame is its byte length and its records (`pack.rs`), and is the one
//! write that carried it: a frame that runs past the file's end is a write
//! that did not land, and the line stands on the frames before it. Its
//! records are the nodes filed, each packed against what the file already
//! carries; this chain's own root, which cuts its own line at or below it;
//! the peer nodes that left, by stamp; and an anchor on every line's last
//! node, which the reader re-derives.

use super::{Keyed, ReceiptChain};
use crate::chronometry::pack::{Packer, Record, Unpacker};

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
    /// What the file already carries, which the next frame is packed
    /// against.
    packer: Packer,
}

impl Default for Journal {
    fn default() -> Self {
        Journal { filed: Keyed::new(), left: Keyed::new(), rooted: false, dead: 0, whole: true, packer: Packer::new() }
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
    pub const LINE_MAGIC: &'static [u8; 4] = b"MYL2";

    /// THE WHOLE LINE as the file carries it, with the packer that wrote
    /// it: the magic, and one frame of the key this chain signs as — the
    /// stream's first signer — the root it stands on, every node it holds in
    /// stamp order, and the anchors. `None` where a node does not write
    /// (LD0).
    fn image(&self) -> Option<(Vec<u8>, Packer)> {
        let mut packer = Packer::new();
        let mut body = Vec::new();
        packer.signer(&mut body, &self.vk);
        if let Some(root) = &self.root {
            packer.root(&mut body, root);
        }
        for r in self.nodes() {
            packer.node(&mut body, &r)?;
        }
        packer.anchors(&mut body);
        let mut out = Self::LINE_MAGIC.to_vec();
        out.extend_from_slice(&frame(body));
        Some((out, packer))
    }

    /// The whole line as the file would carry it. `None` where a node does
    /// not write (LD0).
    #[must_use = "a line that cannot be written is refused by name, never dropped (LD0)"]
    pub fn line_image(&self) -> Option<Vec<u8>> {
        self.image().map(|(bytes, _)| bytes)
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
        let (bytes, packer) = match whole {
            true => self.image().ok_or(UNWRITABLE)?,
            false => {
                let mut packer = j.packer.clone();
                let mut body = Vec::new();
                for (key, ()) in j.filed.scoped() {
                    let Some(at) = super::Stamp::from_key(&key) else { continue };
                    let Some(r) = self.node(&at) else { continue };
                    packer.node(&mut body, &r).ok_or(UNWRITABLE)?;
                }
                if let (true, Some(root)) = (j.rooted, &self.root) {
                    packer.root(&mut body, root);
                }
                for (key, ()) in j.left.scoped() {
                    let Some(at) = super::Stamp::from_key(&key) else { continue };
                    packer.gone(&mut body, &at);
                }
                packer.anchors(&mut body);
                (frame(body), packer)
            }
        };
        self.journal = Journal { dead: if whole { 0 } else { dead }, whole: false, packer, ..Journal::default() };
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
    /// whole, an anchor its line does not re-derive, a node twice, or a node
    /// named gone that is not held.
    pub fn from_line(b: &[u8]) -> Option<(ReceiptChain, bool)> {
        if b.get(0..4)? != Self::LINE_MAGIC {
            return None;
        }
        let mut chain: Option<ReceiptChain> = None;
        let mut unpacker = Unpacker::new();
        let (mut pos, mut written, mut torn) = (4usize, 0u64, false);
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
                let record = unpacker.take(body, &mut p)?;
                // The key the line is written for is the stream's first
                // signer.
                if chain.is_none() {
                    chain = Some(ReceiptChain::with_vk(unpacker.first_signer()?));
                }
                let c = chain.as_mut()?;
                match record {
                    Record::Node(r) => {
                        c.hold(*r)?;
                        written += 1;
                    }
                    Record::Root(root) => {
                        c.remove_own_upto(root.at);
                        c.stand_on(root);
                    }
                    Record::Gone(at) => {
                        c.take_node(&at)?;
                    }
                    Record::Read => {}
                }
            }
        }
        let mut c = chain?;
        let dead = written.checked_sub(c.nodes.len() as u64)?;
        c.journal = Journal { dead, whole: torn, packer: unpacker.into_packer(), ..Journal::default() };
        c.settle().map(|c| (c, torn))
    }
}
