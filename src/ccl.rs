// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CCL DISPLAY SEAM — the ONLY place English names an ANASTOME germ. The kernel
//! germinates and reasons over anonymous exact elements; this module translates
//! one for a human, downstream and one-way. English READS the element; it never
//! defines it, and nothing in the solve path calls this. (Law III: kernel
//! general, translation interface-only.)

use crate::core::ccl::Audience;
use crate::transducer::motor::D4;
use crate::transducer::transform::Transform;
use num_traits::One;

fn rotation_word(d: D4) -> &'static str {
    // English gloss of a group element — a label, not a definition.
    match d {
        D4::E => "",
        D4::R90 => "quarter-turn",
        D4::R180 => "half-turn",
        D4::R270 => "three-quarter-turn",
        D4::FlipRow => "row-mirror",
        D4::FlipCol => "col-mirror",
        D4::Transpose => "transpose",
        D4::AntiTranspose => "anti-transpose",
    }
}

/// Translate an anonymous germ to an English phrase for a human audience. Purely
/// descriptive; the germ's identity remains its ANASTOME address.
pub fn describe(g: &Transform, audience: Audience) -> String {
    let mut parts: Vec<String> = Vec::new();

    if !g.motor.dilation.is_one() {
        parts.push(format!("scale ×{}", g.motor.dilation));
    }
    let rot = rotation_word(g.motor.rotation);
    if !rot.is_empty() {
        parts.push(rot.to_string());
    }
    let (t0, t1) = &g.motor.translation;
    if !num_traits::Zero::is_zero(t0) || !num_traits::Zero::is_zero(t1) {
        parts.push(format!("shift ({t0},{t1})"));
    }
    if !g.relabel.is_empty() {
        let pairs: Vec<String> = g.relabel.iter().map(|(a, b)| format!("{a}→{b}")).collect();
        parts.push(format!("recolour {{{}}}", pairs.join(",")));
    }

    let body = if parts.is_empty() { "hold (identity)".to_string() } else { parts.join(" ∘ ") };
    match audience {
        Audience::Mathematician => format!("germ: {body}"),
        Audience::Biologist => format!("hypha reading: {body}"),
        _ => body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::transform::solve_transform;

    #[test]
    fn english_lives_only_here() {
        // A motion∘recolour germ reads as English at the seam — the kernel never
        // produced these words.
        let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)];
        let b = [(0, 3, 5u8), (0, 4, 5), (1, 3, 5)];
        let g = solve_transform(&a, &b).unwrap();
        let s = describe(&g, Audience::Operator);
        assert!(s.contains("shift (0,3)") && s.contains("recolour {1→5}"), "{s}");
    }

    #[test]
    fn the_identity_reads_as_a_hold() {
        let a = [(0, 0, 1u8), (0, 1, 1)];
        let g = solve_transform(&a, &a).unwrap();
        assert_eq!(describe(&g, Audience::Operator), "hold (identity)");
    }
}
