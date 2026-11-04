// 78.17, PH5 — A COURT'S PHASE, the reading a live mesh's court publishes of
// its last phase window: a typed reading, served on demand, never a grep.

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn put_u64s(out: &mut Vec<u8>, vs: &[u64]) {
    for v in vs {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

fn take_u64s<const N: usize>(b: &[u8], p: &mut usize) -> Option<[u64; N]> {
    let mut out = [0u64; N];
    for v in out.iter_mut() {
        *v = take_u64(b, p)?;
    }
    Some(out)
}

/// PH5 — A COURT'S PHASE (45.9, I1): its last window's coordinates as the
/// court wrote them — comprehension, congruence, lineages, installed,
/// adjoined-by-closure, expansion — and the windows written. Read whole at
/// the question; the later stands.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Phase {
    pub windows: u64,
    pub h0: u64,
    pub h1: u64,
    pub deflated: u64,
    pub held: u64,
    pub sections: u64,
    pub strata: u64,
    pub lineages: u64,
    pub installed: u64,
    pub retired: u64,
    pub adjoined_by_closure: u64,
    pub axes: u64,
    pub splits: u64,
}

impl Phase {
    fn fields(&self) -> [u64; 13] {
        [
            self.windows,
            self.h0,
            self.h1,
            self.deflated,
            self.held,
            self.sections,
            self.strata,
            self.lineages,
            self.installed,
            self.retired,
            self.adjoined_by_closure,
            self.axes,
            self.splits,
        ]
    }

    pub(crate) fn put(&self, out: &mut Vec<u8>) {
        put_u64s(out, &self.fields());
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Phase> {
        let [windows, h0, h1, deflated, held, sections, strata, lineages, installed, retired, adjoined_by_closure, axes, splits] =
            take_u64s::<13>(b, p)?;
        Some(Phase {
            windows,
            h0,
            h1,
            deflated,
            held,
            sections,
            strata,
            lineages,
            installed,
            retired,
            adjoined_by_closure,
            axes,
            splits,
        })
    }
}
