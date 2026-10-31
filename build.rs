// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0 SIGNING — THE AGREEMENT'S HASH AT BUILD TIME: BLAKE3 of the
// exact bytes of `docs/MU-CONCORDANCE.md`, written as a constant the library
// includes (`physics::laws::concordance::CONCORDANCE_HASH`). The library
// also embeds the bytes themselves, and its tests claim that the constant is
// `chronometry::payload_hash` of them. Nothing here invokes cargo.

use std::io::Write;

const CONCORDANCE: &str = "docs/MU-CONCORDANCE.md";

#[expect(clippy::disallowed_methods, reason = "Concordance §0 — the object of this hash is the agreement's exact bytes, which a kernel signs")]
fn hash(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

fn main() {
    println!("cargo:rerun-if-changed={CONCORDANCE}");
    let refuse = |why: String| -> ! {
        eprintln!("build.rs: {why}");
        std::process::exit(1)
    };
    let text = match std::fs::read(CONCORDANCE) {
        Ok(t) => t,
        Err(e) => refuse(format!("{CONCORDANCE}: {e}")),
    };
    let Some(out_dir) = std::env::var_os("OUT_DIR") else { refuse("OUT_DIR is not set".to_string()) };
    let bytes: Vec<String> = hash(&text).iter().map(|b| format!("0x{b:02x}")).collect();
    let path = std::path::Path::new(&out_dir).join("concordance_hash.rs");
    let written = std::fs::File::create(&path).and_then(|mut f| writeln!(f, "[{}]", bytes.join(", ")));
    if let Err(e) = written {
        refuse(format!("{}: {e}", path.display()));
    }
}
