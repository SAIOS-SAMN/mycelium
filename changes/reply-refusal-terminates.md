### An over-budget reply's refusal terminates

`refuse_over_budget` wrote its refusal back through `answer`. When the door's final point was itself past the wire budget, the refusal failed the same way and recursed until the stack overflowed (hypha b, 2026-11-15, after about 21,000 refusals). The refusal is now written beside the final point where it fits, otherwise with no point at all, and never goes back through `answer`. The log line now also names the final point's size.
