// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE MUTANT CATALOGUE: each protected law, broken on purpose by one exact
//! textual replacement, and the narrowest suite that must notice.
//!
//! A `before` span must occur exactly once in its file at HEAD, or the
//! mutant is refused as stale: an anchor that moved names code that moved,
//! never a law that fell. Each `after` removes the guard and nothing else,
//! so a suite that stays green names a law no test constrains.

/// One deliberate breakage of one law.
pub struct Mutant {
    /// The law the guarded code enforces.
    pub law: &'static str,
    /// The file, relative to the repository root.
    pub file: &'static str,
    /// The exact span the guard is written as; unique in `file`.
    pub before: &'static str,
    /// The span with the guard removed.
    pub after: &'static str,
    /// The cargo target selection the suite builds (`--lib`, `--test x`).
    pub target: &'static [&'static str],
    /// The test-name filter of the narrowest suite that must fail.
    pub filter: &'static str,
}

/// The catalogue, in the order it runs.
pub const CATALOGUE: &[Mutant] = &[
    Mutant {
        law: "47.2 step 2: a frame admits only under an Ed25519 signature over its own prefix",
        file: "src/court/admission/admit.rs",
        before: "    if !crate::court::identity::verify(signer, &bytes, sig) {\n        return Err(REFUSE_SIGNATURE);\n    }",
        after: "    if false && !crate::court::identity::verify(signer, &bytes, sig) {\n        return Err(REFUSE_SIGNATURE);\n    }",
        target: &["--lib"],
        filter: "court::admission",
    },
    Mutant {
        law: "47.2 step 3: a duplicate frame at a held place is stale, never an equivocation",
        file: "src/court/admission/admit.rs",
        before: "                Some(held) if held.chain_hash == r.chain_hash => Admitted::Stale,\n",
        after: "",
        target: &["--lib"],
        filter: "court::admission",
    },
    Mutant {
        law: "47.1: a signed prefix with trailing bytes is refused whole on decode",
        file: "src/court/admission/prefix.rs",
        before: "        if p != b.len() {\n            return None;\n        }\n        Some(SignedPrefix {",
        after: "        Some(SignedPrefix {",
        target: &["--lib"],
        filter: "court::admission",
    },
    Mutant {
        law: "47.4: an equivocation proof burns the signer's bond (Court::burn_equivocation)",
        file: "src/server/cognition/state.rs",
        before: "        let burned = self.treasury.burn(signer, bond, anchor);\n",
        after: "        let _ = anchor;\n        let burned = bond;\n",
        target: &["--lib"],
        filter: "server::cognition::tests::admission",
    },
    Mutant {
        law: "§6 (R9): Treasury::audit refuses supply that is not balances + bond + held",
        file: "src/economy/treasury.rs",
        before: "        if held != issued {\n            return Err(format!(",
        after: "        if false {\n            return Err(format!(",
        target: &["--lib"],
        filter: "economy::tests",
    },
    Mutant {
        law: "47.3: a second vantage is a bonded organ with room, independent of the home",
        file: "src/server/cognition/germinate/attest.rs",
        before: "                bond(other).is_some()\n                    && room(other).is_some_and(|r| r.iter().any(|&l| l > 0))",
        after: "                let _ = (&bond, &room, other);\n                true",
        target: &["--lib"],
        filter: "server::cognition::tests::court",
    },
    Mutant {
        law: "MU-ECONOMY §5.2: a lease pays each verified job once",
        file: "src/server/cognition/lease.rs",
        before: "        let _ = self.atlas.drop_lease(&id);\n        lease.jobs.retain(|j| j != job);\n",
        after: "        let _ = self.atlas.drop_lease(&id);\n",
        target: &["--lib"],
        filter: "server::cognition::tests::lease",
    },
    Mutant {
        law: "MU-ECONOMY §5.2: a replayed lease close (spent nonce) is refused as Spent",
        file: "src/server/cognition/lease.rs",
        before: "        if self.atlas.spent.contains(&spent) {\n            return Err(LeaseRefusal::Spent);\n        }\n        if self.atlas.leases.get(&a.lease)",
        after: "        if self.atlas.leases.get(&a.lease)",
        target: &["--lib"],
        filter: "server::cognition::tests::lease",
    },
    Mutant {
        law: "MU-ECONOMY §5.2: a replayed lease open (spent nonce) is refused as Spent",
        file: "src/server/cognition/lease.rs",
        before: "        if self.atlas.spent.contains(&spent) {\n            return Err(LeaseRefusal::Spent);\n        }\n        let distinct",
        after: "        let distinct",
        target: &["--lib"],
        filter: "server::cognition::tests::lease",
    },
    Mutant {
        law: "STD named refusal: a lease the payer cannot hold is refused as Short, never skipped",
        file: "src/server/cognition/lease.rs",
        before: "        if self.treasury.hold(a.payer, &hold_key(&id), a.mu, anchor).is_err() {\n            return Err(LeaseRefusal::Short);\n        }",
        after: "        let _ = self.treasury.hold(a.payer, &hold_key(&id), a.mu, anchor);",
        target: &["--lib"],
        filter: "server::cognition::tests::lease",
    },
];
