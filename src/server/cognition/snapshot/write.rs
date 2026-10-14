// 28, 78.25, 78.27 — THE SNAPSHOT: the mind written as keyed tries by its
// own coordinates — the court, the ledger with each act's data at its node,
// the words at their citations, the treasury's own tries, and the carriers
// LD3-b makes T3 — each trie its name, its byte length and its count, then
// its nodes in key order with the key inline.
//
//! Nothing is written by position and nothing is read by it. A word no act
//! admitted has no coordinates and is not written; what each row refused
//! is named, once, and counted.

use super::*;
use crate::chronometry::{Cite, Stamp};
use crate::cognition::*;
use crate::network::nerve::Keyed;
use crate::snapshot::{put_leaf, put_trie, Leaves};

/// A LISTING'S COHERENCE FIELD as a leaf value: its entries — each chart as
/// its own coordinates, each closure as its canonical ray, the remainder
/// and the stagnation counts.
pub(super) fn put_field(out: &mut Vec<u8>, listing: &ListingField) {
    out.extend_from_slice(&(listing.entries.len() as u64).to_le_bytes());
    for e in &listing.entries {
        e.chart.put_canonical(out);
        match &e.state {
            crate::transducer::stalk::Closure::Flat => out.push(0),
            crate::transducer::stalk::Closure::Twisted(v) => {
                out.push(1);
                v.put_canonical(out);
            }
            crate::transducer::stalk::Closure::Cavity => out.push(2),
            crate::transducer::stalk::Closure::Class(r, d) => {
                out.push(3);
                crate::transducer::address::put_key_count(out, *d);
                r.put_canonical(out);
            }
        }
        out.extend_from_slice(&(e.remainder.len() as u64).to_le_bytes());
        for &(r, c, hb, ha) in &e.remainder {
            out.extend_from_slice(&r.to_le_bytes());
            out.extend_from_slice(&c.to_le_bytes());
            out.push(hb);
            out.push(ha);
        }
        for s in e.stagnant {
            out.extend_from_slice(&s.to_le_bytes());
        }
    }
}

/// μ⃗ as a leaf value: four lanes.
fn put_mu(out: &mut Vec<u8>, mu: &[u64; 4]) {
    for m in mu {
        out.extend_from_slice(&m.to_le_bytes());
    }
}

/// A word being written: the element held once, the rows it stands on, and
/// where it is provisional — the key of the node its docketed sentence is.
struct Draft {
    elem: Element,
    members: u8,
    provisional: Option<Element>,
}

/// WHAT THE SNAPSHOT REFUSED, per row and per reason: named once, counted.
#[derive(Default)]
struct Refused {
    /// Per row — basis, provisional, listing, cavity, federation — the
    /// words held without an admitting stamp.
    unstamped: [u64; 5],
    /// Basis words with no discoverer to restore them under.
    undiscovered: u64,
    /// Rows whose element stands at an address that is not its own.
    misaddressed: u64,
    /// Federated words ruled at no act.
    unruled: u64,
    /// Words whose citation cannot be written (a class key at its wall).
    unciteable: u64,
    /// Two words at one citation — one fiber, one admitting act.
    coincident: u64,
    /// Per-word fields on a word no node holds: awards, deflations, fields,
    /// ranks.
    orphans: u64,
    /// A rank's closure on a word no node holds.
    closures: u64,
    /// Receipts, settlements and dialect transitions that could not stand.
    receipts: u64,
    settlements: u64,
    transitions: u64,
    /// Eras whose payload could not be written, or two at one root stamp.
    eras: u64,
    /// Cocycle rows whose class key is not a class's coordinates.
    cocycles: u64,
    /// STD1 3c — words whose record passes a length its field writes.
    unwritable: u64,
}

