// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 28, 78.25, 78.27 — PERSISTENCE: the mind written as keyed tries by its
//! own coordinates and read back by key, with everything derived
//! re-derived rather than restored. No section is read by its position.

pub mod read;
pub mod write;

/// THE LAYOUT'S ONE MAGIC (78.27): the mind as keyed tries, its line and
/// its eras on disk beside it (45.8).
pub const MAGIC: &[u8; 4] = b"MYT2";

/// THE TRIES WITH THE LINE AND THE ERAS INSIDE THEM, refused BY NAME: every
/// receipt and every era descriptor rewritten on every persist. The recut
/// is the road.
pub const PRE_LINE_MAGIC: &[u8; 4] = b"MYT1";

/// THE POSITIONAL TAPE (78.25), refused BY NAME and never migrated, as
/// `Element::PRE_WHOLE_MAGIC` is: its sections are read by where they
/// stand, which this reader does not do. One equality refuses it; there is
/// no reader for it (§10: the recut is the road).
pub const PRE_TRIE_MAGIC: &[u8; 4] = b"MYC7";

/// THE COURT: one node keyed by the key this court signs as, carrying its
/// court head; where its line is in the snapshot, the fold root that line
/// stands on; where its line is on disk, the stamp and hash the line stood
/// at when the snapshot was written, which the line must still hold.
pub const COURT_TRIE: &str = "court";
pub const LEAF_HEAD: u8 = 1;
pub const LEAF_ROOT: u8 = 2;
pub const LEAF_LINE: u8 = 3;
/// 78.120 — the domains founded on this court, each with its tier keys; absent
/// on a court that founded none (and on every home written before it).
pub const LEAF_DOMAINS: u8 = 4;
/// 78.123 B — the ledger trees: per child fold what remains of it, per domain
/// its rates gauge and its open obstructions; absent where none is held.
pub const LEAF_LEDGERS: u8 = 5;
/// 78.125 — the challenge windows: every fold recorded since 2(c), each
/// tier's last head, the rates edges in the order sealed, the arbitrages
/// booked; absent where none is held.
pub const LEAF_CHALLENGES: u8 = 6;
/// 78.126 — the handoffs: each offer held while a side is open, and the sides
/// each fold consumed; absent where none is held.
pub const LEAF_HANDOFFS: u8 = 7;

/// T2 — THE LEDGER (78.24, 78.27): a node per stamp, the per-act data
/// standing at it, and the act's receipt where the line is in the snapshot.
/// A stamp with rows or settlements and no receipt this court holds — the
/// pre-genesis origin, the fold root, a folded act — is a BARE node.
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
/// The eras held in hand — a kernel with no cold segment, a fixture — each
/// keyed by its root's stamp `h‖signer‖K‖i`: the root's height and place
/// and the signer are the key; its hash, its witness, the two signatures,
/// the identities it named, its first place and count, and its payload are
/// the leaves. A filed era is its own frame in the cold segment and is not
/// written here.
pub const ERAS_TRIE: &str = "eras";
pub const LEAF_ROOT_HASH: u8 = 1;
pub const LEAF_WITNESS: u8 = 2;
pub const LEAF_SIG_HOME: u8 = 3;
pub const LEAF_SIG_WITNESS: u8 = 4;
pub const LEAF_IDENTITIES: u8 = 5;
pub const LEAF_FIRST: u8 = 6;
pub const LEAF_COUNT: u8 = 7;
pub const LEAF_PAYLOAD: u8 = 8;

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

/// 78.68 — THE WINDING CARRIERS, each keyed by `chart ‖ simplex`, its one
/// leaf the carrier written by its own coordinates (`Winding::put`): the
/// ring's period, its traversals since its last return, its Torse rewrite,
/// and the pay accrued toward the next return — a claim, so it is kept.
pub const WINDINGS_TRIE: &str = "windings";
pub const LEAF_WINDING: u8 = 1;

/// 78.68 — EACH CHART'S HOLONOMY ALGEBRA, keyed by the chart's key, its one
/// leaf the basis: a count, then each bivector by its own coordinates.
pub const LIE_TRIE: &str = "lie";
pub const LEAF_BASIS: u8 = 1;
