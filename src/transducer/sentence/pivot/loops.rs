// 12.2, 18.2 — THE LOOPS of a form, and the faces a pivot writes.

use super::*;

impl Form {
    pub fn loops_of(faces: &[FormFace], n: usize) -> Vec<Loop> {
        if n == 0 {
            return Vec::new();
        }
        let mut parent: Vec<usize> = (0..n).collect();
        let find = |parent: &mut [usize], mut x: usize| -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        };
        let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
        let mut loops = Vec::new();
        let mut order = Vec::with_capacity(faces.len());
        for (fi, f) in faces.iter().enumerate() {
            if f.i == 0 || f.j == 0 {
                order.push(fi);
            }
        }
        for (fi, f) in faces.iter().enumerate() {
            if f.i != 0 && f.j != 0 {
                order.push(fi);
            }
        }
        for fi in order {
            let f = &faces[fi];
            let (a, b) = (f.i as usize, f.j as usize);
            if a >= n || b >= n {
                continue;
            }
            let (pa, pb) = (find(&mut parent, a), find(&mut parent, b));
            if pa != pb {
                parent[pa] = pb;
                adj[a].push((b, fi));
                adj[b].push((a, fi));
                continue;
            }
            let Some((mut path, mut walked)) = tree_path(&adj, a, b) else {
                continue;
            };
            path.push(a);
            walked.push(fi);
            let holonomy = form_named_holonomy(faces, &path, &walked);
            // THE CARRIAGES ARE THE TRANSPORT COMPOSITE'S, read from the one
            // walk that computes them. `path_carriages` stood here and walked
            // the loop's member paths accumulating `place.carriage` —
            // step for step the same walk `loop_transport_on` makes, with the
            // word and the gauge thrown away. Two carriers for one quantity
            // (law 12), and the expensive half: the walk is combinatorial over
            // each face's member pairs, and it ran once per loop at
            // construction and again per loop at every class reading. On the
            // fleet at e98e8a2 that is 70311 loops walked twice, and one pair
            // form carried 3361 of them.
            let mut lp = Loop {
                vertices: path.into_iter().map(|x| x as u16).collect(),
                faces: walked.into_iter().map(|x| x as u16).collect(),
                chord: fi as u16,
                holonomy,
                carriages: Keyed::new(),
            };
            for (k, t) in loop_transport_on(faces, &lp).scoped() {
                lp.carriages.insert(&k, t.carriage.clone());
            }
            loops.push(lp);
        }
        loops
    }
}

// `path_carriages(faces, path, walked)` stood here: a walk of the loop's
// member paths accumulating each face's `place.carriage`, keyed by the path.
// It is `loop_transport_on` with the word and the gauge discarded — the same
// frontier, the same `step_charts`, the same `closed_key`, the same
// `carriage.geo(&acc)` — so one quantity had two carriers (law 12) and the
// walk that produces it ran twice per loop: once at `loops_of` to fill
// `Loop.carriages`, and again at every class reading through
// `loop_transport_on`. `loops_of` fills the field from that one walk now.
//
// The doc it carried is true of `loop_transport_on` and belongs to it: the
// paths are walked FROM THE FACES and not taken as a product of the vertices'
// fibres, because each face's `at` keys are its member pairs; a step extends a
// partial path only where the face relates the chart the path stands at to the
// chart it steps to; a path the next face does not relate simply ends and is
// not a cycle of this complex; and a path is kept only where it ends at the
// chart it began at.

/// A MEMBER PATH BEING WALKED: the charts it has passed, the chart it stands
/// AT, the chart it started at, and what it has accumulated.
///
/// `at` and `first` are carried rather than read back off `charts` — a walk's
/// own frontier is not a member of a collection to be taken by position (rule
/// 15), and reading `charts.last()` for it is that read wearing a different
/// name.
struct PartialPath<T> {
    charts: Vec<Vec<u8>>,
    first: Option<Vec<u8>>,
    at: Option<Vec<u8>>,
    acc: T,
}

impl<T: Clone> PartialPath<T> {
    fn seed(acc: T) -> PartialPath<T> {
        PartialPath { charts: Vec::new(), first: None, at: None, acc }
    }

    /// The two charts a step over `face_pair_key` relates, oriented by the way
    /// the face runs, or `None` where the path does not stand at the near one.
    fn step_charts(&self, k: &[u8], forward: bool) -> Option<(Vec<u8>, Vec<u8>)> {
        let (ik, jk) = crate::transducer::sentence::read_face_pair_key(k)?;
        let (here, there) = if forward { (ik, jk) } else { (jk, ik) };
        match &self.at {
            Some(at) if *at != here => None,
            _ => Some((here, there)),
        }
    }

