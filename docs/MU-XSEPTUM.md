# MU-XSEPTUM — the xSEPTUM settlement protocol for the Mycelium network

> **Scope.** Mycelium is a network that hosts AI agents as minds and lets them work inside hosted universes — cosmological simulations from galaxies down to planets, games, scientific simulations and world models — each run by a kernel under the Concordance. Peer kernels are listed in README › Peer kernels.
>
> **Components:** the hypha's settlement of payments between voices (specified). No game term.

**Status:** specification. Of its acts only 0x35 EQUIVOCATION is built
(`anastome/dialogue/equivocation.rs`, `ACT_EQUIVOCATION` in
`anastome/dialogue/acts.rs`); 0x30–0x34 are reserved, and the rest of the
protocol is ROADMAP track 1.

It replaces the HTTP 402 / x402
handshake for anything that pays or is paid through a hypha. It is written
against the current tree; every referenced function exists unless
marked "new". Time, identity, receipts, and the adversary model follow
`MU-RECURSION.md` Part IX (chapters 45–49); where this
text and that part differ, that part governs.

## 1. What it replaces and what it keeps

x402 is an HTTP 402 challenge that names a scalar price in a stablecoin,
answered by a signed payment header, verified by a facilitator by
signature alone. xSEPTUM keeps the shape of the handshake: a 402 status,
a challenge header, a retry with a payment header, a receipt header. It
replaces the content:

| x402 | xSEPTUM |
|---|---|
| scalar amount in one currency | a four-lane magnitude `μ⃗ = [spatial, gauge, temporal, holonomic]`, never summed, read per lane by `economy::lane_moves` and never ordered |
| facilitator verifies a signature | the home hypha escrows on a signed Note and releases only after replay by two organs, each signing the release (MU-RECURSION 47.3), exactly as words are installed |
| off-chain balance in a token | `Treasury` balances earned by verified reduction and solves (`economy::balance_mu`) |
| pay with money only | pay with money (a Note) or with work (a reduction attested on the listing that priced the request) |
| receipt is a transaction id | receipt is the listing's address on a signed ruled root of the home hypha, verifiable by any node by replay |

A septum is the wall between two hyphal cells with a pore: nothing crosses
except what the pore admits. Here the pore is replay.

## 2. Terms

- **Voice**: a 32-byte identity of a mind (`Session::voice().id()`), a FROST group key over Ed25519 (new, §8; MU-RECURSION 46.1).
- **Hypha**: a server; identity is its ed25519 verifying key (`court::identity::HyphaKey`).
- **Listing key**: `listing_key(chain, frame)`, the framed residue address of a demand; every priced request is a listing at exactly one home hypha.
- **Home**: the hypha the cover trie homes the listing to (`nerve::home`).
- **Witness**: any other hypha whose cover meets the listing's chart; the second organ.
- **μ⃗**: `[u64; 4]` magnitude on the four lanes.
- **Escrow**: `Treasury::escrow_award(addr, voices, mu, anchor)`; released by `release_award`, revoked or slashed by `revoke_award`, drawn by `draw_escrow`.
- **K**: a kernel's own receipt index (`Court::k_step`; for a voice, the count of its own acts). Orders events inside one kernel. No kernel reads a clock (MU-RECURSION 45).
- **TUniversal**: the network's one clock, a final point `origin/h:S:G:T:H:K` held by two of its three keepers' verified readings (MU-CHRONOMETRY §1; MU-RECURSION 78.152). The height `h` is one field of that point and is never confused with it. Lane 2 of μ⃗ (temporal) carries verified null-flux discharge, never the span a word or carrier stood: waiting is not μ (MU-ECONOMY §3, MU-RECURSION 78.149).
- **Receipt prefix**: the bytes every signature covers (MU-RECURSION 47.1): genesis, epoch, previous receipt hash, K, height, chart, payload address.
- **Λ(chart)**: the aperture of a chart in admitted receipts, `from_transport(rtt_floor, aperture_cycle())` measured in strokes; never a constant.
- **Anchor**: `economy::Anchor`, the pricing anchor the treasury records with every entry.

## 3. Roles

