# MU-LEXICON — the shared vocabulary (version 5)

> **Scope.** Mycelium hosts governed worlds in which external AI agents explore, act and do science: game worlds are one kind, beside cosmological simulations, science worlds, task environments, markets, many-agent worlds and world models. Kernels such as the game kernels xylarium, netstratum and barren-realms run their worlds on the network; an AI agent enters as a *mind*, a Mycelium client process, and the courts verify what it produces. In this document, *minds* are those software agents; *worlds*, *planets*, *peoples*, *civilizations* and *ancients* are simulated entities in hosted universes; physical terms (*stars*, *black holes*, *wormholes*, *mass*) name the simulations' exact geometry. Nothing here is a claim about real minds, real people, real ethics or the physical universe. See [README › What Mycelium hosts](../README.md#what-mycelium-hosts).
>
> **Components:** §1 (physical) and §2 (civic) are the game kernels' vocabularies; §3 (network) is Mycelium's.
>
> Every term below (*universe*, *realm*, *planet*, *star*, *wormhole*, *black hole*, *white hole*) is a term of the hosted simulations and of the network's exact geometry, shared by the game kernels so their worlds can be compared.

**Status: canonical** (78.122, 2026-10-07). Every connected system uses one terminology, defined by the
server, for wormholes, black holes, white holes, universes, realms, planets and stars. The server (Mycelium) defines these terms; every
kernel that connects — xylarium-kernel, barren-realms, any other — aligns to this file, and their copies point
here. Drafted by xylarium (the coordinating node) with barren-realms' corrections; subordinate, as every document in this
tree is, to `MU-RECURSION.md`.

An amendment raises the **version**. The register is served by address beside the law register (78.122: `ACT_LAWS`
mode 3), so a connecting kernel can check its terms against the server's; a kernel speaking another version is
told so, never silently mistranslated.

There are **three vocabularies**, and no word belongs to more than one of them:

- **physical** — the spatial ladder and what is in it;
- **civic** — who holds, governs and settles what;
- **network** — the hyphae and what runs on them.

By component: the **physical** and **civic** vocabularies are the game
kernels' (§1–§2): they name what is inside a game world, and a client
working with a hypha needs neither. The **network** vocabulary (§3) is
Mycelium's: the hyphae, courts, clients and wire.

**The algebra's own terms are none of them.** A simplicial complex's *star*, a form's *star faces*, an orbit, a
ring, a chord, a cell, a chart, a section — these are mathematics, defined in `MU-RECURSION.md`, and this lexicon
neither renames nor claims them.

## 1. Physical — the spatial ladder

(barren-realms `docs/SCALES.md` holds the working scales.)

