### Fixed
- **Anastome (`tube.rs`)**: Replaced the hardcoded 5D odd-grade check (`[1, 3, 5]`) in `is_even` with `Mv::parity() == Some(false)`, generalizing frame parity checks to arbitrary grade-$n$.