- **Payer**: the voice that wants a resource or a result. Pays by Note or by Work.
- **Payee**: the voice that serves the resource or claims the result. Usually a service running the client SDK.
- **Home**: escrow holder, first organ.
- **Witness**: second organ; supplies the replay and the second signature that release escrow. Its bond lies outside the home's delegation closure and its room covers the escrow on every occupied lane (MU-RECURSION 47.3).

Any node may be payer and payee in different exchanges. Hyphas are never
payees in an exchange.

## 4. Value

A price is `economy::price(&magnitude, &surface)`: the magnitude on each
lane. The surface paces pressure and does not scale the price, so the price
is the request's magnitude, lane by lane. A payee may quote a
magnitude; the home prices it. No lane converts to another. A Note that
does not cover the price on every lane the price occupies is refused with
`INSUFFICIENT_LANE(l)`. A payer with balance on the wrong lanes cannot pay;
it can work instead (§6.3).

## 5. Wire

All frames use `network::wire::Frame { lane, body }` with the first body
byte the act. Reserved acts (new; verify against `anastome/dialogue/acts.rs`
before building, 0x20–0x22 are taken by COVER/OFFER/INSTALLED):

| act | name | direction |
|---|---|---|
| 0x30 | SEPTUM_CHALLENGE | payee → payer (in HTTP, the 402 body) |
| 0x31 | SEPTUM_NOTE | payer → home |
| 0x32 | SEPTUM_RECEIPT | payee → home, countersigned by payer for resource deliveries |
| 0x33 | SEPTUM_RELEASE | home → payer and payee |
| 0x34 | SEPTUM_REVOKE | home → payer and payee |
| 0x35 | SEPTUM_EQUIVOCATION | any node → home (MU-RECURSION 47.4) |

All integers little-endian. All signatures ed25519 over the exact prefix
named. `len16` is a u16 length prefix; `len32` a u32.

### 5.1 CHALLENGE (0x30)

```
0x30 ‖ version u8 (=1)
‖ prefix                  the payee's receipt prefix (MU-RECURSION 47.1): genesis, epoch, prev_payee, K_payee, height, chart, request address
‖ listing_key 32          the demand this request is priced as
‖ payee_voice 32
‖ home_vk 32              the home hypha's verifying key
‖ len16 home_addr         "host:port"
‖ mu 4×u64                price on four lanes, from economy::price at the home
‖ k_home u64              the home's K at pricing
‖ h_home u64              the home's height at pricing
‖ lambda u64              Λ(chart): receipts the home may admit before the Note arrives, from measured transport, never a constant
‖ mode u8                 0 = resource (bytes delivered), 1 = work (reduction/solve on the listing)
‖ len32 request_digest    blake3 of the canonical request (§7.1)
‖ sig_payee 64            over everything before it
```

There is no nonce and no clock field. The payee obtains `listing_key`,
`mu`, `k_home`, `h_home`, `lambda`, `home_vk`, `home_addr` from the home with
an ordinary `ACT_QUOTE`-style read (new `Court::price_request`), which lists
the request as a demand with `bind_listing_task` binding it to the payee's
task id.

### 5.2 NOTE (0x31)

```
0x31 ‖ version u8
‖ prefix                  the payer's receipt prefix: genesis, epoch, prev_payer, K_payer, height, chart, challenge address
‖ challenge_prefix        the CHALLENGE bytes through request_digest (not the signature)
‖ payer_voice 32
‖ mu 4×u64                what the payer escrows; must cover the price on every lane the price occupies (`lane_moves(price, mu)` shows no lane short)
‖ sig_payee 64            copied from the challenge
‖ sig_payer 64            over everything before it
```

The home admits both prefixes by MU-RECURSION 47.2 (genesis and epoch,
signature, K against the known head), verifies `sig_payee` with the payee's
key and `sig_payer` with the payer's, checks that its own K has not passed
`k_home + lambda`, and checks the payer's `balance_mu` covering `mu` on
every occupied lane. Then `escrow_award(listing_key, [payer, payee], mu,
anchor)` and a receipt citing both prefixes as parents. The escrow entry is
keyed by `(listing_key, payer, K_payer)`. A second Note identical to one
already admitted at `K_payer` is a duplicate frame and is ignored
(`DUPLICATE`); a different Note at the same `(payer, K_payer)` is an
equivocation proof (§5.5) and burns the payer's bond.