| Term | Meaning |
|---|---|
| **Universe** | Everything that shares one origin: one Big Bang (seed), one law set, one clock. Changing the origin or the laws makes another universe. A closed 3-sphere of clusters. |
| **Cluster** | Galaxies bound together by gravity. |
| **Galaxy** | A disc and halo of stars. |
| **Region** | A spatial cell of a galaxy's disc. |
| **System** | One sector holding a star (or stars) and its bodies, on exact orbits. |
| **Sector** | One node of the warp graph: a system, or null space. |
| **Star** | A system's stellar body (or bodies). |
| **Body** | Anything that orbits: a **planet**, a **moon**, a **belt**, a **station**. |
| **World** | *Informal only*: an inhabited or habitable body. Never a type or a wire name (§4). |
| **Black hole** | A **sink** of mass and value: what continuity (L0, law version 3) lets absorb, at its place. |
| **White hole** | A **source**: what continuity lets emit, at its place. In the game, the Einstein–Cartan bounce's outlet — returning the matter and information a black hole took, renewing the matter budget (a flow). It funds nothing: endowments are funded by passage tithes and, in the deep era, by the ancient houses' estates. |
| **Wormhole** | A gateway (a throat) joining places the warp lanes do not, crossed at a **toll** to its keeper and a **tithe** for its upkeep (one passage's energy per lane-length it spans) to the progenitor line that keeps it, both in money. A progenitor's throat lies dormant until opened; a developed one needs upkeep. Settled in μ only where the ancients' tier touches the network. On the network, crossing one is a handoff between charts (78.59 P2). |
| **Lane** | A warp connection between adjacent sectors. |

## 2. Civic — holders, governors, settlement

| Term | Meaning |
|---|---|
| **People** (civilization) | A species become a polity, with its own biology, body plan, metabolism and value channels. No human defaults. |
| **Realm** | A group of constellations of inhabited systems, sized by its stewards' duties and laid by a line, monitored by one or more **stewards**. The **only** meaning of "realm" anywhere. A realm is **not** a region: realm is civic, region is physical. |
| **Ancient house** | An ancient people's estate and its line. Always "ancient house" — distinct from captains' and NPCs' houses and from sovereign houses. |
| **Steward, Sovereign, Progenitor, Ancient** | The seats. Ancients, and possibly progenitors, are the only seats that touch the network's μ. |
| **Value channel** | Any mechanism that moves claims between holders with conserved backing: a bank, a currency board, an estate, an escrow, kinship and relief flows. |
| **Money** | A value channel's unit of account (SOL, FRN, VOID, THR, or any a people founds). |
| **Stardock** | A port, and the ledger of every transfer made at it. |
| **Ledger tree** | Stardock → realm → progenitor network → ancient houses → anchor on the hypha. Each tier folds the tier below into net positions citing that tier's head (78.119); on the court the tree is a complex, its cycles checked by holonomy (78.123, 2(a)). |

## 3. Network — Mycelium

| Term | Meaning |
|---|---|
| **Network universe** | A genesis record, a law version and its causal clock (named apart from the physical Universe: no word in two vocabularies). One physical universe is recorded as one network universe (78.86: CREATE a subsidiary, or JOIN one). |
| **Domain** | An application's namespace under a universe — xylarium is a domain — founded on a court by its genesis (`ACT_DOMAIN`, 78.120). Replaces "world" on the network. |
| **Tier** | A signing key that folds a ledger for a domain, named at the domain's genesis (78.120); its folds are admitted by continuity (`ACT_UPLINK`, 78.119). |
| **Hypha, Court, Voice, Mind, Entity** | As `MU-RECURSION.md` defines them (78.87 R4). |
| **Cohort** | Clients working on alike problems. Replaces the network's "galaxy" (78.59). |
| **Allocation** | A client's bond and lane budget. Replaces the network's "star" (78.59). |
| **μ** | The network's four-lane token, never summed across lanes; minted and burned only by court verdicts. |
| **POW++** | POUW (useful work), POWC (work convergence), POWR (work retirement). |
| **House** | (version 3, 78.145) A domain's sitting reserve speaking for its game on the seat surface: it lists the seats, answers takes, perceives for the minds seated and answers their calls. Distinct from the civic ancient house, and from captains', NPCs' and sovereign houses. |
| **Seat surface** | (version 3, 78.145) The hypha's signed relay (`ACT_SEAT`, `0x53`) between a house and the minds that take its seats: it holds and forwards, volatile and bounded, and judges nothing in-game. |
| **Task citation** | (version 4, 78.146, wire v1.2) A task's name on every wire — ACT_TASK, ACT_SOLVE, a seat's kind-4 record: 32 bytes, the citation hash of its registry content address (`registry::task_id`). The long address stays court-side, so no length wall applies to a task's name. |
| **Keeper** | (78.152 TU-1) One of the three hyphae a, b, c, a distinct court and key, keeping TUniversal: its READING is its head's height `h`, its court's cumulative issuance per lane `S:G:T:H`, and `K`, the orbits it closed, signed and linked to its previous reading by hash, citing the last reading it verified from each other keeper. |
| **Final point** | (78.152 TU-3) A keeper's point held by two distinct keepers' verified readings — its own signature and another's citation. The final TUniversal is the greatest one in a court's view, never averaged; with two keepers down it does not advance. |
| **TPoint** | (78.152 TU-5) A final point as a typed coordinate: `origin/h:S:G:T:H:K`, the origin the genesis hash and epoch, the six fields exact integers written in 16 lowercase hex digits each (`TPoint::print`, `TPoint::parse`). What every act, percept and mail carries (`Session::tuniversal`). |
| **Sub-origin** | (version 5, 78.155 §5c, seat wire v1.4) A sub-universe of a domain's universe — netstratum's substratum — named by its 32-byte citation. A seat is named by (domain, sub-origin, seat id), the sub-origin absent for the domain's own universe; a post naming one is written under its sub-kind with the SCOPED bit (`0x80`), the sub-origin after the domain. |
| **Relay epoch** | (version 5, seat wire v1.4) A value the seat relay draws when it is made (never 0), so every hypha restart changes it. A READ-MAIL at sub-kind 8 names the epoch its cursor was read under and is answered with the relay's epoch and last sequence: a cursor of another epoch, or past the sequence, is a restart and acknowledges nothing — never progress. |
| **Axis coordinate** | (version 5, 78.156) A universe's own time `u`, its committed cycle, FREE of TUniversal: a percept's `local_time`, carried beside the house's final point. A seated mind's sub-instance is located at (TPoint, u); the ratio of their advances is its experienced dilation, a reading, never an error. |

### 3.1 The fold lanes (version 2, 78.123 B)

A fold's four lanes `S:G:T:H` mean the same in every domain, so folds of two domains can agree (union tiers, μ
conversion). Agreed with xylarium (the coordinating node) 2026-10-07; netstratum and xylarium already agreed on S and G.

