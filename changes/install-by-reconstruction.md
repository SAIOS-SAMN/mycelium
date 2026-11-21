### A word installs only on a witness's re-derivation, never on a matching root (#434)

`Court::confirm_peer_holds_on` used to install, and pay POWV and the escrow release for, every provisional member of a stratum whose root a peer's digest matched. `confirm_peer_holds_addrs_on` installed a word whenever a peer's have-list named it, comparing this court's own form with itself.
- The root now decides only what to read again: heard addresses under a chart whose root differs are re-opened for the glue.
- The have-list shortcut is deleted, along with its call in the exchange.
- A word installs only through MU-RECURSION 47.3's path: the home offers it, a witness reconstructs it on its own cover, and the two converge (`witness_rederived`).

The tests now install through a fixture that does the same thing. A separate witness court is taught the home's installed words and then reconstructs the provisional form (`witness_installs`). The test that asserted installation on a root now asserts that a matching root installs nothing.

MU-ENGINEERING §3 is amended to match.

**Network contract:** none on the wire or the ledger layout. The change is in when the court installs and pays.
