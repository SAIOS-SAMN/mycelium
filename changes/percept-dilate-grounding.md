### Conformal turn and uniform dilation grounding from `TaskPercept`

- Extended `Mind::ground_from_percept` with `uniform_scale` to lower uniform integer dilations/contractions (`RuleElement::Dilate { up, down }`) and combined dihedral turn + dilation symmetries (`RuleElement::Tile` + `RuleElement::Dilate`) from `TaskPercept` when scenes stop at `Stop::Unknown` or `Stop::NoRule`.
