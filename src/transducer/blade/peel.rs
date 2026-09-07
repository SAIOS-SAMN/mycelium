// 20 — THE PEEL: the outline, the core, the ring, the interior, and the
// exact magnifications of a body.
//
//! This file and the sensor read cell coordinates; everything above them is
//! versors and words (law 11).


pub(crate) fn occupy(cells: &[(i64, i64, u8)]) -> crate::network::nerve::Keyed<u8> {
    let mut k = crate::network::nerve::Keyed::new();
    for &(r, c, h) in cells {
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

/// Content cells that touch background or the horizon (the outline germ).
pub fn outline(cells: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let have = occupy(cells);
    cells
        .iter()
        .copied()
        .filter(|&(r, c, _)| {
            [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                .iter()
                .any(|&(nr, nc)| !have.contains(&pos_key(nr, nc)))
        })
        .collect()
}

/// Content cells whose four neighbours are content (the core germ).
pub fn core(cells: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let have = occupy(cells);
    cells
        .iter()
        .copied()
        .filter(|&(r, c, _)| {
            [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                .iter()
                .all(|&(nr, nc)| have.contains(&pos_key(nr, nc)))
        })
        .collect()
}

/// Background cells one 4-step from content (the adjacent-ring germ).
pub fn adjacent_ring(cells: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let have = occupy(cells);
    let mut out = Vec::new();
    let mut seen = crate::network::nerve::Keyed::new();
    for &(r, c, _) in cells {
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

/// Background cells inside the body's bbox that cannot reach the bbox
/// border through background (enclosed interior / holes).
pub fn enclosed_interior(cells: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    if cells.is_empty() {
        return Vec::new();
    }
    let have = occupy(cells);
    let r0 = cells.iter().map(|c| c.0).min().unwrap();
    let r1 = cells.iter().map(|c| c.0).max().unwrap();
    let c0 = cells.iter().map(|c| c.1).min().unwrap();
    let c1 = cells.iter().map(|c| c.1).max().unwrap();
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

pub(crate) fn bbox(cells: &[(i64, i64, u8)]) -> Option<(i64, i64, i64, i64)> {
    let r0 = cells.iter().map(|c| c.0).min()?;
    let r1 = cells.iter().map(|c| c.0).max()?;
    let c0 = cells.iter().map(|c| c.1).min()?;
    let c1 = cells.iter().map(|c| c.1).max()?;
    Some((r0, r1, c0, c1))
}

/// CUT: the body's bounding box as its own whole, origin at the min cell.
pub fn cut(cells: &[(i64, i64, u8)]) -> Option<Vec<(i64, i64, u8)>> {
    let (r0, _, c0, _) = bbox(cells)?;
    let mut out: Vec<_> = cells.iter().map(|&(r, c, h)| (r - r0, c - c0, h)).collect();
    out.sort_unstable();
    Some(out)
}

/// MAGNIFIED ×k — each cell becomes a k×k block.
pub fn magnified(cells: &[(i64, i64, u8)], k: i64) -> Vec<(i64, i64, u8)> {
    if k <= 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    for &(r, c, h) in cells {
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
pub fn reduced(cells: &[(i64, i64, u8)], k: i64) -> Option<Vec<(i64, i64, u8)>> {
    if k <= 1 {
        return Some(cells.to_vec());
    }
    let (r0, r1, c0, c1) = bbox(cells)?;
    if (r1 - r0 + 1) % k != 0 || (c1 - c0 + 1) % k != 0 {
        return None;
    }
    let have = occupy(cells);
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
pub fn tiled(cells: &[(i64, i64, u8)], down: i64, across: i64) -> Vec<(i64, i64, u8)> {
    let Some(cut) = cut(cells) else { return Vec::new() };
    let Some((_, r1, _, c1)) = bbox(&cut) else { return Vec::new() };
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

/// BORDERED — a one-cell ring of `hue` around the cut body.
pub fn bordered(cells: &[(i64, i64, u8)], hue: u8) -> Vec<(i64, i64, u8)> {
    let Some(cut) = cut(cells) else { return Vec::new() };
    let Some((_, r1, _, c1)) = bbox(&cut) else { return Vec::new() };
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