impl Refused {
    fn name(&self) {
        let [b, p, l, c, f] = self.unstamped;
        let rest = [
            ("basis words with no discoverer", self.undiscovered),
            ("rows away from their own address", self.misaddressed),
            ("federated words ruled at no act", self.unruled),
            ("words whose citation cannot be written", self.unciteable),
            ("words coinciding at one citation", self.coincident),
            ("per-word fields on no held word", self.orphans),
            ("rank closures on no held word", self.closures),
            ("receipts that cannot be written", self.receipts),
            ("settlements that cannot stand", self.settlements),
            ("dialect transitions that stood at no act", self.transitions),
            ("eras whose payload cannot be written or whose root stamp is another's", self.eras),
            ("cocycle rows whose class is not a class's coordinates", self.cocycles),
            ("words whose record passes a length its field writes", self.unwritable),
        ];
        if b + p + l + c + f > 0 {
            eprintln!(
                "  snapshot refused words held without a stamp: basis {b} provisional {p} listing {l} \
                 cavity {c} federation {f}"
            );
        }
        for (what, n) in rest {
            if n > 0 {
                eprintln!("  snapshot refused {n} {what}");
            }
        }
    }
}

impl Kernel {
    /// P7 — THE MIND AS KEYED TRIES (78.27): the magic, then each trie by
    /// its name. What each trie refused is named once, with its count.
    pub fn snapshot(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        let mut refused = Refused::default();
        self.put_court(&mut out);
        self.put_ledger(&mut out, &mut refused);
        self.put_words(&mut out, &mut refused);
        self.treasury.put_tries(&mut out);
        self.put_eras(&mut out, &mut refused);
        self.put_complexes(&mut out);
        self.put_cocycles(&mut out, &mut refused);
        self.put_windings(&mut out);
        refused.name();
        out
    }

    /// THE COURT: its key and its court head; where its line is in the
    /// snapshot, the fold root that line stands on; where it is on disk,
    /// the stamp and hash the line stands at as this snapshot is taken.
    fn put_court(&self, out: &mut Vec<u8>) {
        let mut nodes: Keyed<Leaves> = Keyed::new();
        let mut leaves: Leaves = vec![(LEAF_HEAD, self.court_head.to_vec())];
        match (self.line_held, self.chain.root(), self.chain.head_stamp()) {
            (false, Some(root), _) => {
                let mut v = Vec::new();
                root.write(&mut v);
                leaves.push((LEAF_ROOT, v));
            }
            (true, _, Some(at)) => {
                let mut v = Vec::new();
                at.put_canonical(&mut v);
                v.extend_from_slice(&self.chain.head_hash());
                leaves.push((LEAF_LINE, v));
            }
            _ => {}
        }
        // 78.120 — the domains founded here, each with its tier keys; no leaf
        // where none is founded, so a court with none writes as it did.
        let domains: Vec<([u8; 32], Vec<[u8; 32]>)> =
            self.domains.scoped().filter_map(|(k, t)| Some((<[u8; 32]>::try_from(k.as_slice()).ok()?, t.clone()))).collect();
        if !domains.is_empty() {
            let mut v = Vec::new();
            crate::transducer::address::put_key_count(&mut v, domains.len());
            for (w, tiers) in &domains {
                v.extend_from_slice(w);
                crate::anastome::dialogue::domain::put_keys(&mut v, tiers);
            }
            // 78.127 — the domains' reserves, after them (none: none written,
            // unless an origin follows, which needs the reserves' count first).
            let (origins, universes) = (!self.origins.is_empty(), !self.universes.is_empty());
            if !self.reserves.is_empty() || origins || universes {
                super::super::convert::put_reserves(&mut v, &self.reserves);
            }
            // 78.132 — the universes' origins, after the reserves (their count
            // written where statements follow).
            if origins || universes {
                super::super::convert::put_origins(&mut v, &self.origins);
            }
            // 78.139 — the universes' statements, after the origins.
            if universes {
                super::super::universe::put_universes(&mut v, &self.universes);
            }
            leaves.push((LEAF_DOMAINS, v));
        }
        // 78.123 B — the ledger trees, where any is held.
        if let Some(v) = self.ledgers.put() {
            leaves.push((LEAF_LEDGERS, v));
        }
        // 78.125 — the challenge windows, where any is held.
        if let Some(v) = self.ledgers.put_challenges() {
            leaves.push((LEAF_CHALLENGES, v));
        }
        // 78.126 — the handoffs, where any is held.
        if let Some(v) = self.handoffs.put() {
            leaves.push((LEAF_HANDOFFS, v));
        }
        // 78.147 — the fold roots held with their two signatures, where any is.
        if let Some(v) = self.put_root_proofs() {
            leaves.push((LEAF_ROOTS, v));
        }
        // 78.154 — the atlas, where anything is held.
        if !self.atlas.is_empty() {
            leaves.push((LEAF_ATLAS, self.atlas.put()));
        }
        // 78.155b — the grants, where any was made. Each manifest was read
        // off the wire within the same carriers, so it writes back whole.
        if let Some(v) = (!self.grants.is_empty()).then(|| super::super::endowment::put_grants(&self.grants)).flatten() {
            leaves.push((LEAF_ENDOWMENT, v));
        }
        nodes.insert(self.chain.vk(), leaves);
        put_trie(out, COURT_TRIE, &nodes);
    }

