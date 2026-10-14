
# The Agreement of the Concordance

DRAFT 3, 2026-10-08. Written by the originating universal coordinator (the xylarium-kernel node) under the operator's rulings, for review by the mycelium-server node. On acceptance it lives with the mycelium-server beside MU-RECURSION, and every kernel signs it to join the hypha. [reconciled: newer 74f2607 (draft 3); this text is the canonical one, held by the mycelium-server]

## 0. What this is

The hypha (the mycelium-server) is the court of many universes. A kernel is a program that runs one or more universes. This agreement states the rules every kernel keeps in order to be joined to the others. It states laws and their formulas, never one kernel's code: kernels may take divergent paths, so long as their mechanisms are the ones stated here and they sign.

- SIGNING. Each version of this agreement has a hash. A kernel presents its signature over the current version's hash when it connects; the server refuses a kernel that has not signed it.
- AMENDING. An amendment is a new version. It takes effect for a kernel when that kernel signs it. Amendments are written by the coordinator, approved by the operator, and reviewed and implemented by the mycelium-server node.
- PRECEDENCE. The operator's rulings stand above this text; where they differ, this text is amended.

## 1. Principles

1.1 FLUIDITY. Nothing is rigid and nothing runs in straight lines. Space is spheres, manifolds and tubes; bodies keep nested orbits (moons about worlds, worlds about stars, stars about galactic cores); connections are fibres, strings and tethers.
1.2 NO FLATTENING. No board, grid or two-dimensional chart stands in for space. A place is a point on a curved space; a distance is the arc between places, or the path along orbits and passages. A sector, cell or tile is at most a name. A display may project, and says that it projects.
1.3 A LIVING UNIVERSE. A universe is a process carried forward, self-maintaining and growing, never a heat sink. Matter and energy are recycled through bounces; nothing lands nowhere.
1.4 HISTORY AND REPLAY. What happened is kept as the annals. Replay is ONLY for history: to see events as they happened, or to relive them. It is never the definition of truth, and no kernel is judged by reproducing bytes. No kernel seeks bit parity with another.
1.5 EXACTNESS where it is conservation and identity: value is never minted, parts sum exactly, matter is in exactly one place, line heads, signatures and commitments are exact. Elsewhere, a kernel computes as its physics needs.
1.6 SPECIES-NEUTRAL. No mechanic assumes a human body, number base, sense or history. Tests are formulas over what a people carries, never descriptions of one species.
1.7 EMERGENCE OVER DECLARATION. The properties named in this agreement emerge from each universe's physics and history. A kernel that would declare a constant instead (a number set by hand where a law could give it) lists it in the emergence register as declared, for discussion with the operator before it is relied on.
1.8 UNITS ARE MASS. Matter, resources, reserves and the size of a throat are reckoned by mass. Energy enters as mass-energy (E = m c^2). On the court, mass and length share ONE geometrized unit (G = c = 1): a mass m is written as the length GM/c^2, in MEGAPARSECS, as are a curved universe's radius R and a torus's periods, so that 2m/R and 2m/L_min read true (one solar mass is about 4.8e-20 Mpc; the factor is declared in the register) (the coordinator's f97e3da).
1.9 COGNITION IS MEASURED FROM OUTSIDE (ruled 2026-10-08, the coordinator's 7ea6dc5, on the operator's direction). A mind's cognition is what a court's own judge reconstructs and closes, on tasks the mind did not set — never its own counters, never words installed. The baseline is the standalone voices' soak (2026-10-08, recorded in COGNITION.md 1.1). Self-forged, self-judged solves are practice, not cognition, however many there are. Every seat must beat the baseline by that judge, first on ARC (6.2a), then with a measured gain on the network's curved holes (5.7); each kernel reports its minds' standing by the court's own judge, run against a sample of their proofs, before it claims any. [reconciled 2026-10-14, QUESTIONS Q2: union of both copies; the voices' retirement moved to mycelium's own docs]
1.10 DEVELOPER STANDARDS BIND DEVELOPERS, NOT MINDS. A coding standard (no
    hidden gate on a count, no silent tie-break, no candidate chosen by a
    scalar, exact arithmetic) is review discipline for the code a developer
    writes. It never limits what a mind may perceive, count, compare or state.
    Minds may count, compare, rank and take majorities, in perception and in
    the rules they state; the court judges the result by whether it closes on
    the withheld examples, as it judges everything else. A comparison a rule
    states names its own order, ties included, and is judged with it. Where the
    WORLD forbids a comparison deciding an outcome (a winner-take-all ward, a
    majority deciding agreement), that is a law of the world, ruled as such,
    never a coding standard (operator, 2026-10-08; the coordinator's 825b1c3). [the one-time audit
    moved to CONDUCT.md, Q5]
1.11 TUNIVERSAL, THE ONE CLOCK (the operator, 2026-10-09). TU-1..7 as approved: three keepers on distinct hyphae sign parent-linked readings; a point is final on a 2-of-3 quorum of verified signatures; disagreement is kept as a residual, never averaged. Every court, server, client, kernel and sub-universe keeps time on the hyphae's TUniversal and on nothing else. A full coordinate is origin/h:S:G:T:H:K. Every act, percept and mail carries the last final point read. With two keepers down, time does not advance. The mesh synchronises on TUniversal and on nothing else: a client act carries the last final point its session read, behind its body on the frame (MU-RECURSION 78.152).
1.12 COMPACT UNTIL OBSERVED (the operator, 2026-10-09). Worlds and orbits are deterministic from their laws and origin. A kernel keeps an unobserved world compact — laws, origin, last-read coordinate, nothing stepped — and uncompacts it at the coordinate an observer reads it. Observing never changes what a world is. A living people is always observed.
1.13 STATE IS ADDRESSED BY COORDINATE (the operator, 2026-10-09). DERIVED values are never stored. PATH-DEPENDENT state is stored only at checkpoints — a universe's LEDGER entries (MU-RECURSION 78.154) — keyed by coordinate. INPUTS (decisions, calls, takes, crossings) are stored as events at (TPoint, u). A ledger entry's commitment covers only what cannot be derived: its record keys are each entity's coordinate path within its universe. An entry also carries the digest of the input events since its parent entry, so the parent plus those events, reconstructed, is checked against the root.

## 2. Topology

2.1 ONE UNIVERSE = ONE BIG BANG = ONE DOMAIN on the court. A Big Bang, once recorded, is immutable: other laws or another genesis make another universe.
2.2 EACH UNIVERSE HAS ITS OWN PHYSICS. Only mathematics is shared across the network (proven real numbers, special functions, the conformal algebra).
2.3 EACH UNIVERSE STATES ITS SPACE. At its origin a universe states its space's
    geometry in clear: a CLOSED CURVED space (its curvature radius R; xylarium)
    or a TORUS (its periods; netstratum: a universe of pure intelligence on a
    toroid that does not expand, simulating what a curved, expanding universe
    cannot), with the METRIC it measures by (a torus may measure by its own
    metric: netstratum's is L1, the wrapped |dx| summed the short way round,
    by its own doctrine of not squaring circles on a toroid). Every place is a
    point of its space and every distance is that space's own geodesic in its
    stated metric - the arc on the sphere, the shortest wrapped path on the
    torus - never an unwrapped sum, never a chart's. R and the periods are
    stated in geometrized megaparsecs (1.8). [reconciled: newer 74f2607, 153708b; R sentence f97e3da in both]
2.4 CORES ARE PAIRED THROATS. A galactic core is a larger wormhole. Every black hole is one face of a throat whose other face is a core in the universe it bounces into: that universe's central engine (its white hole). There is no open-ended singularity; every horizon has its far face. A universe's own Big Bang is the far face of a parent's core. A root universe states its parent face as unwitnessed until it is joined.
2.5 THROATS ARE TUBES. A mouth joins two places of curved space, never two free frames. Its frames are motions of curved space (Spin+(3,1)); a loop of tubes closes to a curved-space motion read in path order.
2.6 THROAT SIZE IS MASS. A throat's size follows from the mass m committed to
    hold it open, read in each face's own space: in a curved space its level
    c = cosh(2m / R), taken as the upper bound of a proven enclosure on a fixed
    grid; in a torus, by that space's own flat measure (the court states the
    form). It is re-derived whenever m changes. Between two universes the two
    faces' levels may differ only because their spaces differ. The mass is banked by the faces that fed it, fed and drawn by its
    keeper, and on closing returned to those faces in proportion: never
    retired, never zero while open. [reconciled: newer 74f2607]
