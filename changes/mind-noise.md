### The reference mind's sessions and chatter inside Noise (#132)
- **Network contract:** changed, backward compatible with a plaintext
  peer. A mind's chatter post now reads the channel's opening
  (`noise::OPEN`) and answers it with the
  `Noise_XX_25519_ChaChaPoly_BLAKE2s` handshake where it holds a Noise
  home; every chatter frame after it rides Noise transport messages, its
  `[len u32 LE][frame]` framing unchanged inside. A post with no home
  answers the opening with `chatter::OPENING_REFUSED` (`01 00 00 00 00`),
  which a dialler reads as `NotSpoken`, so it names the post plaintext
  and speaks on. A post built before this change closes on the opening,
  and a mind with a home is refused there (`Closed`) until that post is
  rebuilt. No act, signed byte, ledger layout or law version changes.
- **Sessions:** `Mind::open_with_noise` opens the mind's three sessions
  (hypha, seat, held mail) with `Session::with_noise` on the home's
  static key, each hypha pinned under the address dialled; `Mind::open`
  stays plaintext and is named. `Mind::open_persistent` takes the home as
  an `Option<NoiseHome>`. `mycelium-client` opens the voice home's
  `NoiseHome` at boot (a home that does not open refuses the boot), reads
  the law register inside Noise, and `--call` asks each mind on the home's
  key where one stands.
- **Chatter:** `chatter::send_over` and `ask_over` dial a mind through
  `channel::dial_as`, pinning it on first contact under the voice the
  rendezvous files its address by (`chatter::named`), since a post's port
  is chosen afresh at each opening; an address `MYCELIUM_CHATTER_PEERS`
  names is pinned under itself. `Post::listen_over` answers a caller that
  opens with all five bytes of the opening inside Noise, the caller
  pinned under no name as at a hypha's door, and reads any other caller
  in plaintext with the bytes it read to tell them apart, so a plaintext
  frame whose length begins with `0xff` is read whole. Both sides enter
  the channel's book (`channel::met`), which the observer reads as
  `channels`.
- **Refusals:** a changed key is `PeerKeyChanged` and a pinned mind
  answering in plaintext is `PinnedAnsweredPlain`, both named in the log
  and answered to the carrier as `PermissionDenied`, so the rendezvous
  entry is kept (only `ConnectionRefused` names a mind gone); neither
  falls back to plaintext and neither pin is moved.
- **Claims:** `mind_speaks_inside_noise`,
  `mind_refuses_a_changed_or_unsealed_peer` and
  `post_tells_the_opening_from_a_plaintext_frame`
  (`src/client/sdk/tests/channel.rs`).
