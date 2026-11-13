### Fixed
- **Perception / Transducer**: Admit `1x1` non-identity dihedral frame symmetries in `read_tiles` (`acts.rs`) and add autonomous `TaskPercept` conformal grounding in `Mind::ground_from_percept` (`perceive.rs`) when scenes stop at `Stop::Unknown` or `Stop::NoRule`, speaking factorisation claims when no conformal `Tile` rule closes (#320).