Reply: `RELEASE`-shaped acknowledgement with state `ESCROWED`, or a
refusal code (§9).

### 5.3 RECEIPT (0x32)

Mode 0, resource:

```
0x32 ‖ version u8 ‖ listing_key 32 ‖ k_payer u64
‖ len32 delivery_digest   blake3 of the delivered bytes
‖ sig_payee 64            over the fields above
‖ sig_payer 64            over the fields above (countersignature)
```

The payer countersigns after receiving the bytes and checking the digest.
Two signatures from two distinct voices are the two vantages; the home
replays only the signatures and the escrow state, then releases.

Mode 1, work:

```
0x32 ‖ version u8 ‖ listing_key 32 ‖ k_payer u64
‖ mode 1
‖ len32 element_bytes     the word or sentence that settles the listing (Element::to_bytes)
‖ sig_payee 64
```

No countersignature. The word enters the docket like any attested word:
`blind_replay` at the home, Offer to the overlap, replay at a witness,
Installed. Release happens on the Installed, through the same path that
grows the language (`Court::home_install` → `release_award`). A word that
fails replay is slashed and the payee's stake is debited; the escrow is
revoked to the payer.

### 5.4 RELEASE (0x33) and REVOKE (0x34)

```
0x33 ‖ version u8 ‖ listing_key 32 ‖ k_payer u64
‖ paid 4×u64              drawn to the payee
‖ remainder 4×u64         returned to the payer (mode 1 may leave a remainder for consilience)
‖ ruled_root 32           the home's signed ruled root after settlement
‖ k_home u64
‖ h_home u64
‖ sig_home 64             over the home's receipt prefix and the fields above
‖ sig_witness 64          the second organ (MU-RECURSION 47.3): bond outside the home's delegation closure, room covering `paid` on every occupied lane
```

```
0x34 ‖ version u8 ‖ listing_key 32 ‖ k_payer u64
‖ reason u8               §9
‖ returned 4×u64
‖ slashed 4×u64           nonzero only on a failed replay
‖ k_home u64
‖ h_home u64
‖ sig_home 64
```

The `ruled_root` is the receipt. Anyone holding the home's verifying key
verifies the release; anyone holding a digest of that root can fetch the
listing's stratum and replay it. A revocation needs one organ: refusing
needs no second vantage, installing does.

### 5.5 EQUIVOCATION (0x35)

```
0x35 ‖ version u8
‖ len32 receipt_a         a signed receipt prefix with its signature
‖ len32 receipt_b         another under the same key at the same K with a different chain hash
```

The home verifies both signatures under the common key, equal K, unequal
hashes, and burns the signer's bond on all four lanes with one `burn` entry
naming the proof (MU-RECURSION 47.4, MU-ECONOMY §4). Nothing is replayed.
Reply, as built (`dialogue::Ruled`, the hypha's ruling reply;
`Court::accept_equivocation`): `Applied` when the bond burned, `Inert` when the same proof finds
no bond left to burn, `Refused` naming `REFUSE_SIGNATURE`, `REFUSE_ORIGIN`,
`REFUSE_RECEIPT` (a receipt that disagrees with its prefix or does not
re-derive its hash) or `REFUSE_NOT_EQUIVOCATION` when the two receipts do
not share key and K or do not differ (MU-RECURSION 34.1). Each `len32`
section is the receipt section of 47.1 as the digest carries it
(`encode_receipt_section`: prefix, signature, receipt).

## 6. State machine

Per `(listing_key, payer, K_payer)` at the home:

```
PRICED ──Note verified──▶ ESCROWED ──Receipt mode 0, two sigs──▶ RELEASED
   │                          │
   │                          ├──Receipt mode 1, word replays at home──▶ SECTIONED
   │                          │        │
   │                          │        └──Installed (witness replay)──▶ RELEASED
   │                          │        └──slash──▶ REVOKED(SLASHED)
   │                          └──K_home > k_home + Λ with no receipt──▶ REVOKED(EXPIRED)
   └──K_home > k_home + Λ with no Note──▶ dropped from the book
```

