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

    /// Speak a submitted rule onto withheld tests and compare cells exactly.
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
        let mut paid = [0u64; 4];
        let anchor = self.anchor();
        for addr in &listings {
            if let Some(crate::economy::escrow::Escrowed { mu: held, anchor: a, .. }) =
                self.treasury.release_award(addr)
            {
                let take = self.treasury.settle_solve(voice, held, a);
                for l in 0..4 {
                    paid[l] = paid[l].saturating_add(take[l]);
                }
            }
        }
        if paid.iter().all(|&x| x == 0) {
            return self.treasury.settle_solve(voice, mu, anchor);
        }
        paid
    }

}
