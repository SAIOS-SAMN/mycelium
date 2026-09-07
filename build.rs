// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Fail compilation when cognition reintroduces a Keyed walk or names
//! spend_unit as a number. The gates themselves are `xtask/lint/`, one
//! file per gate; this build script and `xtask`'s own binary run the same
//! module.

#[path = "xtask/lint/mod.rs"]
mod lint;

fn main() {
    println!("cargo:rerun-if-changed=src/server");
    println!("cargo:rerun-if-changed=src/client");
    println!("cargo:rerun-if-changed=src/transducer");
    println!("cargo:rerun-if-changed=src/network");
    println!("cargo:rerun-if-changed=src/bin");
    println!("cargo:rerun-if-changed=xtask/lint");
    if let Err(e) = lint::run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
