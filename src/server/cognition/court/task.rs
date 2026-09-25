// 21, 26 — THE TASK: a listing bound to a corpus family, and what a solve
// against it pays.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

impl Kernel {
    pub fn bind_listing_task(&mut self, listing: impl AsRef<[u8]>, task: impl AsRef<[u8]>) {
        let listing = Address::from_bytes(listing);
        let task = Address::from_bytes(task);
        self.listing_task.insert(&listing, task.clone());
        match self.task_listings.get_mut(task.as_bytes()) {
            Some(v) => {
                if !v.iter().any(|a| a == &listing) {
                    v.push(listing);
                }
            }
            None => {
                self.task_listings.insert(&task, vec![listing]);
            }
        }
    }

    /// The trains a rule must speak to mint at install. Empty trains are ignored.
    pub fn bind_task_family(&mut self, task: impl AsRef<[u8]>, train: Vec<crate::transducer::family::TrainPair>) {
        if !train.is_empty() {
            self.task_family.insert(task, train);
        }
    }

    /// Speak a submitted rule onto withheld tests and compare the grids exactly.
    /// The rule's cited forms resolve through this kernel's ruled cover.
    pub fn rule_matches_tests(&self, rule: &Element, tests: &crate::transducer::family::Family) -> bool {
        if tests.is_empty() {
            return false;
        }
        let Some(sr) = sentence_rule_of(rule) else {
            return false;
        };
        tests.iter().all(|(x, y)| {
            crate::transducer::family::speak_rule(&sr, x, self)
                .and_then(|s| s.grid())
                .is_some_and(|g| g == *y)
        })
    }

    /// Pay a verified solve from the escrow of listings that task deflated.
    /// When a rule is supplied, the home speaks it on the withheld tests;
    /// a full match binds the solve to those listings, draws their escrow,
    /// and Offers the rule on the docket.
    pub fn pay_solve(
        &mut self,
        task: &[u8],
        voice: [u8; 32],
        mu: [u64; 4],
        rule: Option<&Element>,
        tests: &crate::transducer::family::Family,
    ) -> [u64; 4] {
        if let Some(rule) = rule {
            if !self.rule_matches_tests(rule, tests) {
                return [0; 4];
            }
            self.docket_connection(rule, voice);
            self.offered.insert(rule.address(), ());
        }
        // A task with no listings bound to it has none: the reading of
        // absence on a keyed carrier.
        let listings: Vec<Address> = match self.task_listings.get(task) {
            Some(l) => l.clone(),
            None => Vec::new(),
        };
        // EC1: each bound listing's release is a Settled act citing that
        // listing, authored by the solving voice and sealed first; its POUW
        // rows stand at it, never at the act that escrowed the listing.
        let mut paid = [0u64; 4];
        for addr in &listings {
            if self.treasury.escrow_of(addr.as_bytes()).is_none() {
                continue;
            }
            self.append_step(crate::chronometry::ActKind::Settled, addr, voice);
            let settled = self.anchor();
            if let Some(crate::economy::escrow::Escrowed { mu: held, .. }) = self.treasury.release_award(addr) {
                let take = self.treasury.settle_solve(voice, held, settled);
                for l in 0..4 {
                    #[expect(
                        clippy::disallowed_methods,
                        reason = "rule 11 — each take is minted to the one solving voice, whose credit is checked, and nothing in this loop debits it: Σ take ≤ that voice's balance ≤ u64::MAX"
                    )]
                    let p = paid[l].saturating_add(take[l]);
                    paid[l] = p;
                }
            }
        }
        if paid.iter().all(|&x| x == 0) {
            // No listing binds: the solve settles the task itself.
            self.append_step(crate::chronometry::ActKind::Settled, task, voice);
            let settled = self.anchor();
            return self.treasury.settle_solve(voice, mu, settled);
        }
        paid
    }

}