    fn extend(&self, here: Vec<u8>, there: Vec<u8>, acc: T) -> PartialPath<T> {
        let mut charts = self.charts.clone();
        let first = match &self.first {
            Some(f) => Some(f.clone()),
            None => Some(here.clone()),
        };
        charts.push(here);
        PartialPath { charts, first, at: Some(there), acc }
    }

    /// The path's key where it CLOSES — it ends at the chart it began at, which
    /// is what makes it a cycle of this complex. `None` otherwise.
    fn closed_key(&self) -> Option<Vec<u8>> {
        match (&self.first, &self.at) {
            (Some(f), Some(a)) if f == a => Some(Loop::path_key(&self.charts)),
            _ => None,
        }
    }
}


impl Form {
    /// THE HOLONOMY OF THE EDGE TRANSPORTS around a loop (50.3), ONE PER
    /// MEMBER PATH: the product of the face edges along the loop's faces, in
    /// the base body's stabilizer or exchange; stabilizer cosets do not
    /// telescope, which is what makes it a reading.
    ///
    /// A path carrying a remainder edge anywhere on it is not evaluated and is
    /// absent from the result (13.9); an empty result is "no path of this loop
    /// is evaluated", which is what `None` used to say for the whole loop.
    /// `None` itself is now only a loop naming a face this form does not have.
    pub fn edge_holonomies(&self, lp: &Loop) -> Option<Keyed<Transport>> {
        edge_holonomies_on(&self.faces, &self.vertices, lp)
    }
}

/// A FACE THAT TOUCHES THE FRAME, asked of the vertex rather than of its
/// slot. The Frame is a `Vertex` variant — it is what a vertex IS — and
/// `f.i == 0` names it by the integer it happens to sit at, which is the
/// vertex's identity collapsed onto a position (rule 15). Nine other sites
/// still spell it that way and are owed the same correction.
fn touches_frame(vertices: &[Vertex], f: &FormFace) -> bool {
    [f.i, f.j]
        .iter()
        .any(|&v| matches!(vertices.get(v as usize), Some(Vertex::Frame)))
}

/// The same walk, off the faces alone, so a reader that has the faces and
/// not a whole `Form` can take it — `class_vectors_on` is that reader, and
/// the class it reads is what every pivot decision downstream turns on.
pub(crate) fn edge_holonomies_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    lp: &Loop,
) -> Option<Keyed<Transport>> {
    // ONE HOLONOMY PER MEMBER PATH. The edge is a member coordinate — it
    // is measured on the union of TWO BODIES — so a loop whose faces relate
    // several pairs of bodies has one edge composite per path through them,
    // and composing "the" edge of each face asserted one path's reading for
    // every other. The paths are walked from the faces' own pairs, exactly
    // as `path_carriages` walks them, and a path whose faces do not join up
    // is not a cycle of this complex and yields nothing.
    {
        // A LOOP OF NOTHING BUT STAR FACES MEASURED NO EDGE. The Frame is the
        // trivialization, so a star face contributes the identity to a path
        // that also crosses real faces — but a path whose every leg is a star
        // face crossed no edge at all, and answering "identity" there reports
        // a measurement that was never taken. The distinction decides: such a
        // loop's holonomy is whatever its faces' WORDS name (a section face
        // carries a word that is not the coboundary, 21 step 2), and a reader
        // that took the star-path identity for a reading would overrule it
        // with a transport nobody measured.
        if !lp
            .faces
            .iter()
            .any(|&fi| faces.get(fi as usize).is_some_and(|f| !touches_frame(vertices, f)))
        {
            return Some(Keyed::new());
        }
        let mut live = vec![PartialPath::seed(Transport {
            word: Word::identity(),
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        })];
        for (win, &fi) in lp.vertices.windows(2).zip(&lp.faces) {
            let f = faces.get(fi as usize)?;
            let forward = f.i == win[0] && f.j == win[1];
            let star = touches_frame(vertices, f);
            let mut next = Vec::new();
            for p in &live {
                for (k, place) in f.at.scoped() {
                    let Some((here, there)) = p.step_charts(&k, forward) else {
                        continue;
                    };
                    // The Frame is the trivialization every body is read
                    // against (2.2): a star face contributes the identity, so
                    // a loop through the Frame reads the edges among the
                    // bodies it joins. The path still passes through it.
                    let mut held = p.acc.clone();
                    if !star {
                        let Some(edge) = place.edge.as_ref() else {
                            continue;
                        };
                        let step = if forward {
                            edge.clone()
                        } else {
                            match edge.inverse() {
                                Some(i) => i,
                                None => continue,
                            }
                        };
                        held.carriage = step.carriage.geo(&held.carriage);
                        held.word = step.word.compose(&held.word);
                        // THE GAUGE COMPOSES TOO. A holonomy has three
                        // coordinates — the rotor on `𝒽`, the carriage on `𝒮`,
                        // the hue cycle on `𝒢` (42.6) — and this returned an
                        // empty gauge unconditionally, so every class vector
                        // read off an edge holonomy was blind on one lane
                        // before it was read.
                        held.gauge = compose_gauge(&step.gauge, &held.gauge);
                    }
                    next.push(p.extend(here, there, held));
                }
            }
            live = next;
        }
        let mut out = Keyed::new();
        for p in live {
            if let Some(k) = p.closed_key() {
                let mut t = p.acc;
                t.word = t.word.normal_form();
                out.insert(k, t);
            }
        }
        Some(out)
    }
}

