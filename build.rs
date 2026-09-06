// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Fail compilation when cognition reintroduces a Keyed walk or names
//! spend_unit as a number. The same rules live in xtask/lint.rs.

#[path = "xtask/lint.rs"]
mod lint;

fn main() {
    println!("cargo:rerun-if-changed=src/server");
    println!("cargo:rerun-if-changed=src/client");
    println!("cargo:rerun-if-changed=src/transducer");
    println!("cargo:rerun-if-changed=src/network");
    println!("cargo:rerun-if-changed=src/bin");
    println!("cargo:rerun-if-changed=xtask/lint.rs");
    if let Err(e) = lint::run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
