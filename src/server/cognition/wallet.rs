// 78.93 B1 — A PARTY'S WALLET AT THIS COURT: its account off the treasury's
// carriers, standing at this court's head.

use super::*;
use crate::anastome::dialogue::wallet::Account;

impl Court {
    /// THE ACCOUNT `party` HOLDS HERE, at this court's head, exactly.
    pub fn wallet(&self, party: &[u8; 32]) -> Account {
        let (balance, bond, held) = self.treasury.account(party);
        Account { court: self.chain.signer(), balance, bond, held, at: self.chain.head_stamp() }
    }
}
