### Noise XX wired into the live transport, opt-in per peer
- **Network contract:** changed, backward compatible. A connection may
  open with the channel's opening, `noise::OPEN` = `ff 4e 58 58 01`,
  answered by the same five bytes and then the
  `Noise_XX_25519_ChaChaPoly_BLAKE2s` handshake; every frame after it
  rides Noise transport messages (`NoiseStream::{write_frame,
  read_frame}`), the frame's own bytes unchanged inside. The observer
  gains the `channels` reading (tag 26). No act, ledger layout, signed
  byte or law version changes.
- **How a plaintext peer interoperates.** A hypha's door accepts both: a
  first byte that is a lane byte is a plaintext frame, read and answered
  as before, and its caller is named (`PLAINTEXT accepted <address>`). A
  dialler holding a Noise home that reaches a door speaking only
  plaintext reads that door's refusal of the opening (a head over the
  wire budget) as `NotSpoken`, dials again in plaintext, and names the
  peer; it is never pinned, and this process dials it in plaintext from
  then on. A client with no home (`Session::open` alone) speaks plaintext
  as before. Noise-only, a door refusing plaintext and a dialler refusing
  `NotSpoken`, is a later release step and a network-contract change of
  its own.
- **Pinning:** a peer's static key is pinned on first contact under the
  address dialled (`noise.pins` in the home). A pinned peer presenting
  another key is refused by name (`PeerKeyChanged`, both keys), and a
  pinned peer answering in plaintext is refused by name
  (`PinnedAnsweredPlain`); neither falls back to plaintext. Re-pinning is
  an operator act.
- **Where it stands:** `src/network/channel.rs` (the opening, the choice
  per peer, the paced read in strokes, the book of peers met);
  `src/bin/mycelium-server/transport.rs` (the door's `accept`, every ask's
  `tell_peer_over`, the listener's deferral `defer_unread` inside the
  caller's channel); `main.rs` opens the hypha's `NoiseHome` at boot, and
  the console asks on the home's key where one stands; every judge takes
  a `Channel`. The SDK session opts in with `Session::with_noise`
  (`NoiseHome` re-exported in `mycelium_sdk::session`), and the example
  client does. Deferral and backpressure are unchanged inside the
  channel; the round trip is still measured in polls from the write.
- **Layers:** `src/network/channel.rs` joins `TRANSPORT` in
  `gate/src/layers.rs`; it names no carrier type, and no exception is
  added.
- **Claims:** `client_and_hypha_speak_inside_noise`,
  `hypha_peers_speak_inside_noise`, `pinned_key_mismatch_refused`,
  `plaintext_peer_named_not_hidden`
  (`src/bin/mycelium-server/tests/channel.rs`) and
  `opening_reads_as_no_plaintext_frame`. The quickstart mesh runs Noise
  between its three hyphae and its SDK client; the conformance suite and
  the house, opened without a home, are named plaintext and conform.
- **Not yet:** the reference mind's sessions (`Mind::open`) and the
  chatter posts between minds stay plaintext.
