// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 28, 78.25, 78.27 — PERSISTENCE: the mind written as keyed tries by its
//! own coordinates and read back by key, with everything derived
//! re-derived rather than restored. No section is read by its position.

pub mod read;
pub mod write;

/// THE LAYOUT'S ONE MAGIC (78.27): the mind as keyed tries.
pub const MAGIC: &[u8; 4] = b"MYT1";

/// THE POSITIONAL TAPE (78.25), refused BY NAME and never migrated, as
/// `Element::PRE_WHOLE_MAGIC` is: its sections are read by where they
/// stand, which this reader does not do. One equality refuses it; there is
/// no reader for it (§10: the recut is the road).
pub const PRE_TRIE_MAGIC: &[u8; 4] = b"MYC7";

/// THE COURT: one node keyed by the key this court signs as, carrying its
/// court head and the fold root its line stands on.
pub const COURT_TRIE: &str = "court";
pub const LEAF_HEAD: u8 = 1;
pub const LEAF_ROOT: u8 = 2;

/// T2 — THE LEDGER (78.24, 78.27): a node per stamp, the act's receipt and
/// the per-act data standing at it. A stamp with rows or settlements and no
/// hot receipt — the pre-genesis origin, the fold root, a folded act — is a
/// BARE node: its key, and its leaves, and no receipt.
pub const LEDGER_TRIE: &str = "ledger";
pub const LEAF_RECEIPT: u8 = 1;
pub const LEAF_ENTRIES: u8 = 2;
pub const LEAF_SETTLEMENTS: u8 = 3;
pub const LEAF_DIALECT: u8 = 4;

/// T1 — THE WORDS (78.27): a node per word, keyed by `Cite::Word{chart,
/// class, at}` written by its own `put_canonical`, carrying the element
/// held once with the rows it stands on and each per-word field.
pub const WORDS_TRIE: &str = "words";
pub const LEAF_HELD: u8 = 1;
pub const LEAF_DISCOVERER: u8 = 2;
pub const LEAF_PROVISIONAL: u8 = 3;
pub const LEAF_AWARD: u8 = 4;
pub const LEAF_DEFLATION: u8 = 5;
pub const LEAF_FIELD_HOT: u8 = 6;
pub const LEAF_FIELD_COLD: u8 = 7;
pub const LEAF_RANK: u8 = 8;
pub const LEAF_RULED: u8 = 9;

/// The rows a word stands on, as HELD's membership: the basis, the listing
/// book, the cavity, the federation, and — a sentence docketed under an
/// address that is not its own — spoken.
pub const MEMBER_BASIS: u8 = 1;
pub const MEMBER_LISTING: u8 = 2;
pub const MEMBER_CAVITY: u8 = 4;
pub const MEMBER_FEDERATION: u8 = 8;
pub const MEMBER_SPOKEN: u8 = 16;

/// T3 (LD3-b) — THE CARRIERS WITH THEIR OWN COORDINATES.
///
/// The eras, each keyed by its root's stamp `h‖signer‖K‖i` — the root's
/// height and place and the signer are the key; its hash, its witness, the
/// two signatures, the identities it named, its first place and count, and
/// where its payload is (filed at an offset of the cold segment, or carried
/// whole while it is being sealed) are the leaves.
pub const ERAS_TRIE: &str = "eras";
pub const LEAF_ROOT_HASH: u8 = 1;
pub const LEAF_WITNESS: u8 = 2;
pub const LEAF_SIG_HOME: u8 = 3;
pub const LEAF_SIG_WITNESS: u8 = 4;
pub const LEAF_IDENTITIES: u8 = 5;
pub const LEAF_FIRST: u8 = 6;
pub const LEAF_COUNT: u8 = 7;
pub const LEAF_FILED_AT: u8 = 8;
pub const LEAF_PAYLOAD: u8 = 9;

/// The complexes, each keyed by its own key; its node's one leaf is the
/// complex's own tries (`Complex::put_tries`): sections, simplices and
/// remainder, each by its own keys.
pub const COMPLEXES_TRIE: &str = "complexes";
pub const LEAF_TRIES: u8 = 1;

/// The cocycle rows, each keyed by its own key: the two voices, the class
/// INLINE by its own coordinates (78.27 (7)), the gauge as its ray, and the
/// stamp of the receipt the row stood at — the tick it replaced.
pub const COCYCLES_TRIE: &str = "cocycles";
pub const LEAF_FROM: u8 = 1;
pub const LEAF_TO: u8 = 2;
pub const LEAF_CLASS: u8 = 3;
pub const LEAF_GAUGE: u8 = 4;
pub const LEAF_STOOD: u8 = 5;
