### Each Concordance signing is an event on the hypha's ledger, and alignment is read from it (track 24 B2; Concordance §0)
- **Network contract (v0.4.0's break):**
  - **Ledger layout:** a new receipt citation, `Cite::Signing` (tag 16):
    the kernel's key, the domain it is the sitting reserve of (`[0]` none
    or `[1][domain 32]`), and the verdict as mode 8 writes it. It is
    sealed under the existing `Domain` act kind. A build before this one
    does not read a receipt that carries it.
  - **Wire:** `ACT_LAWS` mode 8 now takes the court and seals the verdict
    before it answers; a verdict the line cannot seal answers the new
    refusal `Unrecorded` (tag 7). New mode 9, `[ACT_LAWS][9]` or
    `[ACT_LAWS][9][kernel 32]`: the alignment reading. The statement
    gains `laws alignment`, MUST for the kernel profile and MAY for the
    mind profile.
- **Recorded:** a signing act whose signature reconstructs
  (`Reconstructed::signing`) is judged (`clauses::judge`, which takes only
  a reconstructed act) and recorded, admitted or refused
  (`Court::hear_signing`). A `NotSignatory` is held on the ledger as the
  residue naming both sides. An act whose signature does not verify is
  recorded nowhere. A re-signing is a new event.
- **Read:** `Court::alignment` derives, per kernel and clause version,
  converses, vetoed, collides (with the residue) or unread from the
  signing events the hypha holds (its own line, hot and in its eras, and
  every peer receipt a signature reached), the latest event per clause.
  Nothing is stored beside the events. A peer's signing receipt stays hot
  past its signer's fold. A stroke spent answers `StrokeSpent`.
- **History:** v0.3.0's signings (netstratum and xylarium, Concordance
  version 4, by the retired text hash) are served beside the reading as
  history (`clauses::history`), never admitted as signatures.
- **SDK:** `signing` and `house` export the reading and mode 9; the house
  example prints its alignment after signing; the live KERNEL read checks
  that a clause signed with a fixture presented wrong collides with its
  `NotSignatory` residue and that every other clause converses.
- **Next:** `NotSignatory` at the universal core tube (B3, B4).
