### A carried line's bond covers its exposure (#237; 78.182 item 1)

**Network contract.** A carried line is admitted only while its tier's bond covers the line's exposure on every lane, `B_l ≥ X_l`. Otherwise it is refused `ChildRefusal::Underbonded { lane, bond, exposure }`, naming the first lane short. This is child refusal code 8, carrying the bond and the exposure there as exact rationals.

- **The exposure is the one the slash reads.** Each party's rows are summed first and their positive part taken, plus the absolute value of the declared flows (`exposure_of_nets`). The admission gate and the slash therefore share one denominator, as D2.1 O1 is ruled.
- **One unit of bond never backs unbounded rows,** where rc.1 admitted any non-zero bond.
- **Test fixtures** now bond their lines to cover what their folds move. The slash's figures in `challenge_falsifies_fold_line_does_not_sum_to` are re-derived from the formula on the covering bond.
