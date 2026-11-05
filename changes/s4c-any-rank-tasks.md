### The hypha serves tasks of any rank; a claim of factorisation on the wire (track 26 S4c)
- **The hypha serves tasks of any rank.** The registry holds each task as
  the source posted it (`PostedTask`, samples of any rank and width),
  named by that content alone (`TaskRegistry::post`), and serves it as
  posted (`PostedTask::envelope`). The word machinery's readers take the
  alphabet's section (`TaskRegistry::section`): a solve's replay, the
  families a court binds, and the routes. A task off the section is never
  refused for its rank; a solve on it is refused by the alphabet's name
  alone (`REFUSE_BEYOND_ALPHABET`), and its claims are factorisations.
- **A claim of factorisation** (`dialogue::factorisation`). A mind states,
  for one train pair, the motion (the signed permutation and the
  enclosure it is taken onto) and the residue (rank-`n` links), with its
  stated ground, signed. The court reconstructs the motion through
  `Factorisation::of`, derives the residue itself and admits on a match,
  else refuses with `FactorRefusal::Residue { unclaimed, uncarried }`. It
  never searches for a factorisation. The stated ground is read at any
  rank and width (`cognition::stated_ground_of`).
- **μ, ruled (2026-11-04): an admitted factorisation is never paid.** μ
  is the metabolic cost of traversal. A factorisation is an adjudicated
  perception, and its residue is meaning; meaning is content, never paid
  (MU-SENSING §3.7, §3.9). It is recorded on the book
  (`TaskRegistry::record_factorisation`), and its verdict carries no μ.
- **The mind speaks its factorisations** of a task beyond the alphabet,
  one claim per held factorisation (`Mind::speak_factorisations`).
- **Network contract**, inside v0.4.0's break:
  - **The solve act gains a claim kind**, the byte after its tag
    (`solve_kind`), by 78.126: the wire's bytes below the court's range
    are spent, so a new act is a sub-kind, never a new byte. No act byte
    is added.
  - Kind 0 (`SOLVE_KIND_OUTPUTS`) is the solve of outputs, its layout
    unchanged after the kind byte: `[ACT_SOLVE][0][voice 32][task
    citation 32][n u8][sample·n]`, then the optional rule and ground and
    the signature, now signed as `mycelium solve v4`. A solve without the
    kind byte no longer reads.
  - Kind 1 (`SOLVE_KIND_FACTORISATION`) is the claim of factorisation:
    `[ACT_SOLVE][1][voice 32][task citation 32][pair LEB128][rank
    u16][(source u16)(sense u8)·rank][period u16·rank][link count
    LEB128][link·count]`, optionally `[GRND][len u32][symbol]`, then
    `[SIGN][sig 64]` over the domain `mycelium factorisation v1`. A link
    is its site (`n` by LEB128, then `i64` coordinates) and its two
    symbols, each by LEB128 length.
  - The claim's verdict: `[0]` admitted; `[1][unclaimed
    links][uncarried links]`; `[2][reason, LEB128 length]`.
  - The SDK's `ACT_TABLE` is unchanged; `wire` re-exports `solve_kind`
    and the two kinds. The conformance statement names the solve rows at
    mode 0 and carries the claim as `solve factorisation` (`ACT_SOLVE`,
    mode 1), MAY for both profiles (`decode_factor_verdict`).
    `WIRE_VERSION` is unchanged.
  - No task address changes: a raster task's address was already its
    posted samples'.
  - SDK surface: `task` re-exports the codec and the factorisation types
    (`Motion`, `SignedPermutation`, `Enclosure`, `Sense`, `Link`,
    `factorisations`).
- **Not in this phase:** the word machinery's rank-2 carrier
  (`family::Grid`), the letters past `n = 3`, and versor motions past the
  signed permutations (T3).
