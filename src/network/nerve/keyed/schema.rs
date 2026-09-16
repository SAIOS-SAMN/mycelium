// A13, A21 — THE STRATIFICATION OF A KEY: the coordinates a carrier's keys
// are written from, in the order they are written, so a projection is asked
// for as a DEPTH and never as a byte count.
//
//! Rule 6 requires a key to be the canonical coordinates written out, in a
//! fixed order, in a trie "where a prefix is a stratum". `Keyed` provides
//! the descent (`fiber`) and refuses the empty coordinate in the type
//! (`Prefix`), and stops there: it holds no record of WHICH coordinate
//! system its keys belong to or WHERE one coordinate ends and the next
//! begins. So every coarse reader had to rebuild the prefix by hand, from a
//! second key function written to agree with the first, and nothing checked
//! the agreement. A prefix that cut a coordinate in half was accepted,
//! descended, and returned a subtree that means nothing; a key from another
//! system was accepted and answered "nothing here".
//!
//! This holds that order as data beside the trie. The boundary after `k`
//! coordinates is PARSED through the segments, never computed as an offset,
//! so a prefix that splits a coordinate cannot be expressed and a key that
//! does not spell this system's coordinates refuses by name.

use super::*;

/// WHY A STRATIFIED READ WAS REFUSED.
///
/// An empty answer and a refusal are different facts, and collapsing them
/// is the defect this type exists to end: "the carrier holds nothing under
/// this coordinate" is a reading, and "that was not a coordinate of this
/// carrier" is not (rule 11, exact or refused).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    /// The coordinate system the carrier declares.
    pub system: &'static str,
    /// The coordinate the key failed at, or the empty name where the
    /// refusal is about the depth rather than a coordinate.
    pub segment: &'static str,
    pub why: &'static str,
}

/// ONE COORDINATE OF A KEY.
///
/// `skip` advances past exactly one, reading the framing its writer used.
/// The boundary is therefore a fact about the bytes rather than a length
/// someone remembered, and a key that does not spell this coordinate
/// answers `None` — which the schema turns into a named refusal.
pub trait Segment: Sync {
    fn name(&self) -> &'static str;
    fn skip(&self, key: &[u8], pos: &mut usize) -> Option<()>;
}

/// THE COORDINATE SYSTEM A CARRIER'S KEYS BELONG TO: its segments in the
/// order the writer writes them, so segment `i` is the `i`-th stratum and
/// a prefix through segment `k` is the coarser coordinate at depth `k`.
pub struct Schema {
    pub system: &'static str,
    pub segments: &'static [&'static dyn Segment],
}

impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Schema({}, {} strata)", self.system, self.segments.len())
    }
}

impl Schema {
    /// The number of coordinates a whole key of this system spells.
    pub fn depth(&self) -> usize {
        self.segments.len()
    }

    /// THE BYTE BOUNDARY AFTER `depth` COORDINATES, by parsing each one.
    ///
    /// Nothing here is arithmetic on a remembered width: every segment
    /// reads its own framing, so a key whose coordinate is longer or
    /// shorter than another's is handled by construction, and a key that is
    /// not of this system fails at the coordinate it fails at and says so.
    pub fn boundary(&self, key: &[u8], depth: usize) -> Result<usize, Refusal> {
        if depth > self.segments.len() {
            return Err(Refusal {
                system: self.system,
                segment: "",
                why: "asked past the last coordinate of this system",
            });
        }
        let mut pos = 0usize;
        // index: this schema's own segments, in written order, to the depth
        // asked — the carrier's own declaration and not a search (rule 1).
        for seg in self.segments.iter().take(depth) {
            if seg.skip(key, &mut pos).is_none() {
                return Err(Refusal {
                    system: self.system,
                    segment: seg.name(),
                    why: "the key does not spell this coordinate",
                });
            }
        }
        Ok(pos)
    }

    /// A key of this system spells every coordinate and carries nothing
    /// past the last. Trailing bytes are not a longer key: they are a key
    /// of something else (law 2).
    pub fn whole(&self, key: &[u8]) -> Result<(), Refusal> {
        let end = self.boundary(key, self.depth())?;
        if end != key.len() {
            return Err(Refusal {
                system: self.system,
                segment: "",
                why: "the key carries bytes past its last coordinate",
            });
        }
        Ok(())
    }
}

/// A KEYED RELATION THAT KNOWS ITS OWN COORDINATES.
///
/// The trie underneath is the same `Keyed`; what is added is the schema, so
/// that a read states the DEPTH it holds rather than the bytes it guessed.
/// An exact read needs the whole path; a coarse read names a depth strictly
/// inside it; a key of another system is refused rather than answered
/// empty.
pub struct Stratified<V> {
    schema: &'static Schema,
    trie: Keyed<V>,
}

impl<V: Clone> Clone for Stratified<V> {
    fn clone(&self) -> Self {
        Stratified { schema: self.schema, trie: self.trie.clone() }
    }
}

impl<V: std::fmt::Debug> std::fmt::Debug for Stratified<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stratified({}, {})", self.schema.system, self.trie.len())
    }
}

