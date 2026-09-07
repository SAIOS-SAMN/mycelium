// 24.1 — THE SHED: what this client stops holding, and the aperture in
// receipts that decides when (law 9).

use crate::client::sdk::*;

impl Epigenome {
    pub fn shed_n(&self) -> u64 {
        self.shed_n
    }

    /// A working-atlas read of a stood word is a use, stamped with the
    /// mind's OWN K (45.4). The height beside it is standing extent and is
    /// left alone; the lifecycle is the mind's, and a mind that hears
    /// nothing still thinks.
    pub fn touch(&mut self, addr: &[u8], now_k: u64) {
        if let Some(d) = self.dialect.get_mut(addr) {
            d.last_used_k = now_k;
            d.uses = d.uses.saturating_add(1);
        }
    }

    /// Drop dialect unused for an aperture of the MIND'S OWN K (45.4: the
    /// mind sheds by its own receipt index, never by the mesh's height). A
    /// dialect entry a live idea glues is not shed. The genome never sheds.
    ///
    /// Read against TUniversal this could not fire at all on a mesh that
    /// admitted nothing: heights advance only on admitted receipts (45.2),
    /// so a fleet refusing work also could not reclaim what it held, and the
    /// two failures compounded into a client the kernel killed. A mind's own
    /// K advances on its own acts, so truncation is independent of whether
    /// anyone answered it.
    pub fn shed(&mut self, now_k: u64) -> usize {
        self.shed_except(now_k, &|_| false)
    }

    pub fn note_stroke(&mut self, now_k: u64) {
        let n = crate::physics::aperture_cycle().max(1) as usize;
        self.stroke_k.push(now_k);
        if self.stroke_k.len() > n {
            self.stroke_k.remove(0);
        }
    }

    /// TUniversal span of the last aperture of strokes. One unit, measured.
    /// `None` until a whole aperture of strokes has been noted: an
    /// unmeasured aperture is held, never guessed (rule 15), so nothing
    /// sheds in the first aperture after boot.
    pub(crate) fn aperture_k(&self, now_k: u64) -> Option<u64> {
        let n = crate::physics::aperture_cycle().max(1) as usize;
        if self.stroke_k.len() < n {
            return None;
        }
        let first = *self.stroke_k.first()?;
        Some(now_k.saturating_sub(first))
    }

    pub fn shed_except(&mut self, now_k: u64, glued: &impl Fn(&[u8]) -> bool) -> usize {
        let Some(aperture_k) = self.aperture_k(now_k) else {
            return 0;
        };
        let drop: Vec<Vec<u8>> = self
            .dialect
            .fiber(crate::network::nerve::ROOT)
            .filter(|(a, d)| now_k.saturating_sub(d.last_used_k) > aperture_k && !glued(a))
            .map(|(a, _)| a)
            .collect();
        let n = drop.len();
        for a in &drop {
            self.drop_dialect(a);
        }
        self.shed_n = self.shed_n.saturating_add(n as u64);
        n
    }

}
