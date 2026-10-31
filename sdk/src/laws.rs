// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-EQUATIONS, SHARED LAWS — THE LAWS A CLIENT PINS TO: the L0 register
// the hyphae serve on `ACT_LAWS` and the law forms the `forms` crate hosts,
// each by its id and version.

//! The laws a client pins to.
//!
//! The network serves one law register, L0: [`LAWS`] at [`LAW_VERSION`],
//! each law addressed by its coordinate (`Law::address`) and read back by
//! [`law_at`]. A client reads the register with
//! [`encode_register_request`] and [`decode_register_reply`]; the reply
//! decodes only at this register's version, so a hypha at another version
//! is told apart by name, never read as this one. The `forms` crate hosts
//! the law forms of the Concordance's nested shells, each named in
//! [`FORM_LAW_IDS`]. The shared-law table in MU-EQUATIONS lists every law
//! the network serves or shares, peer laws included, by id and version.

/// The L0 register: its laws in order, its version, and one law as it
/// is served.
pub use mycelium::physics::laws::{Law, LAWS, LAW_VERSION};

/// The register's read: the request (`[ACT_LAWS][0]`), the reply, and the
/// law at a coordinate.
pub use mycelium::physics::laws::codec::{decode_register_reply, encode_register_request, law_at, Served};

/// The register's parameters at its version, by name (`ACT_LAWS` mode 5).
pub use mycelium::physics::laws::PARAMETERS;

/// The `ACT_LAWS` sub-modes a client asks by.
pub use mycelium::physics::laws::codec::{
    MODE_CLOCK, MODE_CONCORDANCE, MODE_LEXICON, MODE_PARAMETERS, MODE_REGISTER, MODE_SIGNATURE, MODE_STATEMENT,
};

/// The Concordance's hash (BLAKE3 of its exact bytes at build time), the
/// bytes themselves, and the read of the hash a hypha serves
/// (`ACT_LAWS` mode 7).
pub use mycelium::physics::laws::concordance::{CONCORDANCE, CONCORDANCE_HASH};
/// The hash's read on the wire.
pub use mycelium::physics::laws::codec::{decode_concordance_reply, encode_concordance_request};

/// A kernel's signature over the Concordance, its verifier and its refusal
/// by name; presented to a hypha on `ACT_LAWS` mode 8. From v0.3.0 the
/// signature is accepted and verified; no kernel is refused for lacking one
/// until the following release.
pub use mycelium::physics::laws::concordance::{
    verify_concordance_signature, ConcordanceRefusal, ConcordanceSignature, SIGNATURE_LEN,
};
/// The presentation on the wire, and the verdict read.
pub use mycelium::physics::laws::codec::{decode_signature_reply, encode_signature_request};

/// Every shell the `forms` crate hosts, `(law id, version)`.
pub use forms::LAW_IDS as FORM_LAW_IDS;
