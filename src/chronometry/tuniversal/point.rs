// 78.152 TU-5 — THE POINT: a final TUniversal coordinate, `h:S:G:T:H:K`
// exact, distinct across universes by its ORIGIN (the genesis hash and the
// epoch), written whole as `origin/h:S:G:T:H:K`.

/// A UNIVERSE'S ORIGIN (45.5): the chain hash of its mesh's first receipt and
/// the epoch it was cut at. Two points of two origins are never compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Origin {
    pub genesis: [u8; 32],
    pub epoch: u32,
}

/// THE POINT (TU-5): a keeper's reading as it stands final — the tick `h`
/// (the keeper court's head height), the court's cumulative measured change
/// per lane `S:G:T:H`, and `K`, the orbits it closed — at its origin. Exact
/// integers, no rounding, no average (TU-3). Ordered as its coordinates are
/// written — origin, tick, lanes, `K` — a canonical key, never a measure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TPoint {
    pub origin: Origin,
    pub h: u64,
    pub lanes: [u64; 4],
    pub k: u64,
}

impl TPoint {
    /// The point's coordinates on the wire: the genesis, the epoch, `h`, the
    /// four lanes and `K`, integers little-endian.
    pub const LEN: usize = 32 + 4 + 8 + 4 * 8 + 8;

    /// The six fields `h, S, G, T, H, K`, in the order they print.
    pub fn fields(&self) -> [u64; 6] {
        let [s, g, t, hh] = self.lanes;
        [self.h, s, g, t, hh, self.k]
    }

    /// THE FIXED WRITING (TU-5), in hexadecimal, one way only: the origin —
    /// the genesis in 64 lowercase hex digits, `.`, the epoch in 8 — then
    /// `/`, then the six fields `h:S:G:T:H:K` in 16 lowercase hex digits
    /// each. Never wrapping at its carrier, so within one origin text order
    /// is numeric order (as `CoreStamp::wide`).
    pub fn print(&self) -> String {
        let mut out = String::with_capacity(64 + 1 + 8 + 1 + 6 * 17);
        for b in self.origin.genesis {
            out.push_str(&format!("{b:02x}"));
        }
        out.push_str(&format!(".{:08x}/", self.origin.epoch));
        out.push_str(&self.local());
        out
    }

    /// The local part alone, `h:S:G:T:H:K`, as `print` writes it.
    pub fn local(&self) -> String {
        self.fields().iter().map(|v| format!("{v:016x}")).collect::<Vec<_>>().join(":")
    }

    /// THE WRITING READ BACK, exactly: every field at its fixed width in
    /// lowercase hex, nothing before, between or after. `None` for any other
    /// text — a short field, an uppercase digit, a sign, a seventh field.
    pub fn parse(text: &str) -> Option<TPoint> {
        let (origin, local) = text.split_once('/')?;
        let (genesis_hex, epoch_hex) = origin.split_once('.')?;
        if genesis_hex.len() != 64 {
            return None;
        }
        let mut genesis = [0u8; 32];
        for (i, g) in genesis.iter_mut().enumerate() {
            *g = u8::try_from(hex_exact(genesis_hex.get(2 * i..2 * i + 2)?, 2)?).ok()?;
        }
        let epoch = u32::try_from(hex_exact(epoch_hex, 8)?).ok()?;
        let mut f = [0u64; 6];
        let mut parts = local.split(':');
        for x in &mut f {
            *x = hex_exact(parts.next()?, 16)?;
        }
        if parts.next().is_some() {
            return None;
        }
        let [h, s, g, t, hh, k] = f;
        Some(TPoint { origin: Origin { genesis, epoch }, h, lanes: [s, g, t, hh], k })
    }

    /// The point written on the wire.
    pub fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.origin.genesis);
        out.extend_from_slice(&self.origin.epoch.to_le_bytes());
        for v in self.fields() {
            out.extend_from_slice(&v.to_le_bytes());
        }
    }

    /// The point read at `p`.
    pub fn take(b: &[u8], p: &mut usize) -> Option<TPoint> {
        let genesis: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        let epoch = u32::from_le_bytes(b.get(*p + 32..*p + 36)?.try_into().ok()?);
        let mut q = *p + 36;
        let mut f = [0u64; 6];
        for x in &mut f {
            *x = u64::from_le_bytes(b.get(q..q + 8)?.try_into().ok()?);
            q += 8;
        }
        *p = q;
        let [h, s, g, t, hh, k] = f;
        Some(TPoint { origin: Origin { genesis, epoch }, h, lanes: [s, g, t, hh], k })
    }

    /// DOES THIS POINT ADVANCE ON `held`: nothing held; or the same origin
    /// at a greater tick; or a later epoch — a universe re-originated
    /// (TU-6 RESET), whose ticks start again. Points of one origin at one
    /// tick never replace one another here: which of two is final is the
    /// keepers' quorum's to say (TU-3), never a reader's tie-break.
    pub fn advances(&self, held: Option<&TPoint>) -> bool {
        match held {
            None => true,
            Some(h) if h.origin == self.origin => self.h > h.h,
            Some(h) => self.origin.epoch > h.origin.epoch,
        }
    }
}

/// `width` lowercase hex digits exactly, as one integer.
fn hex_exact(s: &str, width: usize) -> Option<u64> {
    if s.len() != width || !s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) {
        return None;
    }
    u64::from_str_radix(s, 16).ok()
}
