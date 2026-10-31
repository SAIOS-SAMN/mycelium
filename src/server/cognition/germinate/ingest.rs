// 23.8, 12.6 — THE DOOR AND THE ROOM SWEEP: what the court reads before it
// admits a voice's act, and the redundancy probe the syntropic stroke runs.
//
//! Nothing here composes a sentence (law 8, ruled 2026-09-10): a voice's
//! utterance enters by `perceive_speech` and is listed by `list_sentence`.
//! What stood here — a percept reduced against the language and the commit
//! of a germ — was the court running F on a voice's painted sites, and it is
//! deleted, not refused (50.6, seam 3).

use crate::cognition::*;
use crate::transducer::element::Element;

impl Court {
    /// THE BOOK EXTENDS BEFORE IT REFUSES (12.6, S9.7; Stage 12 S12.5c).
    /// The room sweep before a voice's draw: standing extent that aged out
    /// relocates (T-room), and where the voice's room is still short the
    /// book relocates what no voice can reach — listings stagnant a full
    /// aperture on every occupied lane leave the page for the cavity and
    /// release the draws they held — and the shortfall is read again, per
    /// lane. Only what still does not fit is refused, and the refusal names
    /// the lane. The book's other moves (condense, torse, refine, puncture)
    /// act on the basis and free no room, so they are not run here.
    /// `None` where the voice's room cannot be read: a lane occupied
    /// beyond its cover is a broken count and no shortfall is measured
    /// against it.
    pub(super) fn room_after_extending(
        &mut self,
        voice: &[u8; 32],
        draw: &[u64; 4],
    ) -> Option<[u64; 4]> {
        self.relocate_t_exhausted();
        let short = self.room_shortfall(voice, draw)?;
        if crate::economy::lanes_short(&short).is_empty() {
            return Some(short);
        }
        self.drain_unreachable();
        self.room_shortfall(voice, draw)
    }

    /// THE REFLEX OPENING — under the lock, before an utterance is heard:
    /// the metabolism dissipates its backlog (no voice enrols here, §3.1 as
    /// ruled), the cheap syntropic moves fire in both resting states of the
    /// two-stroke (depleted or idle; the redundancy probe stays with the
    /// idle worker), and the escrow-priced door prices congestion. `Err`
    /// is the door's deferral, naming the lanes it was deferred on (S9.6b):
    /// the honest "come back", which the voice dwells on and speaks again.
    pub fn reflex_open(&mut self, voice: [u8; 32]) -> Result<(), [u64; 4]> {
        self.physics.relax();
        // Nothing funds an arriving identity (MU-ECONOMY §3.1 as ruled
        // 2026-09-09, S13.15): `ensure_bonded` enrolled every new voice at
        // the genesis cover here — a faucet measured as supply three times
        // the seed after two ingests — and an unbonded voice has room for
        // one claim in flight per lane instead (`room_shortfall`). The
        // voice still pays the door below.
        let idle = self.physics.metabolism_rate().iter().all(|&p| p <= 1);
        // A lane short of one expansion, or an idle book, is the syntropic
        // phase: condense and regenerate (S10's reading, S13.2).
        let short = self.physics.reserve_shortfall([true; 4]);
        if !crate::economy::lanes_short(&short).is_empty() || idle {
            self.condense_stroke();
            self.physics.regenerate();
        }
        // The door's reading is per lane (S9.6b): a lane spent to nothing
        // that the speaker paid nothing on defers the act, and the reading
        // names the lanes it was deferred on.
        let deferred = self.door_deferral(voice);
        if !crate::economy::lanes_short(&deferred).is_empty() {
            return Err(deferred);
        }
        Ok(())
    }