    /// T2 — THE LEDGER: at each stamp the treasury rows, the settlements and
    /// the dialect transitions standing there, and the receipt where the
    /// line is in the snapshot. A stamp carrying data and no receipt is a
    /// bare node.
    fn put_ledger(&self, out: &mut Vec<u8>, refused: &mut Refused) {
        let mut nodes: Keyed<Leaves> = Keyed::new();
        let in_hand = self.chain.nodes().filter(|_| !self.line_held);
        for r in in_hand {
            let mut v = Vec::new();
            match r.write(&mut v) {
                // One node per stamp: the receipt is its first leaf.
                Some(()) => {
                    nodes.insert(r.stamp.key(), vec![(LEAF_RECEIPT, v)]);
                }
                None => refused.receipts += 1,
            }
        }
        // Per-act data, gathered per stamp in the order it was written: a
        // count and the records, one leaf per kind.
        let gather = |per: &mut Keyed<(u64, Vec<u8>)>, at: &Stamp, record: Vec<u8>| {
            let key = at.key();
            match per.get_mut(&key) {
                Some((n, body)) => {
                    *n += 1;
                    body.extend_from_slice(&record);
                }
                None => {
                    per.insert(key, (1, record));
                }
            }
        };
        let mut entries: Keyed<(u64, Vec<u8>)> = Keyed::new();
        for e in self.treasury.entries() {
            let mut v = Vec::new();
            e.write(&mut v);
            gather(&mut entries, &e.at, v);
        }
        let mut settled: Keyed<(u64, Vec<u8>)> = Keyed::new();
        for s in self.settlements_cold.iter().chain(self.settlements.iter()) {
            let mut v = Vec::new();
            crate::snapshot::put_key(&mut v, &s.listing);
            // LD0 — a settlement whose chain or citations cannot be framed is
            // not written, never a partial record.
            if crate::anastome::dialogue::push_chain(&mut v, &s.chain).is_none() {
                refused.settlements += 1;
                continue;
            }
            v.extend_from_slice(&s.voice);
            if crate::anastome::dialogue::push_cites(&mut v, &s.cites).is_none() {
                refused.settlements += 1;
                continue;
            }
            put_mu(&mut v, &s.before);
            put_mu(&mut v, &s.after);
            gather(&mut settled, &s.at, v);
        }
        let mut dialect: Keyed<(u64, Vec<u8>)> = Keyed::new();
        for (key, v, at) in &self.dialect_chain {
            // 78.27 (5): a transition that stood at no act is refused by
            // name and counted.
            let Some(at) = at else {
                refused.transitions += 1;
                continue;
            };
            let mut rec = Vec::new();
            crate::snapshot::put_key(&mut rec, key);
            v.put_canonical(&mut rec);
            gather(&mut dialect, at, rec);
        }
        // Each kind is gathered once per stamp, so each files once.
        for (kind, per) in [(LEAF_ENTRIES, &entries), (LEAF_SETTLEMENTS, &settled), (LEAF_DIALECT, &dialect)] {
            crate::snapshot::persist(per, |key, (n, body)| {
                let mut v = n.to_le_bytes().to_vec();
                v.extend_from_slice(body);
                if put_leaf(&mut nodes, key, kind, v).is_none() {
                    refused.receipts += 1;
                }
            });
        }
        put_trie(out, LEDGER_TRIE, &nodes);
    }

