// AT1 (r7), 78.41(2′) — THE ADDRESS SET the atlas compacts, read over the
// corpus's family forms: addresses held, vertices and members distinct
// against their sums per form, forms fusible by a shared member coordinate,
// and the words fusion identifies. Each coordinate is held ONCE here, and
// only for the census's own window.

use crate::network::nerve::{Coordinate, Keyed};
use crate::transducer::sentence::{Form, Vertex};

/// AT1 Q4 — THE FRAME'S SPELLING is unbounded and its byte order is the
/// numeric order: 255 before 256 before 65 536, where u16 LE put 256 first
/// and LEB128 orders by the low bits.
#[test]
fn the_frame_spelling_orders_as_the_frames_do() {
    use crate::transducer::sentence::form::atlas::frame_spelling;
    let frames = [0u64, 1, 2, 255, 256, 65_535, 65_536, u64::MAX];
    let spelled: Vec<Vec<u8>> = frames.iter().map(|&k| frame_spelling(k)).collect();
    assert!(spelled.windows(2).all(|w| w[0] < w[1]), "byte order is numeric order");
    assert_eq!(frame_spelling(0), vec![0], "zero is the empty body");
    assert_eq!(frame_spelling(256), vec![2, 1, 0]);
    assert_eq!(frame_spelling(u64::MAX).len(), 9, "no wall below the value's own width");
}

/// THE CORPUS'S ADDRESS SET, form by form.
#[derive(Default)]
pub(super) struct AddressSet {
    forms: usize,
    /// (r7a) distinct form addresses and the Word vertices' Member
    /// addresses, each held once, with their bytes.
    addresses: Keyed<()>,
    address_bytes: u64,
    words: Keyed<()>,
    word_bytes: u64,
    /// (r7b) class keys and members (class ‖ Invariant): the forms holding
    /// each, and the sums per form.
    vertex_forms: Keyed<Vec<usize>>,
    vertex_sum: u64,
    member_forms: Keyed<Vec<usize>>,
    member_sum: u64,
    /// (r7d) per form, its Word vertices' class keys, one entry per vertex.
    word_classes: Vec<Vec<Vec<u8>>>,
}

fn hold(on: &mut Keyed<Vec<usize>>, k: &[u8], form: usize) {
    match on.get_mut(k) {
        Some(fs) => {
            if !fs.contains(&form) {
                fs.push(form);
            }
        }
        None => {
            on.insert(k, vec![form]);
        }
    }
}

impl AddressSet {
    /// One family form, read into the set.
    pub(super) fn add(&mut self, form: &Form) {
        let ix = self.forms;
        self.forms += 1;
        let address = crate::transducer::sentence::form_vertex_key(form);
        if !self.addresses.contains(address.as_bytes()) {
            self.addresses.insert(address.as_bytes(), ());
            self.address_bytes += address.as_bytes().len() as u64;
        }
        let mut vertices: Keyed<()> = Keyed::new();
        let mut members: Keyed<()> = Keyed::new();
        let mut classes = Vec::new();
        for (_, v) in form.vertices.scoped() {
            let Vertex::Word { address, acts, .. } = v else { continue };
            if !self.words.contains(address.as_bytes()) {
                self.words.insert(address.as_bytes(), ());
                self.word_bytes += address.as_bytes().len() as u64;
            }
            let Some(tc) = crate::transducer::family::transport_class(v) else { continue };
            let class = tc.key().encode().as_bytes().to_vec();
            classes.push(class.clone());
            vertices.insert(&class, ());
            for (_, a) in acts.scoped() {
                let mut member = class.clone();
                crate::transducer::address::put_key_bytes(&mut member, a.chart.key().as_bytes());
                members.insert(&member, ());
            }
        }
        for (k, ()) in vertices.scoped() {
            self.vertex_sum += 1;
            hold(&mut self.vertex_forms, &k, ix);
        }
        for (k, ()) in members.scoped() {
            self.member_sum += 1;
            hold(&mut self.member_forms, &k, ix);
        }
        self.word_classes.push(classes);
    }

    /// THE r7 LINE: charts at frame depth are the atlas tally's own sum,
    /// passed in, since the frames are read at the percept.
    pub(super) fn line(&self, chart_frames: u64) -> String {
        // (r7c) forms sharing a member with another, and the pairs.
        let mut fusible: Keyed<()> = Keyed::new();
        let mut pairs: Keyed<()> = Keyed::new();
        for (_, fs) in self.member_forms.scoped() {
            if fs.len() < 2 {
                continue;
            }
            for (n, &a) in fs.iter().enumerate() {
                fusible.insert((a as u64).to_be_bytes(), ());
                for &b in fs.iter().skip(n + 1) {
                    let mut k = (a as u64).to_be_bytes().to_vec();
                    k.extend_from_slice(&(b as u64).to_be_bytes());
                    pairs.insert(&k, ());
                }
            }
        }
        // (r7d) words IDENTIFIED: Word vertices whose class another form holds.
        let identified: usize = self
            .word_classes
            .iter()
            .flatten()
            .filter(|c| self.vertex_forms.get(c).is_some_and(|fs| fs.len() > 1))
            .count();
        let words: usize = self.word_classes.iter().map(Vec::len).sum();
        format!(
            "address set over {} forms: addresses {} of {} bytes, word addresses {} of {} bytes \
             | vertices distinct {} of {} summed per form | members distinct {} of {} summed per form | charts at frames {} \
             | fusible forms {} in {} pairs | words identified {} of {}",
            self.forms,
            self.addresses.scoped().count(),
            self.address_bytes,
            self.words.scoped().count(),
            self.word_bytes,
            self.vertex_forms.scoped().count(),
            self.vertex_sum,
            self.member_forms.scoped().count(),
            self.member_sum,
            chart_frames,
            fusible.scoped().count(),
            pairs.scoped().count(),
            identified,
            words
        )
    }
}
