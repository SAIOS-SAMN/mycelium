// 78.16 C3 — WHAT ONE PERCEPT READ, HELD TYPED: nothing prints per epoch on
// the fleet. The percept holds this reading beside its cells; the census
// test prints its lines from it; the observer takes it each stroke and
// serves what the existing readings carry.

use crate::transducer::cech::NerveTotals;
use crate::transducer::sentence::{OrbitLine, UnreadTally};

/// THE CLASS'S STEPS ACROSS TWO CHARTS, BY KIND (AT1 A2): carried by a
/// motor, carried by the sections' relative, refused where no place derives,
/// and not taken where a frame holds several charts.
#[derive(Default, Clone, Copy, Debug)]
pub(crate) struct AcrossCharts {
    pub(crate) motor: u64,
    pub(crate) relative: u64,
    pub(crate) unplaced: u64,
    pub(crate) several: u64,
}

#[cfg(test)]
impl AcrossCharts {
    pub(crate) fn line(&self) -> String {
        format!(
            "frame axis across charts: motor {} | sections' relative {} | no place {} | several charts at a frame {}",
            self.motor, self.relative, self.unplaced, self.several
        )
    }
}

/// THE FAMILY FORM'S READING, its values taken at the percept: the running
/// tallies are read there, before comprehension's reads move them.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "78.16 C3 — the census's lines are read off it; the fleet takes it each stroke and serves no console for it (77.1)")
)]
pub(crate) struct FormReading {
    pub(crate) vertices: usize,
    pub(crate) faces: usize,
    pub(crate) meet_faces: usize,
    pub(crate) orbits: usize,
    pub(crate) edges: super::perceive::EdgeTally,
    pub(crate) meet_chords: usize,
    pub(crate) chords: usize,
    /// `(reflexive, no-tree-path, unread, trivial)`.
    pub(crate) gates: (u64, u64, u64, u64),
    pub(crate) unread: (UnreadTally, UnreadTally),
    /// `(vacuum, resonant, charged edges, steps read, steps non-zero)`.
    pub(crate) resonance: (u64, u64, u64, u64, u64, u64),
    pub(crate) defects: (u64, u64, u64, u64, u64),
    pub(crate) spectra: (u64, u64, u64, u64),
    pub(crate) synonyms: (u64, u64, u64, u64),
    pub(crate) operators: usize,
    pub(crate) contacts: super::pairing::ContactTally,
    pub(crate) planes: crate::transducer::cga::PlaneTally,
    pub(crate) mint: super::perceive::DemandTally,
    pub(crate) metabolism: crate::physics::metabolism::Metabolism,
    pub(crate) source: NerveTotals,
    pub(crate) contact_cells: Vec<(usize, u64, u64)>,
    pub(crate) image: NerveTotals,
    pub(crate) phase: (usize, usize, usize, [usize; 13]),
    pub(crate) drops: super::incidence::FaceDrops,
    pub(crate) contact_circuits: [usize; 6],
}

