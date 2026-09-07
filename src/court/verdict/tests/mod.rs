// The verdict's tests, one file per chapter of MU-RECURSION whose claim they
// prove (MU-STANDARDS 3.3). The `#[path]` attributes are here because a Rust
// module name cannot begin with a digit.

pub mod common;

#[path = "23.rs"]
mod chapter_23;
#[path = "42.rs"]
mod chapter_42;