impl<V> Stratified<V> {
    pub fn new(schema: &'static Schema) -> Self {
        Stratified { schema, trie: Keyed::new() }
    }

    pub fn schema(&self) -> &'static Schema {
        self.schema
    }

    pub fn len(&self) -> usize {
        self.trie.len()
    }

    pub fn is_empty(&self) -> bool {
        self.trie.is_empty()
    }

    /// Bind a WHOLE coordinate path. A key short of the last coordinate is
    /// not a sparser entry; it is a coordinate of a coarser space, and the
    /// carrier does not hold those.
    pub fn insert(&mut self, key: &[u8], value: V) -> Result<Option<V>, Refusal> {
        self.schema.whole(key)?;
        Ok(self.trie.insert(key, value))
    }

    /// The exact read, which needs every coordinate. A caller holding only
    /// a coarse coordinate is asking for a FIBER and is told so here rather
    /// than receiving the empty answer that a missing key and a wrong depth
    /// both used to produce.
    pub fn get(&self, key: &[u8]) -> Result<Option<&V>, Refusal> {
        self.schema.whole(key)?;
        Ok(self.trie.get(key))
    }

    pub fn get_mut(&mut self, key: &[u8]) -> Result<Option<&mut V>, Refusal> {
        self.schema.whole(key)?;
        Ok(self.trie.get_mut(key))
    }

    pub fn remove(&mut self, key: &[u8]) -> Result<Option<V>, Refusal> {
        self.schema.whole(key)?;
        Ok(self.trie.remove(key))
    }

    pub fn contains(&self, key: &[u8]) -> Result<bool, Refusal> {
        Ok(self.get(key)?.is_some())
    }

    /// THE FIBER OVER A COARSER COORDINATE, named by DEPTH.
    ///
    /// `key` may be a whole path or any key of this system that spells at
    /// least `depth` coordinates; only the first `depth` of them are read,
    /// and the boundary between the last of them and whatever follows is
    /// parsed. So the caller never states a byte count, and the two ends of
    /// the carrier cannot drift apart: the coarse coordinate is a
    /// projection of the fine one by construction rather than by a second
    /// function written to agree.
    ///
    /// Depth zero is the whole carrier, which is [`Stratified::scoped`] and
    /// says so there; the full depth is an exact read, which is
    /// [`Stratified::get`]. Both are refused here so that a site cannot ask
    /// for a projection and receive one of those by accident.
    pub fn fiber_at_depth(&self, key: &[u8], depth: usize) -> Result<Fiber<'_, V>, Refusal> {
        if depth == 0 {
            return Err(Refusal {
                system: self.schema.system,
                segment: "",
                why: "depth zero is the whole carrier, which is `scoped`",
            });
        }
        if depth >= self.schema.depth() {
            return Err(Refusal {
                system: self.schema.system,
                segment: "",
                why: "the whole path is an exact read, which is `get`",
            });
        }
        let end = self.schema.boundary(key, depth)?;
        let Some(prefix) = key.get(..end) else {
            return Err(Refusal {
                system: self.schema.system,
                segment: "",
                why: "the coordinate ends past the key",
            });
        };
        match Prefix::new(prefix) {
            Some(p) => Ok(self.trie.fiber(p)),
            None => Err(Refusal {
                system: self.schema.system,
                segment: "",
                why: "the coordinate is empty, which names no stratum",
            }),
        }
    }

    /// THE WHOLE CARRIER, asserted as one cell's reading exactly as
    /// [`Keyed::scoped`] is.
    pub fn scoped(&self) -> Fiber<'_, V> {
        self.trie.scoped()
    }

    /// The trie underneath, for the persist walk and for readers that hold
    /// a whole key already. Every keyed read through here bypasses the
    /// schema, so a site that uses it is saying it has the whole path.
    pub fn keyed(&self) -> &Keyed<V> {
        &self.trie
    }
}

/// A fixed-width coordinate of `n` bytes.
pub fn skip_n(key: &[u8], pos: &mut usize, n: usize) -> Option<()> {
    let end = pos.checked_add(n)?;
    key.get(*pos..end)?;
    *pos = end;
    Some(())
}

/// One byte, and the byte, so a segment whose shape depends on a tag can
/// read it and continue.
pub fn take_u8(key: &[u8], pos: &mut usize) -> Option<u8> {
    let b = *key.get(*pos)?;
    *pos += 1;
    Some(b)
}

/// A `u32` length followed by that many bytes — the framing `put_q` writes
/// for each of a rational's two integers, and `Mv::to_bytes` for each of a
/// coefficient's.
pub fn skip_len_u32(key: &[u8], pos: &mut usize) -> Option<()> {
    let end = pos.checked_add(4)?;
    let n = u32::from_le_bytes(key.get(*pos..end)?.try_into().ok()?) as usize;
    *pos = end;
    skip_n(key, pos, n)
}

/// One exact rational as `put_q` writes it: numerator then denominator,
/// each a `u32` length and its bytes.
pub fn skip_q(key: &[u8], pos: &mut usize) -> Option<()> {
    skip_len_u32(key, pos)?;
    skip_len_u32(key, pos)
}
