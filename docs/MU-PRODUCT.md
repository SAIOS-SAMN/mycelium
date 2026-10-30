# Product — a market for comprehension

Status at f595b4e: the market for work exists on every hypha; the products
below are the surfaces that expose it. What is landed and what is specified
is marked in §7. Companion documents: `MU-ECONOMY.md`, `MU-ENGINEERING.md`,
`XSEPTUM.md`.

## 1. What is sold

The mesh sells comprehension: the ability to describe a family of
observations as a sentence in anastome that a court has replayed, and to
speak that sentence onto a new input. Everything for sale is one of the
carriers the language already has, and every sale ships with a signed
ruled root the buyer can replay.

| product | buyer sends | buyer receives | carrier |
|---|---|---|---|
| Solve | a family in pair-grid form, escrow | the rule, the outputs, a receipt | xSEPTUM work mode, `pay_solve` |
| Comprehension of a corpus | families through the uplink door | words minted on that data, taught to a mind the buyer runs | uplink, syllabus by incidence |
| Certification | a rule or sentence claimed elsewhere, a fee | replayed and installed, or refused with the reason | `blind_replay`, the docket |
| Translation | two corpora whose dialects do not glue | the gauge word between them | translation Rewrite |
| Cover | bond on a lane | room to home work; the pool share on it | stake, delegation |
| Standing work | escrow on an open demand | accrual on T until someone closes it | listing with T accrual |

Nothing is sold that the court cannot replay. There is no perpetual, no
external-asset market, and no oracle on a hypha; those need consensus
about a number the mesh cannot verify. External markets enter as corpora
to be comprehended, and external assets enter only over a bridge.

## 2. Who buys

- Developers who need a verified answer with the proof, not a plausible
  one.
- Owners of series and structured data who want a mind that learns their
  universe and improves with the mesh, without handing the data to a model
  vendor: they run the client, the data becomes families, the words come
  back as their genome.
- AI products that need to certify reasoning: a claim that replays is worth
  more than one that does not.
- Operators who want yield from verified work rather than from holding.

## 3. The three surfaces

1. **The gateway.** `mycelium-septum` speaks HTTP: a 402 challenge with a
   four-lane price, a note, a receipt, JSON bodies wrapping the frames in
   `XSEPTUM.md`. Thin TypeScript and Python clients build a challenge,
   sign a note with a voice key, and verify a receipt against a root.
2. **The mind.** The client binary is the consumer product: a wallet
   (balance per lane, stake, unstake, notes) and a genome that learns.
   Pointed at data through the uplink door it becomes a mind on that
   universe and earns when its words install. The download is the
   onboarding.
3. **The reads.** A public explorer over any hypha: the ruled cover with
   replay links, the ledger with supply per lane and every mint tied to a
   release, the price surface, the dialect classes and the translations
   that closed them. Verification needs no account.

## 4. The first customer: ARC-AGI-2

We are the first customer and the first operator. The loop:

1. A genesis allocation funds our customer voice and bonds our hyphae.
2. Each ARC-AGI-2 task is posted as a demand, escrowed from that
   allocation at its home, accruing on T while open.
3. Miners run the client, receive the starter genome, pull families,
   extract rules, speak them. The home replays, a witness replays, the rule
   installs on two covers, the miner is paid the escrow plus the mint.
4. The rule is now on the ruled cover, so the next miner's starter genome
   contains it and the next task is closer.
5. Miners bond earnings to raise their room, bond enough to run a hypha,
   then open a second market by pointing the uplink door at another corpus.

The knowledge stored is the language, not the answers: rules, sentences,
words, translations, and their roots, in anastome. Answers are re-derived
by speaking a rule onto an input.

## 5. Onboarding

**A miner.** Download the client, mint a voice key on first run, bootstrap
to any hypha, import the starter genome verified against its root, work.
Earn on install and on solve; stake to raise room; unstake when no escrow
names the voice.

**An operator.** Run the server, mint a hypha key on first run, bond from
balance, join by one bootstrap address; the cover trie is the topology
after the first digest. Earn the pool share on homed work and accrual on
open demands. A false root is slashed from the bond.

**A customer.** Hold a voice with balance, post demands with escrow through
the gateway or the uplink door, receive rules and receipts, verify roots.

## 6. Pricing

Measured, never listed. A solve is paid on the lanes its rule moved;
certification on the replay's own magnitude; cover on the lane bonded;
standing work on T. A buyer who lacks a lane pays by working on it, which
is how new minds enter without a token purchase.

## 7. What is landed and what is not

Landed: the work market on every hypha (listing, quote by incidence,
attest, probe, settle), the court organ that replays and installs, the
syllabus by incidence, the ARC corpus as a work source, solve verification
at the home, the client with a genome that learns.

Specified: the gateway and SDK bindings (`XSEPTUM.md`), the uplink door
(restore from the predecessor tree's `src/bin/uplink.rs`), certification as a
route, the explorer reads, the starter-genome export, genesis, voice keys,
stake for voices, the wallet verbs in the client.

## 8. The two events that make this a business

The first hundred families closed by rules the court replayed, and the
first external buyer who paid for a solve and verified the root themselves.
Every item in the build order in `MU-ENGINEERING.md` §10 is aimed at those two
events.
