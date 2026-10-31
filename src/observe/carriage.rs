// 78.22, LD1, R6 — THE BYTES A CARRIER CARRIES: per award on the ledger,
// per receipt delta on the chain, per frame on the wire.
//
//! A citation that is coordinates is flat in the size of the form it
//! cites; one that copies the form grows with it. This reading is the
//! measurement that tells the two apart: for each carrier, by the kind it
//! is filed under, how many records, the records' bytes, and the largest one.
//! Awards and deltas are measured at the question from what the court or
//! the voice holds now; frames are counted where the wire writes them.

use std::sync::atomic::{AtomicU64, Ordering};

fn take_u8(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

fn take_len(b: &[u8], p: &mut usize) -> Option<usize> {
    let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
    *p += 4;
    usize::try_from(v).ok()
}

/// A count the body cannot hold at `least` bytes an entry is refused before
/// anything is reserved for it.
fn holds(b: &[u8], p: usize, m: usize, least: usize) -> bool {
    match (b.len().checked_sub(p), m.checked_mul(least)) {
        (Some(rest), Some(need)) => rest >= need,
        _ => false,
    }
}

/// THE CARRIERS, in the reading's order, each with what one record of it
/// is. The ONE statement of the order (law 12).
pub const CARRIERS: [(&str, &str); 3] = [
    ("award", "one escrowed award as the treasury persists its row"),
    ("delta", "one receipt delta as the chain seals it, by act kind"),
    ("frame", "one frame as the wire writes it, by act tag"),
];

pub const CARRIER_AWARD: u8 = 0;
pub const CARRIER_DELTA: u8 = 1;
pub const CARRIER_FRAME: u8 = 2;

/// ONE CARRIER UNDER ONE KIND: the act kind for a delta, the act tag for a
/// frame, zero for an award.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CarrierRow {
    pub carrier: u8,
    pub kind: u8,
    pub records: u64,
    pub bytes: u64,
    pub largest: u64,
}

impl CarrierRow {
    /// Fold one record of `len` bytes.
    pub fn add(&mut self, len: u64) {
        self.records += 1;
        self.bytes += len;
        self.largest = self.largest.max(len);
    }
}

/// LD1 — THE CARRIAGE: every (carrier, kind) that carried anything, in
/// (carrier, kind) order. Read whole at the question; the later stands.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Carriage {
    pub rows: Vec<CarrierRow>,
}

impl Carriage {
    /// Fold one record into the row it is filed under, filing a new row in
    /// order.
    pub fn add(&mut self, carrier: u8, kind: u8, len: u64) {
        let at = self.row_at(carrier, kind);
        self.rows[at].add(len);
    }

    /// The row a (carrier, kind) is filed at, filed empty where it was not.
    fn row_at(&mut self, carrier: u8, kind: u8) -> usize {
        match self.rows.binary_search_by_key(&(carrier, kind), |r| (r.carrier, r.kind)) {
            Ok(at) => at,
            Err(at) => {
                self.rows.insert(at, CarrierRow { carrier, kind, ..CarrierRow::default() });
                at
            }
        }
    }

    /// Every node a ledger holds, its own and its peers', by act kind: the
    /// bytes each node carries as it is written.
    pub fn of_chain(&mut self, chain: &crate::chronometry::ReceiptChain) {
        for r in chain.nodes() {
            let mut b = Vec::new();
            let len = match r.write(&mut b) {
                Some(()) => b.len() as u64,
                None => 0,
            };
            self.add(CARRIER_DELTA, r.kind.byte(), len);
        }
    }

    /// The frames this process wrote, by act tag.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "STD1 C1 — a constant table: a position in FRAMES, 256 rows, one per act tag byte"
    )]
    pub fn of_frames(&mut self) {
        for (tag, row) in FRAMES.iter().enumerate() {
            let records = row[0].load(Ordering::Relaxed);
            if records == 0 {
                continue;
            }
            let at = self.row_at(CARRIER_FRAME, tag as u8);
            self.rows[at] = CarrierRow {
                carrier: CARRIER_FRAME,
                kind: tag as u8,
                records,
                bytes: row[1].load(Ordering::Relaxed),
                largest: row[2].load(Ordering::Relaxed),
            };
        }
    }

    #[must_use]
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        crate::transducer::address::put_u32_count(out, self.rows.len())?;
        for r in &self.rows {
            out.push(r.carrier);
            out.push(r.kind);
            for v in [r.records, r.bytes, r.largest] {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Carriage> {
        let m = take_len(b, p)?;
        if !holds(b, *p, m, 2 + 3 * 8) {
            return None;
        }
        let mut rows = Vec::with_capacity(m);
        for _ in 0..m {
            let (carrier, kind) = (take_u8(b, p)?, take_u8(b, p)?);
            let mut v = [0u64; 3];
            for x in v.iter_mut() {
                *x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
                *p += 8;
            }
            let [records, bytes, largest] = v;
            rows.push(CarrierRow { carrier, kind, records, bytes, largest });
        }
        Some(Carriage { rows })
    }
}

/// Per act tag: frames written, the frames' bytes, the largest.
static FRAMES: [[AtomicU64; 3]; 256] = [const { [const { AtomicU64::new(0) }; 3] }; 256];

/// Count one frame the wire wrote, by the act tag its body opens with.
pub fn note_frame(tag: u8, len: u64) {
    let row = &FRAMES[usize::from(tag)];
    row[0].fetch_add(1, Ordering::Relaxed);
    row[1].fetch_add(len, Ordering::Relaxed);
    row[2].fetch_max(len, Ordering::Relaxed);
}