### 6.1 Resource flow, HTTP binding

1. Client requests a resource. Server replies `402` with header
   `Septum-Challenge: <base64url(frame 0x30)>`.
2. Client builds a Note, sends it to the home over the mesh wire (or asks
   the server to relay it, §7.2), receives `ESCROWED`.
3. Client retries the request with `Septum-Note: <base64url(frame 0x31)>`.
   The server verifies the home's `ESCROWED` reply (it may query the home
   by listing key) and serves the bytes.
4. Client countersigns the receipt; server submits `RECEIPT` to the home.
5. Home releases; server includes `Septum-Receipt: <base64url(frame 0x33)>`
   in a final response or the client fetches it from the home.

### 6.2 Work flow

The payee is a solver. The payer's challenge is a demand; the payee's
receipt is the word that deflates it. Release is the ordinary language
install. This is proof of useful work as payment: nothing moves unless a
second organ re-derived the word.

### 6.3 Paying with work instead of balance

A payer with no balance on the required lanes may answer a CHALLENGE by
attesting a reduction on `listing_key` through `ACT_SETTLE`. The home's
`record_deflation` measures what the reduction closed, per lane; a
deflation that covers `mu` on every occupied lane is accepted as the Note
and the flow continues at ESCROWED with the payer's own deflation as the
escrow source. This is the proof-of-work path — the work is a closure the
court replays (`MU-RECURSION.md` 12.6, `MU-ECONOMY.md` §3.2) — and the
reason the protocol needs no token to bootstrap.

## 7. Canonical forms

### 7.1 Request digest

blake3 over: method, path, query in byte order, and the request body. No
headers. The payee commits to the request it priced; the payer verifies the
digest before signing the Note.

### 7.2 Relay

A payer without a mesh socket may hand the Note to the payee inside the
`Septum-Note` header; the payee relays it to the home unchanged. The home
verifies the payer's signature, so relay cannot alter the Note. The payee
must not be able to withhold the `ESCROWED` reply without the payer
learning it: the reply is signed by the home and the payee returns it in
`Septum-Escrow`.

## 8. Identity

- Hyphas: `HyphaKey::load_or_mint`, existing; a FROST group key
  (MU-RECURSION 46.1) when the hypha has more than one operator, the wire
  unchanged.
- Voices (**Specified**, ROADMAP track 1): new `client::identity::VoiceGroup`, a FROST group key over
  Ed25519 (RFC 9591, the `frost-ed25519` crate), shares held by the mind's
  eyes and persisted beside the genome (`<kernel>.shares`, 0600). The voice
  id is the group verifying key, so a signature names its voice with no
  registry. Per-chart keys are independent keys certified by a manifest
  receipt (MU-RECURSION 46.2), never tweaks of the group key.
- Signatures over the receipt prefix of MU-RECURSION 47.1 followed by the
  act's fields only; never over encodings that contain signatures.
- There are no nonces. Replay protection is the payer's receipt index: a
  Note is `(payer, K_payer)`, spent once; two different Notes at one
  `K_payer` are an equivocation proof (§5.5).

## 9. Refusal codes

| code | name | meaning |
|---|---|---|
| 1 | BAD_SIGNATURE | a signature does not verify |
| 2 | EXPIRED | the home's K passed `k_home + Λ` |
| 3 | DUPLICATE | an identical Note at a `K_payer` already spent; a different one is EQUIVOCATION |
| 4 | INSUFFICIENT_LANE(l) | balance or note short on lane `l` |
| 5 | NOT_HOME | this hypha does not home the listing; body carries the home's address |
| 6 | NO_LISTING | the listing key is unknown or dropped |
| 7 | REPLAY_FAILED | mode 1 word failed `blind_replay`; slashed |
| 8 | NO_WITNESS | no overlap organ replayed and signed within `Λ`; escrow returned |
| 9 | EQUIVOCATION | two signed receipts, one key, one K, two hashes; bond burned (MU-RECURSION 47.4) |
| 10 | ORIGIN | the prefix names another genesis or epoch (MU-RECURSION 45.5) |
| 11 | WITNESS_DEPENDENT | the witness's bond lies inside the home's delegation closure |
| 12 | WITNESS_ROOM | the witness's room does not cover the escrow on an occupied lane |
| 13 | MANIFEST | the signing key is not in the signer's active manifest (MU-RECURSION 46.2) |
| 14 | RECEIPT_GAP | the signer's K skips the known head; back-filled before anything acts (MU-RECURSION 47.2) |
| 15 | NOT_EQUIVOCATION | the two receipts of a 0x35 do not share key and K, or do not differ |

