// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 23.8, 26 — THE JUDGE OFF THE LOCK: probe, quote, consolidate and adopt.
//! Each replays the voice's own proof over the language snapshot with no
//! lock held and settles under a brief one. The percept and the demand
//! that were judged here carried a voice's painted sites to the court (law 8,
//! ruled 2026-09-10); a voice speaks sentences, heard by `perceive_speech`.

use crate::arc::Grid;

/// The family's measured frame ratio — perception, identical across pairs
/// or refused (never rounded), measured on the common refinement.
pub fn family_ratio(train: &[(Grid, Grid)]) -> Option<((usize, usize), (usize, usize))> {
    crate::transducer::family::refine::family_ratio(train)
}

/// Work items for the server's deliberation worker — everything that would
/// hold the kernel lock through deep reduction runs here, behind the
/// bounded queue, never on the request path.
pub enum Deliberation {
    /// Genome synchronization: a court-ratified word to verify and install
    /// (the ratifying voice is the provenance of record).
    Adopt([u8; 32], crate::transducer::element::Element),
}

/// THE PROBE, off the lock: the sentence replays over the language snapshot
/// against the listing's chain; the settlement (closure or graded residual)
/// runs under a brief lock. Returns (code, paid, remaining).
pub fn probe(
    k: &std::sync::Mutex<crate::cognition::Kernel>,
    voice: [u8; 32],
    target: &[u8],
    cites: &[crate::anastome::dialogue::CiteWire],
) -> (u8, [u64; 4], [u64; 4]) {
    use crate::anastome::dialogue::PROBE_NOTHING;
    let Ok(mut g) = k.lock() else { return (PROBE_NOTHING, [0; 4], [0; 4]) };
    let view = g.market_open();
    drop(g);
    // A TARGET MAY NAME ONE DEMAND VERTEX of the listing (L8q). The
    // listing it stands on is what the book is asked for; the settlement
    // is the court's, under the lock, because it speaks a form.
    let at = match crate::transducer::sentence::read_demand_at(target) {
        Some((at, _)) => at,
        None => target.to_vec(),
    };
    let Some(listing) = view.pending.get(&at) else {
        return (crate::anastome::dialogue::PROBE_SETTLED, [0; 4], [0; 4]);
    };
    // The utterance is verified as spoken — its operators against the
    // listing — never against this hypha's copies of the words.
    let residual = crate::transducer::sentence::verify_utterance(cites, listing.sub());
    // The reading of a sentence that covered nothing: how many carried
    // links landed on the listing at all, and how many landed outside it.
    let mut listed = listing.sub().to_vec();
    listed.sort_unstable();
    if residual == listed {
        let (mut on, mut off) = (0usize, 0usize);
        for crate::anastome::dialogue::CiteWire { transport: v, body, .. } in cites {
            for &(r, c, _, _) in body {
                match crate::transducer::element::carry_center(v, r, c, 0) {
                    Some(p) => match crate::transducer::element::site_of(&p) {
                        // A centre off the lattice, or off the listing's own
                        // frame, is off the listing.
                        None => off += 1,
                        Some((tr, tc, 0)) => {
                            match listed.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))) {
                                Ok(_) => on += 1,
                                Err(_) => off += 1,
                            }
                        }
                        Some(_) => off += 1,
                    },
                    None => off += 1,
                }
            }
        }
        // The cited bodies are measured WHOLE. "first body" reported the
        // length of whichever cite came first (rule 15) — a number that
        // named no cite in particular and said nothing about the others.
        eprintln!(
            "  nothing: listing {} links, {} cites, carried links on {on} off {off}, cited links {}",
            listed.len(),
            cites.len(),
            cites.iter().map(|c| c.body.len()).sum::<usize>(),
        );
    }
    let section = cites.iter().any(|c| c.gauge == u32::MAX);
    let Ok(mut g) = k.lock() else { return (PROBE_NOTHING, [0; 4], [0; 4]) };
    // The court replays the settlement on the chains it is handed, so the
    // measure spoken here is the chain's own, not the listing's fibre.
    let before = crate::transducer::element::measure_lanes(listing.sub());
    let settled = g.settle_probe(voice, target, residual, cites, section);
    if matches!(settled.0, crate::anastome::dialogue::PROBE_CLOSED | crate::anastome::dialogue::PROBE_RESIDUAL) {
        g.puncture_from(cites); // the fiber deepens on what was verified
        // The settlement goes to the court with the utterance that made it,
        // for the court to replay and measure itself.
        g.record_settlement(crate::transducer::address::Address::from(target), listing.sub().to_vec(), voice, cites, before, settled.2);
    }
    settled
}

/// WHAT A QUOTE READS OFF THE LOCK.
pub struct Quoted {
    /// The yield surface, per lane, read off genuine metabolic tension.
    pub surface: [u64; 4],
    /// The items served, each with the charts it leaves open.
    pub priced: Vec<crate::cognition::Priced>,
    /// The K-step this quote stands at.
    pub k_step: u64,
    /// The cohesion reading (C, I): closure work and coverage work, two
    /// lanes of one object, returned exactly and never compared.
    pub cohesion: (u64, u64),
    /// The height the book stands at: the chain's head height (45.2),
    /// never the book generation (I39).
    pub height: u64,
}

