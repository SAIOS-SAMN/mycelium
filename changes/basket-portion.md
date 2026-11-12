### A survival basket's entries hold each element's dipole as whole counts (#238)

**Network contract** (v0.5.0 line): `Stated::Basket` and `UniverseReading` change their encoding, and the statement's signing domain moves to `mycelium universe statement v2`.
- **One entry per element**, named by the protons of one nucleus. The entry carries the person-year portion's totals (`protons Z`, `neutrons Q`, `bound_electrons E_bound = 3Z − U`, `mass M = 3(Z + Q)`) as exact whole counts (78.182 item 10; the operator's ruling of 2026-11-12 that the counts are the portion's totals).
- **Refused `Malformed`:**
  - an empty basket, or elements not ascending;
  - an element of 0 protons;
  - a portion with no protons, or not whole nuclei of its element;
  - bound electrons past `3Z`;
  - a mass other than `3(Z + Q)`.
- **`Q` is the integer the reserve states**, per universe and checked against no table (Concordance 1.13).
- **`BasketEntry::of(element, nuclei, neutrons, bound_electrons)`** derives the protons and the mass.
