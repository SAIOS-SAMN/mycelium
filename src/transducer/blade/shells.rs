// 20 — THE SHELLS OF A BODY: the outline, the core, the ring, the
// interior, and the exact magnifications between them.
//
//! This file and the sensor read a site's coordinates; everything above
//! them is versors and words (law 11). A painted site is the grid's
//! token, `(row, col, hue)`.


pub(crate) fn occupy(painted: &[(i64, i64, u8)]) -> crate::network::nerve::Keyed<u8> {
    let mut k = crate::network::nerve::Keyed::new();
    for &(r, c, h) in painted {
        let mut key = [0u8; 32];
        key[..8].copy_from_slice(&r.to_le_bytes());
        key[8..16].copy_from_slice(&c.to_le_bytes());
        k.insert(key, h);
    }
    k
}

pub(crate) fn pos_key(r: i64, c: i64) -> [u8; 32] {
    let mut key = [0u8; 32];
    key[..8].copy_from_slice(&r.to_le_bytes());
    key[8..16].copy_from_slice(&c.to_le_bytes());
    key
}

/// Painted sites that touch background or the horizon (the outline germ).
pub fn outline(painted: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let have = occupy(painted);
    painted
        .iter()
        .copied()
        .filter(|&(r, c, _)| {
            [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                .iter()
                .any(|&(nr, nc)| !have.contains(&pos_key(nr, nc)))
        })
        .collect()
}

/// Painted sites whose four neighbours are painted (the core germ).
pub fn core(painted: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let have = occupy(painted);
    painted
        .iter()
        .copied()
        .filter(|&(r, c, _)| {
            [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                .iter()
                .all(|&(nr, nc)| have.contains(&pos_key(nr, nc)))
        })
        .collect()
}

/// Background sites one 4-step from a painted one (the adjacent-ring germ).
pub fn adjacent_ring(painted: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let have = occupy(painted);
    let mut out = Vec::new();
    let mut seen = crate::network::nerve::Keyed::new();
    for &(r, c, _) in painted {
        for (nr, nc) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
            let k = pos_key(nr, nc);
            if have.contains(&k) || seen.contains(&k) {
                continue;
            }
            seen.insert(k, ());
            out.push((nr, nc, 0u8));
        }
    }
    out.sort_unstable();
    out
}

/// Background sites inside the body's EXTENT that cannot reach its border
/// through background (enclosed interior / holes).
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the extent is the reach of the painted sites, content: two nodes holding one body read the same corners"
)]
pub fn enclosed_interior(painted: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    if painted.is_empty() {
        return Vec::new();
    }
    let have = occupy(painted);
    let rows = || painted.iter().map(|c| c.0);
    let cols = || painted.iter().map(|c| c.1);
    let (Some(r0), Some(r1), Some(c0), Some(c1)) = (rows().min(), rows().max(), cols().min(), cols().max()) else {
        return Vec::new();
    };
    let mut escape = crate::network::nerve::Keyed::new();
    let mut work = Vec::new();
    for r in r0..=r1 {
        for c in [c0, c1] {
            let k = pos_key(r, c);
            if !have.contains(&k) && !escape.contains(&k) {
                escape.insert(k, ());
                work.push((r, c));
            }
        }
    }
    for c in c0..=c1 {
        for r in [r0, r1] {
            let k = pos_key(r, c);
            if !have.contains(&k) && !escape.contains(&k) {
                escape.insert(k, ());
                work.push((r, c));
            }
        }
    }
    while let Some((r, c)) = work.pop() {
        for (nr, nc) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
            if nr < r0 || nr > r1 || nc < c0 || nc > c1 {
                continue;
            }
            let k = pos_key(nr, nc);
            if have.contains(&k) || escape.contains(&k) {
                continue;
            }
            escape.insert(k, ());
            work.push((nr, nc));
        }
    }
    let mut out = Vec::new();
    for r in r0..=r1 {
        for c in c0..=c1 {
            let k = pos_key(r, c);
            if !have.contains(&k) && !escape.contains(&k) {
                out.push((r, c, 0u8));
            }
        }
    }
    out
}

