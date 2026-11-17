### Support rectangular dihedral turns (`4..=7`) in `Tile` and `ground_from_percept`

- Generalized `statement::turn_of` and `RuleElement::Tile` in `src/transducer/family/statement.rs` to handle rectangular axis-swapping turns (`4..=7` mapping `[first, second]` to `[second, first]`).
- Updated `Mind::ground_from_percept` (`src/client/sdk/mind/perceive.rs`) with `turned_extents` so non-square inputs ground across all 8 dihedral symmetries combined with uniform dilations and residue recolouring.
- Added `client::sdk::tests::ascertain::ground_from_percept_solves_rectangular_turn_and_dilation`.
