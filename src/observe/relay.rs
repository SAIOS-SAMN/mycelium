// 78.145 — THE SEAT RELAY'S READING: what the relay holds and, per domain
// with a sitting reserve, the posts it filed by kind and refused by reason,
// since boot — so a house whose posts are all refused is visible on the
// hypha's side, never read off a log.
//
//! The counters are the relay's own (`cognition::seat::SeatRelay`), volatile
//! as it is, and bounded as it is: a domain is counted only once it has a
//! reserve here, so the domains counted are the domains founded. A post
//! naming a domain with no reserve, and a seat frame that does not read far
//! enough to name a domain, are each one count apart.

/// THE KINDS A POST IS FILED UNDER, in the order the counts are kept.
pub const POST_KINDS: [&str; 6] = ["seats", "take", "taken", "percept", "call", "outcome"];

/// THE REASONS A POST NAMING A DOMAIN WITH A RESERVE IS REFUSED, in the
/// order the counts are kept (the relay's `Refusal`, less UnknownDomain —
/// which names no such domain, and is counted once, apart).
pub const POST_REFUSALS: [&str; 3] = ["unsigned", "not-the-house", "malformed"];

/// ONE DOMAIN'S POSTS since boot: filed by kind, refused by reason.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RelayPosts {
    pub domain: [u8; 32],
    pub filed: [u64; 6],
    pub refused: [u64; 3],
}

/// THE SEAT RELAY AT THE QUESTION: its epoch and last sequence, the
/// listings, boxes and pieces it holds and the bytes of each, the pieces dropped
/// at the bound; the posts naming a domain with no reserve (`unknown`) and
/// the seat frames that did not read (`unread`); and each counted domain's
/// posts, in domain order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Relay {
    pub epoch: u64,
    pub seq: u64,
    pub listings: u64,
    pub boxes: u64,
    pub pieces: u64,
    pub bytes: u64,
    pub dropped: u64,
    pub unknown: u64,
    pub unread: u64,
    pub domains: Vec<RelayPosts>,
}

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

impl Relay {
    pub(crate) fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        for v in [self.epoch, self.seq, self.listings, self.boxes, self.pieces, self.bytes, self.dropped, self.unknown, self.unread] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        crate::transducer::address::put_u32_count(out, self.domains.len())?;
        for d in &self.domains {
            out.extend_from_slice(&d.domain);
            for v in d.filed.iter().chain(&d.refused) {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        Some(())
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Relay> {
        let mut head = [0u64; 9];
        for v in &mut head {
            *v = take_u64(b, p)?;
        }
        let [epoch, seq, listings, boxes, pieces, bytes, dropped, unknown, unread] = head;
        let n = usize::try_from(u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?)).ok()?;
        *p += 4;
        // Each domain is its key and nine counts.
        if n > b.len().checked_sub(*p)? / (32 + 9 * 8) {
            return None;
        }
        let mut domains = Vec::with_capacity(n);
        for _ in 0..n {
            let domain: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            let mut filed = [0u64; 6];
            for v in &mut filed {
                *v = take_u64(b, p)?;
            }
            let mut refused = [0u64; 3];
            for v in &mut refused {
                *v = take_u64(b, p)?;
            }
            domains.push(RelayPosts { domain, filed, refused });
        }
        Some(Relay { epoch, seq, listings, boxes, pieces, bytes, dropped, unknown, unread, domains })
    }

    /// The reading as the console prints it: the relay's standing on one
    /// line, then each counted domain's filed and refused counts, by name.
    pub fn lines(&self) -> Vec<String> {
        let mut out = vec![format!(
            "relay epoch {:016x} seq {} · listings {} boxes {} pieces {} bytes {} dropped {} · unknown-domain {} unread {}",
            self.epoch, self.seq, self.listings, self.boxes, self.pieces, self.bytes, self.dropped, self.unknown, self.unread
        )];
        for d in &self.domains {
            let filed: Vec<String> = POST_KINDS.iter().zip(&d.filed).map(|(k, n)| format!("{k} {n}")).collect();
            let refused: Vec<String> = POST_REFUSALS.iter().zip(&d.refused).map(|(k, n)| format!("{k} {n}")).collect();
            let short: String = d.domain.iter().take(4).map(|b| format!("{b:02x}")).collect();
            out.push(format!("  domain {short}… filed {} · refused {}", filed.join(" "), refused.join(" ")));
        }
        out
    }
}