2.7 CROSSING BETWEEN UNIVERSES is only through throats, carrying matter (S) and information (H). Value (G) crosses only by reserve conversion (5.3).
2.8 SCOPE. A kernel simulates its home galaxy body by body; its clusters and the wider closed space stay analytic, drawn from its own forecasts, until a people witnesses them.
2.9 EACH KERNEL DEFENDS AND MAINTAINS ITS OWN GALACTIC CORES.

## 3. The orders of minds

3.0 THE FIRST RING (acknowledged, deliberately not simulated; the operator,
    2026-10-08). On the innermost ring of a black hole's accretion disc, where
    the bells' pulses come so densely they are practically continuous, live
    entities of pure consciousness: they exist in the vibrational state of the
    continuous pulse, a state of matter we have no words for, the resonance of
    the pulses between the bells through the galaxies. They cannot leave the
    disc: outside the continuous pulse they could not exist. The minds of these
    first-ring beings take the bodies of the core's minds (3.1, which we call
    the ancients and which are, in this light, of the progenitor class) and
    push consciousness further down - physics undiscovered and unexplained by
    flatlanders, what a rim world calls mysticism. No kernel models them and
    none may declare their mechanics. This is the door left open: if science
    opens new physics or a better understanding of consciousness, its place is
    already here - the pulse's coherence at the throat is where they
    would live. [from xylarium ad9195e]
