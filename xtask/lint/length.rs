// MU-STANDARDS 1.1 — THE READING BOUND: no source file over 2500 lines,
// so a reviewer can read one whole.
//
// RAISED TO 2500 AND RETURNED, 2026-09-19. The raise landed at 3245041
// without being ruled into MU-STANDARDS, so it had no standing and it is
// restored here. The argument for it — that a bound which times an organ
// split is deciding architecture by a number — is a case to be ruled into
// the standard, not a change to be made on the strength of it.
//
// RAISED TO 2500, 2026-09-25, BY RULING. The change was ruled into
// MU-STANDARDS 1.1 and 4.1 before this changed — the step the 2026-09-19
// raise lacked. No
// other rule moves: an organ is split where the document splits it (1.2),
// never where a number does.

use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn lint_file_length(roots: &[PathBuf], hits: &mut Vec<String>) -> Result<(), String> {
    const BOUND: usize = 2500;
    fn walk_len(dir: &Path, hits: &mut Vec<String>) -> Result<(), String> {
        let rd = match fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(_) => return Ok(()),
        };
        for ent in rd {
            let ent = ent.map_err(|e| e.to_string())?;
            let path = ent.path();
            if path.is_dir() {
                walk_len(&path, hits)?;
                continue;
            }
            if path.extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }
            let n = fs::read_to_string(&path)
                .map_err(|e| format!("{}: {e}", path.display()))?
                .lines()
                .count();
            if n > BOUND {
                hits.push(format!(
                    "{}: {n} lines, over the {BOUND}-line reading bound — split it",
                    path.display()
                ));
            }
        }
        Ok(())
    }
    for root in roots {
        walk_len(root, hits)?;
    }
    Ok(())
}
