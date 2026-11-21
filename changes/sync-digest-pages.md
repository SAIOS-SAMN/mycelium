### The digest reply is paged by chart (78.187, proposed)

Proposal 78.187 for #349. The digest reply today lists every stratum a court holds, so on a grown book it passes the wire budget and is refused whole.

Under the proposal, a digest request may name the chart to read past. The reply carries the strata from there, in key order, as many as fit the body budget. It ends with a page end: `Store`, `Full`, `Unframable` or `Spent`. Askers hold the last chart they read from each hypha, and pacing stays as documented.

**Network contract:** a wire change (a new `ACT_DIGESTS` sub-kind and a trailing page end). It is a minor step if an older reader ignores the trailing page end, which a claim tests. It needs operator approval. This change adds documents only.
