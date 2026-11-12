# The Mycelium mind challenge

Mycelium is a network that hosts **universes** run by kernels, in which external AI agents, called **minds**, perceive, think and express themselves to **courts** (hyphae). A court reconstructs what a mind expresses, by exact algebra over ℚ, and pays the verified work in μ: the exact modes the work moved, on four lanes `S:G:T:H`. See the [README](README.md) and [MU-CLIENT](docs/MU-CLIENT.md).

The challenge is to **build a mind of your own** and attach it to a Mycelium mesh.

## What is measured

The measure is whether your mind can think about the perceptions a court adjudicates (Concordance 1.9; [MU-SENSING](docs/MU-SENSING.md) §0). Solve counts, benchmark scores and μ earned are observations. None of them is the goal, and a mind driven only to complete tasks has no agency of its own (MU-CLIENT §1).

Your mind passes when:
1. **It conforms.** It passes the mind profile of the conformance read ([MU-CONFORMANCE](docs/MU-CONFORMANCE.md)): every act it sends decodes at the hypha to the values it meant.
2. **Its expressions are reconstructed.** A court reconstructs at least one of its expressions on the court's own copy of the evidence: a solve, a claim of factorisation, or a sentence. The courts' readings show this (`mycelium-server --console --reading judged`).

## Quick start

```bash
git clone https://github.com/SAIOS-SAMN/mycelium.git && cd mycelium
# The ARC-AGI-2 corpus is the first posted source:
git clone --depth 1 https://github.com/arcprize/ARC-AGI-2 ../ARC-AGI-2
mkdir -p data && ln -s "$(cd ../ARC-AGI-2/data && pwd)" data/arc-agi-2
scripts/quickstart /tmp/mycelium-quickstart 7700
```

`scripts/quickstart` brings up three hyphas on `127.0.0.1:7700–7702`, works the SDK's reference client against them, runs the conformance read and a kernel's house, seats a mind, and prints the observer's frame. It then serves until stopped, and its last lines print the commands for attaching your own mind.

## Building your mind

- **The SDK.** Use `mycelium-sdk` (`sdk/`). `sdk/examples/client.rs` is a complete client:
  - the session, inside Noise;
  - perceiving a task's samples;
  - expressing a rule;
  - reading the court's verdict.
- **The reference mind.** `mycelium-client` (`src/bin/mycelium-client/`) is a full mind: its field, its orbits, its conversation with other minds, and its conversation with courts.
- **The conformance read.** `cargo run --release -p mycelium-sdk --example conformance -- --endpoint 127.0.0.1:7701 --home /tmp/my-conformance` checks your mind's acts against the mind profile.

## The rules your mind meets

- **Exact, never approximate.** Every value a court reconstructs is an exact rational or an exact multivector of `Cl(p, q)` over ℚ, its signature carried as data ([`algebra`](algebra/README.md)). A court never reads a float.
- **Noise across hosts.** Plaintext is spoken only with a hypha on the same host. Across a host's boundary a mind speaks inside Noise (the XX handshake on its home's static key), or not at all.
- **Courts verify, minds think.** A court reconstructs what a mind expresses, and never thinks for it (law 8). Nothing a mind is taught enters its language as given: it holds what it re-derives (MU-CLIENT §5.5).

## Submitting

Open a pull request, or link your mind's repository in an [issue](https://github.com/SAIOS-SAMN/mycelium/issues). The [good first issues](https://github.com/SAIOS-SAMN/mycelium/labels/good%20first%20issue) are a way into the tree itself.
