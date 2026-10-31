use super::{apply_form, Grid, Speech};
use crate::transducer::element::Element;
use crate::transducer::sentence::Form;
use crate::transducer::stalk::{Invariant, Standing};

/// Source of crystallized forms on a class. Standing is a coordinate on
/// the working atlas, never a gate.
pub trait Crystal {
    /// 24.2: the closed forms held at a CITED FORM's chart (`chart_of_form`),
    /// which is what comprehension of a citation asks for.
    fn on_class(&self, class: &Invariant) -> Vec<(&Element, &Form, Standing)>;
    /// 21 step 3: the closed forms carrying a body on a BODY's chart (its
    /// moduli), which is what the glue asks for. A form is filed under
    /// `chart_of_form` and a body under its moduli, two coordinate systems
    /// that meet only at the empty root, so one reader cannot answer both.
    fn on_body(&self, chart: &Invariant) -> Vec<(&Element, &Form, Standing)>;
    fn speak_form(&self, f: &Form, x: &Grid, placement: &crate::transducer::alphabet::Word) -> Option<Speech>;
    /// 21 step 3 — THE CLOSED FORMS ACTING AT ONE COORDINATE: a member's chart,
    /// whole motion and hue map (`presentation::act_key`), one keyed read. A
    /// crystal that never glues (the court, the cipher) holds none.
    fn on_act(&self, _act: &[u8]) -> Vec<(&Element, &Form, Standing)> {
        Vec::new()
    }
    /// Keyed read of a crystallized form by address. No scan, AND NO CLONE
    /// (Z2 item 2, rule 17): a reader hands back the stratum it holds, and
    /// a caller that resolves one citation 578 times must not pay a form
    /// copy for each. The borrow is the crystal's own and it outlives no
    /// call that does not hold the crystal. A word the crystal holds without
    /// its form — a court's cavity — has its form DERIVED from its address
    /// where it is read, never stored beside it: that arm is owned.
    fn by_address(&self, address: &[u8]) -> Option<(&Element, std::borrow::Cow<'_, Form>)>;
    /// Held words on `chart` whose class vector occupies `axis` (12.6): the
    /// vocabulary a pivot may write onto an orbit on that chart. One keyed read
    /// of the axis index the crystal filled when the word was admitted.
    fn on_axis(&self, chart: &Invariant, axis: crate::transducer::sentence::Axis) -> Vec<crate::transducer::alphabet::Word>;
    /// 1.8, 50.2 step 2 — THE PRESENTATION ON A CHART: the generators
    /// this crystal holds on `chart`, each as the cited form's address,
    /// the address of the `Word` vertex inside it, and that vertex's own
    /// word. A keyed subtree read of the generator index, spending the
    /// budget it is given (rule 10); a crystal that holds nothing on the
    /// chart answers empty, which is a presentation of no generators and
    /// not a refusal.
    fn generators_under(
        &self,
        chart: &Invariant,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        crate::transducer::alphabet::Word,
    )>;

    /// 42.6 AS AMENDED, L12 — THE PRESENTATION ON ONE LANE. The cover of
    /// the language is the product of four, and a reduction is bounded by
    /// a presentation only when it runs against the lane's own basis:
    /// reducing a target that occupies `𝒢` against a generator occupying
    /// only `𝒽` is a basis no lane has.
    ///
    /// The default answers the `𝒽` cover for lane 3 and nothing for the
    /// others, which is what a crystal that has not split its index holds:
    /// `versor_chart` IS the holonomic lane's chart, so lane 3 was never
    /// missing — the other three were, and answering them from it would
    /// be the union this row exists to refuse.
    fn generators_on_lane(
        &self,
        lane: usize,
        chart: &Invariant,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        crate::transducer::alphabet::Word,
    )> {
        match lane {
            3 => self.generators_under(chart, budget),
            _ => Vec::new(),
        }
    }

    /// 78.160 P3 — WHAT A COURT ANSWERED THIS MIND FOR A MOTION: the
    /// generators, each `(form, vertex, exponent)`, whose powers the court
    /// read composing to the motion exactly. `None` where no court has, or
    /// for a crystal that asks no court (the court itself, the cipher).
    fn presented(
        &self,
        _motion: &crate::transducer::cga::Mv,
    ) -> Option<Vec<(crate::transducer::address::Address, crate::transducer::address::Address, i64)>> {
        None
    }

    /// 78.161 — A WORD READ OVER THE RULED WORDS THIS MIND HOLDS, lane by lane
    /// (`presentation::Reading`): its whole motion and its hue map, each mode
    /// cited where a ruled word stands on it at an exponent. `None` for a
    /// crystal with no ear (the court, the cipher).
    fn read_word(
        &self,
        _motion: &crate::transducer::cga::Mv,
        _gauge: &[(u8, u8)],
    ) -> Option<crate::transducer::sentence::presentation::Reading> {
        None
    }

    /// 42.6 AS AMENDED, L12 — THE PRESENTATION AT THE MEET OF THE LANES A
    /// TARGET OCCUPIES. `occupied` is one entry per lane the target carries
    /// content on, with that lane's own chart (`lane_charts_of`).
    ///
    /// THE MEET, NEVER THE UNION. A target occupying `𝒮` and `𝒽` is not
    /// reducible against a generator that lives only on `𝒽`: dividing by it
    /// leaves the `𝒮` content untouched and asserts a bound the presentation
    /// does not give. It is reducible against a generator that lies in BOTH
    /// covers, because that generator moves both coordinates at once, which
    /// is what a single group element does. So a generator enters the basis
    /// when it is held on EVERY occupied lane, and is dropped when any lane
    /// does not hold it.
    ///
    /// FOUR FLAGS, NOT A COUNT. Membership is read per lane and the four
    /// readings are never summed (law 5). A count of lanes hit compared
    /// against a length would be a number deciding a branch (rule 2), and
    /// it cannot tell a generator held twice on one lane from one held on
    /// two — which is the exact confusion the four covers exist to prevent.
    ///
    /// A target occupying NO lane gets no basis. The meet over an empty
    /// family is formally everything, and answering everything here is the
    /// union this row exists to refuse; a target on no lane is the identity
    /// and has nothing to reduce.
    fn generators_at_meet(
        &self,
        occupied: &[(usize, Invariant)],
        budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        crate::transducer::alphabet::Word,
    )> {
        if occupied.is_empty() {
            return Vec::new();
        }
        let mut lanes_hit: crate::network::nerve::Keyed<[bool; 4]> =
            crate::network::nerve::Keyed::new();
        let mut held: crate::network::nerve::Keyed<(
            crate::transducer::address::Address,
            crate::transducer::address::Address,
            crate::transducer::alphabet::Word,
        )> = crate::network::nerve::Keyed::new();
        for (lane, chart) in occupied {
            for (form, vertex, w) in self.generators_on_lane(*lane, chart, budget) {
                let k = crate::transducer::sentence::generator_key(&form, &vertex);
                match lanes_hit.get_mut(&k) {
                    Some(flags) => {
                        if let Some(slot) = flags.get_mut(*lane) {
                            *slot = true;
                        }
                    }
                    None => {
                        let mut flags = [false; 4];
                        if let Some(slot) = flags.get_mut(*lane) {
                            *slot = true;
                        }
                        lanes_hit.insert(&k, flags);
                    }
                }
                held.insert(k, (form, vertex, w));
            }
        }
        let mut out = Vec::new();
        for (k, g) in held.scoped() {
            let Some(flags) = lanes_hit.get(&k) else { continue };
            let on_every = occupied
                .iter()
                .all(|(lane, _)| matches!(flags.get(*lane), Some(true)));
            if on_every {
                out.push(g.clone());
            }
        }
        out
    }
}

/// A crystal that holds nothing. Glue is a no-op.
pub struct NoCrystal;

impl Crystal for NoCrystal {
    fn on_class(&self, _class: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        Vec::new()
    }
    fn on_body(&self, _chart: &Invariant) -> Vec<(&Element, &Form, Standing)> {
        Vec::new()
    }
    fn speak_form(&self, f: &Form, x: &Grid, placement: &crate::transducer::alphabet::Word) -> Option<Speech> {
        apply_form(f, x, placement, self)
    }
    fn by_address(&self, _address: &[u8]) -> Option<(&Element, std::borrow::Cow<'_, Form>)> {
        None
    }
    fn on_axis(&self, _chart: &Invariant, _axis: crate::transducer::sentence::Axis) -> Vec<crate::transducer::alphabet::Word> {
        Vec::new()
    }
    fn generators_under(
        &self,
        _chart: &Invariant,
        _budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        crate::transducer::alphabet::Word,
    )> {
        Vec::new()
    }
}
