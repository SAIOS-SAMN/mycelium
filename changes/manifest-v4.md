### Manifest v4: nucleons in thirds, the phase state per plane, paged

78.184 §8, approved 2026-11-16 (#359), agreed with both peers. Each entry is keyed by `(3Z, 3Q, pattern, planes)`:
- `3Z` and `3Q` are nucleons in whole thirds;
- `pattern` is a kernel's counts that separate nuclides of equal thirds (netstratum's lone-end counts);
- the planes are the phase state in composition order, each `(plane, K, R_p, σ_p)`.

Exact values are written compactly, about a tenth of v3's multivector scalars, and the entry count is a u32.

A plane is at rest when `R_p = −σ_p` (xylarium T24 per plane). The axis plane `0` is optional; where it is stated, its pole is the nucleon split. The tube's image is the reverse: planes reversed, phases and poles negated.

A manifest past one wire body is sent as MANIFEST-PAGE acts (sub-kind 20). The court holds the pages in order and judges the manifest once the last lands, answering `Granted::Paged` meanwhile. Element accounts and crossing gas count nucleons in thirds.
