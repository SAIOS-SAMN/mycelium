// K6.4's reading — WHAT A GENOME FILE HOLDS THAT ITS ADDRESSES DO NOT.
//
//! 54.5 rules the genome on disk to be the trie of addresses, "and a word's
//! value on disk is nothing, because the key is the form". Before that is
//! built, the claim has to be measured: `Value = Form ∖ Key` is a
//! statement about real words, and a word that carries something the key
//! does not determine would be lost by an empty value (law 7).
//!
//! This reads a GEN3 file word by word and reports, per word and in total:
//! the tape it occupies now, the address it would occupy instead, and what
//! it carries BEYOND its form — the magnitude, the A5 chain, the punctured
//! sections, the torsion cycles, the anchor. Nothing is decided here; it
//! is a reading, and the fix follows it rather than this file (§8.1).
//!
//! IT READS THE ENGINE, NOT A LOG. Every other number this session came
//! from an `eprintln!` line grepped out of a journal, which is string
//! matching a trace rather than asking the engine a question. This asks
//! the carrier directly, at a coordinate the operator names.

use mycelium::transducer::element::Element;

/// One word's accounting, in bytes.
#[derive(Default)]
struct Word {
    /// The tape the file holds for it today.
    tape: usize,
    /// The address it is reached by — the trie path under 54.5, and the
    /// whole of its identity.
    address: usize,
    /// The magnitude: four lanes, never summed (law 5). NOT derivable from
    /// the form — it is the descent measured over the bundle, which needs
    /// the train — so it is `Form ∖ Key` and the value must carry it.
    mag: usize,
    /// The A5 chain a residual word carries. For a SENTENCE this is empty
    /// and the remainder rides the form; for a taught residual it is the
    /// word's substance, and it is already inside the address (the key IS
    /// the chain).
    sub: usize,
    /// The punctured axis and its torsion cycles — the fibre a word
    /// deepened. Neither is in a form's key.
    sections: usize,
    torsion: usize,
    /// Whether the word is a sentence (a form) or a residual chain.
    sentence: bool,
}

/// A word whose FORM does not decode has no address reading here, and the
/// reading says so rather than taking what `Element::address` falls back
/// to. For a sentence the address IS `form_vertex_key` of the decoded
/// form; when the tape's version is not this build's, `vertex_address`
/// answers `None` and the fallback is the non-sentence path — a short key
/// that is not the form's and would read as a 50x saving that is not
/// there. Rule 11: exact or refused.
fn read(e: &Element) -> Option<Word> {
    // A sentence whose tape this build cannot read has no address reading:
    // `vertex_address` answers `None` and `Element::address` falls back to
    // the non-sentence path, a short key that is NOT the form's.
    let readable = !e.is_sentence()
        || e.description().get(1).copied()
            == Some(mycelium::transducer::sentence::FORM_TAPE_VERSION);
    if !readable {
        return None;
    }
    let mut w = Word {
        tape: e.description().len(),
        address: e.address().len(),
        mag: 32,
        sub: e.sub().len() * 18,
        sentence: e.is_sentence(),
        ..Word::default()
    };
    for s in e.sections() {
        w.sections += 4 + s.len() * 2;
    }
    for t in e.torsion() {
        w.torsion += 4 + t.sigma.len() * 2;
    }
    Some(w)
}

