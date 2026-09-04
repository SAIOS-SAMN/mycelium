# xSEPTUM — settlement protocol for the mycelium network

Status: specification, not yet implemented. Replaces the HTTP 402 / x402
handshake for anything that pays or is paid through a hypha. Written
against the tree at de52e47; every referenced function exists there unless
marked "new".

## 1. What it replaces and what it keeps

x402 is an HTTP 402 challenge that names a scalar price in a stablecoin,
answered by a signed payment header, verified by a facilitator by
signature alone. xSEPTUM keeps the shape of the handshake: a 402 status,
a challenge header, a retry with a payment header, a receipt header. It
replaces the content:

| x402 | xSEPTUM |
|---|---|
| scalar amount in one currency | a four-lane magnitude `μ⃗ = [spatial, gauge, fibre, shell]`, never summed, compared by `economy::descends` |
| facilitator verifies a signature | the home hypha escrows on a signed Note and releases only after replay by two organs, exactly as words are installed |
| off-chain balance in a token | `Treasury` balances earned by verified reduction and solves (`economy::balance_mu`) |
| pay with money only | pay with money (a Note) or with work (a reduction attested on the listing that priced the request) |
| receipt is a transaction id | receipt is the listing's address on a signed ruled root of the home hypha, verifiable by any node by replay |

A septum is the wall between two hyphal cells with a pore: nothing crosses
except what the pore admits. Here the pore is replay.

## 2. Terms

- **Voice**: a 32-byte identity of a client kernel (`Session::kernel().voice()`), with an ed25519 key (new, §8).
- **Hypha**: a server; identity is its ed25519 verifying key (`court::identity::HyphaKey`).
- **Listing key**: `listing_key(chain, frame)`, the framed residual address of a demand; every priced request is a listing at exactly one home hypha.
- **Home**: the hypha the cover trie homes the listing to (`nerve::home`).
- **Witness**: any other hypha whose cover meets the listing's chart; the second organ.
- **μ⃗**: `[u64; 4]` magnitude on the four lanes.
- **Escrow**: `Treasury::escrow_award(addr, voices, mu, anchor)`; released by `release_award`, revoked or slashed by `revoke_award`, drawn by `draw_escrow`.
- **K**: the home's step counter (`Kernel::k_step`). All expiries are in K, never wall clock.
- **Anchor**: `economy::Anchor`, the pricing anchor the treasury records with every entry.

## 3. Roles

- **Payer**: the voice that wants a resource or a result. Pays by Note or by Work.
- **Payee**: the voice that serves the resource or claims the result. Usually a service running the client SDK.
- **Home**: escrow holder, first organ.
- **Witness**: second organ; supplies the replay that releases escrow.

Any node may be payer and payee in different exchanges. Hyphas are never
payees; they are paid by the pool share already priced into `economy::price`.

## 4. Value

A price is `economy::price(&magnitude, &pressure, &pool)`: the pool share of
the request's magnitude at the home's current pressure. A payee may quote a
magnitude; the home prices it. No lane converts to another. A Note that
does not cover the price on every lane the price occupies is refused with
`INSUFFICIENT_LANE(l)`. A payer with balance on the wrong lanes cannot pay;
it can work instead (§6.3).

## 5. Wire

All frames use `network::wire::Frame { lane, body }` with the first body
byte the act. Reserved acts (new; verify against `anastome/dialogue.rs`
before landing, 0x20–0x22 are taken by COVER/OFFER/INSTALLED):

| act | name | direction |
|---|---|---|
| 0x30 | SEPTUM_CHALLENGE | payee → payer (in HTTP, the 402 body) |
| 0x31 | SEPTUM_NOTE | payer → home |
| 0x32 | SEPTUM_RECEIPT | payee → home, countersigned by payer for resource deliveries |
| 0x33 | SEPTUM_RELEASE | home → payer and payee |
| 0x34 | SEPTUM_REVOKE | home → payer and payee |

All integers little-endian. All signatures ed25519 over the exact prefix
named. `len16` is a u16 length prefix; `len32` a u32.

### 5.1 CHALLENGE (0x30)