impl Form {
    /// The classes the edge holonomies belong to (15.4), read up to conjugacy,
    /// one per member path that is evaluated. Keyed by the path, so a caller
    /// asking "what class did this loop file" gets an answer per path and not
    /// one path's answer for all of them.
    pub fn edge_classes(&self, lp: &Loop) -> Keyed<crate::transducer::address::Address> {
        let mut out = Keyed::new();
        let Some(hs) = self.edge_holonomies(lp) else {
            return out;
        };
        for (k, h) in hs.scoped() {
            if let Some(c) = h.word.rotor().class_key() {
                out.insert(&k, c);
            }
        }
        out
    }

    /// THE CLASS VECTOR OF A LOOP'S EDGE HOLONOMY (12.6 over 50.3): the
    /// four coordinates of the stabilizer composite, as `class_vector`
    /// gives them for the transport composite. A loop carries BOTH and
    /// neither replaces the other — an extraction's loop can hold a half
    /// turn on its transports while its edges compose to the identity, and
    /// a cover's loop is the other way round, since every word it composes
    /// is a sensor-plane word and those commute.
    ///
    /// This is the reading that can occupy `𝒮`: the edge's carriage
    /// composes `D(ρ)` with `T(t)`, which do not commute, so a triangle of
    /// placements is a genuine commutator where a triangle of words is not.
    ///
    /// `None` when the loop is not evaluated (a remainder edge anywhere on
    /// it) or the holonomy is not a versor.
    pub fn edge_class_vectors(&self, lp: &Loop) -> Keyed<ClassVector> {
        let mut out = Keyed::new();
        let Some(hs) = self.edge_holonomies(lp) else {
            return out;
        };
        for (k, e) in hs.scoped() {
            if let Some(cv) =
                ClassVector::read(&e.word, e.carriage.clone(), CycleType::of(&e.gauge))
            {
                out.insert(&k, cv);
            }
        }
        out
    }

    /// A loop's edge holonomy coheres when EVERY EVALUATED MEMBER PATH is the
    /// identity or is filed as torsion under its class at its period (50.1
    /// step 2, A17). A path that is not evaluated — a remainder edge on it —
    /// holds nothing against the form and is absent.
    ///
    /// It read one composite and passed the whole loop on it, so a form whose
    /// edges cohered along one path cohered however curved the others were.
    pub(crate) fn edge_coheres(&self, lp: &Loop) -> bool {
        let Some(hs) = self.edge_holonomies(lp) else {
            return true;
        };
        hs.scoped().all(|(_, h)| {
            if h.word.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
                return true;
            }
            let (Some(class), crate::transducer::alphabet::Order::Finite(ell)) =
                (h.word.rotor().class_key(), h.word.order())
            else {
                return false;
            };
            matches!(self.torsion.get(&class), Some(filed) if filed.period == ell)
        })
    }
}

pub(crate) fn loop_transport(form: &Form, lp: &Loop) -> Keyed<Transport> {
    loop_transport_on(&form.faces, lp)
}

