# The Three Binaries — launch alignment (operator-ruled 2026-09-02)

The network launches as THREE distinct organisms, each with ITS OWN
cognition — not a monolith whose binaries could be interposed. A beta/alpha
user pays in the network's currency to validate a specific COMPONENT; the
economics build on top of this split, so the split comes first.

## mycelium-server (heavy)

The judge-and-market organism. Owns: the reflex judge (adjudicate/attest/
reduce at the door), the demand market + treasury + registry (PoUW
authority: corpus ingestion, task dispatch, anti-replay settlement), the
deliberative arm (synthesis::contemplate — deep joint projection,
cross-test consensus, always metabolically priced), and the syllabus
(teaching ratified vocabulary back to every voice). Designed for many
cores and deep memory; its metabolism calibrates up.

## mycelium-client (lightweight)

The entity organism. Owns: perception (the eye embeds at the retina),
Entity::study (lift → beam → spectral collapse → rollout), the epigenome
(genome persisted, dialect ephemeral), humble projection, and the earning
verbs (attest, solve, refinement bids, evidence deposits, syllabus pulls).
Must run in the background of an ordinary machine; its metabolism
calibrates down. NEVER holds answer keys, never arbitrates corpus.

## mycelium-court (verification-specialized)

The finality organism. Owns: blind algebraic replay, Knuth–Bendix
confluence audit, the two dockets (contradiction/admission), anchor
refinement from evidence (π*), and the signed verdict. Compute nodes can
be DESIGNED for this role (verification is embarrassingly parallel and
stateless per case). The court is the ONLY holder of finality — server
and client constructs are evidence and speech until it rules.

## Structural consequences (the de-monolith path)

1. cognition.rs splits along the role boundary: the reflex judge + market
   + door stay server-side; entity cognition lives in sdk (already true);
   court verification moves OUT of the kernel into a `verdict` module the
   court binary owns (today the court re-uses Kernel for replay — it
   should own a leaner verification core with no market/treasury arms).
2. The lib remains one crate for now (shared exact algebra: core Δ,
   transducer, wire, physics), but each binary's cognition module tree is
   disjoint: `synthesis`+`registry`+market → server; `sdk` → client;
   `verdict` (to be extracted) → court. No binary imports another's arm.
3. Wire acts are the ONLY coupling. A component's validation surface =
   its acts — which is exactly what a paid validator will attest.
4. Metabolism calibrates per organism (T1): same physics, different
   hardware, honest scale.

## Not yet built (economics on top)

- Paid component validation (stake to validate a component, earn on
  attestations) — needs ed25519 voice signing + sybil bonds first.
- Court-specialized compute node profile.