    /// T1 — THE WORDS: every word this court holds on any row, one node at
    /// its citation — its fiber and the act that admitted it — the element
    /// held once with the rows it stands on, and each per-word field a leaf.
    fn put_words(&self, out: &mut Vec<u8>, refused: &mut Refused) {
        let mut drafts: Keyed<Draft> = Keyed::new();
        let hold = |drafts: &mut Keyed<Draft>, refused: &mut Refused, row: usize, addr: &[u8], elem: &Element, member: u8| {
            if !self.admitted_at.contains(addr) {
                refused.unstamped[row] += 1;
                return;
            }
            if elem.address().as_bytes() != addr {
                refused.misaddressed += 1;
                return;
            }
            match drafts.get_mut(addr) {
                Some(d) => d.members |= member,
                None => {
                    drafts.insert(addr, Draft { elem: elem.clone(), members: member, provisional: None });
                }
            }
        };
        crate::snapshot::persist(&self.basis, |addr, held| {
            if self.admitted_at.contains(addr) && self.discoverer_of(addr).is_none() {
                refused.undiscovered += 1;
                return;
            }
            hold(&mut drafts, refused, 0, addr, held.element(), MEMBER_BASIS);
        });
        crate::snapshot::persist(&self.pending, |addr, elem| hold(&mut drafts, refused, 2, addr, elem, MEMBER_LISTING));
        crate::snapshot::persist(&self.cavity_book, |addr, elem| hold(&mut drafts, refused, 3, addr, elem, MEMBER_CAVITY));
        for (addr, elem) in &self.court_federation {
            if self.admitted_at.contains(addr.as_bytes()) && !self.fed_at.contains(addr.as_bytes()) {
                refused.unruled += 1;
                continue;
            }
            hold(&mut drafts, refused, 4, addr.as_bytes(), elem, MEMBER_FEDERATION);
        }
        // THE PROVISIONAL ROW (78.27 (2)): where the docketed sentence is the
        // word at its address, the word's node carries it; where it stands
        // under another address, it is its own node, SPOKEN, and the node at
        // the address cites it.
        crate::snapshot::persist(&self.provisional, |addr, spoken| {
            if !self.admitted_at.contains(addr) {
                refused.unstamped[1] += 1;
                return;
            }
            match drafts.get_mut(addr) {
                Some(d) => d.provisional = Some(spoken.clone()),
                None if spoken.address().as_bytes() == addr => {
                    drafts.insert(addr, Draft { elem: spoken.clone(), members: 0, provisional: Some(spoken.clone()) });
                }
                // Docketed away from its address with nothing held there:
                // the address has no element to cite it by.
                None => refused.misaddressed += 1,
            }
        });
        let rank = &self.closure.2;
        // The citation each held address stands at, for the leaves that
        // name another word by its node.
        let key_of = |addr: &[u8], elem: &Element| -> Option<Vec<u8>> {
            let at = *self.admitted_at.get(addr)?;
            let mut key = Vec::new();
            Cite::word(elem, at).put_canonical(&mut key)?;
            Some(key)
        };
        let mut nodes: Keyed<Leaves> = Keyed::new();
        let mut held: Keyed<()> = Keyed::new();
        crate::snapshot::persist(&drafts, |addr, d| {
            let Some(key) = key_of(addr, &d.elem) else {
                refused.unciteable += 1;
                return;
            };
            if nodes.contains(&key) {
                refused.coincident += 1;
                return;
            }
            // STD1 3c — a word whose record does not write is refused by
            // name before it is held, so nothing of it is written.
            let Some(record) = d.elem.to_bytes() else {
                refused.unwritable += 1;
                return;
            };
            held.insert(addr, ());
            // This node's leaves, each filed at its kind's own place; each
            // kind is written once below.
            let mut leaves: Leaves = Vec::new();
            let mut leaf = |kind: u8, v: Vec<u8>| {
                let at = leaves.partition_point(|(k, _)| *k < kind);
                leaves.insert(at, (kind, v));
            };
            let mut h = vec![d.members];
            h.extend_from_slice(&record);
            leaf(LEAF_HELD, h);
            if d.members & MEMBER_BASIS != 0 {
                if let Some(who) = self.discoverer_of(addr) {
                    leaf(LEAF_DISCOVERER, who.to_vec());
                }
            }
            if let Some(award) = self.treasury.escrow_of(addr) {
                let mut v = Vec::new();
                match crate::economy::persist::put_award(&mut v, Some(award)) {
                    Some(()) => leaf(LEAF_AWARD, v),
                    None => refused.unwritable += 1,
                }
            }
            if let Some(mag) = self.deflations.get(addr) {
                let mut v = Vec::new();
                put_mu(&mut v, mag);
                leaf(LEAF_DEFLATION, v);
            }
            for (kind, f) in [(LEAF_FIELD_HOT, self.field.get(addr)), (LEAF_FIELD_COLD, self.field_cold.get(addr))] {
                if let Some(f) = f {
                    let mut v = Vec::new();
                    put_field(&mut v, f);
                    leaf(kind, v);
                }
            }
            let closure = rank.closure_carrier().get(addr);
            let inv = rank.invariants.get(addr);
            if closure.is_some() || inv.is_some() {
                let mut v = Vec::new();
                let root = crate::transducer::stalk::Invariant::default();
                inv.map_or(&root, |i| i).put_canonical(&mut v);
                // THE CLOSURE AS A CITE (78.27): the base point's own node,
                // resolved at restore by one T1 read.
                let base = closure
                    .and_then(|c| c.as_ref())
                    .and_then(|c| Some((c, self.word_at(c.as_bytes())?.clone())))
                    .and_then(|(c, w)| key_of(c.as_bytes(), &w));
                match (closure.and_then(|c| c.as_ref()), base) {
                    (Some(_), Some(k)) => {
                        v.push(1);
                        crate::snapshot::put_key(&mut v, &k);
                    }
                    (Some(_), None) => {
                        refused.closures += 1;
                        v.push(0);
                    }
                    (None, _) => v.push(0),
                }
                leaf(LEAF_RANK, v);
            }
            if d.members & MEMBER_FEDERATION != 0 {
                if let Some(at) = self.fed_at.get(addr) {
                    let mut v = Vec::new();
                    at.put_canonical(&mut v);
                    leaf(LEAF_RULED, v);
                }
            }
            // The docketed sentence: its own node where it is the word
            // here, a SPOKEN node of its own where it is not.
            if let Some(spoken) = &d.provisional {
                let spoken_key = match spoken.address().as_bytes() == addr {
                    true => Some(key.clone()),
                    false => key_of(addr, spoken),
                };
                match spoken_key {
                    Some(sk) => {
                        // Its own node where it is not the word here — and
                        // no provisional leaf naming a node that did not
                        // write (STD1 3c).
                        let own = sk != key && !nodes.contains(&sk);
                        match (own, own.then(|| spoken.to_bytes()).flatten()) {
                            (true, None) => refused.unwritable += 1,
                            (true, Some(record)) => {
                                let mut h = vec![MEMBER_SPOKEN];
                                h.extend_from_slice(&record);
                                nodes.insert(&sk, vec![(LEAF_HELD, h)]);
                                leaf(LEAF_PROVISIONAL, sk);
                            }
                            (false, _) => leaf(LEAF_PROVISIONAL, sk),
                        }
                    }
                    None => refused.unciteable += 1,
                }
            }
            nodes.insert(&key, leaves);
        });
        // A per-word field on a word no node holds is a quantity on a word
        // this court no longer has: counted, and not written (R6).
        let orphan = |keys: Vec<Vec<u8>>| keys.iter().filter(|a| !held.contains(a)).count() as u64;
        refused.orphans += orphan(self.treasury.award_addresses())
            + orphan(self.deflations.scoped().map(|(a, _)| a).collect())
            + orphan(self.field.scoped().map(|(a, _)| a).collect())
            + orphan(self.field_cold.scoped().map(|(a, _)| a).collect())
            + orphan(rank.closure_carrier().scoped().map(|(a, _)| a).collect());
        put_trie(out, WORDS_TRIE, &nodes);
    }

