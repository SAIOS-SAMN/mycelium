/// THE LAYOUT'S ONE MAGIC (78.27): the mind as keyed tries, its line and
/// its eras on disk beside it (45.8), every μ a holding of modes
/// (`economy::Mu` over `economy::modes::Modes`, MU-ECONOMY §9).
pub const MAGIC: &[u8; 4] = b"MYT4";

/// THE TRIES WHOSE μ WAS ONE RATIONAL PER LANE, refused BY NAME and never
/// migrated: every holding, row and award carried one value per lane where
/// the layout now carries its modes. The recut is the road.
pub const PRE_MODES_MAGIC: &[u8; 4] = b"MYT3";

/// THE TRIES WHOSE μ WAS FOUR `u64` COUNTS, refused BY NAME and never
/// migrated: every holding, row and award carried counts where the layout
/// now carries exact rationals. The recut is the road.
pub const PRE_RATIONAL_MAGIC: &[u8; 4] = b"MYT2";

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
/// FORMAT: court-tree leaf position for the fold root.
pub const LEAF_ROOT: u8 = 2;
/// FORMAT: court-tree leaf position for the line.
pub const LEAF_LINE: u8 = 3;
/// 78.120 — the domains founded on this court, each with its tier keys; absent
/// on a court that founded none (and on every home written before it).
/// FORMAT: court-tree leaf position for domains.
pub const LEAF_DOMAINS: u8 = 4;
/// 78.123 B — the ledger trees: per child fold what remains of it, per domain
/// its rates gauge and its open obstructions; absent where none is held.
/// FORMAT: court-tree leaf position for ledger trees.
pub const LEAF_LEDGERS: u8 = 5;
/// 78.125 — the challenge windows: every fold recorded since 2(c), each
/// tier's last head, the rates edges in the order sealed, the arbitrages
/// booked; absent where none is held.
/// FORMAT: court-tree leaf position for challenge windows.
pub const LEAF_CHALLENGES: u8 = 6;
/// 78.126 — the handoffs: each offer held while a side is open, and the sides
/// each fold consumed; absent where none is held.
/// FORMAT: court-tree leaf position for handoffs.
pub const LEAF_HANDOFFS: u8 = 7;
/// 78.147 — the fold roots this court holds, each with its two signatures, by
/// signer; absent where none is held (and on every home written before it).
/// FORMAT: court-tree leaf position for fold roots.
pub const LEAF_ROOTS: u8 = 8;
/// 78.154 — the atlas: each universe's ledger chain and map, the crossings'
/// edges, the final points held, the claims on stakes; absent where none is
/// held (and on every home written before it).
/// FORMAT: court-tree leaf position for the atlas.
pub const LEAF_ATLAS: u8 = 9;
/// 78.155b — the grants this court made, each with its manifest; absent
/// where none was made (and on every home written before it).
/// FORMAT: court-tree leaf position for endowment grants.
pub const LEAF_ENDOWMENT: u8 = 10;
/// 78.155c — the closures judged in each universe since its last grant, by
/// domain, per lane; absent where none is held (and on every home written
/// before it).
/// FORMAT: court-tree leaf position for closures.
pub const LEAF_CLOSED: u8 = 11;
/// 78.125 (law version 5) — the window book: the height version 5 holds
/// from and the sealed window closes; absent on every home written under law
/// version 4, whose windows close by version 4.
/// FORMAT: court-tree leaf position for the window book.
pub const LEAF_WINDOWS: u8 = 12;
/// MU-ECONOMY §3.4, §11.4 F2b — the closure points in this court's cones, each
/// with the stamp that sealed it first here; absent where none is held.
/// FORMAT: court-tree leaf position for closure points.
pub const LEAF_POINTS: u8 = 13;

/// T2 — THE LEDGER (78.24, 78.27): a node per stamp, the per-act data
/// standing at it, and the act's receipt where the line is in the snapshot.
/// A stamp with rows or settlements and no receipt this court holds — the
/// pre-genesis origin, the fold root, a folded act — is a BARE node.
pub const LEDGER_TRIE: &str = "ledger";
pub const LEAF_RECEIPT: u8 = 1;
/// FORMAT: ledger-tree leaf position for entries.
pub const LEAF_ENTRIES: u8 = 2;
/// FORMAT: ledger-tree leaf position for settlements.
pub const LEAF_SETTLEMENTS: u8 = 3;
/// FORMAT: ledger-tree leaf position for dialect.
pub const LEAF_DIALECT: u8 = 4;

