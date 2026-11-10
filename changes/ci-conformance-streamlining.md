### CI Conformance Workflow Streamlining
- Renamed the GitHub Actions workflow from ambiguous `definition of done` to `Conformance Verification`, ensuring clear, professional status reporting and email notifications.
- The required status check job remains `clippy and tests` matching repository branch protection.
- Streamlined compilation: compiles workspace and test binaries once with `--no-run`, letting Clippy and Test execution build directly on top of the shared artifacts without duplicate compilation.
- Caching:
  - Keyed target cache with `${{ github.sha }}` and lockfile fallback so incremental compilation artifacts are continuously updated and restored.
  - Added caching for the ARC-AGI-2 corpus to eliminate repeated network clones.
- Concurrency: Scoped PR cancellation to pull request events only; pushes to `main` are never cancelled mid-flight, eliminating phantom "No jobs were run" cancellation notifications.
