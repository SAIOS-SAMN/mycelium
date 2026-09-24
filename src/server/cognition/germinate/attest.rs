// 26, 47.3 — THE LOCAL QUOTE AND CONVERGENCE: a reading of the page, the
// voices of an object, and where two organs converge.
//
//! Convergence is on a witness's RE-DERIVATION by `same_vertex` (M1) and
//! never on the bytes it happens to be spelled with. The attested chain
//! that stood here was the court composing by F over a voice's painted sites (law
//! 8, ruled 2026-09-10); it is deleted, and a voice's one act is the
//! sentence `perceive_speech` hears.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

impl Kernel {
    /// QUOTE — a local reading of the page: listings still on_page, in
    /// atlas order, under one stroke Budget. Tests use this; the wire
    /// quote is an incidence query over the voice's charts.
    pub fn quote(&mut self) -> ([u64; 4], Vec<(Element, [u64; 4])>) {
        self.remeasure_pressure();
        let surface = crate::economy::yield_surface(&self.physics);
        let mut priced = Vec::new();
        let mut budget = self.physics.stroke();
        // Resumed from the last quote (rule 10, S13.3): the page is a
        // circuit, so a page beyond one budget is served whole across quotes
        // rather than its head on every one.
        let from = self.quote_cursor.clone();
        let mut cur = self.on_page.cursor(from.as_deref(), &mut budget);
        for (addr, _) in &mut cur {
            let Some(p) = self.pending.get(&addr) else { continue };
            let mut mag = p.magnitude();
            // A listing whose T cannot be measured is not priced against a
            // zero put there for it; it is not quoted on this page.
            let Some(t) = self.t_extent(&p.address(), self.height()) else {
                continue;
            };
            mag[2] = t;
            let mu = crate::economy::price(&mag, &self.physics.pressure);
            priced.push((p.clone(), mu));
        }
        if let Some(k) = cur.last_key() {
            self.quote_cursor = Some(k.to_vec());
        }
        (surface, priced)
    }

    /// The voices that uttered this object — the settle receipt's parties.
    /// A use, credited (24.1); never a vantage.
    pub fn voices_of(&self, germ: &[u8]) -> Vec<[u8; 32]> {
        self.convergence.parties(germ)
    }

    /// THIS COURT re-derived `object` and reached its address, and the voice
    /// that uttered it is a party to the settlement. The second vantage is a
    /// witness hypha's re-derivation of the same address (47.3), recorded by
    /// `converge_witness` when an Offer is glued.
    pub(crate) fn converge_here(&mut self, object: &Element, voice: [u8; 32]) {
        let addr = object.address();
        self.converge_here_at(&addr, object, voice);
    }

    /// The same, where the object is DOCKETED under a key that is not its
    /// own address. The key is the docket's coordinate; the object is what
    /// convergence compares.
    pub(crate) fn converge_here_at(&mut self, addr: &Address, object: &Element, voice: [u8; 32]) {
        let me = self.identity;
        let (k, h) = (self.k_step(), self.height());
        self.convergence.rederived(
            addr.as_bytes(),
            me,
            crate::court::convergence::Rederivation { object: object.clone(), k, height: h },
        );
        self.convergence.used_by(addr.as_bytes(), voice);
    }

    /// A WITNESS HYPHA re-derived `object` and reached `reached`. Equal to
    /// this court's address, the object has converged and may install;
    /// different, it is inconsistent data and refuses (13.9 step 8).
    pub(crate) fn converge_witness(
        &mut self,
        object: &Address,
        witness: [u8; 32],
        reached: Element,
        k: u64,
        h: u64,
    ) -> crate::court::convergence::Converged {
        self.convergence.rederived(
            object.as_bytes(),
            witness,
            crate::court::convergence::Rederivation { object: reached, k, height: h },
        );
        let me = self.identity;
        let bond = |v: &[u8; 32]| self.treasury.bond_of(v);
        let room = |v: &[u8; 32]| self.lane_room_of(v);
        // 47.3: a second VANTAGE, not a second copy. The witness is another
        // organ, bonded outside this home's delegation closure, with room on
        // every lane. Delegation closure is task 5.3; until it lands the
        // test is identity and bond, and that limit is stated rather than
        // hidden behind a passing check.
        let independent = move |a: &[u8; 32], b: &[u8; 32]| -> bool {
            a != b && (*a == me || *b == me) && {
                let other = if *a == me { b } else { a };
                // A vantage whose room cannot be read is not a vantage
                // with room: the count is broken, not spacious.
                bond(other).is_some()
                    && room(other).is_some_and(|r| r.iter().any(|&l| l > 0))
            }
        };
        self.convergence.state(object.as_bytes(), &independent)
    }

}
