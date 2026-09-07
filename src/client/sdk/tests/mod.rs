// The client's tests, one file per chapter of MU-RECURSION whose claim they
// prove (MU-STANDARDS 3.3). The `#[path]` attributes are here because a Rust
// module name cannot begin with a digit.

pub mod common;

#[path = "6.rs"]
mod chapter_6;
#[path = "24.rs"]
mod chapter_24;
#[path = "25.rs"]
mod chapter_25;
