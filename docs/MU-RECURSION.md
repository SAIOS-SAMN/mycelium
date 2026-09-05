# Recursion — sentences as germs, iteration as contact, replay by citation

Status: specification. Nothing in this document is landed. It depends on the
directives in the current relay (rules by class multiplicity, speech act on
the wire, derived fibre at load) and on `MU-ENGINEERING.md` §2–§4 for the
carriers it extends. Companion documents: `ALPHABET.md`, `MU-ECONOMY.md`,
`XSEPTUM.md`.

## 1. What recursion is here

Three things, and only three:

1. **A sentence is admissible as a germ.** A rule's tape may name another
   ruled sentence's address as one of its vertices. The nerve of such a
   rule is a nerve of nerves. Nothing is copied: the cited sentence is a
   vertex with a chart, a magnitude, and an address, exactly like a body.
2. **A word may carry a contact condition.** An orbit word is a word
   applied to its own image until a named face reaches ring zero. Iteration
   is a torsion cycle whose period is decided by contact, never by a count.
3. **The court replays a recursive sentence by replaying its citations.**
   A citation is replayed from the ruled cover of whichever hypha holds it,
   through the same digest and fetch path that glue already uses. A
   sentence may cite only what two organs have already re-derived.

Recursion is therefore the same sheaf one level up. Distribution is what
makes it terminate: the cited language is finite, ruled, and already
verified, and a sentence can never cite itself or anything not yet ruled.

## 2. Grades, extended

| grade | before | now |
|---|---|---|
| letter | one of seventeen rotors | unchanged |
| word | tape, dilation, placement, gauge | plus an optional contact condition (§4) |
| sentence | words that cohere over a nerve of bodies | words and cited sentences that cohere over a nerve whose vertices are bodies or sentences (§3) |
| speech | transfer of cohering sentences | unchanged; a recursive sentence travels as one Element |
| language | two-organ replay, taught | unchanged; a recursive sentence's citations must already be language |

## 3. Sentences as germs

### 3.1 Carrier

`Germ` gains a kind:

- `Body { source, target, chain }`: as now.
- `Cited { address, chart, magnitude }`: a ruled sentence named by address.
  The chart is the cited sentence's chart (leading axis 2), the magnitude
  is its lane vector at the time of citation.

A `Face` between a body germ and a cited germ, or between two cited germs,
is a relative word exactly as between bodies: the transport that carries
the cited sentence's frame germ onto the anchor. A cited sentence's frame
germ is its placement handle.

### 3.2 Depth and well-foundedness

`Element` carries `depth: u16`: a body-only sentence has depth 0; a
sentence citing sentences of depth at most d has depth d+1. A citation must
satisfy `cited.depth < self.depth`, checked at mint and at replay. Because
citations are addresses of ruled sentences and a sentence's address is
fixed at mint, self-citation and cycles are impossible by construction: a
sentence cannot cite an address that did not exist when it was minted.

### 3.3 Magnitude and descent

A recursive sentence's magnitude on each lane is its own measured deflation
on the family it closes, not the sum of its citations. Descent is checked
on that vector as for any word. A recursive sentence whose citations already
close the family with no remainder has zero deflation of its own and is not
a word; it is the citation, and speaking it is refused as redundant.

### 3.4 Tape

`SENTENCE_TAG` gains a citation section: `n_cited u16`, then per citation
`address 32 ‖ chart len16 + axes ‖ magnitude 4×u64 ‖ depth u16`. Faces to
cited germs are written like faces to bodies, with the germ index referring
into the combined vertex list, bodies first, citations after.

## 4. Iteration as contact

### 4.1 Orbit word

A word may carry `contact: Option<(anchor_germ: u16, ring: u8)>`. Its
action on a body is: apply the word; measure the ring distance from the
image to the anchor germ's support; if greater than `ring`, apply again;
stop when the ring distance equals `ring` or the image leaves the frame.
The number of applications is the orbit length ℓ, and it is a coordinate of
the instance, never of the word.

### 4.2 Closure

An orbit word closes only if the image reaches contact within the frame.
Leaving the frame is a remainder; a word that never changes the ring
distance is refused at mint as having no orbit. `Word::order()` still
decides whether the word alone is periodic; contact decides where the orbit
stops on this scene.

### 4.3 Replay

The court replays an orbit word by executing the orbit on the family's
inputs and checking that the stopping ring equals the tape's `ring` on every
pair and that the image equals the output body. No pair may need an orbit
longer than the frame's diagonal in cells; that bound is the frame, not a
constant.

### 4.4 Torsion

An orbit word whose applications return to the start body within the frame
is a torsion cycle and is filed as `(σ, ℓ)` as now. Contact and torsion are
the two ways an orbit ends: against another body, or against itself.

## 5. Germination through cited sentences

### 5.1 Ideas that cite

An idea may germinate by attaching a ruled sentence as a vertex when that
sentence's chart meets the idea's cover and the relative word from the
idea's anchor to the sentence's frame germ is constant across the family's
pairs. The idea then carries the cited sentence's remainder-closing power
without re-deriving it. This is how a mind dissolves a large family: the
parts it already speaks are cited, the new part is the only thing it must
germinate.