/// THE BODY'S EXTENT: the least and greatest lamina coordinate its painted
/// sites occupy, read at the sensor boundary (§1.4, law 11's one place
/// where a pair of integers is a coordinate).
///
/// It was called `bbox`. That is computer science's noun for an
/// axis-aligned box used to INDEX points in a flat grid, and the gate
/// refuses that shape — but this is not it: nothing here partitions,
/// bins, or locates anything by the box. The extent IS the object, the
/// same reading `place_chain`'s `corner_translator` takes when it centres
/// a chain on its min corner. The name carried a Euclidean crutch's
/// vocabulary over a lawful sensor reading, which is §2.1's fault, and
/// renaming it is the whole fix.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 4 — the extent IS this function's object, the reach of the painted sites and not a candidate chosen from them; an empty set has no extent and refuses"
)]
pub(crate) fn extent(painted: &[(i64, i64, u8)]) -> Option<(i64, i64, i64, i64)> {
    let r0 = painted.iter().map(|c| c.0).min()?;
    let r1 = painted.iter().map(|c| c.0).max()?;
    let c0 = painted.iter().map(|c| c.1).min()?;
    let c1 = painted.iter().map(|c| c.1).max()?;
    Some((r0, r1, c0, c1))
}

/// CUT: the body's EXTENT as its own whole, origin at its least site.
pub fn cut(painted: &[(i64, i64, u8)]) -> Option<Vec<(i64, i64, u8)>> {
    let (r0, _, c0, _) = extent(painted)?;
    let mut out: Vec<_> = painted.iter().map(|&(r, c, h)| (r - r0, c - c0, h)).collect();
    out.sort_unstable();
    Some(out)
}

/// MAGNIFIED ×k — each lamina becomes a k×k block.
pub fn magnified(painted: &[(i64, i64, u8)], k: i64) -> Vec<(i64, i64, u8)> {
    if k <= 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    for &(r, c, h) in painted {
        for dr in 0..k {
            for dc in 0..k {
                out.push((r * k + dr, c * k + dc, h));
            }
        }
    }
    out.sort_unstable();
    out
}

/// REDUCED ×k — each k×k block condenses to one hue, or None if a block is mixed.
#[expect(
    clippy::disallowed_methods,
    reason = "rule 11, content extremum — hue 0 IS absence in this encoding, so a site the occupancy does not hold reads as unpainted rather than as a hue put there for it"
)]
pub fn reduced(painted: &[(i64, i64, u8)], k: i64) -> Option<Vec<(i64, i64, u8)>> {
    if k <= 1 {
        return Some(painted.to_vec());
    }
    let (r0, r1, c0, c1) = extent(painted)?;
    if (r1 - r0 + 1) % k != 0 || (c1 - c0 + 1) % k != 0 {
        return None;
    }
    let have = occupy(painted);
    let mut out = Vec::new();
    let mut br = r0;
    while br <= r1 {
        let mut bc = c0;
        while bc <= c1 {
            let mut hue = None;
            for dr in 0..k {
                for dc in 0..k {
                    let h = have.get(&pos_key(br + dr, bc + dc)).copied().unwrap_or(0);
                    match hue {
                        None => hue = Some(h),
                        Some(held) if held != h => return None,
                        _ => {}
                    }
                }
            }
            if let Some(h) = hue {
                if h != 0 {
                    out.push(((br - r0) / k, (bc - c0) / k, h));
                }
            }
            bc += k;
        }
        br += k;
    }
    Some(out)
}

/// TILED down×across — the cut body repeated as a grid of itself.
pub fn tiled(painted: &[(i64, i64, u8)], down: i64, across: i64) -> Vec<(i64, i64, u8)> {
    let Some(cut) = cut(painted) else { return Vec::new() };
    let Some((_, r1, _, c1)) = extent(&cut) else { return Vec::new() };
    let (h, w) = (r1 + 1, c1 + 1);
    let mut out = Vec::new();
    for tr in 0..down.max(0) {
        for tc in 0..across.max(0) {
            for &(r, c, hue) in &cut {
                out.push((tr * h + r, tc * w + c, hue));
            }
        }
    }
    out.sort_unstable();
    out
}

/// BORDERED — a ring of `hue` one site wide around the cut body.
pub fn bordered(painted: &[(i64, i64, u8)], hue: u8) -> Vec<(i64, i64, u8)> {
    let Some(cut) = cut(painted) else { return Vec::new() };
    let Some((_, r1, _, c1)) = extent(&cut) else { return Vec::new() };
    let (rows, cols) = (r1 + 1, c1 + 1);
    let mut out: Vec<_> = cut.iter().map(|&(r, c, h)| (r + 1, c + 1, h)).collect();
    for r in 0..rows + 2 {
        out.push((r, 0, hue));
        out.push((r, cols + 1, hue));
    }
    for c in 1..=cols {
        out.push((0, c, hue));
        out.push((rows + 1, c, hue));
    }
    out.sort_unstable();
    out.dedup();
    out
}
