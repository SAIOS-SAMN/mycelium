# mycelium-algebra

Exact rational Clifford geometric algebra $\mathcal{C}l(p, q)(\mathbb{Q})$ in pure Rust.

No floating-point numbers, no approximations, and no unsafe code (denied by the workspace's lints).

`mycelium-algebra` implements the exact multivector algebra $\mathcal{C}l(p, q)$ over the field of rational numbers $\mathbb{Q}$ (via `num-rational::BigRational`). Every multivector carries its dimension $n$ and the bitmask of its negative generators, computing exact geometric products, inner products, wedge products, reversions, and inversions for arbitrary quadratic signatures.

## Features

- **Exact Arithmetic:** All scalars and components are exact arbitrary-precision rationals ($\mathbb{Q}$). No IEEE-754 floats, no epsilon checks, no rounding drift.
- **Arbitrary Signatures $\mathcal{C}l(p, q)$:** Supports Euclidean, pseudo-Euclidean, Minkowski, and Conformal signatures.
- **Conformal Geometric Algebra $\mathcal{C}l(n+1, 1)$:** the conformal models of $\mathbb{Q}^n$ as a case of the signature, with the generators $e_+$ ($e_+^2 = +1$) and $e_-$ ($e_-^2 = -1$).
- **No unsafe code:** `unsafe_code` is denied by the workspace's lint table.
- **Standalone:** Zero dependency on blockchain, networking, or runtime components. Depends only on `num-rational`, `num-bigint`, and `num-traits`.

## Usage

Add `mycelium-algebra` to your `Cargo.toml`:

```toml
[dependencies]
mycelium-algebra = "0.4.0-rc.1"
```

### Example: the exact geometric product

```rust
use mycelium_algebra::{Mv, E1, E2};
use num_rational::BigRational;

let one = BigRational::from_integer(1.into());
let e1 = Mv::blade(E1, one.clone());
let e2 = Mv::blade(E2, one.clone());

// e1 e2 is the bivector e12.
let bivector = e1.geo(&e2);
assert_eq!(bivector, Mv::blade(E1 | E2, one.clone()));

// A Euclidean generator squares to +1.
assert_eq!(e1.geo(&e1), Mv::scalar(one));

// The generators anticommute: e2 e1 = -e12.
assert!(bivector.add(&e2.geo(&e1)).is_zero());
```

## Mathematical Foundations

In $\mathcal{C}l(p, q)(\mathbb{Q})$, an element $A$ is represented as a formal linear combination of $2^{n}$ basis blades:

$$A = \sum_{I} \alpha_I e_I, \quad \alpha_I \in \mathbb{Q}$$

For the conformal model of $\mathbb{Q}^n$, two generators $e_+, e_-$ close the null cone:
$$e_+^2 = +1, \quad e_-^2 = -1, \quad e_\infty = e_- + e_+, \quad e_o = \frac{1}{2}(e_- - e_+)$$

## License

Licensed under the Apache License, Version 2.0.
