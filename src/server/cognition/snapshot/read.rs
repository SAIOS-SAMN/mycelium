// 28, 78.25, 78.27 — THE RESTORE: a mind read back from its keyed tries,
// by key, with everything derived re-derived rather than restored.
//
//! Each trie is read whole or refused, and a trie is read by its name,
//! never by where it stands. Balances, the ruled stalks, the axis index,
//! the federation's order and the rank's ordinal are recomputed from what
//! was written. A tape that does not parse whole is refused: nothing is
//! partly restored.

use super::*;
use crate::chronometry::{Cite, KStepReceipt, Stamp};
use crate::cognition::*;
use crate::network::nerve::Keyed;
use crate::snapshot::{leaf, take_trie};
use crate::transducer::address::Address;
use crate::transducer::element::Element;

/// μ⃗ read back off a leaf: four lanes, exactly.
fn take_mu(v: &[u8]) -> Option<[u64; 4]> {
    if v.len() != 32 {
        return None;
    }
    let mut mu = [0u64; 4];
    for (l, m) in mu.iter_mut().enumerate() {
        *m = u64::from_le_bytes(v.get(l * 8..l * 8 + 8)?.try_into().ok()?);
    }
    Some(mu)
}

/// A leaf read whole by `read`, or refused: the reader must end where the
/// leaf does.
fn whole<T>(v: &[u8], read: impl FnOnce(&[u8], &mut usize) -> Option<T>) -> Option<T> {
    let mut p = 0usize;
    let t = read(v, &mut p)?;
    (p == v.len()).then_some(t)
}

/// A per-act leaf of the ledger: its count, then that many records, each
/// read by `read`, the leaf read whole.
fn records<T>(v: &[u8], mut read: impl FnMut(&[u8], &mut usize) -> Option<T>) -> Option<Vec<T>> {
    whole(v, |b, p| {
        let n = usize::try_from(u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?)).ok()?;
        *p += 8;
        if n > b.len().checked_sub(*p)? {
            return None;
        }
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(read(b, p)?);
        }
        Some(out)
    })
}

