### 78.74 (amended) — First-degree tether strength, reach, and projection (#250)

- Retires `length2()` and `capacity()` from `PivotFibre` (78.74 inverse-square continuum mechanics and Law 11 scalar metric collapse).
- Restates `PivotFibre::strength()` as the first-degree held face share:
  $$c_{ij} = \frac{m_i + m_j}{r_{ij} + m_i + m_j} = \frac{1}{1 + t_{ij}} \in (0, 1]$$
- Adds `fluidity(&self) -> Q`: $d_{ij} = 1 - c_{ij} = \frac{t_{ij}}{1 + t_{ij}}$, satisfying $c_{ij} + d_{ij} = 1$ identically on every tether.
- Implements first-degree axial reach $r_{ij} = \sum_k |x_{i,k} - x_{j,k}| \in \mathbb{Q}_{\ge 0}$ across active planes without quadratic embeddings or irrational square roots.
- Restates `PivotFibre::project(&self, x)` as the first-degree pole ratio $\mu = \frac{t_{jX}}{t_{iX}} = \frac{m_i \cdot r_{jX}}{m_j \cdot r_{iX}}$, preserving the switch level $\mu = 1$ and switch side semantics in `transducer::field::pivot`.
- Updates `transducer::family::torsion_quotient` binding calculations and tests.
