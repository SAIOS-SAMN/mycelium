// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A21, STD1 3c-2 — A KEY COMPONENT AND ITS LENGTH, each written by its own
//! coordinate: the LEB128 writing a value's coefficients take on the wire.

/// A KEY COMPONENT: a coordinate written into a key, never a record a reader
/// frames at a fixed width (STD1 3c-2, ruled (d)).
///
/// ITS LENGTH IS WRITTEN BY ITS OWN COORDINATE: LEB128, seven bits a byte,
/// the high bit saying more follows. Exact for any length, so the writer is
/// total with no cast and no width wall — an address held once is
/// legitimately megabytes, and the only wall against a blob is the
/// citation's u16 (LD0). Minimal and prefix-free, so the key is injective by
/// construction; and a component under 128 bytes takes one length byte, not
/// four.
pub fn put_key_bytes(out: &mut Vec<u8>, b: &[u8]) {
    put_key_count(out, b.len());
    out.extend_from_slice(b);
}

/// A COUNT IN A KEY, written by its own coordinate (STD1 3c-2, ruled (d)):
/// LEB128, seven bits a byte, minimal. A count in a key is a coordinate of
/// the key exactly as a component's length is, so it has no width wall.
pub fn put_key_count(out: &mut Vec<u8>, mut n: usize) {
    loop {
        let low = n.to_le_bytes()[0] & 0x7f;
        n >>= 7;
        if n == 0 {
            out.push(low);
            break;
        }
        out.push(low | 0x80);
    }
}

/// A key count read back. `None` where the spelling is not minimal (a key
/// has one spelling) or the count does not fit a `usize`.
pub fn take_key_count(b: &[u8], p: &mut usize) -> Option<usize> {
    let mut n = 0usize;
    let mut shift = 0u32;
    loop {
        let byte = *b.get(*p)?;
        *p += 1;
        let v = usize::from(byte & 0x7f);
        let placed = v.checked_shl(shift)?;
        // Bits shifted past the carrier are a count no usize holds.
        if placed >> shift != v {
            return None;
        }
        n |= placed;
        if byte & 0x80 == 0 {
            // A final zero byte after the first is an over-long spelling.
            if byte == 0 && shift > 0 {
                return None;
            }
            return Some(n);
        }
        shift += 7;
    }
}

/// A key component read back: its LEB128 length, then that many bytes.
/// `None` where the length is not minimal, does not fit a `usize`, or runs
/// past the key.
pub fn take_key_bytes(b: &[u8], p: &mut usize) -> Option<Vec<u8>> {
    let n = take_key_count(b, p)?;
    if n > b.len().checked_sub(*p)? {
        return None;
    }
    let out = b.get(*p..*p + n)?.to_vec();
    *p += n;
    Some(out)
}
