// U12 (78.65 as amended 2026-10-09) — δ_causal, THE ASYMMETRIC COBOUNDARY:
// what a fold does with the chains it does not close.
//
//! The fold is a coboundary reduction on the nerve of the line's acts. The
//! closed cycles — the settled rows, the orbits whose closes the prefix
//! holds — lie in its kernel and are integrated into the era root: the
//! epoch rows standing at the root, the receipts cold in the era, citable
//! by their coordinates (law 7: integrated, never discarded). The open
//! chains — an escrow not yet released, drawn or revoked; a listing whose
//! demands are unresolved — do not close, and the fold never waits for
//! them: each is PROJECTED onto the new active surface as a puncture, a
//! typed record citing the word it stands on by its admitting stamp, the
//! place it opened on this line and the lanes it carries. The era records
//! the punctures it projected first; a chain still open at a later fold is
//! carried, counted on that era and cited in its own.
//!
//! Before U12 the fold passed the open carriers (78.65) and said nothing of
//! them: the escrow stayed in `Treasury::escrowed` and the listing on the
//! open book, untyped across the cut, and an era named no boundary.

use crate::chronometry::{ActKind, Chain, Place, Puncture, Stamp};
use crate::cognition::Kernel;
use crate::observe::{note_puncture, PunctureAxis};
use crate::verdict::{REFUSE_PUNCTURE_CLOSED, REFUSE_PUNCTURE_RULED, REFUSE_PUNCTURE_UNCITED};

/// WHAT `δ_causal` PROJECTS at one fold: the punctures it projects first —
/// the open chains that opened in its prefix, strictly after the root the
/// prefix stands on and at or below the new root — the open chains an
/// earlier fold already projected (carried), and the open carriers no
/// admitting stamp cites, which no puncture can name and which are counted,
/// never dropped.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Projection {
    pub punctures: Vec<Puncture>,
    pub carried: u64,
    pub uncited: u64,
}

impl Kernel {
    /// δ_causal — THE OPEN CHAINS ON THE SURFACE A FOLD AT `root` LEAVES,
    /// the prefix standing on `after`. Read off the words by their
    /// admitting stamps (`held_at`, the coordinates a puncture cites), each
    /// word once at its own stamp, never at an alias: its escrow, opened at
    /// the escrow's anchor; its listing, opened at its admitting act. Only
    /// a chain that opened on THIS line crosses this line's fold (law 9:
    /// another line's order is never compared with this one's); one opened
    /// past `root` stands on the hot line and crosses nothing yet.
    pub(crate) fn project_punctures(&self, after: Option<Place>, root: Place) -> Projection {
        let me = self.chain.vk();
        let mut out = Projection::default();
        let (mut escrows, mut demands) = (0u64, 0u64);
        let place = |out: &mut Projection, p: Puncture| {
            if p.opened > root {
                return;
            }
            match after {
                Some(a) if p.opened <= a => out.carried += 1,
                _ => out.punctures.push(p),
            }
        };
        for (key, addr) in self.held_at.scoped() {
            let Some(word) = Stamp::from_key(&key) else { continue };
            if self.admitted_at.get(addr.as_bytes()) != Some(&word) {
                continue;
            }
            if let Some(e) = self.treasury.escrow_of(addr.as_bytes()) {
                escrows += 1;
                if e.anchor.0.vk == me {
                    place(&mut out, Puncture { chain: Chain::Escrow, word, opened: e.anchor.0.place(), lanes: e.mu });
                }
            }
            if let Some(w) = self.pending.get(addr.as_bytes()) {
                demands += 1;
                if word.vk == me {
                    place(&mut out, Puncture { chain: Chain::Demand, word, opened: word.place(), lanes: w.magnitude() });
                }
            }
        }
        // The open carriers no stamp cites: what each holds less what was
        // found at a stamp — each found is a distinct key of its carrier, so
        // never more than it holds. A count read, never a branch.
        out.uncited = (self.treasury.escrowed.len() as u64).abs_diff(escrows) + (self.pending.len() as u64).abs_diff(demands);
        out
    }

    /// THE PUNCTURES STANDING ON THIS LINE'S ACTIVE SURFACE: every open
    /// chain that crossed a fold of it — opened at or below its root — and
    /// is open still. Empty before the first fold.
    pub fn punctures(&self) -> Vec<Puncture> {
        match self.chain.root() {
            Some(r) => self.project_punctures(None, r.at).punctures,
            None => Vec::new(),
        }
    }

    /// SETTLE A PUNCTURE IN THE NEW ERA: exactly the chain it names. The
    /// citation locates the word (`held_at`); the escrow open there must be
    /// the one the puncture carried — opened at the same place — or the
    /// puncture's chain has closed and another stands in its place
    /// (`REFUSE_PUNCTURE_CLOSED`). The settling act is sealed first and the
    /// release mints at it (EC1), paying the escrow's voices what the
    /// closer's measured `discharge` covers of the claim and never more
    /// (78.149 (b), `Treasury::release_discharged`). A demand's puncture
    /// closes by its ruling or its lapse (`REFUSE_PUNCTURE_RULED`).
    /// Answers what was paid, per lane.
    pub fn settle_puncture(&mut self, p: &Puncture, discharge: [u64; 4]) -> Result<[u64; 4], &'static str> {
        let settled = self.settle_cited(p, discharge);
        match settled {
            Ok(_) => note_puncture(PunctureAxis::Settled, 1),
            Err(_) => note_puncture(PunctureAxis::SettleRefused, 1),
        }
        settled
    }

    fn settle_cited(&mut self, p: &Puncture, discharge: [u64; 4]) -> Result<[u64; 4], &'static str> {
        let addr = self.held_at.get(&p.word.key()).cloned().ok_or(REFUSE_PUNCTURE_UNCITED)?;
        if p.chain == Chain::Demand {
            return Err(REFUSE_PUNCTURE_RULED);
        }
        let open = self.treasury.escrow_of(addr.as_bytes()).ok_or(REFUSE_PUNCTURE_CLOSED)?;
        if open.anchor.0.place() != p.opened || open.anchor.0.vk != self.chain.vk() {
            return Err(REFUSE_PUNCTURE_CLOSED);
        }
        let me = self.identity;
        self.append_step(ActKind::Settled, &addr, me);
        let at = self.anchor();
        let paid = self.treasury.release_discharged(addr.as_bytes(), discharge, at).ok_or(REFUSE_PUNCTURE_CLOSED)?;
        self.mark_persist();
        Ok(paid)
    }
}
