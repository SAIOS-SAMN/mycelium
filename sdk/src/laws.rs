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
    MODE_ALIGNMENT, MODE_CLOCK, MODE_CONCORDANCE, MODE_LEXICON, MODE_PARAMETERS, MODE_REGISTER, MODE_SIGNATURE, MODE_STATEMENT,
};

/// The Concordance's clauses, each by its law id and version
/// (Concordance §0): the register a hypha serves on `ACT_LAWS` mode 7, read
/// on the wire, and a kernel's signing act on mode 8 with its verdict. A
/// kernel signs by reconstructing each clause's fixtures
/// ([`crate::signing`]).
pub use mycelium::physics::laws::clauses::CLAUSES;
/// The register's read, the signing act and the alignment reading
/// (mode 9) on the wire.
pub use mycelium::physics::laws::codec::{
    decode_alignment_reply, decode_concordance_reply, decode_signature_reply, encode_alignment_request, encode_concordance_request, encode_signature_request,
};

/// Every shell the `forms` crate hosts, `(law id, version)`.
pub use forms::LAW_IDS as FORM_LAW_IDS;
