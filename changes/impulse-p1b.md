### The impulse: a turn's draw sized by its need against the lane's holding (78.183 P1b)

`physics::rate::impulse` gives a turn's draw on each lane as `⌊need · M / (need + M)⌋`, ruled 2026-11-15. It serves the need where the holding is ample, draws what is left where it is scarce, releases half at the throat, and draws nothing with no need; `drawn + kept = M` exactly. No turn is sized by it yet: a turn's per-lane need comes from its spend sites naming their lanes (P2).