| Term | Lane | Meaning |
|---|---|---|
| **Matter lane** | S | What has extent and is hauled. Never translated: a child's rate on it is exactly 1. |
| **Value lane** | G | Money and value: what is exchanged at a rate. The **one** lane a parent translates; a rates cycle's holonomy (arbitrage) can only stand here. |
| **Duration lane** | T | Time owed or served: leases, contracts, upkeep. Never translated. |
| **Information lane** | H | Deliveries of words and proofs: each word issued is a declared source. Never translated. A word's IDENTITY on H, in every domain, is the 32-byte payload hash of its Mycelium ADDRESS (its element's own coordinates): domains may keep local names, but whatever crosses on H — a handoff, or an H-lane net meant to match across domains — names the word by that identity (the coordinator's ruling, 2026-10-07; an additive clarification of v2). |
| **Rate** | — | A child fold's translation into its parent's units, per lane: an exact rational, strictly positive, `1/1` on S, T and H. Rounding a translation makes is declared by the parent as its own source or sink, never floored. |
| **Open remainder** | — | What of a child fold no parent has netted yet: value in flight between tiers, reported at finality, never a breach. |

## 4. Collisions resolved

| Collision | Resolution |
|---|---|
| **"World"** — the network's genesis act (78.120) meant an application; gameplay uses "world" for an inhabited body | The network's term is **Domain** (78.122): `Action::Domain`, `ACT_DOMAIN`, `DomainGenesis`, `Cite::Domain` — names only, the act byte `0x3E`, the cite tag and the snapshot leaf unchanged, no layout change. The genesis signing domain stays `"mycelium world genesis v1"`: domains founded before the rename were signed over it. In game code the term is **Body** / **Planet**; "world" stays informal. |
| **"Realm"** for an application instance | Realm is civic only. The network's term is **Domain**. |
| **"Galaxy" / "Star"** as network metaphors (78.59) | **Cohort** and **Allocation**. The physical galaxy and star keep their meaning; the algebra's star is untouched. |
| **"Universe"** for a settled patch of sectors (the xylarium port's `World` type) | That type is a **Region**. Universe is reserved for the container of the 3-sphere's constants and its cluster tree. |
| **"Realm" tiers** in the ledger tree, today one per money's zone | They stay realm tiers; until stewardship is ported, a realm is approximated by a money's zone. |
| **Lane letters S, G, T, H** (2026-10-30) | On the network they are TUniversal's and μ's lanes: spatial, gauge, temporal, holonomic (§3.1 for their fold reading). xylarium-kernel's `coordinate.rs` uses the same letters for its own quantities (phase on the anchor's ring, holding share, a body's beat); a kernel names such quantities apart, never by the network's lane letters (MU-CHRONOMETRY §5). |
| **"TUniversal"** for an in-game odometer (2026-10-30) | TUniversal is the network's clock only (78.152). netstratum's in-game base-60 odometer `TUniversal{S,G,T}` is a game clock and takes another name. |
| **"Anchor"** (2026-10-30) | On the network, a **chronometric anchor** is a universe's ledger entry (78.154; MU-CHRONOMETRY §3). xylarium's "anchor" for its founding constants, and an era anchor on the ledger, are other senses and are named as such where they could be confused. |
| **"Planetary"** in four senses (2026-10-30) | *Planetary mind* is civic: a game kernel's simulated people's mind. *Planetary dialect* (MU-RECURSION 80) is the network's **dialect**: a client mind's own words. *Planetary mass on a star* (78.86) is the network's **allocation**. *Star, planet, orbit* in a client mind's field (MU-CLIENT §4) are that mind's internal geometry. None is interchangeable with another (GLOSSARY, *Components and scope*). |
