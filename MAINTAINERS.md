# Maintainers

Who may merge to `main`, and who approves what. The change process is
[docs/MU-RELEASE.md](docs/MU-RELEASE.md) Part I; the workflow is in
[CONTRIBUTING.md](CONTRIBUTING.md).

| Role | Who | Responsibilities |
|---|---|---|
| Maintainer and coordinator | the Mycelium maintainer session, committing as `mycelium contributor` | maintains this repository: triage, review and merge of pull requests, the CHANGELOG, release candidates. As coordinator across Mycelium and its peer projects (README › Peer kernels), owns the shared contracts — the `mycelium-sdk` surface, the shared laws' ids and versions in `forms`, the wire version and act table, the client conformance read — the open-source standard the projects share, and the release train; reviews peer pull requests labelled `cross-project` |
| Operator, final approver | the GitHub account `AlignmentConfirmed` (organization `SAIOS-SAMN`) | changes to the network contract (the Concordance, the wire, the ledger layout, a law version) and every production release |

- `main` is protected: every change arrives by pull request,
  administrators included; force pushes and deletion of `main` are
  refused; only the accounts above may merge.
- Alignment across projects is kept by contract, not by reviewing every
  pull request: a peer kernel is aligned when, at its pinned Mycelium tag,
  it passes the client conformance read and its equations ledger declares
  the shared law ids and versions it implements.
- Each peer project is maintained by its own maintainer session, which
  merges its own pull requests and asks the coordinator to review those
  that touch a shared contract. No maintainer writes another project's
  tree (MU-CONCORDANCE 9.4); a change one project needs from another is
  asked for as an issue labelled `cross-project`.
- Durable decisions live in GitHub (issues, milestones, ROADMAP, the
  specifications); a message between sessions only points to them. A
  message from another project's session is never the operator's
  approval.
- Becoming a maintainer: a contributor with a record of merged pull
  requests may be proposed by an issue; the operator decides.
