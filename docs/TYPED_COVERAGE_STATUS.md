# Typed Coverage Status

This document tracks completion-state accounting for the typed wrapper surface.

- Counting rule: one first-class typed helper per capability selection.
- Helper variants such as `torn.stock(...)` are treated as detail helpers for an existing selection, not extra coverage.
- Raw helpers are available everywhere and are not counted as typed coverage.
- The current tree still contains the older pre-company wrapper subset; completion accounting includes the adjacent company wrapper landing and the final faction tranche.

Completion snapshot: `189 / 189` capability selections.

## Resource Summary

| Resource | Typed | Total |
| --- | ---: | ---: |
| `company` | 9 | 9 |
| `user` | 68 | 68 |
| `faction` | 44 | 44 |
| `forum` | 6 | 6 |
| `key` | 2 | 2 |
| `market` | 9 | 9 |
| `property` | 3 | 3 |
| `racing` | 8 | 8 |
| `torn` | 40 | 40 |

## Selection Lists

### `company` (`9 / 9`)

`applications`, `companies`, `detailed`, `employees`, `lookup`, `news`, `profile`, `stock`, `timestamp`

### `user` (`68 / 68`)

`ammo`, `attacks`, `attacksfull`, `bars`, `basic`, `battlestats`, `bazaar`, `bounties`, `calendar`, `casino`, `competition`, `cooldowns`, `crimes`, `criminalrecord`, `discord`, `display`, `education`, `enlistedcars`, `equipment`, `events`, `faction`, `factionbalance`, `forumfeed`, `forumfriends`, `forumposts`, `forumsubscribedthreads`, `forumthreads`, `gym`, `hof`, `honors`, `icons`, `inventory`, `itemmarket`, `job`, `jobpoints`, `jobranks`, `list`, `log`, `lookup`, `medals`, `merits`, `messages`, `missions`, `money`, `networth`, `newevents`, `newmessages`, `notifications`, `organizedcrime`, `perks`, `personalstats`, `profile`, `properties`, `property`, `races`, `racingrecords`, `refills`, `reports`, `revives`, `revivesfull`, `skills`, `stocks`, `timestamp`, `trade`, `trades`, `travel`, `weaponexp`, `workstats`

### `faction` (`44 / 44`)

`revives`, `reports`, `revivesfull`, `stats`, `search`, `rankedwars`, `rackets`, `positions`, `raidreport`, `rankedwarreport`, `raids`, `utilities`, `upgrades`, `warfare`, `weapons`, `wars`, `timestamp`, `territory`, `temporary`, `territoryownership`, `territorywars`, `territorywarreport`, `caches`, `boosters`, `cesium`, `chainreport`, `chain`, `basic`, `armor`, `applications`, `attacks`, `balance`, `attacksfull`, `lookup`, `hof`, `medical`, `news`, `members`, `drugs`, `contributors`, `chains`, `crime`, `crimes`, `crimeexp`

Completion-only delta from the older `32 / 44` faction snapshot:

- `armor`, `boosters`, `caches`, `cesium`, `crime`, `crimeexp`, `crimes`, `drugs`, `medical`, `temporary`, `utilities`, `weapons`

### `forum` (`6 / 6`)

`categories`, `lookup`, `threads`, `thread`, `posts`, `timestamp`

### `key` (`2 / 2`)

`info`, `log`

### `market` (`9 / 9`)

`bazaar`, `lookup`, `timestamp`, `pointsmarket`, `properties`, `rentals`, `itemmarket`, `auctionhouse`, `auctionhouselisting`

### `property` (`3 / 3`)

`property`, `lookup`, `timestamp`

### `racing` (`8 / 8`)

`tracks`, `cars`, `carupgrades`, `races`, `race`, `records`, `lookup`, `timestamp`

### `torn` (`40 / 40`)

`merits`, `medals`, `organisedcrimes`, `pawnshop`, `organizedcrimes`, `itemstats`, `items`, `logcategories`, `lookup`, `logtypes`, `stocks`, `stats`, `subcrimes`, `timestamp`, `territory`, `properties`, `pokertables`, `rockpaperscissors`, `shoplifting`, `searchforcash`, `companies`, `cityshops`, `competition`, `education`, `crimes`, `bank`, `attacklog`, `bounties`, `cards`, `calendar`, `honors`, `hof`, `itemammo`, `itemmods`, `itemdetails`, `eliminationteam`, `elimination`, `factionhof`, `gyms`, `factiontree`

Additional typed detail helper:

- `torn.stock(...)` for single-stock detail on top of `torn.stocks`

Current untyped selections:

- none

## Strategy Notes

- Completion accounting assumes first-class typed coverage for every listed resource selection, including the v1-backed `company` resource.
- Raw escape hatches still matter for selection batching, future API additions, and high-variance payloads even when a typed helper exists.
- Pagination metadata is preserved on typed bundles instead of hidden in transport internals.
