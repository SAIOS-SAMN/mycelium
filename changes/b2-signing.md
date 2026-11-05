### A kernel signs the Concordance clause by clause, by reconstructing each clause's fixtures; the byte hash retires (track 24 B2; Concordance §0)
- **Network contract (v0.4.0's break):** `ACT_LAWS` modes 7 and 8 and
  the statement (mode 6) change layout. A v0.3.0 kernel's hash request
  reads the register's bytes, which no hash decoder reads, and its
  signature over a hash reads as no signing act (`Malformed`) or fails to
  verify (`Unsigned`).
  - **Mode 7** serves the clause register: `[0][count][clause …]`, each
    clause `[law id][version u16 LE][standing][fixture count][fixture …]`
    (standing 0 network, 1 kernel); or `[1][clause name]` where the hypha
    cannot read a clause of its own register.
  - **Mode 8** carries a kernel's signing act: `[kernel key 32][count]
    [clause …][sig 64]`, each clause its content address (name and
    fixtures' inputs), its assent (0 enabled, 1 vetoed) and, enabled, the
    kernel's reconstruction of each fixture's object; Ed25519 over
    `"mycelium concordance v2" ‖ key ‖ clauses`. The reply is
    `[0][count][(clause name, assent) …]` or `[1][refusal]`: `Malformed`
    (0), `Unsigned` (1), `UnknownClause` (2), `Repeated` (3),
    `NetworkVetoed` (4), `NotSignatory` (5: the clause, the fixture, the
    object presented and the hypha's), `Unregistered` (6).
  - **The statement** carries `clauses`, `[count][(law id, version u16
    LE) …]`, where it carried the 32-byte hash; the single-profile form
    of v0.3.0-rc.1 is no longer read.
- **The clause register** (`physics::laws::clauses`): each clause is a law
  form's id and version and its joint-read fixtures as exact inputs,
  addressed by that content written out, never a digest (A21); the hypha
  reconstructs each fixture through `forms`. Registered: `FORMS-PASSAGE`
  v3 (LAW-PASSAGE v2's four fixtures) and `FORMS-COORDINATE` v1
  (LAW-COORDINATE v1's order, eight sexagesimal sectors, seven `within`
  readings, two ranges). The passage is a network clause (standing 0),
  never vetoed. The coordinate is registered at kernel standing (1),
  signable and vetoable, until 78.173's fixtures are joint-read agreed
  and its clause is drafted: a clause whose fixtures no other kernel has
  read is not made unvetoable. A wording change of the Concordance
  changes no clause. The types are `ConcordanceClause` and
  `ConcordanceClauseId`, named apart from the rule language's clause
  (`transducer::sentence::form::clause`). The library depends on `forms`.
- **Retired:** `physics::laws::concordance` (`CONCORDANCE_HASH`,
  `CONCORDANCE`, `ConcordanceSignature`, `ConcordanceRefusal`,
  `verify_concordance_signature`, `SIGNATURE_LEN`), `build.rs` and its
  blake3 build dependency, and the statement's `concordance_hash`.
- **SDK:** `mycelium_sdk::signing`, a kernel's helper: `reconstruct_fixture`
  (the SDK's own reconstruction through `forms`) and `sign_concordance`,
  re-exported in `house`. The KERNEL profile's `laws concordance` reads
  the served register equal to the SDK's; `laws signature` signs it by
  reconstruction, admitted on every clause, and presents one fixture's
  object wrong, refused `NotSignatory` naming that clause and fixture.
- **Next:** the signed clause set recorded per domain, and `NotSignatory`
  applied at the universal core tube (B3, B4).
