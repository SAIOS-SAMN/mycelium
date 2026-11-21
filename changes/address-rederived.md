### A decoded element's address is re-derived from its content (#435)

`element::record` decoding no longer takes a word's address from its record bytes. For a form, the decoder re-derives the address from the decoded element (`sentence::form_vertex_key`). For any other element, the decoder recomputes it. A record naming an address other than the one its content derives is refused. This follows MU-ANTI-FLATTENING 13 (identity from content) and MU-RECURSION 1.5. The wire and the ledger layout are unchanged.