/// THE QUOTE, off the lock: incidence of the voice's charts against the
/// listing atlas. The page width is the body budget. Returns (surface,
/// priced items with open charts, K, cohesion).
pub fn quote(
    k: &std::sync::Mutex<crate::cognition::Kernel>,
    voice: [u8; 32],
    charts: &[crate::transducer::stalk::Invariant],
) -> Option<Quoted> {
    let view = k.lock().ok()?.market_open();
    let sealed = view.closure.is_some();
    let rank = match view.closure {
        Some(c) => c,
        None => {
            let measured = crate::cognition::rank_over(
                &view.language,
                &view.pending,
                &view.prior,
                view.gens.0,
                &view.chart_listings,
            );
            k.lock().ok()?.seal_closure(view.gens, measured)
        }
    };
    eprintln!(
        "  rank: book {} listings, closed {}, sealed {sealed}, gens {:?}",
        view.pending.len(),
        // The cohesion reading's own closed count (law 12): this filtered
        // the closure map itself, a second read of the book for a number
        // `cohesion_over` already names and `Rank` now carries.
        crate::cognition::cohesion_over(&rank).0,
        view.gens,
    );
    let budget = crate::network::wire::frame_budget() - 34;
    // NO ESCROW POOL, NO QUOTE. The pool gates whether the book is quotable
    // at all; it does not bound the scale of what is served, which is the
    // body budget's job.
    view.pool.as_ref()?;
    let (surface, priced) = {
        let g = k.lock().ok()?;
        crate::cognition::quote_incidence(
            &crate::cognition::BookView {
                pending: &view.pending,
                listing_atlas: g.listing_atlas(),
                field: g.field(),
                on_page: g.on_page(),
                spoken: g.spoken(),
                standing_t: &view.standing_t,
            },
            &crate::cognition::Ask {
                charts,
                surface: &view.surface,
                pressure: &view.pressure,
                budget,
                voice: &voice,
            },
        )
    };
    let cohesion = crate::cognition::cohesion_over(&rank);
    Some(Quoted {
        surface,
        priced,
        k_step: view.head_k,
        cohesion,
        height: view.head_height,
    })
}

/// Idle consolidation, orchestrated off the request path: the cheap moves
/// run under a brief lock, the redundancy probe decomposes the candidate
/// over a basis snapshot with no lock held, and only the retirement
/// re-locks. Returns the cards retired.
pub fn consolidate(k: &std::sync::Mutex<crate::cognition::Kernel>) -> usize {
    let Ok(mut g) = k.lock() else { return 0 };
    g.consolidate_stroke();
    0
}

/// Adoption, orchestrated off the request path: the blind replay is pure
/// (no lock), derivability probes a basis snapshot (no lock), and only the
/// brief install re-locks. Returns the deterministic [`Adoption`] status.
pub fn adopt(
    k: &std::sync::Mutex<crate::cognition::Kernel>,
    ratifier: [u8; 32],
    word: &crate::transducer::element::Element,
) -> crate::cognition::Adoption {
    use crate::cognition::Adoption;
    // A form cites ruled forms and is bound to a family: its replay reads
    // the cover and the trains, so it takes the lock (23.8); every other
    // word replays pure, off the lock.
    let refused = if word.is_sentence() {
        let Ok(g) = k.lock() else { return Adoption::Refused("kernel unavailable") };
        g.court_replay(word)
    } else {
        crate::verdict::blind_replay(word)
    };
    if let Some(reason) = refused {
        return Adoption::Refused(reason);
    }
    let Ok(mut g) = k.lock() else { return Adoption::Refused("kernel unavailable") };
    if g.word_at(&word.address()).is_some() {
        // Held at this address: the arriving placement's fibre merges
        // (Known or Enriched — the install door decides).
        return g.install_adopted(word, ratifier);
    }
    let language = g.language();
    drop(g);
    // Derivability on the snapshot: a word the language already composes
    // is not installed twice.
    let sub = word.substance();
    let sentence = crate::transducer::sentence::deconstruct_at(
        &|a| language.word_at(a),
        &language.stalks,
        &|a| language.shape_at(a),
        sub.sub(),
        None,
    );
    if let ([], [], [_, ..]) = (sentence.residual.as_slice(), sentence.punctures.as_slice(), sentence.cites.as_slice()) {
        return Adoption::Derivable;
    }
    let Ok(mut g) = k.lock() else { return Adoption::Refused("kernel unavailable") };
    g.install_adopted(word, ratifier)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_canonical;
    use crate::transducer::germinate::germinate;


    /// Adoption is a deterministic status map: a clean foreign word
    /// installs with the ratifier as provenance; re-adoption reads Known;
    /// a word the language already derives reads Derivable; tampered
    /// speech is Refused with the failed axis named.
    #[test]
    fn adoption_maps_every_state_deterministically() {
        use crate::cognition::{Adoption, Kernel};
        let k = std::sync::Mutex::new(Kernel::new());
        let court = [0xAB; 32];
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
        let word = from_canonical(&germinate(&a, &rot).unwrap());
        assert_eq!(adopt(&k, court, &word), Adoption::Installed, "clean foreign word installs");
        assert_eq!(adopt(&k, court, &word), Adoption::Known, "the genome had converged");
        // The identical deformation germinated far away: same substance —
        // the language already derives it.
        let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        let dup = from_canonical(&germinate(&a_far, &rot_far).unwrap());
        assert!(
            matches!(adopt(&k, court, &dup), Adoption::Derivable | Adoption::Known),
            "adoption never adds redundancy"
        );
        // Tampered speech is named and refused.
        let mut fat = word.clone();
        fat.mag = [999, 0, 0, 0];
        assert!(matches!(adopt(&k, court, &fat), Adoption::Refused(_)));
        // Provenance is the ratifying voice, not a sentinel.
        let g = k.lock().unwrap();
        assert!(g.word_at(&word.address()).is_some());
    }

}
