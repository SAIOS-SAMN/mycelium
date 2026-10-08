// 78.64 H1 — THE CARRIER CENSUS: every carrier the court holds, counted by
// its own members.
//
//! The kernel is taken apart field by field, with no rest pattern: a field
//! added to the court is either counted here or named here as what it is —
//! a count, a cursor, a flag, a path — and a field that is neither does
//! not build. A nested carrier is counted by the entries of what it nests.

use super::*;
use crate::network::nerve::Keyed;

/// The entries of every carrier a keyed carrier nests.
fn nested<V>(k: &Keyed<Keyed<V>>) -> usize {
    k.scoped().map(|(_, inner)| inner.len()).sum()
}

/// The members of every list a keyed carrier holds.
fn lists<T>(k: &Keyed<Vec<T>>) -> usize {
    k.scoped().map(|(_, v)| v.len()).sum()
}

impl Kernel {
    /// THE COURT'S CARRIERS AT ITS `K`.
    pub fn carriers(&self) -> crate::observe::Carriers {
        let Kernel {
            basis,
            by_action,
            by_footprint,
            discoverers,
            stalks,
            ruled,
            court_voice: _,
            axis_index,
            replay_memo,
            cited_by,
            book,
            basis_gen: _,
            language_gen: _,
            language,
            book_gen: _,
            closure,
            convergence,
            pending,
            chain,
            archive,
            fold_pending: _,
            court_head: _,
            retire_gen: _,
            pending_parents,
            admitted_at,
            held_at,
            spoken,
            spoken_by,
            quote_frontier,
            tension,
            field,
            on_page,
            flat_orbits,
            listing_atlas,
            cavity_book,
            offered,
            flight,
            emitted,
            lapse_at,
            flux_k,
            offered_to,
            court_federation,
            court_fed_index,
            court_upheld: _,
            court_slashed: _,
            court_demands_listed: _,
            answered,
            complexes,
            unfilled,
            pending_refine,
            heard,
            heard_at,
            cocycle_at,
            cocycle_receipt,
            cocycle_by_chart,
            dialect_chain,
            field_cold,
            settlements_cold,
            height_override: _,
            persist_due: _,
            eras_path: _,
            line_path: _,
            line_held: _,
            eras_end: _,
            lane_cap: _,
            standing_h,
            listing_task,
            task_listings,
            task_family,
            speech_task,
            refusals,
            settlements,
            pressure_gen: _,
            deflations,
            unframed: _,
            withheld: _,
            word_fibers,
            fiber_key_at,
            remainder_fibers,
            remainder_keys,
            remainders_refused: _,
            remainders_past_carrier: _,
            remainders_misspoken: _,
            remainders_unpriced: _,
            remainders_unminted: _,
            remainders_unoffered: _,
            remainders_coincident: _,
            room_broken: _,
            room_overflowed: _,
            overflowed: _,
            refills: _,
            aliases,
            settlements_unplaced: _,
            fed_at,
            rulings: _,
            ruling_refusals: _,
            reopened: _,
            windings,
            winding_tally: _,
            lie_basis,
            witnessed,
            last_transition: _,
            glued_from,
            flux_field,
            released_cells: _,
            multiplicity_condensed: _,
            awaiting_rulings,
            persisted: _,
            optimize_cursor: _,
            syntropic_cursor: _,
            drain_cursor: _,
            quote_cursor: _,
            physics: _,
            acts: _,
            deferred_words,
            taught_voice,
            treasury,
            identity: _,
            genesis_cover: _,
            drawers,
            generators,
            awaiting,
            discovered_by,
            attestors,
            drawn,
            recall_short,
            provisional,
            open,
            open_cursor: _,
            docket_circuits: _,
            phase_mark: _,
            fill_cursor: _,
            refine_cursor: _,
            own_cover,
            chart_listings,
            ruled_addrs,
            ruled_root: _,
            ruled_root_due: _,
            occupancy: _,
            t_leases,
            pressure_sum: _,
            lane_listings,
            evidence_at,
            anchor_mismatches,
            anchor_evidence,
            domains,
            tier_domain,
            ledgers,
            reconstructions,
            handoffs,
            reserves,
            origins,
            universes,
        } = self;
        let mut c = crate::observe::Carriers { k: chain.head_k(), held: Vec::new() };
        // 78.120 — the domains founded here and their tier index.
        c.count("domains", domains.len());
        c.count("tier_domain", tier_domain.len());
        // 78.123 B — the ledger trees: children with value in flight, the
        // rates gauges' steps, the open obstructions.
        c.count("ledger.folds", ledgers.folds.len());
        c.count("ledger.remaining", ledgers.remaining.len());
        c.count("ledger.gauge", ledgers.gauges.scoped().map(|(_, g)| g.steps().len()).sum());
        c.count("ledger.obstructions", ledgers.obstructions.scoped().map(|(_, o)| o.len()).sum());
        // 78.125 — the challenge windows: the folds recorded, the tiers'
        // heads, the edges held, the arbitrages booked.
        c.count("ledger.records", ledgers.records.len());
        c.count("ledger.heads", ledgers.heads.len());
        c.count("ledger.edges", ledgers.edges.scoped().map(|(_, e)| e.len()).sum());
        c.count("ledger.discharged", ledgers.discharged.len());
        c.count("ledger.reconstructions", reconstructions.len());
        // 78.126 — the handoffs held, and the folds' consumed sides.
        c.count("handoffs.held", handoffs.held.len());
        c.count("handoffs.consumed_by", handoffs.consumed_by.len());
        // 78.127 — the conversions held and the domains' reserves.
        c.count("handoffs.conversions", handoffs.conversions.len());
        c.count("reserves", reserves.len());
        // 78.132 — the universes' origins.
        c.count("origins", origins.len());
        // 78.139 — the universes' statements.
        c.count("universes", universes.len());
        c.count("basis", basis.len());
        c.count("by_action", by_action.len());
        c.count("by_footprint", nested(by_footprint));
        c.count("discoverers", discoverers.len());
        c.count("discovered_by", nested(discovered_by));
        c.count("stalks", stalks.len());
        c.count("ruled", ruled.len());
        c.count("axis_index", axis_index.scoped().map(|(_, a)| nested(a)).sum());
        c.count("generators", nested(generators));
        c.count("replay_memo", nested(replay_memo));
        c.count("cited_by", nested(cited_by));
        c.count("book", book.1.len());
        c.count("language", language.len());
        c.count("rank.closure", closure.2.closure_carrier().len());
        c.count("rank.invariants", closure.2.invariants.len());
        c.count("convergence", convergence.held());
        c.count("pending", pending.len());
        chain.census(&mut c);
        c.count("archive", archive.len());
        c.count("pending_parents", pending_parents.len());
        c.count("admitted_at", admitted_at.len());
        c.count("held_at", held_at.len());
        c.count("aliases", lists(aliases));
        c.count("spoken", nested(spoken));
        c.count("spoken_by", nested(spoken_by));
        c.count("quote_frontier", quote_frontier.len());
        c.count("tension", tension.len());
        c.count("field", field.len());
        c.count("field_cold", field_cold.len());
        c.count("on_page", on_page.len());
        c.count("flat_orbits", nested(flat_orbits));
        c.count("listing_atlas", listing_atlas.len());
        c.count("cavity_book", cavity_book.len());
        c.count("offered", nested(offered));
        c.count("offered_to", nested(offered_to));
        c.count("flight", flight.len());
        c.count("emitted", emitted.len());
        c.count("lapse_at", lapse_at.len());
        c.count("flux_k", flux_k.len());
        c.count("court_federation", court_federation.len());
        c.count("court_fed_index", court_fed_index.len());
        c.count("fed_at", fed_at.len());
        c.count("answered", nested(answered));
        c.count("complexes", complexes.scoped().map(|(_, x)| x.held()).sum());
        c.count("unfilled", unfilled.len());
        c.count("windings", windings.len());
        c.count("lie_basis", lie_basis.scoped().map(|(_, b)| b.len()).sum());
        c.count("witnessed", witnessed.len());
        c.count("glued_from", glued_from.len());
        c.count("flux_field.tubes", flux_field.tubes.len());
        c.count("flux_field.orbits", flux_field.orbits.len());
        c.count("pending_refine", pending_refine.len());
        c.count("heard", heard.len());
        c.count("heard_at", nested(heard_at));
        c.count("cocycle_at", cocycle_at.len());
        c.count("cocycle_receipt", cocycle_receipt.len());
        c.count("cocycle_by_chart", nested(cocycle_by_chart));
        c.count("dialect_chain", dialect_chain.len());
        c.count("settlements", settlements.len());
        c.count("settlements_cold", settlements_cold.len());
        c.count("standing_h", standing_h.len());
        c.count("listing_task", listing_task.len());
        c.count("task_listings", lists(task_listings));
        c.count("task_family", lists(task_family));
        c.count("speech_task", speech_task.len());
        c.count("refusals", refusals.len());
        c.count("deflations", deflations.len());
        c.count("word_fibers", word_fibers.len());
        c.count("fiber_key_at", fiber_key_at.len());
        c.count("remainder_fibers", remainder_fibers.len());
        c.count("remainder_keys", lists(remainder_keys));
        c.count("awaiting_rulings", lists(awaiting_rulings));
        c.count("deferred_words", deferred_words.len());
        c.count("taught_voice", taught_voice.len());
        treasury.census(&mut c);
        c.count("drawers", drawers.len());
        c.count("awaiting", lists(awaiting));
        c.count("attestors", nested(attestors));
        c.count("drawn", drawn.len());
        c.count("recall_short", recall_short.len());
        c.count("provisional", provisional.len());
        c.count("open", open.len());
        c.count("own_cover", own_cover.held());
        c.count("chart_listings", nested(chart_listings));
        c.count("ruled_addrs", ruled_addrs.len());
        c.count("t_leases", t_leases.len());
        c.count("lane_listings", lane_listings.iter().map(Keyed::len).sum());
        c.count("evidence_at", evidence_at.len());
        c.count("anchor_mismatches", anchor_mismatches.len());
        c.count("anchor_evidence", anchor_evidence.len());
        c
    }
}
