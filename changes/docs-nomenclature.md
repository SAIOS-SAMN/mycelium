### The documents read in common terms and match the code

An audit of 36 documents (281 findings) was followed by the safe corrections in 33 of them, about 210 findings. Each statement about the code was checked against the source before it was changed.
- **Stale status corrected to the code:**
  - wire `0x0300`, law version 5, manifest v4, FORMS-COORDINATE v4, snapshot `MYT4`, line `MYL3`;
  - `NotSignatory` and B3 built, #327 closed, CI present, attachment built;
  - the `OPEN_PHYSICS` register as it stands;
  - a README `LeaseAct` example that did not compile, and the plaintext-across-hosts rule (refused `PlainAcrossBoundary`).
- **Retired words replaced where the replacement is fixed:** "utterance" becomes "expression", a path's "loop" becomes "orbit", verification by "rebuild" becomes reconstruction, and the transport's "frame" and "envelope" become "delimited message" and "task". The senses of "frame" that wait on #239 are left.
- **Caps emphasis** is in ordinary case, and the unsourced claims in MU-HOSTING §4 are removed.
- **Readability:** documents open by saying what they specify, project terms link to the GLOSSARY on first use, and MU-RELEASE §10 states the four rulings of 78.182 (items 14–17) word for word.
- **Not changed:** no GLOSSARY-defined term, no code identifier, no Concordance clause's rule, number or version, and DISCLAIMER.md. The 53 findings that need a ruling are listed on the pull request.