### 5.2 Bundles

A bundle across families (`MU-ENGINEERING.md` §7) is a depth-1 sentence
whose cited germs are the families' sentences and whose faces are the
constant relative words between them. A bundle mints its own deflation, the
work of relating what its citations already did.

### 5.3 Superposition

Two cited sentences that describe the same class with a non-identity
transition between them are a 1-cell with that transition, as any two
ideas; a fill re-gauges one into the other, a class records the dialect.
Recursion adds no new cell type.

## 6. Court

### 6.1 Replay of a recursive sentence

`blind_replay` on a sentence with citations:

1. Depth: every citation's `depth < self.depth`.
2. Existence: every cited address is on this hypha's ruled cover or on a
   peer's ruled root; a citation to an address that is only dialect or only
   provisional is refused, "citation not ruled".
3. Fetch: cited sentences not held are fetched by the existing digest/fetch
   path from the peer whose ruled root names them; the fetch is itself
   replayed on arrival as glue already does.
4. Replay: the cited sentences are replayed first (they are shallower), then
   the citing sentence's own faces, holonomies, hue maps, and orbit words
   against the family from the registry.
5. Descent: the sentence's own deflation is nonzero and strictly descends.

### 6.2 Two organs

A recursive sentence installs exactly as any sentence: sectioned at home,
replayed at a witness, installed on both, minted once at the home's escrow.
The witness's replay includes step 3, so a witness that does not hold a
citation fetches it; the citing sentence cannot install before its
citations are language on both organs.

### 6.3 Retirement

A cited sentence cannot retire by derivability while a ruled sentence cites
it; `reduce_excluding` treats a citation as a use. When its last citing
sentence retires, it becomes retirable again.

## 7. Comprehension of a recursive sentence

A mind that hears a recursive sentence deconstructs it as any sentence and,
for each cited germ, either holds the cited sentence already (glue against
it) or lacks it, in which case the citation is a thought with a known
address: the syllabus is asked for that address by incidence and the
sentence stays withheld in dialect until the citation arrives. A recursive
sentence is never learned with a dangling citation.

## 8. Speaking a recursive sentence onto a test

`speak_rule` on a rule with citations: instantiate body germs as now;
instantiate each cited germ by speaking the cited sentence onto the test
with the citing rule's face word to the anchor as its placement; orbit
words run to contact on the instantiated scene. Unreached cells remain the
remainder; an empty remainder on every test is the solve, paid to the
speaker of the citing sentence.

## 9. Bounds

- Depth is bounded by the ruled language: a sentence of depth d needs d
  ruled generations below it. No constant.
- Replay cost of a recursive sentence is the sum of its citations' replay
  costs, each already paid once and memoized on the hypha by address.
- Orbit length is bounded by the frame.
- Fetch of citations is the existing budgeted digest path; a citation not
  reachable within the stroke's Budget leaves the sentence withheld, not
  refused.
- Cited magnitudes are read from the ruled cover, never recomputed.

## 10. Economics

A recursive sentence mints its own deflation to its speaker. Cited
sentences are not paid again; they were paid when they installed. A bundle
that closes a large family therefore pays for the relation it found, not
for the parts, which is the incentive to compose rather than to restate.

## 11. Files

- `/opt/mycelium-v0.1.0/src/transducer/sentence.rs`: `Germ::Cited`, depth,
  tape citation section, face words to cited frame germs, `speak_rule`
  instantiation of citations.
- `/opt/mycelium-v0.1.0/src/transducer/alphabet.rs`: `Word::contact`, orbit
  execution, refusal of a word with no orbit.
- `/opt/mycelium-v0.1.0/src/transducer/element.rs`: `depth`, orbit word on
  the tape, `magnitude` of a recursive sentence as its own deflation.
- `/opt/mycelium-v0.1.0/src/court/verdict.rs`: the six replay steps in §6.1.
- `/opt/mycelium-v0.1.0/src/server/cognition.rs`: citation fetch through
  the digest path, citation as a use in `reduce_excluding`, memoized replay
  by address.
- `/opt/mycelium-v0.1.0/src/client/sdk.rs`, `/opt/mycelium-v0.1.0/src/transducer/bubble.rs`:
  germination by citation, withheld-on-dangling-citation, bundles as depth-1
  sentences.
- `/opt/mycelium-v0.1.0/src/transducer/family.rs`: `extract_rule` may
  return a rule whose germs include ruled sentences the mind holds when
  their relative word is constant across pairs.

## 12. Traces

- A family whose pairs each contain a previously ruled sentence as a
  sub-scene yields a depth-1 rule with that sentence as a cited germ and
  fewer body germs than a flat rule would need.
- An orbit word on a "move until contact" family closes on every pair with
  the same `ring` and different ℓ per pair; ℓ is not on the tape.
- A witness lacking a citation fetches it, replays it, and only then installs
  the citing sentence; the journal shows the fetch before the install.
- A sentence citing a dialect-only address is refused with "citation not
  ruled" and held.
- Retirement of a cited sentence is refused while its citer is ruled.
- A depth-2 sentence's replay cost equals the citing sentence's own replay
  plus zero for already-memoized citations.