    /// The reduction with one generator EXCLUDED from the alphabet — the syntropic
    /// stroke's redundancy probe: "does the REST of the basis already derive this
    /// generator?" (`exclude = None` is the ordinary full reduction.)
    pub(crate) fn reduce_excluding(
        &self,
        g: &Element,
        exclude: Option<&[u8]>,
        budget: &mut crate::physics::Budget,
    ) -> NormalForm {
        // A held residual word is a CENTRED COBOUNDARY CHAIN: the chain
        // reduction decomposes it over the other held words. The group path
        // below remains for pure-algebra elements.
        let sub = g.substance();
        if !sub.sub().is_empty() {
            // The address passes straight through: it was converted to a
            // slot here only because the reduction addressed words by slot.
            return self.reduce_chain(sub.sub().to_vec(), exclude, budget);
        }
        // A12, ONCE. This peeled one generator at a time for up to 64
        // steps, re-deriving the whole question at every step from the new
        // residue -- and each step scanned the basis and kept a running best
        // by `descends`, which is selection by measure (law 10) and dropped
        // every factorisation that did not dominate (rule 15). A12 answers
        // the whole question in one reading: membership in the lattice the
        // held words span, decided by Hermite normal form, with the
        // factorisation as its witness and a canonical residue when it
        // fails. There is nothing left to peel, so the peel is gone.
        let s = sub;
        if s.is_identity() {
            return NormalForm::Decomposed(Vec::new()); // a pure transposition
        }
        // Terminal: the substance IS an installed generator.
        if let Some(k) = action_index_key(&s) {
            if let Some(addr) = self.by_action.get(&k) {
                if exclude.map(|e| e == addr.as_bytes()) != Some(true) {
                    // THE ACTION IS RE-DERIVED, NOT ASSERTED. `by_action`
                    // is an index and the terminal it names must carry the
                    // action being decomposed; `same_action` is that
                    // re-derivation and it stood in a `debug_assert!`,
                    // which the release build the mesh runs does not
                    // perform. A stale or colliding index therefore ended
                    // the decomposition at a word that does not speak the
                    // substance, and the reduction reported Decomposed on
                    // it (law 2). Where it does not hold there is no
                    // terminal here, and the general path below runs.
                    let held = self.basis.get(addr);
                    if held.is_some_and(|h| h.substance().same_action(&s))
                        && self.is_live(addr)
                    {
                        return NormalForm::Decomposed(vec![
                            crate::transducer::address::Address::new(addr.to_vec()),
                        ]);
                    }
                }
            }
        }
        // THE SPELLING IS THE WORD'S, NOT ITS SUBSTANCE'S. `substance()`
        // empties the description tape — that is its job, the address is
        // BASE-only — and `letters()` parses that tape, so read here it
        // answered `None` for every word and this whole A12 path has never
        // run: every pure-algebra element that was not the terminal above
        // reported `Irreducible`. Measured at c1e9e0b: a basis holding `q`
        // and `q²` reported `q²` irreducible.
        //
        // The substance is the right object for the ACTION — the terminal
        // above compares actions and keeps using it — and the wrong one
        // for the SPELLING, which a translation does not change. The basis
        // loop below already reads `gen.letters()` off the held word and
        // not off its substance; this line is the same reading on the
        // candidate's side.
        let Some(gf) = g.letters().and_then(|l| l.spelling.word.form()) else {
            return NormalForm::Irreducible;
        };
        // EACH FACTOR CARRIES ITS OWN COORDINATES. `slots` held the live
        // slot of each generator so the factorisation could report offsets;
        // the address is what the basis is keyed by and what `Decomposed`
        // now names.
        let mut at: Vec<crate::transducer::address::Address> = Vec::new();
        let mut forms = Vec::new();
        for (addr, gen) in self.basis.scoped() {
            if exclude.map(|e| e == addr.as_slice()) == Some(true) {
                continue;
            }
            if !self.is_live(&addr) {
                continue;
            }
            if gen.substance().reverse().is_none() {
                continue; // only group generators factor
            }
            let Some(f) = gen.letters().and_then(|l| l.spelling.word.form()) else {
                continue;
            };
            at.push(crate::transducer::address::Address::new(addr.clone()));
            forms.push(f);
        }
        if forms.is_empty() {
            return NormalForm::Irreducible;
        }
        match crate::transducer::factor::factor_on_plane(&gf, &forms) {
            // `Decomposed` is the CITATION SET -- which held words the
            // substance decomposes through -- so the exponents are dropped
            // here and the tape itself is `factor::word_of` when a caller
            // needs the ordered product.
            crate::transducer::factor::Factored::Understood(tape) => {
                let mut cited: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
                let mut word = Vec::new();
                for (i, _) in tape {
                    let Some(a) = at.get(i).cloned() else { continue };
                    if cited.contains(a.as_bytes()) {
                        continue;
                    }
                    cited.insert(a.as_bytes(), ());
                    word.push(a);
                }
                NormalForm::Decomposed(word)
            }
            // A remainder is a demand and an unmeasured factorisation is
            // neither: both leave the substance irreducible here, and the
            // remainder travels as the residual it already is.
            _ => NormalForm::Irreducible,
        }
    }

    /// The kernel's chain reduction: the shared decomposition over the
    /// language snapshot (`normal_form`, A12, M6 step 4), which factors the
    /// chain through the held words and leaves the remainder as the demand.
    /// The header that stood here described a greedy over "the generator
    /// whose magnitude descends most, tie-broken by basis order" — a choice
    /// by a scalar (rule 4) that this function does not make and the
    /// doctrine refuses; whether `normal_form` still makes one is row I29.
    pub(crate) fn reduce_chain(
        &self,
        first: Vec<(i64, i64, u8, u8)>,
        exclude: Option<&[u8]>,
        budget: &mut crate::physics::Budget,
    ) -> NormalForm {
        normal_form(&self.language, &first, exclude, budget)
    }

}
