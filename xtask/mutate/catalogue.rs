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
    Mutant {
        law: "law 2: a task's identity is never formed with a train pair skipped (MU-DERIVATIONS D3.2)",
        file: "src/server/registry.rs",
        before: "refine_delta(a, b, &mut stroke) else {\n            return None;\n        };",
        after: "refine_delta(a, b, &mut stroke) else {\n            continue;\n        };",
        target: &["--lib"],
        filter: "server::tests::registry::task_address",
    },
    Mutant {
        law: "78.125 (law version 5): reach is a citation AT OR ABOVE the window's opening, never strictly above",
        file: "src/chronometry/tuniversal/reach.rs",
        before: "carried.is_none_or(|c| c < self.through)",
        after: "carried.is_none_or(|c| c <= self.through)",
        target: &["--lib"],
        filter: "server::cognition::tests::challenge::window",
    },
    Mutant {
        law: "78.125 (law version 5): the window waits on EVERY other keeper, none dropped",
        file: "src/chronometry/tuniversal/reach.rs",
        before: "            if keeper == court {\n                continue;\n            }",
        after: "            if keeper == court || !witnesses.is_empty() {\n                continue;\n            }",
        target: &["--lib"],
        filter: "server::cognition::tests::challenge::window",
    },
    Mutant {
        law: "78.125 (law version 5): the return is a reading strictly above the reach reading",
        file: "src/chronometry/tuniversal/reach.rs",
        before: "w.back.tick.h <= w.reach.tick.h",
        after: "w.back.tick.h < w.reach.tick.h",
        target: &["--lib"],
        filter: "server::cognition::tests::challenge::window",
    },
    Mutant {
        law: "78.134 (MU-DERIVATIONS D1.2 Phase A): the sphere fit reads r's side of π by refinement, the right way round",
        file: "src/core/certified.rs",
        before: "        if r < &x.lo {\n            return Some(std::cmp::Ordering::Less);",
        after: "        if r > &x.lo {\n            return Some(std::cmp::Ordering::Less);",
        target: &["--lib"],
        filter: "server::cognition::tests::sphere",
    },
    Mutant {
        law: "78.140, 78.162 (5) (MU-DERIVATIONS D1.2 Phase A): a torus throat fits only APART from its period image",
        file: "src/server/cognition/mouth.rs",
        before: "!= crate::transducer::cga::Containment::Apart {",
        after: "!= crate::transducer::cga::Containment::On {",
        target: &["--lib"],
        filter: "server::cognition::tests::sphere",
    },
    Mutant {
        law: "A9 (MU-DERIVATIONS D3.3): π* holds only terms common to EVERY training evidence's carrying set",
        file: "src/court/verdict/selection.rs",
        before: "        if !rest.iter().all(|s| s.contains(&key)) {",
        after: "        if false && !rest.iter().all(|s| s.contains(&key)) {",
        target: &["--lib"],
        filter: "court::verdict::tests::selection",
    },
    Mutant {
        law: "MU-STANDARDS §2.4 (MU-DERIVATIONS D4.4): a decoded selection term deeper than the language mints is refused, never an abort",
        file: "src/transducer/anchor/codec.rs",
        before: "        if usize::from(depth) >= super::SELECTION_HEIGHT {",
        after: "        if false && usize::from(depth) >= super::SELECTION_HEIGHT {",
        target: &["--lib"],
        filter: "transducer::anchor::tests::depth",
    },
    Mutant {
        law: "NO CONSTANT (MU-DERIVATIONS D1.4): a registered number keeps its class tag, or the numbers gate refuses the tree",
        file: "src/server/cognition/mouth.rs",
        before: "/// REGISTERED: THROAT_GRID\npub const THROAT_GRID",
        after: "pub const THROAT_GRID",
        target: &["--manifest-path", "xtask/Cargo.toml"],
        filter: "numbers_refuses_nothing_in_tree",
    },
];
