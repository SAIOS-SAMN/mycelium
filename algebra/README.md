# mycelium-algebra

Exact rational Clifford geometric algebra $\mathcal{C}l(p, q)(\mathbb{Q})$ in pure Rust.

Zero floating-point numbers. Zero approximations. Zero unsafe code.

`mycelium-algebra` implements the exact multivector algebra $\mathcal{C}l(p, q)$ over the field of rational numbers $\mathbb{Q}$ (via `num-rational::BigRational`). Every multivector carries its dimension $n$ and the bitmask of its negative generators, computing exact geometric products, inner products, wedge products, reversions, and inversions for arbitrary quadratic signatures.

## Features

- **Exact Arithmetic:** All scalars and components are exact arbitrary-precision rationals ($\mathbb{Q}$). No IEEE-754 floats, no epsilon checks, no rounding drift.
- **Arbitrary Signatures $\mathcal{C}l(p, q)$:** Supports Euclidean, pseudo-Euclidean, Minkowski, and Conformal signatures.
- **Conformal Geometric Algebra $\mathcal{C}l(n+1, 1)$:** First-class support for conformal models of $\mathbb{Q}^n$ with null generators $e_+$ ($e_+^2 = +1$) and $e_-$ ($e_-^2 = -1$).
- **No Unsafe Code:** Strictly `#![forbid(unsafe_code)]`.
- **Standalone:** Zero dependency on blockchain, networking, or runtime components. Depends only on `num-rational`, `num-bigint`, and `num-traits`.

## Usage

Add `mycelium-algebra` to your `Cargo.toml`:

```toml
[dependencies]
mycelium-algebra = "0.4.0-rc.1"
```

### Example: Exact Rotations and Reflections

```rust
use mycelium_algebra::{Clifford, Mv, E1, E2, E3};
use num_rational::BigRational;

fn main() {
    // Exact unit vector e1 in Cl(3, 0)
    let e1 = Mv::generator(1);
    let e2 = Mv::generator(2);

    // Exact geometric product: e1 * e2 = e12 (bivector)
    let bivector = e1.geo(&e2);

    // e1 * e1 = 1 (Euclidean basis vector squares to +1)
    let scalar_one = e1.geo(&e1);
    assert!(scalar_one.is_one());

    // Anti-commutation: e2 * e1 = -e12
    let reverse = e2.geo(&e1);
    assert_eq!(bivector.add(&reverse), Mv::zero());
}
```

## Mathematical Foundations

In $\mathcal{C}l(p, q)(\mathbb{Q})$, an element $A$ is represented as a formal linear combination of $2^{n}$ basis blades:

$$A = \sum_{I} \alpha_I e_I, \quad \alpha_I \in \mathbb{Q}$$

For the conformal model of $\mathbb{Q}^n$, two generators $e_+, e_-$ close the null cone:
$$e_+^2 = +1, \quad e_-^2 = -1, \quad e_\infty = e_- + e_+, \quad e_o = \frac{1}{2}(e_- - e_+)$$

## License

Licensed under the Apache License, Version 2.0.
