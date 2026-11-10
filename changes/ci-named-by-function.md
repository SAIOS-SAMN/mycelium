### CI is named for the two commands it runs: `clippy and tests`
- One check had three names: the file `definition-of-done.yml`, the workflow `Conformance Verification` and the job `clippy and tests`. Only the job's name said what runs:
  - "definition of done" is process vocabulary. It overclaimed, because CI runs neither the quickstart nor a network contract's approval, and it was circular with AGENTS.md.
  - "Conformance Verification" collided with two defined terms: the kernel conformance suite (MU-CONFORMANCE, `sdk/src/conformance`) and verification by reconstruction (MU-ANTI-FLATTENING 14).
- The file is now `.github/workflows/clippy-and-tests.yml`, and the workflow and job are both `clippy and tests`. The required status is unchanged.
- AGENTS.md's section is *Checks*. It states what CI runs and what it does not (the quickstart, the fragment's content, a network contract's approval), and when a change is ready for a pull request. Every link to it is rebound: README, CONTRIBUTING, ROADMAP, MU-STANDARDS, MU-ANTI-FLATTENING, MU-ENGINEERING, MU-RELEASE, the pull request template, `Cargo.toml`, the gate and the header of `scripts/quickstart`.
- **Network contract:** none.