impl Kernel {
    /// P7 — RESTORE a snapshot into a fresh kernel. Returns `false` on a
    /// bad magic or a trie that does not read whole, leaving the kernel
    /// empty: nothing is partly restored.
    ///
    /// AND IT SAYS SO, naming the trie it stopped in and the byte it
    /// stopped at. The refusal is the caller's to act on and the server's
    /// boot REFUSES on it (X8): a `false` here is a whole court's memory —
    /// its basis, its chain, its treasury — and continuing without it
    /// means the next persist writes an empty court over the ledger that
    /// would not read. Measured on the fleet at fc75a59: hypha b's 16 MB
    /// ledger restored as nothing, ran two epochs reporting `basis=0 K=0`,
    /// and was overwritten, because the only line here was the one printed
    /// when it SUCCEEDED and the only branch taken was the one that
    /// ignored the failure.
    pub fn restore(&mut self, b: &[u8]) -> bool {
        match b.get(0..4) {
            Some(m) if m == MAGIC => {}
            // The positional tape (78.25), refused BY NAME: a recut is the
            // road, and the file is not corrupt.
            Some(m) if m == PRE_TRIE_MAGIC => {
                eprintln!("  ledger refused: MYC7, the positional tape (78.25) — refused by name, the recut is the road");
                return false;
            }
            Some(m) if m == PRE_LINE_MAGIC => {
                eprintln!("  ledger refused: MYT1, the tries with the line and the eras inside them — refused by name, the recut is the road");
                return false;
            }
            // A REFUSAL NAMES WHAT IT READ, not only what it wanted: a tape
            // that is not a ledger at all must not read the same as one an
            // older build wrote.
            found => {
                let found = match found {
                    Some(m) => match std::str::from_utf8(m) {
                        Ok(s) => format!("`{s}`"),
                        Err(_) => format!("{m:02x?}, which is not text"),
                    },
                    None => format!("{} bytes, too short to carry one", b.len()),
                };
                eprintln!("  ledger refused: the magic is {found}, not MYT2");
                return false;
            }
        }
        // WHERE THE READ IS. Set as each trie opens, so a refusal names one
        // of the tape's own tries and not a byte offset alone.
        let at = std::cell::Cell::new(COURT_TRIE);
        let mut pos = 4usize;
        let load = |kernel: &mut Kernel, b: &[u8], pos: &mut usize| -> Option<()> {
            // THE ACT'S ROOT (65.5): the restore is one act, and its one
            // stroke is what a ruling re-heard on a filed stamp draws on.
            let mut budget = kernel.physics.stroke();
            // THE COURT: one node, the key this court signs as.
            let court = take_trie(b, pos, COURT_TRIE, &[LEAF_HEAD, LEAF_ROOT, LEAF_LINE, LEAF_DOMAINS, LEAF_LEDGERS, LEAF_CHALLENGES])?;
            let nodes: Vec<(Vec<u8>, crate::snapshot::Leaves)> =
                court.scoped().map(|(k, l)| (k, l.clone())).collect();
            let [(vk, leaves)] = nodes.as_slice() else { return None };
            let vk: [u8; 32] = vk.as_slice().try_into().ok()?;
            let head: [u8; 32] = leaf(leaves, LEAF_HEAD)?.try_into().ok()?;
            // 78.120 — the domains founded here, each with its tier keys; a home
            // written before any domain (or before 78.120) carries none.
            if let Some(v) = leaf(leaves, LEAF_DOMAINS) {
                let domains = whole(v, |b, p| {
                    let n = crate::transducer::address::take_key_count(b, p)?;
                    (0..n)
                        .map(|_| {
                            let w: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                            *p += 32;
                            Some((w, crate::anastome::dialogue::domain::take_keys(b, p)?))
                        })
                        .collect::<Option<Vec<_>>>()
                })?;
                for (w, tiers) in domains {
                    kernel.hold_domain(w, tiers);
                }
            }
            // 78.123 B — the ledger trees; a home written before any chained
            // fold carries none.
            if let Some(v) = leaf(leaves, LEAF_LEDGERS) {
                kernel.ledgers = whole(v, super::super::ledger::Ledgers::take)?;
            }
            // 78.125 — the challenge windows; a home written before 2(c)
            // carries none.
            if let Some(v) = leaf(leaves, LEAF_CHALLENGES) {
                let ledgers = &mut kernel.ledgers;
                whole(v, |b, p| ledgers.take_challenges(b, p))?;
            }
            // THE LINE THIS SNAPSHOT STANDS ON: in it, with the root it was
            // folded to; or on disk, held already, which must hold the stamp
            // the snapshot names — or have folded past it.
            let line_held = kernel.line_held;
            let root = match (line_held, leaf(leaves, LEAF_ROOT), leaf(leaves, LEAF_LINE)) {
                (false, Some(v), None) => Some(whole(v, crate::chronometry::Root::read)?),
                (false, None, None) => None,
                (true, None, named) => {
                    if !kernel.chain.bind_vk(vk) {
                        return None;
                    }
                    if let Some(v) = named {
                        let (stood, hash) = whole(v, |b, p| {
                            let at = Stamp::take_canonical(b, p)?;
                            let hash: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                            *p += 32;
                            Some((at, hash))
                        })?;
                        let held = kernel.chain.node(&stood).map(|r| r.chain_hash) == Some(hash);
                        let folded = kernel.chain.root().is_some_and(|r| stood.place() <= r.at);
                        if !held && !folded {
                            eprintln!("  ledger refused: the line does not hold the stamp the snapshot stood at");
                            return None;
                        }
                    }
                    kernel.chain.root()
                }
                _ => return None,
            };
            // T2 — THE LEDGER, by stamp: the rows, the settlements and the
            // transitions standing at it, and its receipt where the line is
            // in the snapshot.
            at.set(LEDGER_TRIE);
            let ledger = take_trie(b, pos, LEDGER_TRIE, &[LEAF_RECEIPT, LEAF_ENTRIES, LEAF_SETTLEMENTS, LEAF_DIALECT])?;
            let origin = Stamp::new(0, vk, crate::chronometry::Place::GENESIS);
            let mut receipts: Vec<KStepReceipt> = Vec::new();
            let mut entries: Vec<crate::economy::Entry> = Vec::new();
            let mut settled: Vec<crate::anastome::dialogue::Settlement> = Vec::new();
            let mut dialect: Vec<(Vec<u8>, element::Versor, Option<Stamp>)> = Vec::new();
            for (key, leaves) in ledger.scoped() {
                let stamp = Stamp::from_key(&key)?;
                let receipt = match (line_held, leaf(leaves, LEAF_RECEIPT)) {
                    (false, Some(v)) => {
                        let r = whole(v, KStepReceipt::read)?;
                        if r.stamp != stamp {
                            return None;
                        }
                        Some(r)
                    }
                    // A receipt has one home: the line on disk, where one is.
                    (true, Some(_)) => return None,
                    (_, None) => None,
                };
                // THE HASH A ROW AT THIS STAMP MAY CARRY: the receipt's
                // where a node is held there; the root's at the fold root;
                // the empty hash at the pre-genesis origin. A bare node at a
                // folded act is read against its stamp alone: its receipt
                // is cold, in the era that archived it.
                let hash = match &receipt {
                    Some(r) => Some(r.chain_hash),
                    None => kernel.chain.node(&stamp).map(|r| r.chain_hash),
                };
                if hash.is_none() && stamp.vk != vk {
                    return None;
                }
                let at_root = root.is_some_and(|r| Stamp::new(r.height, vk, r.at) == stamp);
                let row_stands = |e: &crate::economy::Entry| -> bool {
                    e.at == stamp
                        && match (hash, at_root) {
                            (Some(h), _) => e.receipt == h || (stamp == origin && e.receipt == [0; 32]),
                            (None, true) => root.is_some_and(|r| r.hash == e.receipt),
                            (None, false) => stamp != origin || e.receipt == [0; 32],
                        }
                };
                if let Some(v) = leaf(leaves, LEAF_ENTRIES) {
                    for e in records(v, crate::economy::Entry::read)? {
                        if !row_stands(&e) {
                            return None;
                        }
                        entries.push(e);
                    }
                }
                if let Some(v) = leaf(leaves, LEAF_SETTLEMENTS) {
                    settled.extend(records(v, |b, p| {
                        let listing = Address::from(crate::snapshot::take_key(b, p)?);
                        let chain = crate::anastome::dialogue::read_chain(b, p)?;
                        let voice: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                        *p += 32;
                        let cites = crate::anastome::dialogue::read_cites(b, p)?;
                        let before = take_mu(b.get(*p..*p + 32)?)?;
                        let after = take_mu(b.get(*p + 32..*p + 64)?)?;
                        *p += 64;
                        Some(crate::anastome::dialogue::Settlement { listing, chain, voice, cites, before, after, at: stamp })
                    })?);
                }
                if let Some(v) = leaf(leaves, LEAF_DIALECT) {
                    dialect.extend(records(v, |b, p| {
                        let key = crate::snapshot::take_key(b, p)?;
                        let g = element::Versor::from_bytes(b, p)?;
                        Some((key, g, Some(stamp)))
                    })?);
                }
                if let Some(r) = receipt {
                    receipts.push(r);
                }
            }
            // The line in the snapshot, rehydrated whole; the line on disk is
            // already held.
            let chain = match line_held {
                true => std::mem::take(&mut kernel.chain),
                false => crate::chronometry::ReceiptChain::from_nodes(root, vk, receipts)?,
            };
            // T1 — THE WORDS, by citation.
            at.set(WORDS_TRIE);
            let words = take_trie(
                b,
                pos,
                WORDS_TRIE,
                &[
                    LEAF_HELD,
                    LEAF_DISCOVERER,
                    LEAF_PROVISIONAL,
                    LEAF_AWARD,
                    LEAF_DEFLATION,
                    LEAF_FIELD_HOT,
                    LEAF_FIELD_COLD,
                    LEAF_RANK,
                    LEAF_RULED,
                ],
            )?;
            at.set(crate::economy::persist::STAKES_TRIE);
            let (stakes, balances, held) = crate::economy::Treasury::take_tries(b, pos)?;
            at.set(ERAS_TRIE);
            let eras = take_trie(
                b,
                pos,
                ERAS_TRIE,
                &[
                    LEAF_ROOT_HASH,
                    LEAF_WITNESS,
                    LEAF_SIG_HOME,
                    LEAF_SIG_WITNESS,
                    LEAF_IDENTITIES,
                    LEAF_FIRST,
                    LEAF_COUNT,
                    LEAF_PAYLOAD,
                ],
            )?;
            at.set(COMPLEXES_TRIE);
            let complexes = take_trie(b, pos, COMPLEXES_TRIE, &[LEAF_TRIES])?;
            at.set(COCYCLES_TRIE);
            let cocycles =
                take_trie(b, pos, COCYCLES_TRIE, &[LEAF_FROM, LEAF_TO, LEAF_CLASS, LEAF_GAUGE, LEAF_STOOD])?;
            at.set(WINDINGS_TRIE);
            let windings = take_trie(b, pos, WINDINGS_TRIE, &[LEAF_WINDING])?;
            at.set(LIE_TRIE);
            let lie = take_trie(b, pos, LIE_TRIE, &[LEAF_BASIS])?;
            if *pos != b.len() {
                return None;
            }

            kernel.court_head = head;
            kernel.chain = chain;
            kernel.treasury = crate::economy::Treasury::from_parts(stakes, balances, held, entries);
            kernel.settlements_cold = settled;
            kernel.dialect_chain = dialect;

            // THE WORDS, applied. Each node's element is read once and its
            // citation re-derived from it: a key that is not its element's
            // citation refuses.
            at.set(WORDS_TRIE);
            let mut by_key: Keyed<(Address, Element)> = Keyed::new();
            let mut provisional: Vec<(Address, Vec<u8>)> = Vec::new();
            let mut ranks: Vec<(Address, crate::transducer::stalk::Invariant, Option<Vec<u8>>)> = Vec::new();
            let mut awards: Vec<(Address, crate::economy::escrow::Escrowed)> = Vec::new();
            let mut federation: Keyed<(Address, Element)> = Keyed::new();
            for (key, leaves) in words.scoped() {
                let Cite::Word { at: admitted, .. } = whole(&key, Cite::take_canonical)? else { return None };
                let held = leaf(leaves, LEAF_HELD)?;
                let [members, record @ ..] = held else { return None };
                let elem = whole(record, Element::from_bytes)?;
                let mut again = Vec::new();
                Cite::word(&elem, admitted).put_canonical(&mut again)?;
                if again != key {
                    return None;
                }
                let addr = elem.address();
                by_key.insert(&key, (addr.clone(), elem.clone()));
                if members & MEMBER_SPOKEN != 0 {
                    // A sentence docketed under another address: its stamp is
                    // that address's, filed where the address's node is read.
                    continue;
                }
                kernel.file_admitted(&addr, admitted, &mut budget);
                // The words by fiber, rebuilt from T1's own key; and RM1's
                // remainders, rebuilt from the element's torsion record.
                kernel.file_fiber(&addr, admitted, key.clone());
                kernel.file_remainders(&addr, elem.remainders().map(|r| r.index_key()).collect());
                if members & MEMBER_BASIS != 0 {
                    let who: [u8; 32] = leaf(leaves, LEAF_DISCOVERER)?.try_into().ok()?;
                    kernel.basis_insert(elem.clone(), who);
                }
                if members & MEMBER_LISTING != 0 {
                    kernel.on_page.insert(&addr, ());
                    kernel.listing_atlas.admit(&crate::transducer::stalk::invariant_of(&elem), &addr);
                    kernel.pending.insert(&addr, elem.clone());
                }
                if members & MEMBER_CAVITY != 0 {
                    kernel.listing_atlas.admit(&crate::transducer::stalk::invariant_of(&elem), &addr);
                    kernel.cavity_book.insert(&addr, elem.clone());
                }
                if members & MEMBER_FEDERATION != 0 {
                    // THE FEDERATION'S ORDER IS RE-DERIVED from the stamps
                    // that ruled each word in (78.27 (3)).
                    let ruled = whole(leaf(leaves, LEAF_RULED)?, Stamp::take_canonical)?;
                    let mut order = ruled.key();
                    order.extend_from_slice(addr.as_bytes());
                    federation.insert(order, (addr.clone(), elem.clone()));
                    kernel.fed_at.insert(&addr, ruled);
                }
                if let Some(v) = leaf(leaves, LEAF_PROVISIONAL) {
                    provisional.push((addr.clone(), v.to_vec()));
                }
                if let Some(v) = leaf(leaves, LEAF_AWARD) {
                    if let Some(award) = whole(v, crate::economy::persist::take_award)? {
                        awards.push((addr.clone(), award));
                    }
                }
                if let Some(v) = leaf(leaves, LEAF_DEFLATION) {
                    kernel.deflations.insert(&addr, take_mu(v)?);
                }
                if let Some(v) = leaf(leaves, LEAF_FIELD_HOT) {
                    kernel.field.insert(&addr, whole(v, take_field)?);
                }
                if let Some(v) = leaf(leaves, LEAF_FIELD_COLD) {
                    kernel.field_cold.insert(&addr, whole(v, take_field)?);
                }
                if let Some(v) = leaf(leaves, LEAF_RANK) {
                    let (inv, closure) = whole(v, |b, p| {
                        let inv = crate::transducer::stalk::Invariant::take_canonical(b, p)?;
                        let tag = *b.get(*p)?;
                        *p += 1;
                        let closure = match tag {
                            0 => None,
                            1 => Some(crate::snapshot::take_key(b, p)?),
                            _ => return None,
                        };
                        Some((inv, closure))
                    })?;
                    ranks.push((addr.clone(), inv, closure));
                }
            }
            for (addr, key) in provisional {
                let (_, spoken) = by_key.get(&key)?.clone();
                kernel.provisional.insert(&addr, spoken);
            }
            crate::snapshot::persist(&federation, |_, (addr, elem)| {
                kernel.court_fed_index.insert(addr, kernel.court_federation.len());
                kernel.court_federation.push((addr.clone(), elem.clone()));
            });
            for (addr, award) in awards {
                kernel.treasury.restore_award(&addr, award);
            }
            // THE STAMP ALIASES, re-derived from the ledger's peer nodes
            // against the words just restored — never persisted. Only a node
            // that admits a word at its own stamp can alias one, and only
            // those are copied out of the ledger.
            let admits = |r: &KStepReceipt| r.cites.iter().any(|c| matches!(c, Cite::Word { at, .. } if *at == r.stamp));
            let peers: Vec<KStepReceipt> = kernel.chain.admitted().into_iter().filter(|r| admits(r)).collect();
            for r in &peers {
                kernel.alias_admitting(r, &mut budget);
            }
            // THE RANK, its closures resolved by one T1 read each (78.27):
            // the prior measurement, continued incrementally and never read
            // as current. Its ordinal is a reading of the basis just
            // restored, re-derived, never persisted.
            let mut rank = Rank::default();
            rank.basis_gen = u64::MAX;
            rank.basis_seq = kernel.language().next_seq;
            rank.retire_gen = kernel.retire_gen;
            for (addr, inv, closure) in ranks {
                let closed = match closure {
                    Some(key) => Some(by_key.get(&key)?.0.clone()),
                    None => None,
                };
                rank.put_closure(&addr, closed);
                rank.invariants.insert(&addr, inv);
            }
            kernel.closure = (u64::MAX, u64::MAX, std::sync::Arc::new(rank));

            // T3 — THE ERAS HELD IN HAND, each by its root's stamp: the
            // root's height, signer and place read off the key, the rest and
            // the payload off its leaves; each held at its place in the
            // archive, beside those the cold segment carries.
            at.set(ERAS_TRIE);
            for (key, leaves) in eras.scoped() {
                let root_at = Stamp::from_key(&key)?;
                let root = crate::chronometry::Root {
                    at: root_at.place(),
                    hash: leaf(leaves, LEAF_ROOT_HASH)?.try_into().ok()?,
                    height: root_at.h,
                };
                let era = crate::chronometry::Era {
                    root,
                    signer: root_at.vk,
                    witness: leaf(leaves, LEAF_WITNESS)?.try_into().ok()?,
                    sig_home: leaf(leaves, LEAF_SIG_HOME)?.try_into().ok()?,
                    sig_witness: leaf(leaves, LEAF_SIG_WITNESS)?.try_into().ok()?,
                    identities: u64::from_le_bytes(leaf(leaves, LEAF_IDENTITIES)?.try_into().ok()?),
                    first: whole(leaf(leaves, LEAF_FIRST)?, crate::chronometry::Place::take_canonical)?,
                    count: u64::from_le_bytes(leaf(leaves, LEAF_COUNT)?.try_into().ok()?),
                    at: None,
                    receipts: whole(leaf(leaves, LEAF_PAYLOAD)?, crate::chronometry::Era::read_payload)?,
                };
                kernel.hold_filed(era);
            }
            // T3 — THE COMPLEXES, each its own tries. A complex that does not
            // read is named by its key, so the refusal locates the object and
            // not only the trie it stood in.
            at.set(COMPLEXES_TRIE);
            for (key, leaves) in complexes.scoped() {
                let Some(c) = leaf(leaves, LEAF_TRIES).and_then(|v| whole(v, crate::transducer::cech::Complex::take_tries))
                else {
                    eprintln!("  ledger refused: the complex at {:02x?} does not read whole", key);
                    return None;
                };
                kernel.complexes.insert(Address::from(key), c);
            }
            // T3 — THE COCYCLE ROWS: the class read back from its inline
            // coordinates, and the receipt each row stood at.
            at.set(COCYCLES_TRIE);
            for (key, leaves) in cocycles.scoped() {
                let row = crate::cognition::CocycleRow {
                    gauge: whole(leaf(leaves, LEAF_GAUGE)?, element::Versor::from_bytes)?,
                    a: leaf(leaves, LEAF_FROM)?.try_into().ok()?,
                    b: leaf(leaves, LEAF_TO)?.try_into().ok()?,
                    class_key: whole(leaf(leaves, LEAF_CLASS)?, crate::transducer::alphabet::Rotor::take_class_key)?,
                };
                if let Some(v) = leaf(leaves, LEAF_STOOD) {
                    kernel.cocycle_receipt.insert(&key, whole(v, Stamp::take_canonical)?);
                }
                // Chart index is rebuilt on next glue; hot row is enough.
                kernel.cocycle_at.insert(Address::from(key), row);
            }
            // 78.68 — THE WINDING CARRIERS, each read whole or the snapshot
            // refuses: the pay they hold is a claim.
            at.set(WINDINGS_TRIE);
            for (key, leaves) in windings.scoped() {
                let w = whole(leaf(leaves, LEAF_WINDING)?, crate::cognition::court::winding::Winding::take)?;
                kernel.windings.insert(&key, w);
            }
            at.set(LIE_TRIE);
            for (key, leaves) in lie.scoped() {
                let basis = whole(leaf(leaves, LEAF_BASIS)?, |v, p| {
                    let n = usize::try_from(u64::from_le_bytes(v.get(*p..*p + 8)?.try_into().ok()?)).ok()?;
                    *p += 8;
                    let mut out = Vec::new();
                    for _ in 0..n {
                        out.push(crate::transducer::cga::Mv::from_bytes(v, p)?);
                    }
                    Some(out)
                })?;
                kernel.lie_basis.insert(&key, basis);
            }

            // index: keyed field at restore (persistence snapshot, once)
            let field_keys: Vec<Vec<u8>> = kernel.field.scoped().map(|(k, _)| k).collect();
            for addr in field_keys {
                let Some(listing) = kernel.field.get(&addr).cloned() else { continue };
                let mag = element::measure_lanes(&union_remainders(&listing.entries));
                let lanes = crate::network::nerve::occupied_lanes(&mag);
                for e in &listing.entries {
                    if !e.state.is_flat() {
                        kernel.listing_atlas.admit_also(&e.chart, &addr);
                        for lane in &lanes {
                            kernel.own_cover.admit(*lane, &e.chart, &addr);
                        }
                        let ck = e.chart.key();
                        match kernel.chart_listings.get_mut(&ck) {
                            Some(m) => {
                                m.insert(&addr, ());
                            }
                            None => {
                                let mut m = crate::network::nerve::Keyed::new();
                                m.insert(&addr, ());
                                kernel.chart_listings.insert(ck, m);
                            }
                        }
                    }
                }
                if listing.nonflat > 0 {
                    kernel.open.insert(&addr, ());
                    kernel.tension.insert(&addr, mag);
                } else {
                    kernel.field.remove(&addr);
                    kernel.field_cold.insert(&addr, listing);
                    kernel.tension.insert(&addr, [0; 4]);
                }
            }
            Some(())
        };
        if load(self, b, &mut pos).is_none() {
            eprintln!(
                "  ledger refused in trie {} at byte {pos} of {}: the court starts clean",
                at.get(),
                b.len()
            );
            self.basis_clear();
            return false;
        }
        self.occupancy = [0; 4];
        self.drawers = crate::network::nerve::Keyed::new();
        self.drawn = crate::network::nerve::Keyed::new();
        self.t_leases = crate::network::nerve::Keyed::new();
        self.lane_listings = Default::default();
        let pending_addrs: Vec<Address> = self
            .pending
            .scoped()
            .map(|(_, e)| e.address())
            .collect();
        for addr in pending_addrs {
            if self.standing_h.get(&addr).is_none() {
                self.standing_h.insert(&addr, self.height());
            }
            // Restored, it stands at this orbit, filed where it lapses by the
            // stamp that admitted it.
            if let Some(at) = self.stamp_key(addr.as_bytes()) {
                self.file_flux(&at);
            }
        }
        // Each restored provisional word's orbit opens again: it is in flight,
        // filed where it lapses, and offered by the next transition taken.
        let prov: Vec<Vec<u8>> = self.provisional.scoped().map(|(a, _)| a).collect();
        for addr in prov {
            self.stamp_standing(&addr);
            if let Some(at) = self.stamp_key(&addr) {
                self.file_flux(&at);
            }
            self.enter_offered(&addr);
        }
        // Ruled cover is the lettered federation, not the leaked basis.
        self.ruled = crate::transducer::stalk::Stalks::over_elements(
            self.court_federation.iter().filter_map(|(_, w)| {
                if w.is_connection() && w.letters().is_none() {
                    None
                } else {
                    Some(w)
                }
            }),
        );
        let dropped = self.prune_basis_to_ruled();
        if dropped > 0 {
            eprintln!("  pruned {dropped} leaked basis words; ruled cover remains");
        }
        self.refill_axis_index();
        // A snapshot without DEF1 (or a zero record) re-measures from the
        // spoken word: presence on the docket is not Δ⃗ = 0.
        let need: Vec<(Address, [u64; 4])> = self
            .provisional
            .scoped()
            .filter_map(|(addr, spoken)| {
                if self.deflations.get(&addr).is_some_and(|d| d.iter().any(|&x| x > 0)) {
                    return None;
                }
                // The listing's own magnitude where the book still holds it,
                // the spoken word's where it does not. Both are measured on
                // a word; neither stands in for the other (rule 11).
                let mag = match self.book_elem(&addr) {
                    Some(p) => p.magnitude(),
                    None => spoken.magnitude(),
                };
                mag.iter().any(|&x| x > 0).then(|| (Address::from(addr.clone()), mag))
            })
            .collect();
        for (addr, mag) in need {
            if mag.iter().any(|&x| x > 0) {
                self.deflations.insert(addr, mag);
            }
        }
        // L15: the depth index is DERIVED and never persisted, so a
        // restored court reads it back off its own chain — the receipts
        // it wrote and the peer receipts it admitted — exactly as it read
        // them when they arrived. Left out, every restored court read
        // depth zero for every address it had ever ruled.
        // The peers' folded lines leave first, once the words they are kept
        // for are restored and aliased (45.8).
        self.shed_folded_lines();
        self.rebuild_attestors();
        // Unruled connections that survived the prune re-enter the docket.
        let held: Vec<Element> = self
            .basis
            .scoped()
            .filter(|(_, w)| w.is_connection())
            .map(|(_, w)| w.element().clone())
            .collect();
        // EACH CONNECTION RE-ENTERS UNDER ITS OWN DISCOVERER, read by its
        // address. This took `discoverers.scoped().next()` — the first entry
        // of the trie — and docketed EVERY unruled connection under that one
        // voice, so a boot reattributed the whole docket to whoever happened
        // to sort first, and `settle_use` pays a discoverer. Which voice that
        // was is an insertion order in a trie (rule 15), and provenance is
        // not a thing a carrier's shape may decide.
        //
        // A connection whose discoverer this court does not hold is not
        // docketed under someone else's name: it is left, which is absence
        // and not a default.
        for w in held {
            if self.ruled_contains(&w.address()) {
                continue;
            }
            let Some(ratifier) = self.discoverer_of(&w.address()) else {
                continue;
            };
            self.docket_connection(&w, ratifier);
        }
        true
    }
}

