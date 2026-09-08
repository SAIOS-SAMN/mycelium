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
            let mut carriage = element::identity_versor();
            for (win, &wfi) in path.windows(2).zip(&walked) {
                let ff = &faces[wfi];
                let step = if ff.i as usize == win[0] && ff.j as usize == win[1] {
                    ff.transport.carriage.clone()
                } else {
                    match ff.transport.carriage.versor_inverse() {
                        Some(inv) => inv,
                        None => element::identity_versor(),
                    }
                };
                carriage = step.geo(&carriage);
            }
            loops.push(Loop {
                vertices: path.into_iter().map(|x| x as u16).collect(),
                faces: walked.into_iter().map(|x| x as u16).collect(),
                chord: fi as u16,
                holonomy,
                carriage,
            });
        }
        loops
    }
}


pub(crate) fn loop_transport(form: &Form, lp: &Loop) -> Option<Transport> {
    let mut acc_word = Word::identity();
    let mut acc_carriage = element::identity_versor();
    let mut acc_gauge = Vec::new();
    for (win, &fi) in lp.vertices.windows(2).zip(&lp.faces) {
        let f = form.faces.get(fi as usize)?;
        let step = if f.i == win[0] && f.j == win[1] {
            f.transport.clone()
        } else {
            f.transport.inverse()?
        };
        acc_carriage = step.carriage.geo(&acc_carriage);
        acc_word = step.word.compose(&acc_word);
        acc_gauge = compose_gauge(&step.gauge, &acc_gauge);
    }
    Some(Transport {
        word: acc_word.normal_form(),
        carriage: acc_carriage,
        gauge: acc_gauge,
    })
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
                f.transport.word = pivoted_face(&f.transport.word, word, *side);
            }
        }
        PivotBy::Placement { face, translator } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.carriage = translator.geo(&f.transport.carriage);
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.gauge = compose_gauge(&f.transport.gauge, gauge);
            }
        }
    }
}

pub(crate) fn unapply_by(faces: &mut [FormFace], by: &PivotBy) {
    match by {
        PivotBy::OpenAxis { .. } => {}
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.word = unpivoted_face(&f.transport.word, word, *side);
            }
        }
        PivotBy::Placement { face, translator } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                if let Some(inv) = translator.versor_inverse() {
                    f.transport.carriage = inv.geo(&f.transport.carriage);
                }
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.transport.gauge = compose_gauge(&f.transport.gauge, &gauge_invert(gauge));
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