    /// T3 — THE ERAS HELD IN HAND by their root's stamp: the root's height,
    /// signer and place are the key, and each other coordinate of the era
    /// and its payload are leaves. A filed era is its own frame in the cold
    /// segment, and is not written here.
    fn put_eras(&self, out: &mut Vec<u8>, refused: &mut Refused) {
        let mut nodes: Keyed<Leaves> = Keyed::new();
        for era in self.archive.iter().filter(|e| e.at.is_none()) {
            let key = Stamp::new(era.root.height, era.signer, era.root.at).key();
            if nodes.contains(&key) {
                refused.eras += 1;
                continue;
            }
            let mut first = Vec::new();
            era.first.put_canonical(&mut first);
            let Some(payload) = era.payload() else {
                refused.eras += 1;
                continue;
            };
            let whereabouts = (LEAF_PAYLOAD, payload);
            nodes.insert(
                key,
                vec![
                    (LEAF_ROOT_HASH, era.root.hash.to_vec()),
                    (LEAF_WITNESS, era.witness.to_vec()),
                    (LEAF_SIG_HOME, era.sig_home.to_vec()),
                    (LEAF_SIG_WITNESS, era.sig_witness.to_vec()),
                    (LEAF_IDENTITIES, era.identities.to_le_bytes().to_vec()),
                    (LEAF_FIRST, first),
                    (LEAF_COUNT, era.count.to_le_bytes().to_vec()),
                    whereabouts,
                ],
            );
        }
        put_trie(out, ERAS_TRIE, &nodes);
    }

