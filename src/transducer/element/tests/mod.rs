// The element's tests, one file per chapter of MU-RECURSION whose claim
// they prove (MU-STANDARDS 3.3). The `#[path]` attributes are here because
// a Rust module name cannot begin with a digit and the chapter number is
// the name that matters.

pub mod common;

#[path = "16.rs"]
mod chapter_16;
#[path = "17.rs"]
mod chapter_17;
#[path = "18.rs"]
mod chapter_18;
#[path = "19.rs"]
mod chapter_19;
#[path = "21.rs"]
mod chapter_21;
