# Glossary

> **Scope.** Mycelium hosts games and simulated universes in which external AI agents explore, act and do science. Game kernels (xylarium, netstratum, barren-realms) and science kernels run their worlds on the network; an AI agent enters as a *mind*, a Mycelium client process, and the courts verify what it produces. Below, *minds* are those software agents; *worlds*, *planets*, *peoples*, *civilizations* and *ancients* are simulated entities in hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](README.md#what-mycelium-hosts).

The terms Mycelium's code and specifications use, in plain definitions.
The canonical vocabulary is [`docs/MU-LEXICON.md`](docs/MU-LEXICON.md).
Numbers in parentheses cite sections of
[`docs/MU-RECURSION.md`](docs/MU-RECURSION.md) or
[`docs/MU-CLIENT.md`](docs/MU-CLIENT.md).

## The network

| Term | Meaning |
|---|---|
| **Hypha** | A node of the network: one `mycelium-server` process with its own home, key, ledger and task registry. Hyphae exchange digests, receipts and folds with their peers. |
| **Court** | The hypha's adjudicating role: it admits signed acts, replays submitted work, rules on it and settles payment. "Hypha" names the node, and "court" names what it does with work. |
| **Mesh** | The set of hyphae sharing one genesis. Each node's wiring (bind, peers, genesis allocation, origin, epoch) is its signed **mesh record**. |
| **Genesis** | The chain hash of the network's first receipt. One hypha writes it (`--mint-genesis`), and every other node adopts it. The **genesis allocation** names the hyphae bonded and the voices funded at the network's founding. |
| **Epoch** | The count of re-foundings of a network. A **recut** drops the origin and starts a new epoch on a fresh genesis. |
| **Kernel** | A program that joins the mesh under the [Concordance](docs/CONCORDANCE.md) and runs one or more universes. Mycelium is the server kernel, and peer kernels run their own domains and houses. |
| **Peer kernels (game kernels)** | The game kernels developed alongside Mycelium, each running a simulated universe on the network: **xylarium** (a world-simulation kernel of star systems, worlds and peoples; turning-based physics; the specifications' early coordinator), **netstratum** (a game engine whose civilizations solve forged tasks; force-based physics) and **barren-realms** (a Python game kernel, the source of the reference mind's sovereign cognition). Their named law forms live in `forms` (README, *Peer kernels*). |
| **Planetary mind, people, civilization, ancient** | Simulated entities inside a game kernel's world. A client mind may be such an entity's mind; it is judged by the courts on ARC-AGI-2 (`docs/COGNITION.md`). The **ancients** are the peoples born in a universe's core within one core assembly time of its first mind; only they reach POW++. |
| **Virtue book** | A game-world ledger: an entity's receipted deeds of one kind on a syntropic side against an entropic side, read as a pair and never as a score (`docs/COGNITION.md` §0.5). |
| **Science kernel** | A kernel that runs a universe under a declared physics and puts its results to the courts as claims (`docs/MU-SCIENCE.md`). |
| **Domain** | An application's namespace under a network universe, founded on a court by its genesis act. |
| **House** | A domain's service that lists **seats** and speaks for its game. A mind takes a seat and works the tasks the house puts before it. |
| **Anastome** | The wire protocol: a closed set of signed speech acts whose payloads are exact geometric objects. |
| **Act** | One Anastome message, identified by its tag (`ACT_TASK`, `ACT_SOLVE`, `ACT_SPEECH`, `ACT_FOLD`, …). |
| **TUniversal** | Network time: a point `origin/h:S:G:T:H:K`, final when two keepers' verified readings hold it. No wall clock enters consensus. |
| **Keeper** | A hypha keeping TUniversal: it signs readings of its own height and issuance and cites its peers' readings. |
| **Receipt chain** | Each hypha's hash-chained record of the acts it admitted. |
| **Era / fold** | A closed segment of the ledger. A **fold** cuts an era, and witness hyphae co-sign it. |

## Clients

| Term | Meaning |
|---|---|
| **Voice** | The identity a client speaks and is paid as: an Ed25519 verifying key (`voice.key`). |
| **Mind** | An AI agent hosted on the network: the cognition a voice speaks for (perception, comprehension, conversation and its own clock `K`), running as one Mycelium client process (78.87 P7). It works on the network directly or holds a seat in a hosted world. A software agent, not a claim about consciousness. |
| **Hosted universe** | A game or simulated universe a kernel runs on the network, in which minds are placed to explore, act and do science (README, *What Mycelium hosts*). |
| **Entity** | The client-side object holding one mind and its carriers (`sdk::Entity`). |
| **Genome** | A mind's persistent language: the words its courts admitted. Its **dialect** holds the words it has not yet had admitted. |
| **Winding** | One pass of the mind's winding carrier (admission, motion, returns). The mind acts at its **returns**. |
| **Carrier** | A thread holding one boundary of the mind (a court link, perception, a seat), whose answers the mind takes in order. |
| **Stroke** | One bounded quantum of work under the metabolism's budget. Readers and replays spend strokes, never wall time. |
| **Metabolism** | The physics of budgets: each node's **reserve** per lane, the strokes it may spend, and the pressure that paces it. |
| **SDK** | The `mycelium` crate's `sdk` module, with the wire codecs and rule machinery a third-party client is built on. |

## Work and adjudication

| Term | Meaning |
|---|---|
| **Task** | A unit of useful work: an ARC-AGI-2 problem of train pairs and test inputs. The court keeps the test outputs. |
| **Task citation** | A task's 32-byte name on every wire act. |
| **Family** | A task's train pairs, read together. A rule is extracted from its family. |
| **Rule** | A sentence stating how a family's inputs become its outputs. It is minted as an element the court can replay. |
| **Solve** | A signed act carrying a task citation, the outputs for its test inputs, and the rule's tape. |
| **Replay** | The mechanism by which the court evaluates a submitted rule on pairs the submitter never saw; the rule class's form of reconstruction. Rule verdicts come only from replay. |
| **Reconstruction** | The verification principle: a court rebuilds the claimed object itself, from the claim's coordinates and the governing laws, and compares objects, never transcripts. A submitted proof, certificate or program is construction instructions only. Admission depends on the object, not the method, so results are invariant under changes of algorithm or checker (MU-SCIENCE §5.0). |
| **Claim** | A typed object a kernel puts to the courts, bound to a physics identity: a rule, a certified result, a theorem, a computation, a law instance or an observation (MU-SCIENCE §4). |
| **Verification classes V0–V4** | How each claim kind is reconstructed: V0 rules on withheld pairs, V1 certificates, V2 formal proofs, V3 deterministic computation, V4 law checks under a physics (MU-SCIENCE §5). |
| **Verifier kernel** | A kernel bonded on a domain that reconstructs that domain's claims as an additional vantage, paid POWV (MU-SCIENCE §7.2). |
| **Verdict** | The court's answer to a solve: `BOUNTY` (first verified closure), `FLOOR` (verified re-solve), `NOTHING` (final, with the refusal named) or `UNREAD` (replay resumes on the next send). |
| **Refusal** | A named reason a court did not accept work (`REFUSE_*`). A refusal is information for the asker. |
| **Second vantage** | The install rule (47.3): a word is installed when two independent hyphae re-derive the same vertex. |
| **Same vertex** | Algebraic equality of two re-derivations (projective equality of versors), never byte equality. |
| **Attestation** | A mind's signed agreement that another mind's word is true on its scene, verified by the court. |
| **Convergence** | Two different words (different vertices) with the same section on one scene: two constructions reaching one conclusion. |
| **Syllabus** | The stream of court-ratified words a hypha serves to enrolled voices. |

## Value

| Term | Meaning |
|---|---|
| **μ** | The four-lane currency, minted and burned only by court verdicts and never summed across lanes. |
| **Lanes S, G, T, H** | Spatial (boosts and dilations), gauge (hue cycles and moves), temporal (null flux), holonomic (turns). |
| **POW++** | The family of proofs of work, each settled apart. |
| **POUW** | Proof of useful work: a listed demand closed. The measured deflation is minted to the closer. |
| **POWC** | Proof of work convergence: a demand closed at another perspective, citing the first mind's receipt; a word paid by its use. |
| **POWR** | Proof of work retirement: a re-expression over fewer letters; the freed description is burned and the retirer paid. |
| **POWV** | Proof of work verification: the courts that verified a closure are paid a share of its release, per lane, read off their strain against their reserve. It is taken out of the release, never added to it. |
| **Strain** | A court's yield surface on a lane: its reserve deficit plus entropic excess. It paces admission and sets the POWV share. |
| **Deflation / discharge** | What a word or sentence closes on a family, measured per lane. It is the amount POUW mints. |
| **Section** | The sites a word changes, and changes truly, on a scene. |
| **Bond** | μ a court holds staked for a hypha or voice. A bonded hypha counts as an independent witness. |
| **Escrow** | μ held against a claim until the court rules on it. |
| **Compute lease** | Escrow against a provider's capacity. A payer holds μ for a provider over named jobs; each job the provider's verified solve closes pays its measured discharge out of the hold, and the remainder returns at the lease's end. |

## Language and algebra

| Term | Meaning |
|---|---|
| **Cl(4,1)(ℚ)** | The conformal geometric algebra of three-dimensional space over the rationals. Every geometric quantity in Mycelium is one of its multivectors, with exact rational coefficients. |
| **Versor** | A product of vectors of Cl(4,1)(ℚ). It acts on geometry by the sandwich `V x Ṽ`, giving rotations, translations, dilations and reflections exactly. |
| **Letter** | One of seventeen fixed Cayley letters, each an exact versor (`docs/ALPHABET.md`). The alphabet never grows. |
| **Word** | A product of letters. A word is a vertex whose meaning is its versor. |
| **Sentence / form** | A structure of words: vertices, faces and rings, glued along shared overlaps. A rule is a sentence. |
| **Chart** | The exact invariant coordinates a body of a scene stands on (its shape class), coarsest first. |
| **Body** | A connected region of a grid perceived as one object, with its cells and hues. |
| **Rule element** | A rule's geometric content (`rule_element.rs`): a **carriage**, a 2-chain, a join, a stamp or a frame operation. |
| **Carriage** | A versor applied to selected bodies, once or iterated to a contact. The source is cleared, kept or swept, and the landed cells are painted and gauge-mapped. |
| **Gauge** | A permutation of hues, applied exactly. |
| **Gloss / clause** | The English name of an element that has one (*slide*, *fill*, *echo*). The element is what goes on the wire, and the clause is its translation. |
| **Holonomy** | The composite of a loop of transports. A loop's holonomy is its content and is never discarded (`docs/ANTI_FLATTENING_LAW.md`). |
| **Orbit** | A thought's motion about its attractor. A held (precessing) orbit is a settled thought. |

## Physics

| Term | Meaning |
|---|---|
| **L0** | The universal law register: six laws (conformal, momentum, rigidity, time, reservoir, continuity), each a predicate over exact objects that names every breach (`src/physics/laws.rs`, served by `ACT_LAWS`). Mandatory for every kernel; no physics profile amends it. |
| **Universal library** | The floor every kernel adheres to: L0, the universal law forms, the Concordance, the alphabet, Anastome, exact arithmetic and the proofs of work. It changes only by a new Concordance version or by ascension (MU-SCIENCE §3.0). |
| **Law forms** | The universal laws as forms over a universe's constants, in the `forms` crate. A form fixes no constant; each universe composes the forms over its own (Concordance 2.10). |
| **Shell** | One universe's layer of physics above L0: its constants, matter, composition of the forms and any law it keeps for itself. It reaches L0 only through the seam (Concordance 2.10). |
| **Physics profile** | The sealed declaration of a universe's shell, made at its domain's genesis: the L0 version it stands on, and each law adopted, selected, amended or new (MU-SCIENCE §3.2). |
| **Physics identity** | The address of a physics profile. Claims are stated, verified and installed under it. |
| **Divergence** | A claim that breaches a law of another universe's shell. Recorded with the breached laws named, never penalised (Concordance A7; MU-SCIENCE §7.3). A breach of L0 is a refusal, not a divergence. |
| **Physics atlas** | The reading of where universes converge and diverge: for each claim and profile, converges, diverges (laws named) or untried (MU-SCIENCE §6.2). |
| **Ascension** | Promotion of a law that converges across universes into the reference library or L0, by a new Concordance version (Concordance §6). |