/// T1 — THE WORDS (78.27): a node per word, keyed by `Cite::Word{chart,
/// class, at}` written by its own `put_canonical`, carrying the element
/// held once with the rows it stands on and each per-word field.
pub const WORDS_TRIE: &str = "words";
pub const LEAF_HELD: u8 = 1;
/// FORMAT: words-tree leaf position for discoverer.
pub const LEAF_DISCOVERER: u8 = 2;
/// FORMAT: words-tree leaf position for provisional.
pub const LEAF_PROVISIONAL: u8 = 3;
/// FORMAT: words-tree leaf position for award.
pub const LEAF_AWARD: u8 = 4;
/// FORMAT: words-tree leaf position for deflation.
pub const LEAF_DEFLATION: u8 = 5;
/// FORMAT: words-tree leaf position for field hot.
pub const LEAF_FIELD_HOT: u8 = 6;
/// FORMAT: words-tree leaf position for field cold.
pub const LEAF_FIELD_COLD: u8 = 7;
/// FORMAT: words-tree leaf position for rank.
pub const LEAF_RANK: u8 = 8;
/// FORMAT: words-tree leaf position for ruled.
pub const LEAF_RULED: u8 = 9;

/// The rows a word stands on, as HELD's membership: the basis, the listing
/// book, the cavity, the federation, and — a sentence docketed under an
/// address that is not its own — spoken.
pub const MEMBER_BASIS: u8 = 1;
/// FORMAT: bit flag for listing-book membership in the HELD byte.
pub const MEMBER_LISTING: u8 = 2;
/// FORMAT: bit flag for cavity membership in the HELD byte.
pub const MEMBER_CAVITY: u8 = 4;
/// FORMAT: bit flag for federation membership in the HELD byte.
pub const MEMBER_FEDERATION: u8 = 8;
/// FORMAT: bit flag for spoken membership in the HELD byte.
pub const MEMBER_SPOKEN: u8 = 16;

/// T3 (LD3-b) — THE CARRIERS, EACH WITH ITS OWN COORDINATES.
///
/// The eras held in hand — a court with no cold segment, a fixture — each
/// keyed by its root's stamp `h‖signer‖K‖i`: the root's height and place
/// and the signer are the key; its hash, its witness, the two signatures,
/// the identities it named, its first place and count, and its payload are
/// the leaves. A filed era is its own record in the cold segment and is not
/// written here.
pub const ERAS_TRIE: &str = "eras";
pub const LEAF_ROOT_HASH: u8 = 1;
/// FORMAT: eras-tree leaf position for the witness.
pub const LEAF_WITNESS: u8 = 2;
/// FORMAT: eras-tree leaf position for the home signature.
pub const LEAF_SIG_HOME: u8 = 3;
/// FORMAT: eras-tree leaf position for the witness signature.
pub const LEAF_SIG_WITNESS: u8 = 4;
/// FORMAT: eras-tree leaf position for identities.
pub const LEAF_IDENTITIES: u8 = 5;
/// FORMAT: eras-tree leaf position for first.
pub const LEAF_FIRST: u8 = 6;
/// FORMAT: eras-tree leaf position for count.
pub const LEAF_COUNT: u8 = 7;
/// FORMAT: eras-tree leaf position for payload.
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
/// FORMAT: cocycles-tree leaf position for the to-voice.
pub const LEAF_TO: u8 = 2;
/// FORMAT: cocycles-tree leaf position for class.
pub const LEAF_CLASS: u8 = 3;
/// FORMAT: cocycles-tree leaf position for gauge.
pub const LEAF_GAUGE: u8 = 4;
/// FORMAT: cocycles-tree leaf position for stood.
pub const LEAF_STOOD: u8 = 5;

/// 78.68 — THE RING CARRIERS, each keyed by `chart ‖ simplex`, its one
/// leaf the carrier written by its own coordinates (`RingCarrier::put`): the
/// ring's period, its traversals since its last return, its Torse rewrite,
/// and the pay accrued toward the next return — a claim, so it is kept.
pub const WINDINGS_TRIE: &str = "windings";
pub const LEAF_WINDING: u8 = 1;

/// 78.68 — EACH CHART'S HOLONOMY ALGEBRA, keyed by the chart's key, its one
/// leaf the basis: a count, then each bivector by its own coordinates.
pub const LIE_TRIE: &str = "lie";
pub const LEAF_BASIS: u8 = 1;
