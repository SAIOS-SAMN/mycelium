// MU-STANDARDS 4.1, 6.1 — THE GATE BINARY: the reading bound and clippy
// with warnings denied, both run and both reported. Non-zero on either.

mod lint;

#[path = "lint/warnings.rs"]
mod warnings;

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

#[cfg(test)]
mod tests {
    use super::verdict;

    /// I4 — A TREE RED ON THE BOUND STILL REPORTS CLIPPY, and each reading
    /// is refused on its own.
    #[test]
    fn a_red_bound_still_reports_clippy() {
        let both = verdict(Err("bound: over".into()), Err("clippy: 67".into()));
        assert_eq!(both, Err("bound: over\nclippy: 67".into()), "both readings, the first not hiding the second");
        assert_eq!(verdict(Err("bound: over".into()), Ok(())), Err("bound: over".into()));
        assert_eq!(verdict(Ok(()), Err("clippy: 1".into())), Err("clippy: 1".into()));
        assert_eq!(verdict(Ok(()), Ok(())), Ok(()), "the tree passes only where both do");
    }
}
