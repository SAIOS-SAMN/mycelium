// The mind's tests, one file per chapter of MU-RECURSION whose claim they
// prove (MU-STANDARDS 3.3). The `#[path]` attributes are here because a Rust
// module name cannot begin with a digit and the chapter number is the name
// that matters.

pub mod common;

#[path = "13.rs"]
mod chapter_13;
#[path = "21.rs"]
mod chapter_21;
#[path = "23.rs"]
mod chapter_23;
#[path = "24.rs"]
mod chapter_24;
#[path = "26.rs"]
mod chapter_26;
#[path = "28.rs"]
mod chapter_28;
#[path = "42.rs"]
mod chapter_42;
#[path = "45.rs"]
mod chapter_45;
#[path = "47.rs"]
mod chapter_47;