#[cfg(test)]
impl FormReading {
    pub(crate) fn line(&self) -> String {
        let (e, d, u) = (&self.edges, &self.drops, &self.unread);
        let c = &self.contact_circuits;
        format!(
            "family form {} vertices, {} faces ({} cover meets, {} site+ring), {} orbits \
             | edges asked: measured {}, unspelled {}, unread {}, not rigid {} \
             | chords {} offered {} | cycles: reflexive {} no-tree-path {} unread {} \
             (cone {} not rigid {} no face {}; edge refused: cone {} not rigid {} unrefracted {}) \
             trivial {} (vacuum {} resonant {} over {} charged edges; \
             charge read on {} steps, {} of them charged, {} unread) \
             | defects: trivial {} elliptic {} hyperbolic {} parabolic {} non-simple {} \
             | spectra: closes {} open {} undecomposed {} isoclinic {} \
             | ρ: |im| {} |conj| {} of {} generators, {} in ker | this form: b₁ {} |im| {} \
             | contacts: {} \
             | planes: {} \
             | mint: {} \
             | metabolism: {} syntropy {} \
             | nerve/source: {} charts over {} cells, {} shared by two or more, \
             {} \
             | contact complex: certified cells (grade cells meet≠0) {:?} \
             | nerve/image: {} charts over {} cells, {} shared, {} \
             | phase: {} finite {} infinite {} unread, periods {:?} \
             | faces offered {} built {} dropped: member_key {} vertex {} chart {} word {} place {} frameless {} \
             | contact faces offered {} built {} unnucleated {} | contact circuits: curved {} (gauge static {}) \
             coset closed {} modulo-held {} open {} unread {}",
            self.vertices,
            self.faces,
            self.meet_faces,
            d.built,
            self.orbits,
            e.measured,
            e.unspelled,
            e.unread,
            e.not_rigid,
            self.meet_chords,
            self.chords,
            self.gates.0,
            self.gates.1,
            self.gates.2,
            u.0.cone,
            u.0.not_rigid,
            u.0.no_face,
            u.1.cone,
            u.1.not_rigid,
            u.1.unrefracted,
            self.gates.3,
            self.resonance.0,
            self.resonance.1,
            self.resonance.2,
            self.resonance.3,
            self.resonance.4,
            self.resonance.5,
            self.defects.0,
            self.defects.1,
            self.defects.2,
            self.defects.3,
            self.defects.4,
            self.spectra.0,
            self.spectra.1,
            self.spectra.2,
            self.spectra.3,
            self.synonyms.0,
            self.synonyms.1,
            self.synonyms.2,
            self.synonyms.3,
            self.orbits,
            self.operators,
            self.contacts.line(),
            self.planes.line(),
            self.mint.line(),
            self.metabolism.line(),
            self.contacts.condensed,
            self.source.charts,
            self.source.cells,
            self.source.shared,
            self.source.cells_line(),
            self.contact_cells,
            self.image.charts,
            self.image.cells,
            self.image.shared,
            self.image.cells_line(),
            self.phase.0,
            self.phase.1,
            self.phase.2,
            self.phase.3,
            d.offered,
            d.built,
            d.member_key,
            d.vertex,
            d.chart,
            d.word,
            d.place,
            d.frameless,
            d.contact_offered,
            d.contact_built,
            d.unnucleated,
            c[0],
            c[1],
            c[2],
            c[3],
            c[4],
            c[5]
        )
    }
}

/// ONE PERCEPT'S READING (78.16 C3): what perception passed over, the family
/// form, each frameless pair with its content (78.46(5)(d)), the class's
/// steps across charts (AT1 A2), each orbit's line (OR1), the cycles' spans
/// (FR2), the motors running (MT1) and the atlas (AT1 r1–r6).
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "78.16 C3 — the census's lines are read off it; on the fleet the observer reads the unheard triple and takes the rest each stroke")
)]
pub(crate) struct PerceptReading {
    /// `(unheard, sectionless, coarse)` — taken by the percept, their one
    /// taker; the observer's `Loss` reads them here (law 12).
    pub(crate) unheard: [u64; 3],
    pub(crate) form: FormReading,
    pub(crate) frameless: Vec<super::incidence::Frameless>,
    pub(crate) across: AcrossCharts,
    pub(crate) orbits: Vec<OrbitLine>,
    pub(crate) spans: super::span::SpanTally,
    pub(crate) motors: crate::transducer::cga::MotorTally,
    pub(crate) atlas: crate::transducer::sentence::form::atlas::FormAtlas,
    pub(crate) searches: crate::transducer::sentence::form::atlas::AtlasTally,
}

#[cfg(test)]
impl PerceptReading {
    /// Its lines, in the order the percept reads them.
    pub(crate) fn lines(&self) -> Vec<String> {
        let mut out = vec![
            format!(
                "perceive unheard {}: components whose moduli are not read at their n | sectionless {} | coarse sections {}",
                self.unheard[0], self.unheard[1], self.unheard[2]
            ),
            self.form.line(),
        ];
        out.extend(self.frameless.iter().map(super::incidence::Frameless::line));
        out.push(self.across.line());
        out.extend(self.orbits.iter().map(OrbitLine::line));
        out.push(self.spans.line());
        out.push(format!("running {}", self.motors.line()));
        out.push(format!("atlas: {} | running {}", self.atlas.line(), self.searches.searches_line()));
        out
    }
}