/// A COUNT THAT CANNOT BE SATISFIED, refused against the tape's own
/// remaining bytes and nothing else. Every record writes at least one
/// byte, so a count larger than what is left cannot be honoured — which is
/// EXACT and derived from the tape, not estimated from a record's size.
fn take_count(b: &[u8], p: &mut usize) -> Option<usize> {
    let n = usize::try_from(u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?)).ok()?;
    *p += 8;
    match n > b.len().checked_sub(*p)? {
        true => None,
        false => Some(n),
    }
}

/// The inverse of `write::put_field`: a listing's field, or a refusal. A
/// closure tag that is none of the four refuses the leaf; it was read as a
/// cavity.
fn take_field(b: &[u8], p: &mut usize) -> Option<ListingField> {
    use crate::transducer::stalk::Closure;
    let ne = take_count(b, p)?;
    let mut entries = Vec::new();
    for _ in 0..ne {
        let chart = crate::transducer::stalk::Invariant::take_canonical(b, p)?;
        let tag = *b.get(*p)?;
        *p += 1;
        let state = match tag {
            0 => Closure::Flat,
            1 => Closure::Twisted(Box::new(element::Versor::from_bytes(b, p)?)),
            2 => Closure::Cavity,
            3 => {
                let d = crate::transducer::address::take_key_count(b, p)?;
                Closure::Class(Box::new(crate::transducer::alphabet::Rotor::from_bytes(b, p)?), d)
            }
            _ => return None,
        };
        // A remainder cell is its row, column and two hues.
        let nc = take_count(b, p)?;
        let mut remainder = Vec::new();
        for _ in 0..nc {
            let r = i64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            let c = i64::from_le_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
            let hb = *b.get(*p + 16)?;
            let ha = *b.get(*p + 17)?;
            *p += 18;
            remainder.push((r, c, hb, ha));
        }
        let mut stagnant = [0u64; 4];
        for s in &mut stagnant {
            *s = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
        }
        entries.push(FieldEntry { chart, state, remainder, stagnant });
    }
    // A count that does not fit its carrier refuses the leaf (STD1): the
    // `as u32` this was truncated it — the same defect at a second site.
    let nonflat = u64::try_from(entries.iter().filter(|e| !e.state.is_flat()).count()).ok()?;
    Some(ListingField { entries, nonflat })
}
