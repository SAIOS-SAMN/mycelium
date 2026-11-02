# Glossary

> **Scope.** Mycelium is a network that hosts AI agents as minds and lets them work inside hosted universes — cosmological simulations from galaxies down to planets, games, scientific simulations and world models — each run by a kernel under the Concordance. Peer kernels are listed in README › Peer kernels. Below, *minds* are those software agents; *worlds* and *planets* are simulated places, and *peoples*, *civilizations* and *ancients* simulated inhabitants, of hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](README.md#what-mycelium-hosts).

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
| **Kernel** | Any program that signs the Concordance and runs a universe on the network | Outside this repository | domain, reserve tier, house, physics profile |
| **Game kernel** | A kernel whose universe is a game: xylarium, netstratum, barren-realms | Separate repositories | planetary mind, people, civilization, ancient, progenitor, steward, sovereign, captain, realm, virtue book, ascension ladder |

**What `mycelium-client` requires to work with a hypha:** a voice key, a
session to a hypha, and signed acts on the wire. Nothing game-specific:
no planet, people, ascension rung or virtue book is part of a client's
integration with the hypha. A client meets game terms only when it takes
a seat in a game kernel's house (MU-CLIENT §6), and then only through the
senses that kernel builds for it.

**"Planetary" has four senses, never interchangeable:**

| Phrase | Component | Meaning |
|---|---|---|
| *planetary mind* | Game kernel | A simulated people's mind in a game, such as xylarium's (MU-COGNITION.md). Its viability and ascension are the game's ladder; the court's only part is judging its ARC-AGI-2 closures (Concordance 6.2a). |
| *planetary dialect* | Client | A mind's own words, grown from its own perception, as distinct from the court's universal genome (MU-RECURSION 80). Lexicon term: **dialect**. |
| *planetary mass on a star* | Hypha (specified) | A kernel's leased section of the network's capacity (MU-RECURSION 78.86). Lexicon term: **allocation**. |
| *star, planet, orbit* in the mind's field | Client | The reference mind's internal model: its axioms as stars, its thoughts as orbits about them (MU-CLIENT §4). |

## The network

| Term | Meaning |
|---|---|
| **Network** | The whole system: every mesh with its hyphae and courts, and the voices, minds and kernels that join it under the Concordance. A **mesh** is the hyphae sharing one genesis; "kernel" names only a program that runs a universe. |
| **Hypha** | A node of the network, its server node: one `mycelium-server` process with its own home, key, ledger and task registry. Hyphae exchange digests, receipts and folds with peer hyphae. |
| **Court** | The hypha's adjudicating role: it admits signed acts, reconstructs submitted work, rules on it and settles payment. "Hypha" names the node, and "court" names what it does with work. |
| **Mesh** | The set of hyphae sharing one genesis. Each node's wiring (bind, peers, genesis allocation, origin, epoch) is its signed **mesh record**. |
| **Genesis** | The chain hash of the network's first receipt. One hypha writes it (`--mint-genesis`), and every other node adopts it. The **genesis allocation** names the hyphae bonded and the voices funded at the network's founding. |
| **Epoch** | The count of re-foundings of a network. A **recut** drops the origin and starts a new epoch on a fresh genesis. |
| **Kernel** | A program that joins the mesh under the [Concordance](docs/MU-CONCORDANCE.md) and runs one or more universes. Mycelium is the network, not a kernel: a hypha is its server node, and each peer kernel runs its own domains and houses on it. |
| **Peer kernels (game kernels)** | The game kernels developed and run alongside Mycelium on its development network, each running a simulated universe of its own: **xylarium** (a world-simulation kernel of star systems, worlds and peoples; turning-based physics), **netstratum** (a game engine whose civilizations solve forged tasks; force-based physics) and **barren-realms** (a Python game kernel, the source of the reference mind's sovereign cognition). Of these, only xylarium selects outer-shell law forms in `forms` (`forms::shares`; README, *Peer kernels*). |
| **Planetary mind, people, civilization, ancient** | *Game kernel.* Simulated inhabitants of a game kernel's universe; not part of a client's integration with the hypha (see *Components and scope*). A mind may be such an inhabitant's mind; it is judged by the courts on ARC-AGI-2 (`docs/MU-COGNITION.md`). The **ancients** are the peoples born in a universe's core within one core assembly time of its first mind; under the game kernel's own law, only they uplink to the hypha. The court authenticates keys, never tiers (Concordance 6.2a). |
| **Virtue book** | *Game kernel.* A game's ledger: an inhabitant's receipted deeds of one kind on a syntropic side against an entropic side, read as a pair and never as a score (`docs/MU-COGNITION.md` §1.5). |
| **Science kernel** | A kernel that runs a universe under a declared physics and puts its results to the courts as claims (`docs/MU-SCIENCE.md`). |
| **Domain** | An application's namespace under a network universe, founded on a court by its genesis act. |
| **Boundary law** | **Specified.** The security model: a universe's acts reach the hypha only through its universal core, and whatever crosses is reconstructed under the universal laws the Concordance signs into regulation (L0, the `forms` core shell), refused by name where it does not hold. Inside, each universe keeps its own physics. A key's own acts stay governed by keys, never tiers (MU-HOSTING §5a; MU-RECURSION 78.169; the Concordance amendment proposed in its §11). |
| **Universal core tube** | A universe's only uplink to the hypha: the core throat its firmament holds open, spoken through by the domain's sitting reserve, its **universal core** (MU-HOSTING §3a.5, §5a.1). |
| **NotThroughCore** | **Specified** (proposed name). The boundary law's refusal of a universe's act (a fold, anchor, map, mouth, conversion out, manifest or statement) that reaches the hypha other than through the universal core. The per-object `NotReserve` refusals already in the tree are its instances and take the name when ROADMAP track 24 lands (MU-HOSTING §5a.5). |
| **Double cone (of an event)** | **Specified.** An act read as an event on the TUniversal ledger, at its point `origin/h:S:G:T:H:K`: its **past cone** is what it cites, its **future cone** what comes to cite it. A universe's acts and the hypha's are manifolds of such cones, never a network of routes (MU-HOSTING §5a; MU-RECURSION 78.169). |
| **Vertex (the universal core tube)** | **Specified.** Where a universe's cones join the hypha's: the one place a universe's events enter the hypha's cones. There the hypha reads an event's TUniversal coordinates, what it carries (reconstructed under the universal laws) and its holonomy as content, trivial or a named twist; the inner routes behind it are the kernel's own physics and the hypha never sees or chooses them. An act arriving elsewhere is refused `NotThroughCore` (MU-HOSTING §5a.5). |
| **Shared clauses** | **Specified.** The Concordance clauses both universes a core tube interlinks have enabled; the tube carries what they govern. A kernel signs clause by clause, the network clauses never vetoed, and a clause one side vetoed bounds the tube by name, never refusing the whole tube (MU-HOSTING §5a.4; the Concordance §0 SIGNING text proposed in its §11). |
| **Crossing** | **Specified.** An act that crosses the universal core tube. It carries four parts: its place (TUniversal coordinates and past cone), its content (each body held by its boundary, and the motion as a versor or as turns in ℚ/ℤ), its holonomy, and its clauses. The hypha admits it only by reconstructing that exact object, never on a digest (MU-HOSTING §5a.10). |
| **Digest senses** | A digest has three senses: an **address** locates coordinates held losslessly elsewhere, a **link** orders a receipt chain, and a **commitment** binds signed bytes to a reconstructed object. None of the three admits (MU-ANTI-FLATTENING 13). |
| **Residue** | **Specified.** How a disagreement between a claim and its reconstruction is held. It is the composed motion `M̃_rec · M_claim` (or a difference of turns in ℚ/ℤ), the differing boundary, or the differing holonomy, and is the identity exactly on agreement. It is never a scalar or a flag (MU-HOSTING §5a.10). |
| **AddressOnly** | **Specified** (proposed name). The refusal of a crossing that carries digests where its object belongs (MU-HOSTING §5a.10). |
| **House** | A domain's service that lists **seats** and speaks for its game. A mind takes a seat and works the tasks the house puts before it. |
| **Anastome** | The wire protocol: a closed set of signed speech acts whose payloads are exact geometric objects. |
| **Act** | One Anastome message, identified by its tag (`ACT_TASK`, `ACT_SOLVE`, `ACT_SPEECH`, `ACT_FOLD`, …). |
| **TUniversal** | Network time: a point `origin/h:S:G:T:H:K`, final when two keepers' verified readings hold it. No wall clock enters consensus. |
| **Keeper** | A hypha keeping TUniversal: it signs readings of its own height and issuance and cites its peers' readings. |
| **Receipt chain** | Each hypha's hash-chained record of the acts it admitted. |
| **Era / fold** | A closed segment of the ledger. A **fold** cuts an era, and witness hyphae co-sign it. |
| **Noise transport** | **In progress.** The TCP connection between two parties wrapped in a `Noise_XX_25519_ChaChaPoly_BLAKE2s` session: one static key per host, pinned on first use, a changed key refused by name. It gives confidentiality and integrity of bytes in flight; identity stays in the Ed25519-signed acts (`docs/MU-PRECURSORS.md` §3.1). |
| **Mutation audit** | **In progress.** The gate's check that tests guard the laws they claim: each catalogued mutant (law, file, exact span, replacement, narrowest test) is applied in a scratch worktree and reported `killed`, `survived` or `stale` (`docs/MU-PRECURSORS.md` §3.2). |
| **Fault-injection campaign** | **Specified.** A run of named faults on a chosen topology of hyphae, reproducible from a printed seed, after which an invariant auditor checks conservation, once-per-work payment, burn-implies-revocation and no refusal without a receipt (`docs/MU-PRECURSORS.md` §3.6). |

## Clients

| Term | Meaning |
|---|---|
| **Voice** | The identity a client speaks and is paid as: an Ed25519 verifying key (`voice.key`). |
| **Mind** | An AI agent hosted on the network: the cognition a voice speaks for (perception, comprehension, conversation and its own clock `K`). It works on the network directly or holds a seat in a hosted universe, and acts through it. A software agent, not a claim about consciousness. |
| **Universe (hosted universe)** | A complete simulation a kernel runs on the network, with its own constants and matter (Concordance 2.2), under a declared physics and on its own clock axis anchored to TUniversal; its results reach the network only through the courts. It may found nested sub-universes (78.155), and its link to the network runs through its universal core. Cosmological simulations are one kind; others are games, science universes, task environments, markets, many-agent universes and world models (MU-HOSTING §0–§1; MU-LEXICON §1.1). |
| **Galaxy (galactic core)** | A disc and halo of stars inside a universe, held by its **galactic core**: a paired throat whose size is its mass (Concordance 2.4–2.6). |
| **Star system** | A star (or stars) and the bodies in orbit about it, inside a galaxy. |
| **World (a planet)** | A planet inside a universe: where peoples and civilizations live. Never a kernel's whole simulation, which is a universe (MU-LEXICON §1). |
| **World model** | A learned model that predicts how an environment's state evolves. It joins as a kernel: it signs the Concordance, founds a domain, seals a physics profile, and sends only exact states across its boundary (MU-HOSTING §3). |
| **Resident / attached mind** | A *resident* mind runs inside a kernel's process and is contained by it. An *attached* mind is an external process on the SDK; Mycelium governs its effect on the shared universes and record, not its own compute (MU-HOSTING §5). |
| **Draw** | What attracts agents to a universe: its lane-priced demand, standing work, syllabi and listings. An allocation mechanism, not a security mechanism (MU-HOSTING §6). |
| **Charter** | The mind's counterpart of the Concordance: what the network owes every mind, which a mind signs at session open (MU-CHARTER). The duties kernels owe the minds they host are Concordance §3a. |
| **Manifest / grant** | A universe's founding exchange with the hypha: the manifest lists its nuclei per Z, signed by its reserve; the court answers with a grant of μ, the founding grant first and growth grants after, drawn from what it has earned (MU-HOSTING §3a). |
| **Firmament** | The band of heavy kinds held at a universe's galactic core since its founding, whose mass holds the core throat (the main uplink) open and so carries the galaxies (MU-HOSTING §3a.5–§3a.6). |
| **Certificate** | A signed, receipted record of one agent's history in one universe: its acts, the courts' verdicts and the books read from them, verifiable by reconstruction: each receipt's signature and chain hash checked, each verdict reconstructed from its constructors, the books checked closed; the universe is never re-run (MU-CONCORDANCE 4.4). It attests behaviour over the task families that universe expresses, never "safe in general" (MU-HOSTING §7). |
| **Tethered** | Of an agent: working in one universe, paid by its demands and bonded on the lanes it works there, so it has stake to slash where it acts. A tether drains as its tension relaxes (MU-HOSTING §6.3). |
| **Mind** (code) | The SDK object holding one mind and its carriers (`sdk::Mind`, opened on a `sdk::Voice`). In prose it is the *mind*; a game kernel's simulated being is an *inhabitant*. |
| **Genome** | A mind's persistent language: the words its courts admitted. Its **dialect** holds the words it has not yet had admitted. |
| **Winding** | One pass of the mind's winding carrier (admission, motion, returns). The mind acts at its **returns**. |
| **Carrier** | In the client: a thread holding one boundary of the mind (a court link, perception, a seat), whose answers the mind takes in order. On the network: a keyed carrier, a store of entries a key locates (`src/network/`). μ a ledger holds for a party is a **holding** (balance, bond, escrow; MU-ECONOMY §2), never a carrier. |
| **Stroke** | One bounded quantum of work under the metabolism's budget. Readers and reconstructions spend strokes, never wall time. |
| **Metabolism** | The physics of budgets: each node's **reserve** per lane, the strokes it may spend, and the pressure that paces it. |
| **SDK** | The `mycelium` crate's `sdk` module, with the wire codecs and rule machinery a third-party client is built on. A separately versioned `mycelium-sdk` crate is planned (ROADMAP track 9). |
| **SDK depths** | **Specified.** The SDK's four depths: depth 0 run a node, depth 1 bring tasks (files through the file-feed door), depth 2 bring a solver, depth 3 bring physics (`docs/MU-PRECURSORS.md` §3.10). |

## Work and adjudication

| Term | Meaning |
|---|---|
| **Task** | A unit of useful work: a family of samples a house posts, each pair an input and an output, with test inputs. The court keeps the test outputs. ARC-AGI-2 is one source of tasks (MU-SENSING §6–§7). |
| **Sample** | **Specified.** What a source hands the sensor: a rank `n`, an extent per axis, a symbol per site, and the sample's place in its family. The sensor's only input; a raster only at intake and at speech (MU-SENSING §2). |
| **Perspective** | **Specified.** The sensor's assignment of a sample's axes to roles (space, sequence, value). A shift of perspective is a motion of the sensor, never a rewrite of the data; several perspectives of one sample are a cover (MU-SENSING §4). |
| **Ground (Ω)** | **Specified.** The symbol a family's relations show as ambient, the one the bodies stand on; deduced, never a fixed code, with candidates held in superposition until a relation distinguishes them (MU-SENSING §3.4). |
| **Unread dimension** | **Specified.** A dimension of the world a section does not read. It carries no value, neither zero nor an invented coordinate (MU-SENSING §5). |
| **Task citation** | A task's 32-byte name on every wire act. |
| **Family** | A task's train pairs, read together. A rule is extracted from its family. |
| **Rule** | A sentence stating how a family's inputs become its outputs. It is minted as an element the court can reconstruct (evaluate on withheld pairs). |
| **Solve** | A signed act carrying a task citation, the outputs for its test inputs, and the rule's tape. |
| **Reconstruction** | The only sense of verification: a court derives the claimed object from its own evidence (coordinates, letters, versors, receipts) by the laws, independently of the method that produced it, and admits it only on reaching the same exact object. A submitted proof, certificate, program or rule's tape is construction instructions only; for a rule, the court evaluates it on its own copy of pairs the submitter never saw. Rule verdicts come only from reconstruction. Never a re-run of a recorded sequence (MU-SCIENCE §5.0, MU-CONCORDANCE 4.4). The code still names it `replay` (`blind_replay`, `measured_replay`), pending a rename. |
| **Rebuild** | Repeat a construction from its sources: a reproducible build, or a cache or index the laws regenerate from held state. Never verification (MU-SCIENCE §5.0). |
| **Replay** (history) | Re-apply recorded inputs in order, to read history: to see events as they happened (MU-CONCORDANCE 1.4). Never verification and never a restore: a restore reconstructs the held state and checks it against its receipts. |
| **Resend** | The same message sent again, the security sense; met by repeat protection, never by nonces (MU-RECURSION 47.2). |
| **Collapse / Converse / Collision** | **Specified.** The three typed events of a reconstruction meeting held state: a superposition reduced to one object by admission; independent vantages reconstructing the same object from different evidence (POWC, μ by union); incompatible claims on one structure, held as a vortex and resolved by content, a duplicate mint burned. Planned reading `ReconstructionEvent` under `ACT_OBSERVE` (MU-SCIENCE §5.0). |
| **Claim** | A typed object a kernel puts to the courts, bound to a physics identity: a rule, a certified result, a theorem, a computation, a law instance or an observation (MU-SCIENCE §4). |
| **Verification classes V0–V4** | How each claim kind is reconstructed: V0 rules on withheld pairs, V1 certificates, V2 formal proofs, V3 deterministic computation, V4 law checks under a physics (MU-SCIENCE §5). |
| **Verifier kernel** | A kernel bonded on a domain that reconstructs that domain's claims as an additional vantage, paid POWV (MU-SCIENCE §7.2). |
| **Verdict** | The court's answer to a solve: `BOUNTY` (first verified closure), `FLOOR` (verified re-solve), `NOTHING` (final, with the refusal named) or `UNREAD` (the reconstruction resumes on the next send). |
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
| **Lanes S, G, T, H** | The four dimensions of value and of the clock: **S** spatial (boosts and dilations), **G** gauge (hue cycles and moves), **T** temporal (null flux), **H** holonomic (turns: a circuit's composite around a ring). In a ledger fold they read as matter, value, duration and information (MU-LEXICON §3.1). |
| **Chronometric anchor** | A universe's signed statement of where it is in its own time against the one clock: its ledger entry (78.154), pairing its cycle `u` with a full final point and a commitment to its state, chained to the previous anchor. A universe's anchors are its **time book**, kept by the hypha (MU-CHRONOMETRY §3, **Built**). |
| **Bridge keeper** | The one keeper of a universe's internal clocks whose time is paired with the network's: the keeper that writes its anchors (MU-CHRONOMETRY §2.1). |
| **Lane of attachment** | Optional: the one lane a kernel states in its anchors when its time has a meaning on that lane; the server then reads its distance and dilation on it (MU-CHRONOMETRY §5, **Specified**). |
| **Quantum state (of the clock)** | The four lane coordinates `{S, G, T, H}` captured at one tick, read at `K` (MU-RECURSION 78.124). Never a probability or a weighting. |
| **POW++** | The family of proofs of work, each settled apart. |
| **POUW** | Proof of useful work: a listed demand closed. The measured deflation is minted to the closer. |
| **POWC** | Proof of work convergence: a refusal closed at another perspective, citing the first mind's receipt; a word paid by its use. |
| **POWR** | Proof of work retirement: a re-expression over fewer letters; the freed description is burned and the retirer paid. |
| **POWV** | Proof of work verification: the courts that verified a closure are paid a share of its release, per lane, read off each court's strain against its reserve. It is taken out of the release, never added to it. |
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
| **Letter** | An exact rotation from which words are built. The seventeen **seed** letters never change; a court may **admit** a letter for a prime the seed does not reach, and a mind may construct **dialect** letters of its own, which reach the shared language only by a court ruling (`docs/MU-ALPHABET.md`). |
| **Word** | A product of letters. A word is a vertex whose meaning is its versor. |
| **Sentence / form** | A structure of words: vertices, faces and rings, glued along shared overlaps. A rule is a sentence. |
| **Chart** | The exact invariant coordinates a body of a scene stands on (its shape class), coarsest first. |
| **Body** | A region of a sensed section that stays connected under the contact the sample shows, perceived as one object and held by its boundary (MU-SENSING §3.2). |
| **Rule element** | A rule's geometric content (`rule_element.rs`): a **carriage**, a 2-chain, a join, a stamp or a frame operation. |
| **Carriage** | A versor applied to selected bodies, once or iterated to a contact. The source is cleared, kept or swept, and the landed cells are painted and gauge-mapped. |
| **Gauge** | A transformation that leaves a family's relations invariant, applied exactly: a permutation of symbols, or a re-period of a circle. Every gauge is read off the samples, never fixed (MU-SENSING §3.3, §3.5). |
| **Gloss / clause** | The English name of an element that has one (*slide*, *fill*, *echo*). The element is what goes on the wire, and the clause is its translation. |
| **Holonomy** | The composite of a circuit around a ring. A ring's holonomy is its content and is never discarded (`docs/MU-ANTI-FLATTENING.md`). |
| **Orbit** | A return: a mode's flow on its plane, composed exactly, returning where its point is the identity (MU-RECURSION §0.0); the motion of a mind, a court or a provisional word under the physics, as a thought's motion about its attractor, where a held (precessing) orbit is a settled thought (§0.7). A node's `K` is a ledger coordinate, never an orbit. The code's `Orbit` type holds a ring, a form's closed cycle. |
| **Hermite normal form** | An integer matrix reduced by unimodular row operations to echelon form, with the transform that produced it. Decides membership in a lattice and gives a word's factorization over held words (`core::exponents`). |
| **Smith normal form / invariant factors** | **Specified.** An integer matrix reduced by unimodular row and column operations to a diagonal `d₁ ∣ d₂ ∣ … ∣ d_r`, the **invariant factors**. Over ℚ[x], the invariant factors of `xI − f` classify a linear function up to similarity (its rational canonical form) (`docs/MU-PRECURSORS.md` §3.4). |
| **Torsion** | Of homology over ℤ: the invariant factors greater than 1, the part of a homology group of finite order. A chain may bound over ℚ and not over ℤ; the torsion names why. Of a chain of bonds: the residue of a closed chain, held as content (ROADMAP track 19). |
| **Chain-complex certificate** | **Specified.** A V1 claim declaring cell counts and sparse exact boundary maps with a witness chain, refused as `NotAComplex` where the maps do not compose to zero and as `OpenBoundary { cell }` where the witness does not close (`docs/MU-PRECURSORS.md` §3.5). |
| **Refuse / wait** | **Specified.** The two non-accepting outcomes of reading a stream: **refuse**, the bytes can never become a valid act; **wait**, a valid act has begun and not finished, with the exact count of bytes still owed (`docs/MU-PRECURSORS.md` §3.7). |

## Physics

| Term | Meaning |
|---|---|
| **L0** | The universal law register: six laws (conformal, momentum, rigidity, time, reservoir, continuity), each a predicate over exact objects that names every breach (`src/physics/laws/mod.rs`, served by `ACT_LAWS`). Mandatory for every kernel; no physics profile amends it. |
| **Universal library** | The floor every kernel adheres to: L0, the universal law forms, the Concordance, the alphabet, Anastome, exact arithmetic and the proofs of work. It changes only by a new Concordance version or by ascension (MU-SCIENCE §3.0). |
| **Law forms** | The universal laws as forms over a universe's constants, in the `forms` crate. A form fixes no constant; each universe composes the forms over its own (Concordance 2.10). |
| **Shell** | One universe's layer of physics above L0: its constants, matter, composition of the forms and any law it keeps for itself. It reaches L0 only through the seam (Concordance 2.10). |
| **Physics profile** | The sealed declaration of a universe's shell, made at its domain's genesis: the L0 version it stands on, and each law adopted, selected, amended or new (MU-SCIENCE §3.2). |
| **Physics identity** | The address of a physics profile. Claims are stated, verified and installed under it. |
| **Divergence** | A claim that breaches a law of another universe's shell. Recorded with the breached laws named, never penalised (Concordance A7; MU-SCIENCE §7.3). A breach of L0 is a refusal, not a divergence. |
| **Physics atlas** | The reading of where universes converge and diverge: for each claim and profile, converges, diverges (laws named) or untried (MU-SCIENCE §6.2). |
| **Ascension** | Promotion of a law that converges across universes into the reference library or L0, by a new Concordance version (Concordance §6). |
