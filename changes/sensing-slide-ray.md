### The ray of a slide or an extend is a translation versor of the sample's rank (track 26, #32)
- **Ruled 2026-11-06 on #32 (5).** `Act::Slide` and `Act::Extend` (and the
  SDK's `Create::Extend`) hold a `clause::Ray`: one step per axis of the
  sample, `n` components at rank `n`, no plane assumed. Its versor is the
  translator by its steps (`Ray::versor`), which the lowered carriage holds.
- **Writing.** The clause reference writes a ray as its count of steps,
  then each step a byte. A form writes its elements (the translator), so no
  form, word or chart address moves.
- Claim: `slide_ray_is_a_translation_versor_of_the_sample_rank`;
  `put_clauses_round_trips_reactions` reads the new writing.
