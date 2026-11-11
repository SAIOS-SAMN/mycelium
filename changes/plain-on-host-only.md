### Plaintext stays on the host (78.182 item 19, D1.3 O2)

- **A plaintext channel is spoken only with a peer on this host**: a loopback address, or the address the socket itself stands at (`channel::on_this_host`).
- **Across the host's boundary only Noise is spoken.** A plaintext opening at the door from another host, or a plaintext dial to one (no Noise home, or a peer that answers in plaintext), is refused `NoiseRefusal::PlainAcrossBoundary { peer }`.
- **The one-host path is unchanged**: `scripts/quickstart` and a workstation's mesh on loopback run as before.
