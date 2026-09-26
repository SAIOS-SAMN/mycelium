// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! EXPANSION 1, 1.2 — ALGEBRAIC REWRITING OVER THE PRESENTATION: the
//! relators oriented as rules, the critical pairs their overlaps make, the
//! completion that resolves them, and the geodesic normal form a word
//! reduces to.
//!
//! WHY THIS IS NOT THE WORD PROBLEM. The word problem is already DECIDED
//! here, and decided exactly: a word evaluates to a `Rotor` over ℚ and two
//! words name one group element iff their rotors are projectively equal.
//! That representation is faithful, so no completion is needed to answer
//! "are these the same operator" — `Word::operator_eq` answers it in one
//! product and one comparison, with no search and no rewriting at all.
//!
//! What rewriting gives that the rotor does not is a CANONICAL SHORTEST
//! SPELLING. Two words with one rotor are one operator, but they are not
//! one tape, and the vocabulary is tapes: `q·q·q` and `q⁻¹` name one vertex
//! and cost three letters against one. A presentation that reduces a word
//! to its geodesic tells the refinement which of the two the language should
//! hold, and lets a word be recognised as redundant because it REWRITES to
//! something held — not because its compartments happen to tile.
//!
//! THE REDUCTION ORDER IS SHORTLEX and it is what makes this terminate. A
//! rule rewrites `lhs → rhs` only where `rhs` is strictly smaller in
//! (length, then key bytes); every rewrite therefore strictly decreases a
//! well-founded order on a finite alphabet, so no reduction can loop. The
//! orientation is not a choice made here — it is read off the order.

use super::{plane_of, LetterId, Rotor, Word};
use crate::network::nerve::Keyed;

/// A rewrite rule `lhs → rhs`, oriented so the right side is strictly
/// smaller under shortlex. Both sides name the same group element: that is
/// the invariant every constructor checks and `holds` re-derives.
#[derive(Clone, Debug)]
pub struct Rule {
    pub lhs: Word,
    pub rhs: Word,
}

/// THE TAPE'S OWN IDENTITY: its letters, in order, with the inverse flag.
///
/// NOT `Word::key`, and this is the whole of why the first cut of this
/// module produced no rules at all. `Word::key` is the ROTOR's key — its own
/// doc says "mixed-plane spellings of one rotor share this key" — so it
/// identifies the ELEMENT. A rewriting system is about tapes: `q·q·q` and
/// `q⁻¹` are one element and two tapes, and that difference is the entire
/// content of a rule. Asked through the element's key, every rotation of
/// every relator compared equal to its own inverse and `orient` correctly
/// refused all of them (rule 11 — two identical tapes are no rule), so the
/// seed came back empty.
///
/// The letters are content: a word IS its sequence, and position in it is
/// where a letter acts (law 2).
pub fn tape_key(w: &Word) -> Vec<u8> {
    let mut out = Vec::with_capacity(w.len() * 2);
    for (id, inv) in w.letters() {
        out.push(id.raw());
        out.push(u8::from(*inv));
    }
    out
}

/// SHORTLEX ON TAPES: fewer letters first, then the tape's own bytes.
///
/// A total order on a finite alphabet with no infinite descending chain,
/// which is exactly what a terminating rewriting system needs — and it must
/// be an order on TAPES, because an order on elements would call every
/// spelling of one element equal and orient nothing.
pub fn shortlex_lt(a: &Word, b: &Word) -> bool {
    match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Greater => false,
        std::cmp::Ordering::Equal => tape_key(a) < tape_key(b),
    }
}

impl Rule {
    /// Orient a relation into a rule. `None` where the two sides are the
    /// same tape — there is nothing to rewrite — and the smaller side
    /// always becomes the right, whichever way the relation was written.
    pub fn orient(a: &Word, b: &Word) -> Option<Rule> {
        if tape_key(a) == tape_key(b) {
            return None;
        }
        let (lhs, rhs) = match shortlex_lt(a, b) {
            true => (b.clone(), a.clone()),
            false => (a.clone(), b.clone()),
        };
        Some(Rule { lhs, rhs })
    }

    /// THE RULE IS A THEOREM ABOUT THE GROUP, re-derived and not asserted:
    /// both sides must evaluate to the same element. A rule that fails this
    /// would rewrite one operator into another, which is not a
    /// simplification but a corruption, and it is checked in the exact
    /// algebra rather than trusted from the source it came from.
    pub fn holds(&self) -> bool {
        self.lhs.rotor().projective_eq(&self.rhs.rotor())
    }
}