```
0x30 ‖ version u8 (=1)
‖ listing_key 32          the demand this request is priced as
‖ payee_voice 32
‖ home_vk 32              the home hypha's verifying key
‖ len16 home_addr         "host:port"
‖ mu 4×u64                price on four lanes, from economy::price at the home
‖ k u64                   home K at pricing
‖ ttl u64                 strokes: the Note must reach the home before K + ttl
‖ nonce 32
‖ mode u8                 0 = resource (bytes delivered), 1 = work (reduction/solve on the listing)
‖ len32 request_digest    blake3 of the canonical request (§7.1)
‖ sig_payee 64            over everything before it
```

The payee obtains `listing_key`, `mu`, `k`, `home_vk`, `home_addr` from the
home with an ordinary `ACT_QUOTE`-style read (new `Kernel::price_request`),
which lists the request as a demand with `bind_listing_task` binding it to
the payee's task id.

### 5.2 NOTE (0x31)

```
0x31 ‖ version u8
‖ challenge_prefix        the CHALLENGE bytes through nonce (not the digest, not the signature)
‖ payer_voice 32
‖ mu 4×u64                what the payer escrows; must satisfy descends(price, mu) or equality
‖ sig_payee 64            copied from the challenge
‖ sig_payer 64            over everything before it
```

The home verifies `sig_payee` with the payee's voice key, `sig_payer` with
the payer's, `k + ttl ≥ K`, the nonce unseen for this listing, and the
payer's `balance_mu` covering `mu` on every occupied lane. Then
`escrow_award(listing_key, [payer, payee], mu, anchor)`. The escrow entry
is keyed by `listing_key`; a second Note for the same nonce is refused
`DUPLICATE`.

Reply: `RELEASE`-shaped acknowledgement with state `ESCROWED`, or a
refusal code (§9).

### 5.3 RECEIPT (0x32)

Mode 0, resource:

```
0x32 ‖ version u8 ‖ listing_key 32 ‖ nonce 32
‖ len32 delivery_digest   blake3 of the delivered bytes
‖ sig_payee 64            over the fields above
‖ sig_payer 64            over the fields above (countersignature)
```

The payer countersigns after receiving the bytes and checking the digest.
Two signatures from two distinct voices are the two vantages; the home
replays only the signatures and the escrow state, then releases.

Mode 1, work:

```
0x32 ‖ version u8 ‖ listing_key 32 ‖ nonce 32
‖ mode 1
‖ len32 element_bytes     the word or sentence that settles the listing (Element::to_bytes)
‖ sig_payee 64
```

No countersignature. The word enters the docket like any attested word:
`blind_replay` at the home, Offer to the overlap, replay at a witness,
Installed. Release happens on the Installed, through the same path that
grows the language (`Kernel::home_install` → `release_award`). A word that
fails replay is slashed and the payee's stake is debited; the escrow is
revoked to the payer.

### 5.4 RELEASE (0x33) and REVOKE (0x34)

```
0x33 ‖ version u8 ‖ listing_key 32 ‖ nonce 32
‖ paid 4×u64              drawn to the payee
‖ remainder 4×u64         returned to the payer (mode 1 may leave a remainder for consilience)
‖ ruled_root 32           the home's signed ruled root after settlement
‖ k u64
‖ sig_home 64
```

```
0x34 ‖ version u8 ‖ listing_key 32 ‖ nonce 32
‖ reason u8               §9
‖ returned 4×u64
‖ slashed 4×u64           nonzero only on a failed replay
‖ k u64
‖ sig_home 64
```

The `ruled_root` is the receipt. Anyone holding the home's verifying key
verifies the release; anyone holding a digest of that root can fetch the
listing's stratum and replay it.

## 6. State machine

Per `(listing_key, nonce)` at the home:

