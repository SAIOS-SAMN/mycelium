// 24 — the working atlas a client reads its fibres on.

/// Standing of a fibre member on the working atlas.
pub use crate::transducer::stalk::Standing;

/// One working atlas, three strata. A fibre read is `(address, Standing)`,
/// never a boolean. Genome-only `atlas()` remains for persistence. The
/// Held stratum is the contemplation's forms, read at the coordinate
/// (24.1 as ruled 2026-09-09): the Word vertices its held forms hold on
/// a chart, through `Contemplation::held_at`, never a list kept beside it.
pub struct WorkingAtlas<'a> {
    pub(crate) genome: &'a crate::transducer::stalk::Stalks,
    pub(crate) dialect: &'a crate::transducer::stalk::Stalks,
    pub(crate) held: Option<&'a crate::transducer::bubble::Contemplation>,
}

/// The members of one stratum split by lane (42.6 as amended): four
/// independent lists and never the lists' concatenation.
pub type LaneMembers = [Vec<(Vec<u8>, Standing)>; 4];

impl<'a> WorkingAtlas<'a> {
    /// Addresses on this chart, each carrying the stratum it came from.
    /// Ruled first, then Stood, then Held; an address appears once, at its
    /// strongest standing.
    pub fn fibre(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<(Vec<u8>, Standing)> {
        let mut out: Vec<(Vec<u8>, Standing)> = Vec::new();
        let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for a in self.genome.fibre(inv) {
            have.insert(&a, ());
            out.push((a, Standing::Ruled));
        }
        for a in self.dialect.fibre(inv) {
            if have.contains(&a) {
                continue;
            }
            have.insert(&a, ());
            out.push((a, Standing::Stood));
        }
        if let Some(held) = self.held {
            // index: the Word vertices the held forms hold on this chart
            for v in held.held_at(inv) {
                let crate::transducer::sentence::Vertex::Word { address, .. } = v else {
                    continue;
                };
                if have.contains(address) {
                    continue;
                }
                have.insert(address, ());
                out.push((address.clone().into_vec(), Standing::Held));
            }
        }
        out
    }

    /// THE STRONGEST STANDING this mind has on a chart — whether ANY word
    /// on it is ruled, else whether any stands, else held.
    ///
    /// No member is taken. 24.1 puts several words on one stratum as
    /// multiplicity, so the head of the fibre is one of possibly many
    /// ruled words, and taking it would be a choice (rule 15) even though
    /// the value read off it is the same for all of them. The reading is
    /// over the fibre, so the multiplicity never has to be resolved.
    pub fn standing_on(&self, inv: &crate::transducer::stalk::Invariant) -> Standing {
        let on = self.fibre(inv);
        if on.iter().any(|(_, s)| matches!(s, Standing::Ruled)) {
            Standing::Ruled
        } else if on.iter().any(|(_, s)| matches!(s, Standing::Stood)) {
            Standing::Stood
        } else {
            Standing::Held
        }
    }

    /// THE GENERATORS OF ONE STRATUM (42.6, 16.12, A21 §2): every address
    /// under a prefix of `inv`, coarsened one axis at a time until the read
    /// is not empty, with the stratum it was found at.
    ///
    /// A12's basis is "the held words on the chart's fibre", and read at the
    /// EXACT chart that basis is almost always empty or a single word: a
    /// connection's chart is its rotor's trace and determinant (M4 step 2),
    /// so every distinct rotation is its own chart and the only words a mind
    /// could factor through were the powers of one word. The lattice could
    /// never have more than one generator and the whole Hermite computation
    /// was decorative. A stratum is the fibre under a PREFIX — "a demand is
    /// reduced against the generators of one stratum, never the language"
    /// — and `peel` is what reaches it.
    ///
    /// It coarsens no further than it must: the first non-empty stratum is
    /// the one returned, so a mind factors against the nearest generators it
    /// holds and never against everything it holds.
    pub fn stratum(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> (crate::transducer::stalk::Invariant, Vec<(Vec<u8>, Standing)>) {
        let mut at = inv.clone();
        loop {
            let held = self.fibre_under(&at);
            if !held.is_empty() || at.0.is_empty() {
                return (at, held);
            }
            at = at.peel();
        }
    }

    /// THE STRATUM AS A BASIS PER LANE (42.6 as amended, L12).
    ///
    /// The cover of the language is the PRODUCT of four covers, one per
    /// lane, and a reduction is bounded by a presentation only if it runs
    /// against the lane's own. `stratum` answers the whole stratum, which
    /// is the union; this splits it, so lane `l`'s basis is the members
    /// whose class vector occupies an axis on `l`.
    ///
    /// A member may stand in more than one lane's basis, and that is not a
    /// double count: a generator that turns AND recolours is a generator
    /// on `𝒽` and a generator on `𝒢`, and each lane's reduction is
    /// entitled to it. What is refused is the other direction — reducing a
    /// target that occupies `𝒢` against a generator that occupies only
    /// `𝒽`, which is a basis no lane has.
    ///
    /// A member whose form this mind no longer holds, or whose word has no
    /// class vector, stands in no lane's basis. It is dropped rather than
    /// placed in all four (rule 11).
    /// `class_of` is the caller's: this carrier holds the chart INDEXES
    /// and not the words, so the class vector of a member is resolved by
    /// whoever holds them. Passing it in rather than reaching for a second
    /// carrier is what keeps the atlas one object.
    pub fn stratum_by_lane(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        class_of: &dyn Fn(&[u8]) -> Option<crate::transducer::sentence::ClassVector>,
    ) -> (crate::transducer::stalk::Invariant, LaneMembers) {
        let (at, held) = self.stratum(inv);
        let mut lanes: LaneMembers = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for (address, st) in held {
            let Some(cv) = class_of(&address) else { continue };
            for (l, occupied) in cv.lanes().into_iter().enumerate() {
                if occupied {
                    lanes[l].push((address.clone(), st));
                }
            }
        }
        (at, lanes)
    }

    /// Every address under a chart prefix, not only those exactly at it:
    /// `fibre` reads one node's own members, and a stratum is the subtree
    /// (A21 §2). Ruled first, then Stood, then Held, each address once.
    pub fn fibre_under(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Vec<(Vec<u8>, Standing)> {
        let mut out: Vec<(Vec<u8>, Standing)> = Vec::new();
        let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for (addrs, st) in [
            (self.genome.fibre_under(inv), Standing::Ruled),
            (self.dialect.fibre_under(inv), Standing::Stood),
        ] {
            for a in addrs {
                if have.contains(&a) {
                    continue;
                }
                have.insert(&a, ());
                out.push((a, st));
            }
        }
        out
    }

    /// The standing of an address on a chart: the chart is the coordinate
    /// the read happens at, in every stratum.
    pub fn standing_of(&self, address: &[u8], inv: &crate::transducer::stalk::Invariant) -> Option<Standing> {
        if self.genome.chart_of(address) == Some(inv) {
            return Some(Standing::Ruled);
        }
        if self.dialect.chart_of(address) == Some(inv) {
            return Some(Standing::Stood);
        }
        let on_chart = |v: &crate::transducer::sentence::Vertex| {
            matches!(v, crate::transducer::sentence::Vertex::Word { address: held, .. } if held.as_bytes() == address)
        };
        if self.held.is_some_and(|held| held.held_at(inv).into_iter().any(on_chart)) {
            return Some(Standing::Held);
        }
        None
    }
}

/// The working atlas read as one atlas (24.1): a thought meets the genome's
/// charts and the dialect's alike, and the members at a chart carry both
/// strata.
///
/// A `children` arm stood here that walked BOTH tries from one cursor under
/// one budget and returned whichever cursor came back last — two
/// independent traversals sharing a position, which is a resume point that
/// belongs to neither. It is gone with the trait method: a chart's members
/// are a keyed read of both fibres, which needs no cursor to be correct
/// across strokes.
impl crate::transducer::stalk::AtlasRead for WorkingAtlas<'_> {
    fn members(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        self.fibre(inv).into_iter().map(|(a, _)| a).collect()
    }
}