/// A RULE'S OWN IDENTITY: both tapes, length-delimited so `(u, vw)` and
/// `(uv, w)` are two rules and not one key.
pub fn rule_key(r: &Rule) -> Vec<u8> {
    let mut k = Vec::new();
    crate::transducer::address::put_key_bytes(&mut k, &tape_key(&r.lhs));
    crate::transducer::address::put_key_bytes(&mut k, &tape_key(&r.rhs));
    k
}

/// THE RULES THE STATED RELATORS GIVE. A relator is a tape that multiplies
/// to the identity, so every rotation of it splits into a rule: `uv = 1`
/// means `u → v⁻¹` for each way of cutting the relator in two.
///
/// ONLY THE FINITE-ORDER RELATORS PARTICIPATE. `PrimeInfinite` records that
/// a prime letter has infinite order — there is no relation to orient, and
/// including it would assert one. The 90° turns (order four), the
/// complementary-plane commutators and the sheet are the finite content of
/// the presentation and the whole of what can be completed.
pub fn rules_from_relators() -> Vec<Rule> {
    let mut out: Vec<Rule> = Vec::new();
    let mut seen: Keyed<()> = Keyed::new();
    for r in super::relators() {
        if matches!(r.kind, super::RelatorKind::PrimeInfinite) {
            continue;
        }
        let tape = r.word.letters().to_vec();
        if tape.is_empty() {
            continue;
        }
        // THE RELATOR ITSELF IS THE FIRST RULE: `w = 1` means `w → ε`, and
        // it is the one that actually shortens — every other rule from this
        // relator is a rotation of it.
        let mut candidates: Vec<(Word, Word)> =
            vec![(Word::from_letters(&tape), Word::identity())];
        // Every cut: `w = u·v` with `u·v = 1` gives `u → v⁻¹`. Cutting at
        // each of the relator's own positions is reading the one relation
        // the alphabet states, not searching the group.
        //
        // THE TAPES ARE RAW ON BOTH SIDES. Normalising the right first
        // collapses `v⁻¹` into `v`'s own reduced spelling — `q⁻³` becomes
        // `q³` — so both sides become one tape and the relation carries no
        // information. Measured: every rule from `q⁴ = 1` was discarded and
        // the seed came back empty. The whole content of `u → v⁻¹` is that
        // the two are DIFFERENT TAPES for one element, and shortlex is what
        // decides which is the rewrite.
        for cut in 1..tape.len() {
            let u = Word::from_letters(&tape[..cut]);
            let v = Word::from_letters(&tape[cut..]);
            candidates.push((u, v.inverse()));
        }
        for (a, b) in candidates {
            let Some(rule) = Rule::orient(&a, &b) else {
                continue;
            };
            if !rule.holds() {
                continue;
            }
            let key = rule_key(&rule);
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key, ());
            out.push(rule);
        }
    }
    out
}

/// Where `needle` occurs in `hay`, as start indices. A tape scan over one
/// word's own letters — bounded by the object, which is what rule 10 allows.
fn occurrences(hay: &[(LetterId, bool)], needle: &[(LetterId, bool)]) -> Vec<usize> {
    if needle.is_empty() || needle.len() > hay.len() {
        return Vec::new();
    }
    (0..=hay.len() - needle.len())
        .filter(|&i| hay[i..i + needle.len()] == *needle)
        .collect()
}

/// THE LEFTMOST OCCURRENCE, named rather than taken by position off a list.
///
/// A tape's order IS content: a word is its sequence of letters, and
/// position in it is where the letter acts, not the order the letter
/// arrived. So choosing the leftmost redex is not rule 15's choice-by-
/// arrival — it is the standard deterministic reduction strategy, and it
/// must be deterministic or two runs of `reduce` could return two normal
/// forms and confluence would be unverifiable.
fn leftmost(hay: &[(LetterId, bool)], needle: &[(LetterId, bool)]) -> Option<usize> {
    if needle.is_empty() || needle.len() > hay.len() {
        return None;
    }
    (0..=hay.len() - needle.len()).position(|i| hay[i..i + needle.len()] == *needle)
}

/// REDUCE A WORD UNDER A RULE SET, to a form no rule applies to.
///
/// Terminates because every rewrite replaces a factor by a shortlex-smaller
/// one, so the whole word strictly decreases in a well-founded order. The
/// budget is therefore not what makes it stop — it is what bounds the COST
/// on a pathological rule set, and a reduction that spends it is reported as
/// unfinished rather than passed off as normal (rule 11).
pub fn reduce(w: &Word, rules: &[Rule], budget: &mut crate::physics::Budget) -> Option<Word> {
    let mut cur = w.normal_form();
    loop {
        let mut rewrote = false;
        for r in rules {
            if !budget.spend() {
                return None;
            }
            let hay = cur.letters().to_vec();
            let needle = r.lhs.letters();
            let Some(at) = leftmost(&hay, needle) else {
                continue;
            };
            let mut next: Vec<(LetterId, bool)> = hay[..at].to_vec();
            next.extend_from_slice(r.rhs.letters());
            next.extend_from_slice(&hay[at + needle.len()..]);
            let next = Word::from_letters(&next).normal_form();
            // The rewrite must strictly descend, or the system could cycle
            // between two spellings of one element forever.
            if !shortlex_lt(&next, &cur) {
                continue;
            }
            cur = next;
            rewrote = true;
            break;
        }
        if !rewrote {
            return Some(cur);
        }
    }
}

