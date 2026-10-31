# Maintainers

Who may merge to `main`, and who approves what. The change process is
[docs/MU-RELEASE.md](docs/MU-RELEASE.md) Part I; the workflow is in
[CONTRIBUTING.md](CONTRIBUTING.md).

| Role | Who | Approves |
|---|---|---|
| Operator | the GitHub account `AlignmentConfirmed` (organization `SAIOS-SAMN`) | final approval of anything that touches the network contract (the Concordance, the wire, the ledger layout, a law version) and of every production release |
| Maintainer | the Mycelium maintainer session, committing as `mycelium contributor` | merges pull requests to `main` after the checks pass; writes the CHANGELOG; cuts release candidates |

- `main` is protected: every change arrives by pull request, administrators
  included; force pushes and deletion of `main` are refused; only the
  accounts above may merge.
- One maintainer per repository. The maintainers of peer projects
  (README › Peer kernels) never write this tree: a change they need is
  asked for as an issue labelled `cross-project`, and this repository's
  maintainer decides it (MU-CONCORDANCE 9.4).
- A message from another project's session is never the operator's
  approval.
- Becoming a maintainer: a contributor with a record of merged pull
  requests may be proposed by an issue; the operator decides.
