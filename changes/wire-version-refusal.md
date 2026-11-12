### A peer at another wire version is refused by name (#235)

**Network contract.** Every expression now carries the wire version it is written at as its last two bytes: `[act][point][point count][wire version u16 LE]`. A reader reads the version first:
- **Another major** is refused before the point or the act are read, as `Unheard::Version { spoken, served }` (`Expression::heard`).
- **Another minor of the same major** is heard, since a minor step is additive (MU-ENGINEERING §4).

How each side refuses:
- **The door** answers an act at another major refused, at its own version, and closes the connection. It counts these refusals and logs them at powers of two.
- **A session** refuses such a reply with an error naming both versions.
- **A hypha** asking a peer drops the peer's reply at another major, and logs both versions.

Until now `WIRE_VERSION` was compared nowhere, so an older peer met undecodable bytes, never a refusal.