/// A CRITICAL PAIR: two rules whose left sides OVERLAP, so one tape can be
/// rewritten two ways. Knuth–Bendix is the statement that a system is
/// confluent exactly when every such pair reduces to one form, and this is
/// the reading that finds them.
///
/// Two kinds of overlap, and both are read off the tapes rather than
/// searched for: an INCLUSION, where one left side occurs inside the other,
/// and a SUFFIX–PREFIX, where the tail of one is the head of the other.
pub fn critical_pairs(a: &Rule, b: &Rule) -> Vec<(Word, Word)> {
    let (la, lb) = (a.lhs.letters(), b.lhs.letters());
    let mut out = Vec::new();
    // INCLUSION: `lb` inside `la`. Rewriting `la` by `a` gives `a.rhs`;
    // rewriting the inner occurrence by `b` gives the same tape with `lb`
    // replaced. Both name one element, so they are a pair to resolve.
    for at in occurrences(la, lb) {
        let mut by_b: Vec<(LetterId, bool)> = la[..at].to_vec();
        by_b.extend_from_slice(b.rhs.letters());
        by_b.extend_from_slice(&la[at + lb.len()..]);
        out.push((a.rhs.clone(), Word::from_letters(&by_b).normal_form()));
    }
    // SUFFIX–PREFIX: the last `k` of `la` are the first `k` of `lb`, so the
    // joined tape `la · lb[k..]` is rewritable by either rule.
    let max = la.len().min(lb.len());
    for k in 1..max {
        if la[la.len() - k..] != lb[..k] {
            continue;
        }
        // By `a`: rewrite the `la` prefix.
        let mut by_a: Vec<(LetterId, bool)> = a.rhs.letters().to_vec();
        by_a.extend_from_slice(&lb[k..]);
        // By `b`: rewrite the `lb` suffix.
        let mut by_b: Vec<(LetterId, bool)> = la[..la.len() - k].to_vec();
        by_b.extend_from_slice(b.rhs.letters());
        out.push((
            Word::from_letters(&by_a).normal_form(),
            Word::from_letters(&by_b).normal_form(),
        ));
    }
    out
}

/// KNUTH–BENDIX COMPLETION over the presentation's finite part.
///
/// Repeatedly: take every pair of rules, form their critical pairs, reduce
/// both sides, and where the two normal forms DISAGREE add the rule that
/// orients them. A system with no unresolved critical pair is confluent —
/// every word then has one normal form whatever order the rules are applied
/// in — which is what makes a geodesic spelling canonical rather than an
/// artefact of the reduction path.
///
/// IT IS BUDGETED AND MAY NOT FINISH, and that is stated rather than hidden.
/// Completion need not terminate for an arbitrary presentation; this one's
/// finite part is a finite group, so it does, but a caller that spends the
/// budget gets the rules found SO FAR together with the fact that the run
/// was cut short. A partial system is still sound — every rule in it is a
/// theorem re-derived by `holds` — it is only not known to be confluent, and
/// `is_confluent` is the separate reading that says whether it is.
pub fn complete(seed: &[Rule], budget: &mut crate::physics::Budget) -> (Vec<Rule>, bool) {
    let mut rules: Vec<Rule> = seed.to_vec();
    let mut seen: Keyed<()> = Keyed::new();
    for r in &rules {
        seen.insert(rule_key(r), ());
    }
    loop {
        let mut added = false;
        let n = rules.len();
        for i in 0..n {
            for j in 0..n {
                if !budget.spend() {
                    return (rules, false);
                }
                for (x, y) in critical_pairs(&rules[i], &rules[j]) {
                    let (Some(nx), Some(ny)) = (reduce(&x, &rules, budget), reduce(&y, &rules, budget))
                    else {
                        return (rules, false);
                    };
                    if tape_key(&nx) == tape_key(&ny) {
                        continue;
                    }
                    let Some(rule) = Rule::orient(&nx, &ny) else {
                        continue;
                    };
                    // A pair that does not name one element is not a
                    // critical pair of this group — it would mean a rule
                    // above it was unsound — so it is refused rather than
                    // added (rule 11).
                    if !rule.holds() {
                        continue;
                    }
                    let k = rule_key(&rule);
                    if seen.contains(&k) {
                        continue;
                    }
                    seen.insert(k, ());
                    rules.push(rule);
                    added = true;
                }
            }
        }
        if !added {
            return (rules, true);
        }
    }
}

