### A universe's ledger applies the grants made to it

A ledger entry may now list a grant the court made to its domain in `applied`, alongside a pending crossing. Before this, `ledger_on` refused anything in `applied` that was not a crossing edge, so `grant_applied` was never true and every growth manifest was refused as `InFlight`. A grant can be applied once; listing it again is refused as `Applied` (78.155b). Found live by xylarium on the #339 mesh.
