// The complex's tests, one file per chapter of MU-RECURSION whose claim they
// prove (MU-STANDARDS 3.3). The `#[path]` attributes are here because a Rust
// module name cannot begin with a digit.

pub mod common;

#[path = "13.rs"]
mod chapter_13;
#[path = "14.rs"]
mod chapter_14;
