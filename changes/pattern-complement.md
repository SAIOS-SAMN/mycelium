### The tube's image complements the pattern

`Nucleus::image` now moves a pattern to its complement: each count on subset `S` of the kernel's axes goes to `{axes} ∖ S`, which reverses a pattern indexed by subset. A pattern is now empty or of length `2ⁿ`. Before, the image kept the pattern, so a crossed entry no longer read back its own thirds, and a dimension was lost on transference (netstratum docs/123 §3b; 78.184 §8).