/// Walk a GEN3 genome file and report what its words carry.
pub(crate) fn report(path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("  genome refused: {path} does not read");
        std::process::exit(1);
    };
    match bytes.get(..4) {
        Some(b"GEN3") => {}
        _ => {
            eprintln!("  genome refused: {path} is not GEN3");
            std::process::exit(1);
        }
    }
    let Some(n) = bytes.get(4..8).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes) else {
        eprintln!("  genome refused: no count");
        std::process::exit(1);
    };
    let mut pos = 8usize;
    let mut words: Vec<Word> = Vec::new();
    let mut unread = 0usize;
    for _ in 0..n {
        let Some(len) = bytes.get(pos..pos + 4).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes)
        else {
            break;
        };
        pos += 4;
        let len = len as usize;
        let Some(slice) = bytes.get(pos..pos + len) else { break };
        pos += len;
        let mut ep = 0usize;
        match Element::from_bytes(slice, &mut ep).as_ref().and_then(read) {
            Some(w) => words.push(w),
            None => unread += 1,
        }
    }
    let n = words.len();
    if n == 0 {
        println!(
            "  {path}: {unread} words whose form this build does not decode, and no reading taken."
        );
        println!(
            "    The file's tapes are not this build's `FORM_TAPE_VERSION`, so every address \n    would be `Element::address`'s non-sentence FALLBACK rather than the form's key.\n    A bytes-per-word reading needs a genome written by this build: recut first."
        );
        return;
    }
    let sum = |f: fn(&Word) -> usize| words.iter().map(f).sum::<usize>();
    let tape = sum(|w| w.tape);
    let address = sum(|w| w.address);
    let mag = sum(|w| w.mag);
    let sub = sum(|w| w.sub);
    let sections = sum(|w| w.sections);
    let torsion = sum(|w| w.torsion);
    let sentences = words.iter().filter(|w| w.sentence).count();
    let file = bytes.len();

    println!("  {path}: {n} words read, {unread} unread, {file} bytes on disk");
    println!("    {sentences} sentences, {} residual chains", n - sentences);
    println!();
    // EXACT INTEGERS, NOT FLOATS (law 4). A reading printed for a human
    // is still a reading: bytes and an integer quotient say what a
    // rounded decimal only suggests, and the engine holds no float.
    println!("    component                 total bytes   bytes/word");
    for (name, v) in [
        ("tape (today's value)", tape),
        ("address (the trie path)", address),
        ("magnitude", mag),
        ("A5 chain (in the address)", sub),
        ("punctured sections", sections),
        ("torsion cycles", torsion),
    ] {
        println!("    {name:<26} {v:>13} {:>12}", v / n);
    }
    // WHAT THE VALUE MUST CARRY under 54.5: what the key does not
    // determine. The A5 chain is inside the address already (`the key IS
    // the chain`), so it is NOT counted here; the magnitude is not
    // derivable from the form and the fibre's sections and torsion are
    // not in a form's key.
    let value = mag + sections + torsion;
    println!();
    println!(
        "    VALUE = Form \\ Key: {value} bytes, {} per word, {} parts in 10000 of the tape",
        value / n,
        match tape { 0 => 0, t => value * 10000 / t }
    );
    println!("    the addresses alone: {address} bytes, {} per word", address / n);
}

/// WHY THE TAPE IS SO MUCH LARGER THAN THE ADDRESS, reconciled by the
/// tape's own decomposition rather than left as a gap. A key of 0.9 KB
/// beside a tape of 46 KB is only believable once the 45 KB is named.
pub(crate) fn reconcile(path: &str) {
    let Ok(bytes) = std::fs::read(path) else { return };
    let Some(n) = bytes.get(4..8).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes) else {
        return;
    };
    let mut pos = 8usize;
    let mut total = mycelium::transducer::sentence::TapeParts::default();
    let mut read = 0usize;
    for _ in 0..n {
        let Some(len) = bytes.get(pos..pos + 4).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes)
        else {
            break;
        };
        pos += 4;
        let len = len as usize;
        let Some(slice) = bytes.get(pos..pos + len) else { break };
        pos += len;
        let mut ep = 0usize;
        let Some(e) = Element::from_bytes(slice, &mut ep) else { continue };
        let Some(p) = mycelium::transducer::sentence::tape_parts(&e) else { continue };
        read += 1;
        total.total += p.total;
        total.addr_word += p.addr_word;
        total.addr_glued += p.addr_glued;
        total.carriage += p.carriage;
        total.charts += p.charts;
        total.sections += p.sections;
        total.faces += p.faces;
        total.loops += p.loops;
        total.rotor += p.rotor;
        total.certs += p.certs;
        total.demands += p.demands;
        total.rest += p.rest;
    }
    if read == 0 {
        return;
    }
    println!();
    println!("    the tape's own decomposition over {read} sentences:");
    let t = total.total.max(1);
    // DERIVED means `from_address` recomputes it and the file need not
    // hold it: the loops and their carriages come from `loops_of` on the
    // faces the key carries (§1.5: "loops and holonomies are derived and
    // never keyed"), and the certificates are evidence (54.2).
    for (name, v, derived) in [
        ("glued addresses", total.addr_glued, false),
        ("loops + carriages", total.loops, true),
        ("member carriages", total.carriage, false),
        ("charts", total.charts, false),
        ("sections", total.sections, false),
        ("faces", total.faces, false),
        ("certificates", total.certs, true),
        ("word addresses", total.addr_word, false),
        ("rotors", total.rotor, false),
        ("demands", total.demands, false),
    ] {
        println!(
            "      {name:<20} {v:>12} bytes {:>5} parts in 10000  {}",
            v * 10000 / t,
            if derived { "derived by from_address" } else { "in the key" }
        );
    }
    println!(
        "      {:<20} {:>12} bytes {:>5} parts in 10000",
        "unaccounted",
        total.rest,
        total.rest * 10000 / t as i64
    );
}
