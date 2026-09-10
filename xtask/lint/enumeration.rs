// Rules 1 and 8 — ENUMERATION: a loop over a carrier reads a keyed source,
// or it is a search.
//
//! `.entries()`, `.for_each(` and `.for_each_from(` walk a carrier whole
//! where the move is a keyed read; `spend_unit()` names the spend count as
//! a number, and only the file that defines it and the wire may say it.

use super::walker::Gate;
use std::path::Path;

pub(crate) const GATE: Gate = Gate {
    name: "enumeration",
    allow: &[],
    owed: &[],
    scan,
};

fn scan(path: &Path, src: &str, hits: &mut Vec<String>) {
    let p = path.to_string_lossy().replace('\\', "/");
    // `spend_unit()` names the spend count as a number. Two files may:
    // `physics.rs`, which DEFINES it and is where the number lives, and
    // `network/wire.rs`, because the wire MTU is the byte size of one spend.
    // One pattern of the gate, not the whole gate, so it is a condition
    // here rather than an allow row. `physics.rs` is new to this gate: the
    // old scope was four directories and the crate root was outside it.
    // It became `physics/mod.rs` when the read token moved to
    // `physics/budget.rs` at the §1.1 reading bound; the allowance follows
    // the definition, which did not move.
    let allow_spend = p.ends_with("network/wire.rs") || p.ends_with("src/physics/mod.rs");
    // FIVE PATTERNS SCOPED TO ONE DIRECTORY, inherited. `.position(`,
    // `.find(`, `.find_map(`, `Vec<bool>` and `unwrap_or` are rules 1, 8, 11
    // and 15, which hold everywhere, and the gate applies them under
    // `sentence/` alone. Widening them is measured at 119 files —
    // `unwrap_or` in 82 of them — and is its own item, not this migration.
    let sentence_only = p.contains("/sentence/");
    for (i, line) in src.lines().enumerate() {
        let mut check = |pat: &str| {
            if line.contains(pat) {
                hits.push(format!("{}:{}: {pat} (rules 1, 8)", path.display(), i + 1));
            }
        };
        check(".entries()");
        check(".for_each(");
        check(".for_each_from(");
        if !allow_spend {
            check("spend_unit()");
        }
        if sentence_only {
            check(".position(");
            check(".find(");
            check(".find_map(");
            check("Vec<bool>");
            check("unwrap_or");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// A walk over a carrier's entries is a hit anywhere.
    #[test]
    fn a_walk_over_entries_is_a_hit() {
        let mut hits = Vec::new();
        super::scan(
            Path::new("src/server/cognition/basis.rs"),
            "for (k, v) in map.entries() {}\n",
            &mut hits,
        );
        assert_eq!(hits.len(), 1, "the walk is a hit: {hits:?}");
    }

    /// `spend_unit()` is a hit everywhere EXCEPT the two files that may
    /// name it — the one that defines the number and the one where the MTU
    /// is one spend. Both halves are checked: an exemption nothing tests
    /// is an exemption that could have swallowed the whole pattern.
    #[test]
    fn spend_unit_is_a_hit_except_in_the_two_files_that_own_it() {
        let src = "let n = spend_unit();\n";
        let mut hit = Vec::new();
        super::scan(Path::new("src/economy/treasury.rs"), src, &mut hit);
        assert_eq!(hit.len(), 1, "elsewhere it is a number standing in: {hit:?}");

        for owner in ["src/physics.rs", "src/network/wire.rs"] {
            let mut clean = Vec::new();
            super::scan(Path::new(owner), src, &mut clean);
            assert!(clean.is_empty(), "{owner} may name it: {clean:?}");
        }
    }

    /// The five inherited patterns are scoped to `sentence/` and hit there
    /// alone. The scope is a known gap (the auditor's F3k), and this test
    /// is what makes the gap a measurement rather than an assumption.
    #[test]
    fn the_five_inherited_patterns_hit_under_sentence_only() {
        let src = "let i = xs.position(|x| x);\nlet f: Vec<bool> = vec![];\nlet u = o.unwrap_or(0);\n";
        let mut inside = Vec::new();
        super::scan(Path::new("src/transducer/sentence/chain.rs"), src, &mut inside);
        assert_eq!(inside.len(), 3, "inside sentence/ all three hit: {inside:?}");

        let mut outside = Vec::new();
        super::scan(Path::new("src/server/cognition/basis.rs"), src, &mut outside);
        assert!(outside.is_empty(), "outside it none do — this is F3k's gap: {outside:?}");
    }
}