/// Is every critical pair of this system resolved? The definition of local
/// confluence, checked rather than assumed — a completion that spent its
/// budget answers `false` here and says so.
pub fn is_confluent(rules: &[Rule], budget: &mut crate::physics::Budget) -> Option<bool> {
    for a in rules {
        for b in rules {
            for (x, y) in critical_pairs(a, b) {
                let nx = reduce(&x, rules, budget)?;
                let ny = reduce(&y, rules, budget)?;
                if tape_key(&nx) != tape_key(&ny) {
                    return Some(false);
                }
            }
        }
    }
    Some(true)
}

impl Word {
    /// EXPANSION 1 — THE DECIDED WORD PROBLEM. Two words name one group
    /// element iff their rotors are projectively equal: the representation
    /// is faithful and exact over ℚ, so this is an answer and not a search.
    ///
    /// Projective, never coefficient-wise: `Φ` and `−Φ` are one vertex
    /// because the action is the sandwich (1.5), so `q` and `q·q·q·q·q` are
    /// the same operator and must compare so.
    pub fn operator_eq(&self, other: &Word) -> bool {
        self.rotor().projective_eq(&other.rotor())
    }

    /// THE GEODESIC SPELLING of this word under a rule set: the shortlex
    /// least tape reachable by rewriting, with the operator unchanged.
    ///
    /// The operator is re-derived after reducing and the result is REFUSED
    /// if it moved. A reduction that changed the element would be a rule
    /// that is not a theorem, and returning a shorter tape for a different
    /// operator is worse than returning nothing (rule 11).
    pub fn geodesic_under(
        &self,
        rules: &[Rule],
        budget: &mut crate::physics::Budget,
    ) -> Option<Word> {
        let out = reduce(self, rules, budget)?;
        self.operator_eq(&out).then_some(out)
    }
}

/// The plane a rule acts on, where all its letters share one. A reading for
/// a caller that wants the rules of one plane; `None` where the rule spans
/// planes, which the commutators do.
pub fn rule_plane(r: &Rule) -> Option<(u8, u8)> {
    let mut held: Option<(u8, u8)> = None;
    for (id, _) in r.lhs.letters() {
        let p = plane_of(*id)?;
        match held {
            None => held = Some(p),
            Some(q) if q == p => {}
            Some(_) => return None,
        }
    }
    held
}

/// THE ALPHABET'S OWN PRESENTATION, COMPLETED ONCE (Expansion 1). The
/// finite relators the alphabet states — the 90° turns' order, the
/// commutators of complementary planes, the sheet — oriented into rules
/// and completed by Knuth–Bendix, so every word the language writes reduces
/// to one geodesic tape under one system. Held once per process: the
/// relators are the alphabet's and do not move.
///
/// THE BUDGET IS THE SEED'S OWN EXTENT (rule 10): every round of the
/// completion touches every pair of rules once, and a pair's critical
/// pairs are bounded by the tapes' lengths, so the spend is sized by the
/// rules and their letters and not by a number a caller chose. The finite
/// part of this presentation is a finite group, so the completion
/// terminates and the bound is a cost cap; a completion that spent it
/// keeps the rules found — each a theorem re-derived by `holds` — and
/// says it is not known confluent, which `presentation().1` carries.
pub fn presentation() -> &'static (Vec<Rule>, bool) {
    static PRESENTATION: std::sync::OnceLock<(Vec<Rule>, bool)> = std::sync::OnceLock::new();
    PRESENTATION.get_or_init(|| {
        let seed = rules_from_relators();
        let n = seed.len().max(1);
        let extent: usize = seed.iter().map(|r| r.lhs.len() + r.rhs.len() + 1).sum::<usize>().max(1);
        let mut budget = crate::physics::Physics::default().stroke_n(n * n * n * extent);
        complete(&seed, &mut budget)
    })
}

/// A rotor's canonical spelling where the seventeen can spell it: the
/// alphabet's own factorisation, which is a CONSTRUCTION from the algebra
/// and not a search over tapes. Where they cannot, the rotor is an adjoined
/// generator and has no spelling (37.7) — which is a refusal, not a failure.
pub fn spelled_geodesic(
    r: &Rotor,
    rules: &[Rule],
    budget: &mut crate::physics::Budget,
) -> Option<Word> {
    let w = match super::factor_rotor(r)? {
        Ok(w) => w,
        Err(_) => return None,
    };
    w.geodesic_under(rules, budget)
}
