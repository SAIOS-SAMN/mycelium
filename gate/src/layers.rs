// AI-GENERATED CONTENT — see DISCLAIMER.md
//
// MU-ENGINEERING §9 — THE TRANSPORT HOLDS NO MEANING: it moves bytes between
// two endpoints (the connection, the channel's security, a length and the
// deferral of flow) and never names a carrier, the exact object that holds
// meaning at full dimension.
//
//! The transport's sources are listed once, in [`TRANSPORT`]. Each is read
//! as Rust tokens (`proc_macro2`, so comments and doc text are not read) and
//! every identifier naming a carrier type of [`CARRIERS`] is refused, with
//! its file and line.
//!
//! The exceptions stand in one short list, [`EXCEPTIONS`], each naming its
//! file, the carrier it names, the release that removes it and why it stands.
//! An exception whose carrier no longer stands in its file is refused as
//! stale, so the list shrinks when the code does and never outlives it.

use std::path::Path;

/// The transport's sources, repository-relative: the wire's delimitation,
/// the Noise channel, the choice of channel per peer and the hypha's door.
pub const TRANSPORT: &[&str] = &[
    "src/network/wire.rs",
    "src/network/noise.rs",
    "src/network/channel.rs",
    "src/bin/mycelium-server/transport.rs",
];

/// The carrier types the transport never names: the multivector, μ's modes
/// and its four-lane predecessor, the body coordinate, the samples and their
/// sites, a turn, the TUniversal point, and a lane.
pub const CARRIERS: &[&str] =
    &["Mv", "Modes", "Mu", "Coordinate", "Sample", "SectionSample", "SectionSite", "Turn", "TPoint", "Lane"];

/// One listed exception: a carrier a transport source still names.
#[derive(Debug, Clone, Copy)]
pub struct Exception {
    /// The transport source, as listed in [`TRANSPORT`].
    pub file: &'static str,
    /// The carrier type it names.
    pub name: &'static str,
    /// The release that removes it.
    pub until: &'static str,
    /// Why it stands until then.
    pub why: &'static str,
}

/// THE EXCEPTIONS, kept short so their growth is visible. Each is a known
/// violation of MU-ENGINEERING §9, removed by the release it names.
pub const EXCEPTIONS: &[Exception] = &[
    Exception {
        file: "src/network/wire.rs",
        name: "Lane",
        until: "v0.5.0",
        why: "the frame's lane byte: the envelope labels meaning; removed in the breaking wire release",
    },
    Exception {
        file: "src/bin/mycelium-server/transport.rs",
        name: "Lane",
        until: "v0.5.0",
        why: "the door writes the lane byte on a refusal; removed with the lane byte",
    },
    Exception {
        file: "src/network/wire.rs",
        name: "TPoint",
        until: "v0.5.0",
        why: "the pointed frame's trailer (78.152 TU-7): the point read rides the envelope, not the utterance; moved with the lane byte",
    },
    Exception {
        file: "src/network/noise.rs",
        name: "TPoint",
        until: "v0.5.0",
        why: "the channel carries the pointed frame's trailer; moved with the lane byte",
    },
    Exception {
        file: "src/bin/mycelium-server/transport.rs",
        name: "TPoint",
        until: "v0.5.0",
        why: "the door bounds a read by the point's trailer; moved with the lane byte",
    },
];

fn flatten(stream: proc_macro2::TokenStream, out: &mut Vec<proc_macro2::Ident>) {
    for t in stream {
        match t {
            proc_macro2::TokenTree::Ident(i) => out.push(i),
            proc_macro2::TokenTree::Group(g) => flatten(g.stream(), out),
            proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Literal(_) => {}
        }
    }
}

/// Refuse every carrier `carriers` names in the `transport` sources under
/// `repo`, but where `exceptions` lists the file and the carrier; refuse each
/// exception whose carrier no longer stands in its file. Answers the number
/// of identifiers read.
pub fn lint_layers(
    repo: &Path,
    transport: &[&str],
    carriers: &[&str],
    exceptions: &[Exception],
    hits: &mut Vec<String>,
) -> Result<usize, String> {
    let mut read = 0;
    let mut used: Vec<(&str, &str)> = Vec::new();
    for file in transport {
        let path = repo.join(file);
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let stream: proc_macro2::TokenStream =
            text.parse().map_err(|e| format!("{}: the layers gate cannot read the tokens: {e}", path.display()))?;
        let mut idents = Vec::new();
        flatten(stream, &mut idents);
        read += idents.len();
        for i in &idents {
            let name = i.to_string();
            let Some(carrier) = carriers.iter().find(|c| **c == name) else { continue };
            match exceptions.iter().find(|x| x.file == *file && x.name == *carrier) {
                Some(x) => {
                    if !used.contains(&(x.file, x.name)) {
                        used.push((x.file, x.name));
                    }
                }
                None => hits.push(format!(
                    "  {file}:{}: `{carrier}` — the transport names a carrier (MU-ENGINEERING §9): bytes, lengths, rtt, peer keys and capacity only",
                    i.span().start().line
                )),
            }
        }
    }
    for x in exceptions {
        if transport.contains(&x.file) && !used.contains(&(x.file, x.name)) {
            hits.push(format!(
                "  {}: the exception for `{}` (until {}) is stale: the carrier no longer stands there; remove it from the list",
                x.file, x.name, x.until
            ));
        }
    }
    Ok(read)
}
