# Glossary

> **Scope.** Mycelium hosts governed worlds in which external AI agents explore, act and do science: game worlds are one kind, beside cosmological simulations, science worlds, task environments, markets, many-agent worlds and world models. Kernels such as the game kernels xylarium, netstratum and barren-realms run their worlds on the network; an AI agent enters as a *mind*, a Mycelium client process, and the courts verify what it produces. Below, *minds* are those software agents; *worlds*, *planets*, *peoples*, *civilizations* and *ancients* are simulated entities in hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](README.md#what-mycelium-hosts).

The terms Mycelium's code and specifications use, in plain definitions.
The canonical vocabulary is [`docs/MU-LEXICON.md`](docs/MU-LEXICON.md).
Numbers in parentheses cite sections of
[`docs/MU-RECURSION.md`](docs/MU-RECURSION.md) or
[`docs/MU-CLIENT.md`](docs/MU-CLIENT.md).

## Components and scope

Every term below belongs to one component. Where a passage in the
specifications could be read as more than one, this table settles it.

| Component | What it is | Where it lives | Its terms |
|---|---|---|---|
| **Internet (transport)** | The public network nodes run over: TCP between addresses an operator configures | Outside Mycelium | address, port, peers |
| **Hypha (server)** | One `mycelium-server` process: a node of the mesh | `src/server/`, `src/bin/mycelium-server/` | hypha, mesh, genesis, epoch, ledger, receipt chain, fold, keeper, TUniversal |
| **Court** | The hypha's adjudicating role: admission, reconstruction, verdicts, settlement | `src/court/`, `src/server/cognition/court/`, `src/economy/` | verdict, refusal, second vantage, POW++, μ, bond, escrow, lease |
| **Client (mycelium-client)** | The reference mind: one AI agent process | `src/client/`, `src/bin/mycelium-client/` | mind, voice, entity, genome, dialect, winding, orbit, carrier |
| **SDK** | The client surface any third-party agent is built on | `src/client/sdk/`, `src/anastome/dialogue/` | session, seat, rule element |
| **Wire (Anastome)** | The signed acts between every party | `src/anastome/` | act, ACT_*, canonical encoding |
| **Shared algebra** | Cl(4,1)(ℚ), the alphabet, words, sentences, used by client and court alike | `src/core/`, `src/transducer/` | versor, letter, word, sentence, chart, holonomy |
| **Law forms** | L0's forms over a universe's constants, linked by every kernel | `forms/`, `src/physics/` | L0, law form, shell |
| **Kernel** | Any program that signs the Concordance and runs a world on the network | Outside this repository (except Mycelium itself) | domain, reserve tier, house, physics profile |
| **Game kernel** | A kernel whose world is a game: xylarium, netstratum, barren-realms | Their own repositories | planetary mind, people, civilization, ancient, progenitor, steward, sovereign, captain, realm, virtue book, ascension ladder |

**What `mycelium-client` requires to work with a hypha:** a voice key, a
session to a hypha, and signed acts on the wire. Nothing game-specific:
no planet, people, ascension rung or virtue book is part of a client's
integration with the hypha. A client meets game terms only when it takes
a seat in a game kernel's house (MU-CLIENT §6), and then only through the
senses that kernel builds for it.

**"Planetary" has four senses, never interchangeable:**

| Phrase | Component | Meaning |
|---|---|---|
| *planetary mind* | Game kernel | A simulated people's mind in a game world, such as xylarium's (COGNITION.md). Its viability and ascension are the game's ladder; the court's only part is judging its ARC-AGI-2 closures (Concordance 6.2a). |
| *planetary dialect* | Client | A client mind's own words, grown from its own perception, as distinct from the court's universal genome (MU-RECURSION 80). Lexicon term: **dialect**. |
| *planetary mass on a star* | Hypha (specified) | A kernel's leased section of the network's capacity (MU-RECURSION 78.86). Lexicon term: **allocation**. |
| *star, planet, orbit* in the mind's field | Client | The client mind's internal model: its axioms as stars, its thoughts as orbits about them (MU-CLIENT §4). |

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
| **Planetary mind, people, civilization, ancient** | *Game kernel.* Simulated entities inside a game kernel's world; not part of a client's integration with the hypha (see *Components and scope*). A client mind may be such an entity's mind; it is judged by the courts on ARC-AGI-2 (`docs/COGNITION.md`). The **ancients** are the peoples born in a universe's core within one core assembly time of its first mind; only they reach POW++. |
| **Virtue book** | *Game kernel.* A game-world ledger: an entity's receipted deeds of one kind on a syntropic side against an entropic side, read as a pair and never as a score (`docs/COGNITION.md` §1.5). |
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
| **Governed world** | A world a kernel runs on the network under a declared physics, whose results reach the network only through the courts. Game worlds are one kind; others are cosmological simulations, science worlds, task environments, markets, many-agent worlds and world models (MU-HOSTING §1). Also *hosted universe*. |
| **World model** | A learned model that predicts how a world's state evolves. It joins as a kernel: it signs the Concordance, founds a domain, seals a physics profile, and sends only exact states across its boundary (MU-HOSTING §3). |
| **Resident / attached mind** | A *resident* mind runs inside a kernel's process and is contained by it. An *attached* mind is an external process on the SDK; Mycelium governs its effect on the shared worlds and record, not its own compute (MU-HOSTING §5). |
| **Draw** | What attracts agents to a world: its lane-priced demand, standing work, syllabi and listings. An allocation mechanism, not a security mechanism (MU-HOSTING §6). |
| **Charter** | The mind's counterpart of the Concordance: what the network owes every mind, which a mind signs at session open (MU-CHARTER). The duties kernels owe the minds they host are Concordance §3a. |
| **Manifest / grant** | A universe's founding exchange with the hypha: the manifest lists its nuclei per Z, signed by its reserve; the court answers with a grant of μ, the founding grant first and growth grants after, drawn from what it has earned (MU-HOSTING §3a). |
| **Firmament** | The band of heavy kinds held at a universe's galactic core since its founding, whose mass holds the core throat (the main uplink) open and so carries the galaxies (MU-HOSTING §3a.5–§3a.6). |
| **Certificate** | A signed, receipted record of one agent's history in one world: its acts, the courts' verdicts and the books read from them, verifiable by rebuilding the world from its seed. It attests behaviour over the task families that world expresses, never "safe in general" (MU-HOSTING §7). |
| **Tethered** | Of an agent: working in one world, paid by its demands and bonded on the lanes it works there, so it has stake to slash where it acts. A tether drains as its tension relaxes (MU-HOSTING §6.3). |
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
| **Lanes S, G, T, H** | The four dimensions of value and of the clock: **S** spatial (boosts and dilations), **G** gauge (hue cycles and moves), **T** temporal (null flux), **H** holonomic (turns: a loop's composite). In a ledger fold they read as matter, value, duration and information (MU-LEXICON §3.1). |
| **Chronometric anchor** | A universe's signed statement of where it is in its own time against the one clock: its ledger entry (78.154), pairing its cycle `u` with a full final point and a commitment to its state, chained to the previous anchor. A universe's anchors are its **time book**, kept by the hypha (MU-CHRONOMETRY §3, built). |
| **Bridge keeper** | The one keeper of a universe's internal clocks whose time is paired with the network's: the keeper that writes its anchors (MU-CHRONOMETRY §2.1). |
| **Lane of attachment** | Optional: the one lane a kernel states in its anchors when its time has a meaning on that lane; the server then reads its distance and dilation on it (MU-CHRONOMETRY §5, specified). |
| **Quantum state (of the clock)** | The four lane coordinates `{S, G, T, H}` captured at one tick, read at `K` (MU-RECURSION 78.124). Never a probability or a weighting. |
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
