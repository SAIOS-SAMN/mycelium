// The fixtures below name carrier types in a transport source in order to
// prove the check refuses them; they are written to a temporary directory.

use super::super::layers::{lint_layers, Exception, CARRIERS};

fn hits(text: &str, exceptions: &[Exception]) -> Vec<String> {
    let dir = tempfile::tempdir().expect("a directory to write into");
    std::fs::write(dir.path().join("wire.rs"), text).expect("written");
    let mut h = Vec::new();
    lint_layers(dir.path(), &["wire.rs"], CARRIERS, exceptions, &mut h).expect("the source reads");
    h
}

/// A transport source naming a carrier is refused with its file, line and
/// carrier, in a path as in a type; an exception passes only its own carrier
/// in its own file, and an exception that names nothing standing is stale.
#[test]
fn lint_layers_refuses_carrier_in_transport() {
    let text = "pub struct Envelope { pub body: Vec<u8> }\n\
                pub fn read(m: &crate::economy::Mu) -> Option<crate::transducer::cga::Mv> { None }\n";
    let h = hits(text, &[]);
    assert_eq!(h.len(), 2, "{h:?}");
    assert!(h.iter().any(|l| l.contains("wire.rs:2") && l.contains("`Mu`")), "{h:?}");
    assert!(h.iter().any(|l| l.contains("wire.rs:2") && l.contains("`Mv`")), "{h:?}");

    let listed = [
        Exception { file: "wire.rs", name: "Mu", until: "v0.5.0", why: "a fixture" },
        Exception { file: "wire.rs", name: "Lane", until: "v0.5.0", why: "a fixture" },
    ];
    let h = hits(text, &listed);
    assert_eq!(h.len(), 2, "{h:?}");
    assert!(h.iter().any(|l| l.contains("wire.rs:2") && l.contains("`Mv`")), "{h:?}");
    assert!(h.iter().any(|l| l.contains("`Lane`") && l.contains("stale")), "{h:?}");
}

/// A transport that moves bytes passes: lengths, budgets and peer keys, and
/// a carrier's name in a comment or a string, which are not identifiers.
#[test]
fn lint_layers_passes_bytes_only_transport() {
    let text = "/// Never a `Coordinate` or a `Lane`: bytes and a length.\n\
                pub struct Delimited { pub body: Vec<u8>, pub peer: [u8; 32] }\n\
                // A Sample is the mind's, not the wire's.\n\
                pub fn budget() -> &'static str { \"TPoint\" }\n";
    let h = hits(text, &[]);
    assert!(h.is_empty(), "{h:?}");
}
