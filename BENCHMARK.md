# ENS Indexer Benchmark

Generated at `1782005170s since Unix epoch`.

Settings: `3` warmups, `5` measured iterations, `100000`ms timeout, `raw` timing.

| endpoint | URL | network baseline median | network baseline p95 |
| --- | --- | ---: | ---: |
| ensindexer-rs (Hosted) | https://ensindexer-rs.namespace.ninja/subgraph | 155.502ms | 165.064ms |
| ensnode | https://api.alpha.ensnode.io/subgraph | 216.618ms | 227.199ms |
| the graph indexer | https://gateway.thegraph.com/api/subgraphs/id/5XqPmWe6gjyrJtFn9cLy237i4cWw2j9HcUJEXsP5qGtH | 309.976ms | 360.953ms |

| operation | ensindexer-rs (Hosted) | ensnode | the graph indexer |
| --- | ---: | ---: | ---: |
| `01-domain-batch` | 161.760ms (2.2x) | 236.639ms (1.5x) | 349.134ms (slowest) |
| `02-names-for-address` | 165.385ms (2.1x) | 230.997ms (1.5x) | 341.654ms (slowest) |
| `03-eth-subnames` | 172.312ms (8.9x) | 1529.751ms (slowest) | 394.077ms (3.9x) |
| `04c-subnames-search-3-letter` | 478.278ms (3.6x) | 1734.080ms (slowest) | 370.343ms (4.7x) |
| `04d-subnames-search-4-letter` | 289.240ms (6.2x) | 1804.081ms (slowest) | 386.593ms (4.7x) |
| `04e-subnames-search-5-letter` | 231.535ms (7.4x) | 1722.582ms (slowest) | 597.656ms (2.9x) |
| `05-decoded-label` | 154.759ms (2.3x) | 219.236ms (1.6x) | 359.036ms (slowest) |
| `06-resolver-records` | 194.695ms (2.2x) | 232.694ms (1.9x) | 430.787ms (slowest) |
| `07-registrations` | 188.012ms (4.3x) | 802.861ms (slowest) | 352.961ms (2.3x) |
| `07a-subgraph-registrant` | 156.093ms (2.1x) | 218.612ms (1.5x) | 331.832ms (slowest) |
| `08-name-history` | 170.191ms (2.0x) | 229.845ms (1.5x) | 343.990ms (slowest) |
| `09-event-scan` | 243.978ms (24.4x) | unsupported | 5957.348ms (slowest) |
| `10-relationship-filter` | 163.389ms (2.7x) | unsupported | 444.856ms (slowest) |
| `11-text-search` | 282.678ms (1.8x) | 352.325ms (1.5x) | 511.326ms (slowest) |

Relative speed is calculated against the slowest supported numeric result in each row. Unsupported, timeout, and error cells are excluded from the numeric baseline.