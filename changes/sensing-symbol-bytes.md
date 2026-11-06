### `Symbol::bytes` public for decoders and the speech end (track 26, #32; netstratum's request)
- **Library surface, no network contract.** `transducer::sample::Symbol::bytes`
  is public, documented for a source's decoder and the speech end only,
  beside `Sample::sites` and `Sample::symbol_at`, so a house reads a mind's
  outputs back into its own format. No `raster_of` is exported.
- The ARC-AGI-2 decoder writes a sample back into the corpus's rows
  (`mycelium_source_arc_agi::rows_of`), refusing a sample of another rank
  or width.
- Claim: `rows_of_writes_a_sample_back_into_the_corpus_rows`.
