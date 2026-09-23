// 78.15(1), CX1 — the Coxeter element's conjugacy, stated before it is read
// (78.8); its test in `tests/coxeter.rs`.

/// CX1 · TREE CONJUGACY. Where a set of constructors is a recognised
/// Coxeter system (GR2) and its diagram — the constructors, joined where
/// their product is not of order two — is a TREE, the product of all the
/// reflections in EVERY order is one conjugacy class: the Coxeter element,
/// its exponents and Coxeter number the group's, whatever circuit ordered
/// it. Where the diagram has a CYCLE the orders fall into several classes,
/// and the difference between the kept products' classes is content read by
/// comparing them, never by choosing one (C2a). For a set that is not a
/// system the product is only the product of its reflections, and no
/// conjugacy is asserted.
pub const CX1_TREE_CONJUGACY: &str =
    "a recognised Coxeter system whose diagram is a tree has one Coxeter element class over every order; \
     a cycle gives several, compared and never chosen";