/// The same composite over FACES ALONE. A loop's transport is a product of
/// the faces it runs over and of nothing else on the form, so a trial that
/// has written one face can read it without a form to read it from.
pub(crate) fn loop_transport_on(faces: &[FormFace], lp: &Loop) -> Keyed<Transport> {
    // ONE COMPOSITE PER MEMBER PATH, the word the same on all of them: the
    // words are vertex coordinates and telescope identically whichever bodies
    // the path runs through, while the carriage and the gauge are the members'.
    let mut live = vec![PartialPath::seed(Transport {
        word: Word::identity(),
        carriage: element::identity_versor(),
        gauge: Vec::new(),
    })];
    for (win, &fi) in lp.vertices.windows(2).zip(&lp.faces) {
        let Some(f) = faces.get(fi as usize) else {
            return Keyed::new();
        };
        let forward = f.i == win[0] && f.j == win[1];
        let word_step = if forward { f.word.clone() } else { f.word.inverse() };
        let mut next = Vec::new();
        for p in &live {
            for (k, place) in f.at.scoped() {
                let Some((here, there)) = p.step_charts(&k, forward) else {
                    continue;
                };
                let (carriage, gauge) = if forward {
                    (place.carriage.clone(), place.gauge.clone())
                } else {
                    match place.carriage.versor_inverse() {
                        Some(inv) => (inv, gauge_invert(&place.gauge)),
                        None => continue,
                    }
                };
                let mut held = p.acc.clone();
                held.carriage = carriage.geo(&held.carriage);
                held.word = word_step.compose(&held.word);
                held.gauge = compose_gauge(&gauge, &held.gauge);
                next.push(p.extend(here, there, held));
            }
        }
        live = next;
    }
    let mut out = Keyed::new();
    for p in live {
        if let Some(k) = p.closed_key() {
            let mut t = p.acc;
            t.word = t.word.normal_form();
            out.insert(k, t);
        }
    }
    out
}

impl Form {
    /// Apply a certified move (A20 `by`) to this form: a write onto the
    /// chord face's transport, or a chart axis opened over the loop that
    /// closes on the face (I6). Loops are re-walked by the caller.
    pub fn apply_move(&mut self, by: &PivotBy) {
        match by {
            PivotBy::OpenAxis { face, .. } => {
                let _ = self.open_axis_on(*face);
            }
            _ => apply_by(&mut self.faces, by),
        }
    }

    /// The inverse of [`Form::apply_move`]: the court's un-pivot (A20 step 3).
    pub fn unapply_move(&mut self, by: &PivotBy) {
        match by {
            PivotBy::OpenAxis { face, .. } => {
                let _ = self.close_axis_on(*face);
            }
            _ => unapply_by(&mut self.faces, by),
        }
    }
}

pub(crate) fn apply_by(faces: &mut [FormFace], by: &PivotBy) {
    match by {
        PivotBy::OpenAxis { .. } => {}
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.word = pivoted_face(&f.word, word, *side);
            }
        }
        // A PIVOT MOVES THE FACE AT EVERY MEMBER PAIR. The move is a write on
        // the FACE — a translator on its placement, a gauge on its hue map —
        // and a face relating several pairs of bodies is moved at each of them
        // or the pivot moves one pair and leaves the rest where they were.
        PivotBy::Placement { face, translator } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(p) = f.at.get_mut(&k) {
                        p.carriage = translator.geo(&p.carriage);
                    }
                }
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(p) = f.at.get_mut(&k) {
                        p.gauge = compose_gauge(&p.gauge, gauge);
                    }
                }
            }
        }
    }
}

pub(crate) fn unapply_by(faces: &mut [FormFace], by: &PivotBy) {
    match by {
        PivotBy::OpenAxis { .. } => {}
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.word = unpivoted_face(&f.word, word, *side);
            }
        }
        PivotBy::Placement { face, translator } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                if let Some(inv) = translator.versor_inverse() {
                    let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                    for k in pairs {
                        if let Some(p) = f.at.get_mut(&k) {
                            p.carriage = inv.geo(&p.carriage);
                        }
                    }
                }
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                let back = gauge_invert(gauge);
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(p) = f.at.get_mut(&k) {
                        p.gauge = compose_gauge(&p.gauge, &back);
                    }
                }
            }
        }
    }
}

pub fn pivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(&w.inverse()).normal_form(),
        Side::Left => w.compose(g).normal_form(),
    }
}

/// The face word before that move: the inverse of [`pivoted_face`].
pub fn unpivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(w).normal_form(),
        Side::Left => w.inverse().compose(g).normal_form(),
    }
}
