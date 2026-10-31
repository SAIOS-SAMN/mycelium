// MU-STANDARDS 4.1, 6.1 — THE GATE BINARY: the reading bound and clippy
// with warnings denied, both run and both reported. Non-zero on either.
//
// `gate mutate` runs the mutation audit instead (mutate/): each
// protected law broken on purpose in a scratch worktree, its suite required
// to fail. Non-zero on any mutant not killed.

mod lint;
mod mutate;

#[path = "lint/warnings.rs"]
mod warnings;

#[cfg(test)]
mod tests;

/// I4 — BOTH READINGS, WHATEVER THE FIRST SAID (§4.1). The gate stopped at
/// the reading bound and never ran clippy while the bound was red, so 67
/// clippy errors accumulated unseen for two days behind six oversized files.
/// Each reading is taken and reported; the tree passes only where both do.
fn verdict(bound: Result<(), String>, clippy: Result<(), String>) -> Result<(), String> {
    let refusals: Vec<String> = [bound, clippy].into_iter().filter_map(Result::err).collect();
    match refusals.is_empty() {
        true => Ok(()),
        false => Err(refusals.join("\n")),
    }
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        None => gate(),
        Some("mutate") => match mutate::run() {
            Ok(true) => {}
            Ok(false) => std::process::exit(1),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1)
            }
        },
        Some(other) => {
            eprintln!("gate: unknown subcommand `{other}`; the subcommands are none (the gate) and `mutate`");
            std::process::exit(2)
        }
    }
}

/// The gate: the reading bound and clippy, both reported.
fn gate() {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let crate_dir = if manifest.join("src/server").is_dir() {
        Some(manifest)
    } else {
        manifest.parent().map(std::path::Path::to_path_buf)
    };
    let bound = lint::run();
    let clippy = match crate_dir {
        Some(dir) => warnings::run(&dir),
        None => Err("xtask has no parent".to_string()),
    };
    if let Err(e) = verdict(bound, clippy) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
