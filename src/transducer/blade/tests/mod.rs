// The blade's tests, one file per chapter of MU-RECURSION whose claim they
// prove (MU-STANDARDS 3.3). The `#[path]` attributes are here because a Rust
// module name cannot begin with a digit.

pub mod common;

#[path = "16.rs"]
mod chapter_16;
#[path = "20.rs"]
mod chapter_20;
