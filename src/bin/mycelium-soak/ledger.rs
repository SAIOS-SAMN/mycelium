// §8.1 — READING A LEDGER'S COC1 SECTION WITHOUT BOOTING A COURT.
//
//! `snapshot/read.rs` refuses a ledger by naming the section it was in and
//! the byte it stopped at, which localises a failure to a section but not to
//! the object inside it. This walks COC1's complexes one at a time through
//! the same public `Complex::from_bytes` the restore uses — no second parser,
//! so nothing here can disagree with the reader it is diagnosing — and names
//! the complex that does not read and what it held.
//!
//! Prints only. The file is opened read-only and no court is constructed.

use mycelium::transducer::cech::Complex;

/// Walk the complexes at `at`, which must be the offset of the COC1 tag.
/// The offset is given rather than searched for: a four-byte scan through a
/// multi-megabyte ledger finds the tag's bytes wherever they occur as
/// content, and a diagnostic that guesses its own starting point reports on
/// whatever it happened to land in.
pub fn scan(path: &str, at: usize) {
    let Ok(b) = std::fs::read(path) else {
        println!("  ledger scan: {path} is unreadable");
        return;
    };
    if b.get(at..at + 4) != Some(b"COC1") {
        println!("  ledger scan: {path} does not open a section at {at}");
        return;
    }
    let mut pos = at + 4;
    let Some(raw) = b.get(pos..pos + 4).and_then(|s| <[u8; 4]>::try_from(s).ok()) else {
        println!("  ledger scan: {path} ends inside COC1's count");
        return;
    };
    let claimed = u32::from_le_bytes(raw) as usize;
    pos += 4;
    println!("  ledger scan: {path} COC1 claims {claimed} complexes");
    let mut held = 0usize;
    for i in 0..claimed {
        let Some(key) = mycelium::transducer::address::take_addr(&b, &mut pos) else {
            println!("  complex {i}: its key does not read");
            break;
        };
        match Complex::from_bytes(&b, &mut pos) {
            Some(c) => {
                held += 1;
                println!(
                    "  complex {i} {:02x}{:02x}…: {} sections, {} degrees",
                    key[0],
                    key[1],
                    c.sections.len(),
                    c.degrees.len()
                );
            }
            None => {
                println!("  complex {i} {:02x}{:02x}…: DOES NOT READ", key[0], key[1]);
                break;
            }
        }
    }
    // THE COUNT AGAINST THE CONTENT, which is the reading this exists for. A
    // section that claims more objects than it holds sends its reader past
    // the section's own end into the next one, and the refusal it raises
    // there names a place that has nothing to do with the fault.
    match claimed == held {
        true => println!("  ledger scan: {claimed} claimed, {held} read — the count is the content"),
        false => println!(
            "  ledger scan: {claimed} CLAIMED, {held} READ — the count is not a reading of the \
             content, and the reader believed the count"
        ),
    }
}