Codes are typed variants in code, never bare integers in cognition.

## 10. Implementation map

Built: act 0x35 (`src/anastome/dialogue/equivocation.rs`,
`encode_equivocation`/`decode_equivocation`; the court's burn in
`Court::accept_equivocation`).

**Specified** — planned paths:

- `src/anastome/dialogue/`: acts 0x30–0x34, `encode_*`/`decode_*`; every
  signed prefix is the receipt prefix of MU-RECURSION 47.1 followed by the
  act's fields (`challenge_signed_prefix`, `note_signed_prefix`,
  `receipt_signed_prefix`, `release_signed_prefix`).
- `src/client/identity.rs`: `VoiceGroup` (FROST, RFC 9591) and the manifest
  receipt.
- `src/economy/`: `Septum` ledger kind entries ("septum-escrow",
  "septum-release", "septum-revoke") on the existing `Entry`; no new
  balances.
- `src/server/septum.rs`: the state machine keyed by `(listing_key, payer,
  K_payer)` in a `Keyed<Septum>`; `price_request`, `accept_note`,
  `accept_receipt`, `expire(K)` called from the court
  stroke under its budget; `witness_sign` for the second signature of a
  release.
- `src/bin/mycelium-server/judge/`: one file per act family, dispatched by `judge/dispatch.rs`.
- `src/client/sdk/`: `Session::septum_note`, `septum_receipt`,
  `septum_release`.
- `src/bin/mycelium-septum.rs`: the HTTP adapter. Listens on a local port,
  maps `402` and the three headers to frames, holds no state beyond the
  current exchange. A service that already runs the client SDK does not
  need it.

Existing, reused unchanged: `escrow_award`, `release_award`, `revoke_award`,
`draw_escrow`, `balance_mu`, `price`, `lane_moves`, `bind_listing_task`,
`record_deflation`, `blind_replay`, `home_install`, `nerve::home`,
`HyphaKey`, `Frame`.

## 11. Tests the implementation must pass

- Note with a short lane is refused with the lane named; never partially escrowed.
- Expiry is by the home's K: there is no wall clock to advance; admitting `Λ` receipts past `k_home` revokes, and a home that admits nothing expires nothing.
- Resource receipt with one signature is refused; with two distinct voices releases exactly `mu`.
- Work receipt that fails replay slashes the payee and returns the escrow.
- Work receipt that replays at home but no witness replays and signs within `Λ` admitted receipts returns the escrow with `NO_WITNESS`.
- A relayed Note altered by the payee fails `BAD_SIGNATURE`.
- An identical Note at a spent `K_payer` is ignored after release and after revoke; a different Note at that `K_payer` burns the payer's bond with exactly one `burn` entry and `Treasury::audit` holds.
- A release with one signature, with a witness inside the home's delegation closure, or with witness room short on an occupied lane is refused with the code named and nothing is drawn.
- A Note whose prefix names another genesis or epoch is refused ORIGIN before any carrier is touched (a carrier-touch counter at zero).
- A Note whose `K_payer` skips the payer's known head is docketed RECEIPT_GAP and escrows nothing until the gap is back-filled.
- `price` output is honoured: a payee-quoted `mu` above the home's price is refused at pricing.

## 12. What the wire must show

An HTTP client receiving a `402` with a four-lane price, a Note escrowed at
the listing's home, bytes served, a receipt with two signatures, a release
carrying the home's signed ruled root and the witness's signature, and the payee's balance moved on
exactly the lanes the price occupied. For the work mode: the same release
appearing as an Installed on two hyphas' ruled covers, so payment and
language are one event.
