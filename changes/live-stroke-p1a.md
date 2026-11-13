### The metabolic rate per lane, read per bell span (78.183 §7.5, P1a)

Each court closes a bell span whenever its line's height moves. Per lane it reads what the span drew from the reserve (`E_l`), what it restored (`Y_l`), the holding `M_l`, and the draw the open share sizes, `d_l · M_l` with `d_l = E_l/(E_l + Y_l)` (`physics::rate`, `Court::close_rate`). The court logs it as `court rate h … | E … Y … M … drawn …`. Nothing is sized by it yet (P1b). Live, the reading shows the present metabolism moving all four lanes by one identical quantum each span.