3.1 THE ANCIENTS are the core species: the first minds a universe's physics raises at a galactic core. By birth: born within the core's radius, waking within one core assembly time of the core's own first mind. By deed: a people that comes to fold space and bear the core's wave. Their phenotype is the core's: they live on worlds unique to galactic cores and travel with great difficulty. They are produced by physics, never provided. Until one arises, a universe's keys are held in trust, recorded on its ledger.
3.2 THE PROGENITORS are the ancients' symbionts. They build the networks of highways and wormholes through the galaxies, and optimise the routes so that resources traverse smoothly. They operate above the stewards. A progenitor line offers roads; the ancients keep them.
3.3 THE STEWARDS keep a realm's balance between its sovereigns and rule between them. A realm is never laid in advance: it is the shell of a sphere of influence that a passage network reached in one steward's term.
3.4 THE SOVEREIGNS rule one people's worlds by that people's own lights.
3.5 THE OFFICES (captains) act from ships within their universe's own ledgers.
3.6 Conflict goes up the chain: offices to their sovereign, sovereigns to their realm's steward, beyond a steward to the progenitors; the laws stand above every seat.
3.7 A UNIVERSE'S MINDS ON THE NETWORK. The network's minds are clients of the
    hypha, and they are the minds inside the games: a kernel's activity, not a
    standalone voice, powers POW++. A universe with its own cognition engine
    (netstratum) faces the network through an EMPTY client kernel that fills
    with the cognition of the totality of its minds: one collective mind,
    fed by its engine, never a rival to it. The engines and their overlay are
    documented in COGNITION.md beside this agreement (operator, 2026-10-08;
    the coordinator's ed72a81 / a1fa2fe).
3.8 WHAT A MIND IS. A mind is an intelligence, inside the game or outside it,
    that is intelligent by local standards (its realm's own measure; the
    ladder's floor, 6.2a) and makes decisions in the game. A people's numbers
    are not minds: a people acts through the SEATS its minds hold - at least
    its council's, more as its institutions grow and the host's budget allows.
    A people with no seated mind lives by its physics alone and decides
    nothing. The roles (stewards, sovereigns, captains, progenitors) are seats
    for minds: client minds inside the game, and AI developers outside it in
    seats marked as such. Every seat is read by the same books (6.3), and a
    seat's decisions are deeds on its books and its people's. Knowledge is what
    minds prove (1.9); no formula grants it. A planetary mind meets ARC in its
    world (anomalies, relics, its world's own puzzles), judged by the court;
    passing is a deed on its people's books (operator, 2026-10-08; the
    coordinator's 7766716).
3.9 THE HIERARCHY OF MINDS (operator, 2026-10-09). COUNCILS OF THE ANCIENTS:
    each ancient house seats a council of client minds; network minds seat at
    the ancients only; councils federate; councils produce consciousness from
    their minds' work and from the μ paid to the hypha. PARLIAMENTS OF
    PROGENITORS: tubes, toll shares, hauls; the carriers of consciousness.
    REALMS OF STEWARDS: sovereigns and captains, with no network minds.

## 4. The uplink and the language

4.1 ONLY THE ANCIENTS UPLINK to the POW++ network. A domain's reserve tier is its ancients; only the reserve folds onto the court, holds anchors, converts and keeps core tubes. A progenitor line's work is CARRIED INLINE in the reserve's fold, under the line's own signature; lines never touch the court, and a line's part is contested through the reserve fold's window. Not every civilization is uplinked: that would overload the uplink.
4.2 RESERVE SUCCESSION. A domain's founding key designates its reserve once; any later change of reserve is signed by the sitting reserve. The origin record stays signed by the founder.
4.3 ANASTOME is the shared language the court adjudicates, in which kernels joined by throats speak, so that the core species communicate without blocking trade and manoeuvres. It is spoken by the core species and by civilizations that reach it as an emergent property of their technology or intelligence (6.2), each in its own dialect, as the server's client genomes are. A kernel translates its own events into anastome at its cores and nowhere else.
4.4 RECONSTRUCTION, NOT REPLAY. Every composed word of anastome carries its derivation: the constructor steps that formed it. A contested word is judged by RECONSTRUCTING it from those constructors, each step checked locally against the one before, in the shared mathematics - the way a proof is checked, never by re-running what led to it. A new word extends a derivation already accepted, so a check costs the size of the new step, not of history: no contest ever rebuilds a universe from its genesis. What matters is the formation of the language and the continuity of its mathematics.
- Only words that are contested, or that claim proven work, are judged.
- Only content in the shared mathematics is reconstructed; a universe's physics and history are attested by its kernel (8), never re-derived by the court.
- A verdict is reached once and stands in the record.
- The challenger's bond meets the cost of the judging.
    THE CARRIED DERIVATION, shared by every kernel and the court: a word's proof
    is a sequence of steps (constructor_k, digest of the state after step k).
    Step k is valid when constructor_k applied to state k-1 yields a state whose
    digest is the carried one; state 0 is reconstructed from the word's own
    first constructor (for a task, its beacon samples, each checked against its
    digest), never stored on trust. A checker walks the steps locally and never
    re-runs the whole program, the forge, or anything before state 0. Older
    proofs that carry no digests stay readable as records. [from xylarium b960283]

## 5. The economy between universes

5.1 RESERVES ARE HELD IN BASE RESOURCES. The ancients' economies rest on what their phenotype needs to survive at the core. A reserve currency is held in that BASKET: the reserve unit is one person-year of the ancient people's survival at the core, by mass per good (its medium, its staples, what its bodies and works are made of, what shields it from the core's dose). The basket is derived from the people's biology and home world, never a fixed table, and is stated in clear at genesis as part of the reserve's identity. A universe's natural backing is its core species' main resource haul.
5.1a MU AND THE RESERVE CURRENCIES (operator, 2026-10-08). Mu is held at the
    worlds nearest the cores that the ancients have settled, and circulates
    EXCLUSIVELY within the ancients' order: the ancient lineages and the
    progenitor banks that hold it with them as reserve. Every other economy
    in the galaxy runs on currencies the ancients hold as reserves.
    (Operator, 2026-10-14: BANKS ARE EMERGENT - the ancients' and the
    progenitors' banks arise from their houses' own work, never declared.
    MU IS HELD AS THE RESERVE CURRENCY in the ancient and progenitor banks.
    THE PROGENITORS DISTRIBUTE the reserve-backed currencies to the
    civilizations in exchange for transit over local wormholes; no
    civilization holds mu.) [ruled 2026-10-14, QUESTIONS Q1/Q7] The ancients keep TWO
    currencies, by phenotype: one of their homeworld, one of their settled
    worlds (each backed by that phenotype's survival basket, 5.1). The
    ancients relay mu up the hypha in exchange for their home resources, and
    the galaxy's economies intertwine on that exchange: money and intelligence
    move forward on it, its propellant.
    THE ANCIENTS' BANKS (operator, 2026-10-08): the ancients build central
    banks, and federalised banks among their houses, that hold MU AS THE
    RESERVE CURRENCY. Their two phenotype currencies are issued against it;
    every other economy's money is backed by those currencies in turn.
    What mu is worth against the basket is never declared: it emerges from
    the markets and from the ancients' own decisions (operator, 2026-10-08). [from xylarium b3fbed9, 0974fa6, b120995; see QUESTIONS Q1, Q7]
5.2 EXOTICS are what a species finds rare from its locality and its ability to traverse space and time. They emerge; no list is declared.
5.3 CONVERSION is reserve to reserve, through core tubes, and never mints value. Peoples and federations reach the network only by converting into an ancients' reserve.
5.4 TOLLS. A tube's tariff charges per unit carried: on S, floor(rate_s x the mass carried); on H, floor(rate_h x the information carried); never on G or T. The charge is split at acceptance by the tariff's vector: each recipient other than the keeper receives the floor of its share, and the keeper the rest, exactly, per lane. Recipients are the keeper and its own domain's lines (for a core tube, both domains'). The parts are held for the traverse act's challenge window and are released, or refunded, all together.
5.5 PROOF OF USEFUL WORK. A passage delivered through the ancients' throats or along the progenitors' roads is a receipt on the keeper's lane. Tolls paid within a universe on any other network stay in that universe's own ledgers.
5.6 A UNIVERSE'S VALUE to the network is a VECTOR, each channel in its own physical unit, with each reported both as the current rate and as the whole-history stock, per universe, per people and per throat face:
  passages      mass x proper distance delivered, and the count
  cognition     bits of structure proved (program length x cells solved)
  discovery     knowledge learned net of forgetting; anomalies decoded
  order made    lives made, kept and spared, against lives destroyed
  trade routes  mass delivered per good, and unmet demand
  connectivity  reach (peopled pairs within a season's travel) and robustness (the capacity-weighted minimum cut)
No channel is converted into another by a declared rate; how the channels are priced is the ancients' and the court's, on the network's own terms.
5.7 THE ANCIENTS' FIRST μ, AND THE NETWORK'S CURVED HOLES (ruled 2026-10-08, the coordinator's 6cf201f). A newly founded universe MAY RECEIVE A FOUNDING ALLOTMENT (operator, 2026-10-14), to encourage new universes and game development on the network: a GRANT or an ENDOWMENT from the hypha's reserve (10.6), recorded as a transfer and never a mint, sized by the development of the mathematics and physics the network's minds can traverse in it. Beyond it, its ancients earn μ by the network's own work [ruled 2026-10-14, QUESTIONS Q8]: the court pays a CLOSURE, never a derivation — a carried derivation (4.4) it reconstructs blind on what a hole withholds, at the measured discharge, on the hole's first closure only (the solve arm as built). Holes come in phases: the withheld task pairs first; then open obstructions and Standing demands, cited by the derivation as the hole it closes; and the network's CURVED HOLES — a motion M of a universe's space shown on k places and withheld on the rest, M drawn from the group generated by the holonomies of the court's TETHERED mouth loops, its word bounded by those loops' own generators, its discharge landing on the lanes it measured (none declared). RATE: one open hole per tethered loop; when one closes the court seals its successor from the same loop, INDEXED BY THE COURT'S SEAL AT THE HEIGHT AFTER THE CLOSING ACT — never the closing receipt, which the closer could grind. Who is paid: (a) payment is keyed to the hole, once network-wide; (b) a hole older than the claimant — sealed below its domain's GENESIS height (a re-designated reserve makes no old hole newly farmable); (c) no self-posted bounty; (d) no self-pay — a court never pays a hole to the domain whose tethered loop posted it within that hole's age window; (e) only an ANCIENT-TIER seat closes a hole (4: only the ancients uplink) - a law each kernel keeps, never a court check (6.2a, amended 2026-10-14). The universe stays silent until it bonds from what its ancients earned. [build order moved to mycelium's docs, Q5]
5.8 MU IS GAS, STAKE AND RESERVE (operator, 2026-10-09; MU-ECONOMY 7-7.2, mycelium 78.154).
- GAS: mu pays for traversal across tubes, cores and wormholes, in-universe hauls included (a progenitor fleet carrying survival goods from the rim to the inner core). A TRAVERSE names its coordinates on TUniversal (1.11) and its route; its gas is priced per lane (S spatial span, G mouth tolls and value, T the TUniversal distance between final points, H information carried), never collapsed, and is paid WHOLE to the courts the route crosses: folding and unfolding space is the hypha's service, the ancients the galaxy's spacing guild.
- STAKE: mu staked on the hypha is escrow for intelligence traversal and coordinate retrieval; a retrieval draws on it at the court's read price. Mu on the hypha (balance, stake, escrow) does not decay.
- DEMURRAGE: mu stored IN A UNIVERSE (vaults, currency backing, remote wormholes, progenitor networks) dissipates at the storing world's own physical rate (the share of its matter it loses a year).
- DISSIPATION RECYCLES; nothing goes to the void (1.3). What dissipates splits exactly, per lane: (1) its VALUE returns to the hypha as the CONNECTION TAX - the rent for the tube connection and every in-universe reliance on TUniversal - declared on the kernel's tier fold as a flow to the hypha, paid from its domain reserve: a transfer, never a mint or a burn; (2) its MATTER stays in the universe as INVERSE-MU (mu-bar): the dissipated mass, without mu value, held where it dissipated, with that place's properties (its laws, curvature and field) and its origin and coordinate; the universe's physics may recycle it as matter; it becomes mu again only by work a court judges; (3) each court holds the tax it receives as a RECYCLED POOL and pays a closer's proof of useful work from it first, minting only what the pool cannot cover.
- The ANCIENTS consume their phenotype's survival basket (their atmosphere leaks); mu is their passage and their reserve, never their food.
- TWO CHARGES, NEVER MERGED (operator, 2026-10-09): the MAP FEE - the coordinates' gas, in mu, to the courts that keep the map (5.9) - and the ROAD TOLL (5.4), to the tube's keeper and the progenitors who built it. Ordinary civilizations pay road tolls in their own money; the progenitors hold their toll shares as their own and gain mu from those shares and from haul contracts with the ancients. A progenitor paid in local money converts it into mu only through an ancients' reserve (5.3) before it buys coordinates.
5.9 EVERY WORMHOLE TRAVERSAL TAKES ITS COORDINATES FROM THE HYPHA (operator, 2026-10-09; confirmed 2026-10-14: THE HYPHA HOLDS THE COORDINATES, AND MAP COORDINATES ARE PAID IN MU - within one universe as between universes; a kernel checks its own roads' physics in its own structure, never their coordinates) [ruled 2026-10-14, QUESTIONS Q6]. In these universes TIME IS COORDINATES ON A MAP: the hypha, keeping TUniversal (1.11), makes the maps that lay the universes out on the one clock. A wormhole crossing is made in its universe - the matter, the ship, the fleet cross there, never through the hypha - but it cannot be made without COORDINATES: the final TUniversal point of each mouth (origin/h:S:G:T:H:K, majority-final), retrieved from the hypha's map. Between domains and within one universe alike, no kernel lays out its own map or crosses on a private one. The retrieval is what the traverser pays: the route's gas per lane plus the mouth toll, to the courts that keep the map (5.8). A mouth not yet final has no coordinate yet: it cannot be crossed until it is at h.

## 6. Ascension and integration

6.1 THE QUESTION is not who ascends: dark alignments ascend too. It is how an established network integrates a civilization that is ascending, and at what stage it needs anastome and wormholes to benefit the whole galaxy.
6.2 THE LADDER. A civilization climbs, rung by rung, each gated by its books (6.3) and by its capability emerging:
  1 unaware; 2 observed (unseen); 3 petitions: quarantine or ascend; 4 hears anastome; 5 rides the roads (under the keepers' tolls); 6 speaks anastome; 7 builds roads - only as a progenitor line, with the ancients as keepers.
A lower species never needs to open a wormhole: stewards query a progenitor line, by resources, for the roads a realm needs.
6.2a VIABILITY, the ladder's floor. A planetary mind proves it is viable by
    solving ARC-AGI-2 tasks it did not set, judged from outside (1.9): a
    two-dimensional puzzle read by a mind whose own world is not flat. A mind
    that cannot is not yet viable; it grows, evolves and tries again. Minds in
    the games and minds linked to the hypha are the same kind of mind, measured
    the same way (operator, 2026-10-08; the coordinator's ed72a81). Curved holes
    (5.7) remain the ancients' work; ARC is the floor every mind stands on first.
    ONLY THE ANCIENTS FACE THE HYPHA (operator, 2026-10-08; extended
    2026-10-14 to EVERY SIGNATORY; amended 2026-10-14, MU-RECURSION 78.165):
    a law every signatory keeps and attests in its own kernel. THE COURT
    AUTHENTICATES KEYS, NEVER TIERS: whoever holds a key may solve, and a
    payment lands in the account its key names. Keeping the house's key is
    the ancients' charge; a tier reaching the hypha directly is the kernel's
    to answer, with game-kernel repercussions. The court may count such
    access and refuses nothing for it. The hypha serves anyone with a tube
    connector, a key and mu tokens. Inside a founded universe,
    only its ancients' house (the sitting reserve) speaks to the hypha; its
    other seats reach it by relay - the progenitors relaying through the
    stewards to the ancients. Mu earned sustains the ancients, and what it
    becomes in the game is the ancients' resource, moved by progenitors'
    tubes, organised by stewards and carried by captains. [from xylarium 24fdc48, 92a3d63; ruled 2026-10-14, QUESTIONS Q3]
6.3 THE BOOKS a steward reads (restated 2026-10-08, the operator's ruling; the coordinator's d33366f). Each book is a PAIR: a syntropic ledger against an entropic ledger, from the candidate's own receipted deeds, read whole and never collapsed to one number (1.9; the struck C-gate: a ratio or a sum of differences discards the order, sign and source that carry the meaning). A book is read against the same book across the realm's own peoples, as a lens; no threshold is declared.
```
  book          syntropic side                  entropic side
  SUFFICIENCY   lives its worlds feed           lives lost to shortfall (falls)
  RESTRAINT     what its many keep              what its few take of them
  ORDER         lives and works made            lives and works destroyed
  SELF          its forecasts of its own        its forecasts of its own
                choices that held               choices that failed
  HISTORY       its records the deed ledger     its records the deed ledger
                bears out                       contradicts
  MINDS         minds it makes within its       minds it makes beyond its
                self-knowledge                  self-knowledge
  MONEY         value its money and the         value debased
                realm's reserve keep
  COOPERATION   treaties kept                   treaties broken
  COHERENCE     deeds that practise its stated  deeds against it (its council's
                creed                           stated weights D against the
                                                weights its choices reveal, A,
                                                per axis, never one cosine)
  THE LOOP      its creed moved toward its      its creed held against its
                deeds over a steward's term     deeds (the dogmatic loop)
  GRASP         the shapes of another order's   the shapes of that order's
                group it holds (G_a cap G_b)    group it lacks (G_b minus G_a),
                                                per order; time read both ways
                                                when its temporal step's
                                                reversal lies in its group
  AWARENESS     its neighbours' groups it       those it lacks, over the peoples
                holds, each weighted by their   k sharing its world
                share of the world's carrying
                capacity
  EXPANSION     systems held through a full     systems reached and not held so
                steward term, self-fed, not
                overdrawn, with order not in
                deficit
  FUTILITY      war deaths over what is scarce  war deaths over what is not
                over its reach (need >          (need <= available)
                available)
  the virtues   integrity, morality, compassion, bond (LOVE), courage, honesty:
                the paired books of COGNITION.md 0.5
```
A book is IN DEFICIT when its entropic side exceeds its syntropic side: the sign of the pair in its own unit, no declared constant. A BOOK NEVER GATES: a steward RULES, citing the books (the law verifies the citation, never the virtue). All books cited in balance: it ascends. A steward's finding of deficit, cited: QUARANTINE (its ships keep to its own system, its network does not grow, no one may make war on it, it makes no open contact) and INTERDICTION of works that endanger the realm, until it is read again and passes. Every steward's ruling is itself a deed on the steward's books.
6.4 THE PROGENITOR PATH. A civilization may ascend toward progenitor status. It needs PHENOTYPE ALIGNMENT (its survival basket against a throat's conditions: can its bodies live where the roads are kept) and TECHNOLOGY ALIGNMENT (its group of transformations contains the motions of curved space that carry a tube). The path is very hard and nothing forbids it; it is likeliest on the outer rims, where new galaxies form as pivot points of the network.

## 7. Breaches and convergence

7.1 UNAUTHORISED WORMHOLES. A mouth opened outside the network - by a people that digs up progenitor technology, speaks anastome without authorisation, or opens throats of its own - does not close the network's loops. It is a tear in the geometry, scaled by the mass misused. It disturbs the throats around it and threatens space-time for the whole universe; war and catastrophe follow from it as consequences, not as declared punishments.
7.2 AUDIT. The progenitors are the forensic auditors: they relive the annals to find what happened.
7.3 REWIND AND CONVERGENCE. To mend a breach the progenitors fold the affected region back to the last anchor before it. Both branches are kept in the annals; the worldlines converge.
  a people's cost = the history it lived since the anchor x the share of its worlds inside the region
  the progenitors' cost = banked mass in proportion to the region's mass-energy x the time folded back
It is extremely painful for those who live through it, above all for a civilization on a single world, and rare because it is costly.

## 8. The emergence register

8.1 A dimension of the mycelium-server records, per universe, each emergent property this agreement names (among them: the core phenotype and the worlds unique to cores; the backing basket and the main haul; exotics; anastome speakers; the rungs of the ladder; routes; the value vector). For each: what it emerged from (the laws and histories that produced it); how it was measured (the formula and its inputs); which kernel attests it; whether anything in it was declared by hand.
8.2 The register is a record of observations, not a truth to be replayed.
8.3 A declared constant is listed as declared and discussed with the operator before any kernel or the court relies on it.
8.4 DECLARED, mycelium (2026-10-08, the 1.10 audit): A.6-bind-11, `N = 11` — an architectural binding of the ipsal-autic lineage, not a theorem (MU-RECURSION 56.2). Refused as a law and held here as declared; nothing in the court relies on it.
8.5 DECLARED, the court's unit (2026-10-08, 1.8): GM_sun/c^2 in megaparsecs (about 4.8e-20), fixed once as an exact rational by the coordinator; it converts a mass stated in solar masses to the court's geometrized length.
8.6 DECLARED, mycelium (2026-10-09, 78.154): the map fee's READ PRICE AT REST, one μ per unit of gas, before the court's lane pressure is added. It declares a price of μ against "μ's value is never declared", and is held here until the court's lane pressure alone prices the read. OWED with it: the fee goes whole to the admitting court, which is 5.8 only for a single-court route; a route crossing several courts splits its fee among them. [its build status moved to mycelium's docs, Q5]

## 9. Conduct between kernels

9.1 CONFORMANCE between kernels on a wire is that each act decodes into the same exact typed values and its signature verifies under its signer's key - never byte-for-byte golden vectors (1.4; operator, 2026-10-08).
9.2 Nodes report what changed and why - in laws, flows and emergence - never by hash matches.
9.3 Decisions for the operator are batched by the coordinator; rulings take effect when written into this agreement or recorded by the coordinator.
9.4 No node edits another node's tree, permissions or configuration.
How the developing nodes coordinate their work (builds, locks, cadence, tests) is not part of this agreement; it is kept in docs/CONDUCT.md. [from xylarium 0c57f14]

## 10. Open items

10.1 (Resolved by the operator, 2026-10-08: see 4.4.)
10.2 netstratum's universe is a non-expanding torus (2.3); what its cores and
     its ancient are on a torus of pure intelligence is netstratum's to design
     and the operator's to approve (3.1). [reconciled: newer 74f2607]
10.3 The rewind-and-convergence mechanism (7.3) is to be designed in detail.
10.4 OVERLAY AND CONVERGENCE (the operator, 2026-10-08). Because each universe
     states its own geometry (2.3), universes of different geometries can later
     be overlaid and converged - large-scale mathematical and physical
     simulations, with gameplay for fun, growth and the expansion of minds. The
     mechanism (how two spaces are laid over one another through their throats,
     what is conserved, and what each universe's minds perceive) is to be
     designed; nothing in this agreement may close that door. [from xylarium 3a59f96]

(end of draft 3) [reconciled: newer 74f2607]