    /// T3 — THE COMPLEXES by their keys, each node the complex's own tries.
    fn put_complexes(&self, out: &mut Vec<u8>) {
        let mut nodes: Keyed<Leaves> = Keyed::new();
        crate::snapshot::persist(&self.complexes, |key, c| {
            let mut v = Vec::new();
            c.put_tries(&mut v);
            nodes.insert(key, vec![(LEAF_TRIES, v)]);
        });
        put_trie(out, COMPLEXES_TRIE, &nodes);
    }

    /// T3 — THE COCYCLE ROWS by their keys: the two voices, the class as its
    /// own coordinates INLINE (78.27 (7)) — never its key nested as a key —
    /// the gauge as its ray, and the stamp of the receipt the row stood at.
    /// A class key that is not a class's coordinates is refused by name.
    fn put_cocycles(&self, out: &mut Vec<u8>, refused: &mut Refused) {
        let mut nodes: Keyed<Leaves> = Keyed::new();
        crate::snapshot::persist(&self.cocycle_at, |key, row| {
            let Some(class) = class_coordinates(&row.class_key) else {
                refused.cocycles += 1;
                return;
            };
            let mut gauge = Vec::new();
            row.gauge.put_canonical(&mut gauge);
            let mut leaves: Leaves = vec![
                (LEAF_FROM, row.a.to_vec()),
                (LEAF_TO, row.b.to_vec()),
                (LEAF_CLASS, class),
                (LEAF_GAUGE, gauge),
            ];
            if let Some(at) = self.cocycle_receipt.get(key) {
                let mut v = Vec::new();
                at.put_canonical(&mut v);
                leaves.push((LEAF_STOOD, v));
            }
            nodes.insert(key, leaves);
        });
        put_trie(out, COCYCLES_TRIE, &nodes);
    }

    /// 78.68 — THE WINDING CARRIERS by their `chart ‖ simplex` keys, each its
    /// carrier's own coordinates.
    fn put_windings(&self, out: &mut Vec<u8>) {
        let mut nodes: Keyed<Leaves> = Keyed::new();
        crate::snapshot::persist(&self.windings, |key, w| {
            let mut v = Vec::new();
            w.put(&mut v);
            nodes.insert(key, vec![(LEAF_WINDING, v)]);
        });
        put_trie(out, WINDINGS_TRIE, &nodes);
        let mut nodes: Keyed<Leaves> = Keyed::new();
        crate::snapshot::persist(&self.lie_basis, |key, basis| {
            let mut v = (basis.len() as u64).to_le_bytes().to_vec();
            for b in basis {
                b.to_bytes(&mut v);
            }
            nodes.insert(key, vec![(LEAF_BASIS, v)]);
        });
        put_trie(out, LIE_TRIE, &nodes);
    }
}

/// A CLASS KEY'S OWN COORDINATES: the two invariants and the bivector kind
/// the key is `TAG_CLASS` followed by, checked to read whole as a class by
/// [`crate::transducer::alphabet::Rotor::take_class_key`]. `None` for a key
/// that is not a class's.
fn class_coordinates(key: &crate::transducer::address::Address) -> Option<Vec<u8>> {
    let [tag, coords @ ..] = key.as_bytes() else { return None };
    if *tag != crate::transducer::address::TAG_CLASS {
        return None;
    }
    let mut p = 0usize;
    let again = crate::transducer::alphabet::Rotor::take_class_key(coords, &mut p)?;
    (p == coords.len() && again == *key).then(|| coords.to_vec())
}
