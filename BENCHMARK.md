# ENS Indexer Benchmark

Generated at `1781973703s since Unix epoch`.

Settings: `5` warmups, `25` measured iterations, `100000`ms timeout, `raw` timing.

| endpoint               | URL                                                                                        | network baseline median | network baseline p95 |
| ---------------------- | ------------------------------------------------------------------------------------------ | ----------------------: | -------------------: |
| ensindexer-rs (Local)  | http://127.0.0.1:8080/subgraph                                                             |                 0.700ms |              0.916ms |
| ensindexer-rs (Hosted) | https://ensindexer-rs.namespace.ninja/subgraph                                             |               155.518ms |            184.175ms |
| ensnode                | https://api.alpha.ensnode.io/subgraph                                                      |               220.124ms |            240.957ms |
| the graph indexer      | https://gateway.thegraph.com/api/subgraphs/id/5XqPmWe6gjyrJtFn9cLy237i4cWw2j9HcUJEXsP5qGtH |               314.363ms |            346.183ms |

| operation                | ensindexer-rs (Local) | ensindexer-rs (Hosted) |              ensnode |    the graph indexer |
| ------------------------ | --------------------: | ---------------------: | -------------------: | -------------------: |
| `01-domain-batch`        |       4.466ms (73.8x) |       164.032ms (2.0x) |     231.052ms (1.4x) |  329.601ms (slowest) |
| `02-names-for-address`   |       4.792ms (67.9x) |       181.218ms (1.8x) |     238.918ms (1.4x) |  325.377ms (slowest) |
| `03-eth-subnames`        |      7.881ms (194.0x) |       174.631ms (8.8x) | 1529.253ms (slowest) |     562.128ms (2.7x) |
| `04-subnames-search`     |      202.868ms (7.5x) |       352.376ms (4.3x) | 1517.865ms (slowest) |     374.081ms (4.1x) |
| `05-decoded-label`       |      1.683ms (190.7x) |       154.375ms (2.1x) |     221.266ms (1.5x) |  320.911ms (slowest) |
| `06-resolver-records`    |      13.136ms (26.9x) |       208.793ms (1.7x) |     235.499ms (1.5x) |  353.409ms (slowest) |
| `07-registrations`       |       8.801ms (92.1x) |       204.791ms (4.0x) |  810.759ms (slowest) |     325.109ms (2.5x) |
| `08-name-history`        |       4.822ms (68.9x) |       174.701ms (1.9x) |     227.133ms (1.5x) |  332.475ms (slowest) |
| `09-event-scan`          |     30.295ms (111.4x) |      270.159ms (12.5x) |          unsupported | 3375.832ms (slowest) |
| `10-relationship-filter` |       7.715ms (42.1x) |       166.263ms (2.0x) |          unsupported |  324.780ms (slowest) |
| `11-text-search`         |       79.652ms (4.5x) |       348.343ms (1.0x) |  355.755ms (slowest) |     319.334ms (1.1x) |

Relative speed is calculated against the slowest supported numeric result in each row. Unsupported, timeout, and error cells are excluded from the numeric baseline.