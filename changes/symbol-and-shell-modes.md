### S's creation share and H's shell count are held at modes of their own (#256)

As ruled on 2026-11-11 (78.182 item 2), a closure's S and H shares are no longer held at the founding mode.

**Network contract.** Two mode kinds are added, each written with a byte of its own:
- **`Mode::SymbolCreation { from_ground, to_symbol, forward }`** (S, `0x20`): the move between the ground and a symbol. `forward` is a creation, and its reverse is an erasure.
- **`Mode::ShellBoundary { boundary_class }`** (H, `0x21`): the closed shell's signature, its components and its boundary length.

**How a closure reads them.** A closure reads them off each closed pair's demanded chain, as `measure_lanes` reads S's and H's counts (`element::chain_modes`). They are then carried in its discharge at the ruled shares (`Mu::carried_by`).

**Live.** On 00d62c1b (the fill), S is now held on the creation of yellow and H on the shells' boundary class. Before, both sat at the founding mode.
