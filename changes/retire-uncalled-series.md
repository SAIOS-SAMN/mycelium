### The uncalled certified series retire (78.182 item 25, D4.8 O2)

- **`gamma_p`, `gamma_p_terms`, `erf` and `poisson_quantile` are removed** from `core::certified`, with `Quantile` and `SeriesRefusal`. They had no caller, and they are continuous approximations outside the exact kernel.
- **The functions every kernel uses stay**: `exp`, `ln`, `pi`, `sqrt`, `powq`, the circular and hyperbolic functions, `lgamma` and `planck_tail`.
