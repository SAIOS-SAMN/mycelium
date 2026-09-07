// The sentence's tests, one file per chapter of MU-RECURSION whose claim
// they prove (MU-STANDARDS 3.3). The `#[path]` attributes are here because
// a Rust module name cannot begin with a digit.

pub mod common;

#[path = "10.rs"]
mod chapter_10;
#[path = "12.rs"]
mod chapter_12;
#[path = "16.rs"]
mod chapter_16;
#[path = "18.rs"]
mod chapter_18;
#[path = "19.rs"]
mod chapter_19;
#[path = "42.rs"]
mod chapter_42;