```
PRICED ──Note verified──▶ ESCROWED ──Receipt mode 0, two sigs──▶ RELEASED
   │                          │
   │                          ├──Receipt mode 1, word replays at home──▶ SECTIONED
   │                          │        │
   │                          │        └──Installed (witness replay)──▶ RELEASED
   │                          │        └──slash──▶ REVOKED(SLASHED)
   │                          └──K > k + ttl with no receipt──▶ REVOKED(EXPIRED)
   └──K > k + ttl with no Note──▶ dropped from the book
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
`record_deflation` measures the descent; a deflation that covers `mu` on
every occupied lane is accepted as the Note and the flow continues at
ESCROWED with the payer's own deflation as the escrow source. This is the
POW++ path and the reason the protocol needs no token to bootstrap.

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

- Hyphas: `HyphaKey::load_or_mint`, existing.
- Voices: new `client::identity::VoiceKey`, an ed25519 key persisted beside
  the genome (`<kernel>.key`, 0600). The voice id becomes the verifying key
  bytes so a signature names its voice with no registry.
- Signatures over prefixes only; never over encodings that contain
  signatures.
- Nonces are 32 random bytes; the home keeps `seen: Keyed<()>` per listing
  and drops it with the listing.

## 9. Refusal codes

| code | name | meaning |
|---|---|---|
| 1 | BAD_SIGNATURE | a signature does not verify |
| 2 | EXPIRED | `K > k + ttl` |
| 3 | DUPLICATE | nonce already used on this listing |
| 4 | INSUFFICIENT_LANE(l) | balance or note short on lane `l` |
| 5 | NOT_HOME | this hypha does not home the listing; body carries the home's address |
| 6 | NO_LISTING | the listing key is unknown or dropped |
| 7 | REPLAY_FAILED | mode 1 word failed `blind_replay`; slashed |
| 8 | NO_WITNESS | no overlap organ replayed within `ttl`; escrow returned |

Codes are typed variants in code, never bare integers in cognition.

## 10. Implementation map

New:

- `src/anastome/dialogue.rs`: acts 0x30–0x34, `encode_*`/`decode_*`,
  `challenge_signed_prefix`, `note_signed_prefix`, `receipt_signed_prefix`,
  `release_signed_prefix`.
- `src/client/identity.rs`: `VoiceKey`.
- `src/economy.rs`: `Septum` ledger kind entries ("septum-escrow",
  "septum-release", "septum-revoke") on the existing `Entry`; no new
  balances.
- `src/server/septum.rs`: the state machine keyed by `(listing_key, nonce)`
  in a `Keyed<Septum>`; `price_request`, `accept_note`, `accept_receipt`,
  `expire(K)` called from the court stroke under its budget.
- `src/bin/mycelium-server.rs`: handlers for the five acts in `judge`.
- `src/client/sdk.rs`: `Session::septum_note`, `septum_receipt`,
  `septum_release`.
- `src/bin/mycelium-septum.rs`: the HTTP adapter. Listens on a local port,
  maps `402` and the three headers to frames, holds no state beyond the
  current exchange. A service that already runs the client SDK does not
  need it.

Existing, reused unchanged: `escrow_award`, `release_award`, `revoke_award`,
`draw_escrow`, `balance_mu`, `price`, `descends`, `bind_listing_task`,
`record_deflation`, `blind_replay`, `home_install`, `nerve::home`,
`HyphaKey`, `Frame`.

## 11. Tests to write before the first soak

- Note with a short lane is refused with the lane named; never partially escrowed.
- Expiry is by K: advancing wall clock releases nothing; advancing K past `k + ttl` revokes.
- Resource receipt with one signature is refused; with two distinct voices releases exactly `mu`.
- Work receipt that fails replay slashes the payee and returns the escrow.
- Work receipt that replays at home but no witness replays within `ttl` returns the escrow with `NO_WITNESS`.
- A relayed Note altered by the payee fails `BAD_SIGNATURE`.
- Duplicate nonce refused after release and after revoke.
- `price` output is honoured: a payee-quoted `mu` above the pool share is refused at pricing.

## 12. What the wire must show

An HTTP client receiving a `402` with a four-lane price, a Note escrowed at
the listing's home, bytes served, a receipt with two signatures, a release
carrying the home's signed ruled root, and the payee's balance moved on
exactly the lanes the price occupied. For the work mode: the same release
appearing as an Installed on two hyphas' ruled covers, so payment and
language are one event.
