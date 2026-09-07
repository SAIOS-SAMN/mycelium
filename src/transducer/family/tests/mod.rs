// The family's tests, one file per chapter of MU-RECURSION whose claim they
// prove (MU-STANDARDS 3.3). The `#[path]` attributes are here because a Rust
// module name cannot begin with a digit.

pub mod common;

#[path = "20.rs"]
mod chapter_20;
#[path = "21.rs"]
mod chapter_21;
#[path = "22.rs"]
mod chapter_22;
#[path = "26.rs"]
mod chapter_26;
