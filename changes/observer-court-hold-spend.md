### Observer: read each court hold's spend per lane (#351)

- Added `hold_spend` to `scripts/observer` to parse `court hold <site> took N spent S:G:T:H [..] unnamed M` lines from each hypha's sibling log and print each site's latest spend per lane and unnamed count, sorted by unnamed descending.
