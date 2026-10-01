// 78.64 H1 — THE CARRIER CENSUS: what each of a court's carriers holds,
// counted by its own members, at the `K` it was read.
//
//! A carrier that is filled and never emptied is the leak H1 named in the
//! ledger: admission without its inverse. This reading counts every carrier
//! the court holds — each by its entries, a nested carrier by the entries
//! of what it nests — so one that grows per `K` names itself on the fleet.
//! The kernel's census takes itself apart field by field, so a carrier
//! added to the court is counted here or the build refuses it.

/// A COURT'S CARRIERS AT ONE `K`: each by its name and the entries it
/// holds, in the order the census reads them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Carriers {
    pub k: u64,
    pub held: Vec<(String, u64)>,
}

impl Carriers {
    /// One carrier counted.
    pub fn count(&mut self, name: &str, entries: usize) {
        self.held.push((name.to_string(), entries as u64));
    }

    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        out.extend_from_slice(&self.k.to_le_bytes());
        crate::transducer::address::put_u32_count(out, self.held.len())?;
        for (name, n) in &self.held {
            crate::transducer::address::put_bytes(out, name.as_bytes())?;
            out.extend_from_slice(&n.to_le_bytes());
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Carriers> {
        let k = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
        let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
        *p += 4;
        // Each row is at least its name's length and its count.
        if n > b.len().checked_sub(*p)? / 10 {
            return None;
        }
        let mut held = Vec::with_capacity(n);
        for _ in 0..n {
            let name = String::from_utf8(crate::transducer::address::take_bytes(b, p)?).ok()?;
            let count = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            held.push((name, count));
        }
        Some(Carriers { k, held })
    }

    /// The census as the console prints it: the `K`, then each carrier
    /// holding anything, one to a line.
    pub fn lines(&self) -> Vec<String> {
        let mut out = vec![format!("carriers at K {}", self.k)];
        out.extend(self.held.iter().filter(|(_, n)| *n > 0).map(|(name, n)| format!("  {name:<28} {n}")));
        out
    }
}
