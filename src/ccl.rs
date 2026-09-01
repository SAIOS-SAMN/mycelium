// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! CCL DISPLAY SEAM — the ONLY place English names an ANASTOME germ. The kernel
//! germinates and reasons over anonymous exact elements; this module translates
//! one for a human, downstream and one-way. English READS the element; it never
//! defines it, and nothing in the solve path calls this. (Law III: kernel
//! general, translation interface-only.) Post-excision there are no named
//! generator kinds to gloss — a germ is a germinated pattern, described only by
//! its measured shape (cells touched, lanes, magnitude), never by a solver word.

use crate::core::ccl::Audience;
use crate::transducer::element::Element;

/// Translate an anonymous germ to an English phrase for a human audience. Purely
/// descriptive; the germ's identity remains its ANASTOME address.
pub fn describe(g: &Element, audience: Audience) -> String {
    let cells = g.sub().len();
    let m = g.magnitude();
    let body = if g.is_identity() {
        "a pure transposition (no substance)".to_string()
    } else if cells > 0 {
        format!("a germinated pattern touching {cells} cell{}", if cells == 1 { "" } else { "s" })
    } else {
        "an algebraic generator".to_string()
    };
    match audience {
        Audience::Engineer => format!("{body}; lanes {:?}", g.lanes()),
        Audience::Mathematician => format!("{body}; μ⃗ = {m:?}"),
        _ => body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;

    /// The seam is one-way and downstream: English describes the germ's MEASURED
    /// shape; it names no generator kind (there are none to name).
    #[test]
    fn english_reads_the_pattern_but_never_names_a_kind() {
        let g = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap());
        let s = describe(&g, Audience::Operator);
        assert!(s.contains("pattern"), "described by shape: {s}");
        for banned in ["rotation", "recolour", "translate", "reflect"] {
            assert!(!s.to_lowercase().contains(banned), "no solver vocabulary: {s}");
        }
        let ident = Element::identity(crate::transducer::element::GERM_DIM);
        assert!(describe(&ident, Audience::Operator).contains("transposition"));
    }
}
