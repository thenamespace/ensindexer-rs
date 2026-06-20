# ENS Indexer Benchmark

Generated at `1781959064s since Unix epoch`.

Settings: `5` warmups, `25` measured iterations, `30000`ms timeout, `raw` timing.

| endpoint | URL | network baseline median | network baseline p95 |
| --- | --- | ---: | ---: |
| ensindexer-rs (Local) | http://127.0.0.1:8080/subgraph | 0.894ms | 1.545ms |
| ensnode | https://api.alpha.ensnode.io/subgraph | 221.073ms | 298.620ms |
| the graph indexer | https://gateway.thegraph.com/api/subgraphs/id/5XqPmWe6gjyrJtFn9cLy237i4cWw2j9HcUJEXsP5qGtH | 325.838ms | 371.504ms |

| operation | ensindexer-rs (Local) | ensnode | the graph indexer |
| --- | ---: | ---: | ---: |
| `01-domain-batch` | 5.709ms (57.1x) | 227.092ms (1.4x) | 326.183ms (slowest) |
| `02-names-for-address` | 5.677ms (61.7x) | 230.692ms (1.5x) | 350.331ms (slowest) |
| `03-eth-subnames` | 6.345ms (231.2x) | 1467.357ms (slowest) | 527.315ms (2.8x) |
| `04-subnames-search` | 180.241ms (8.4x) | 1509.342ms (slowest) | 475.438ms (3.2x) |
| `05-decoded-label` | 0.727ms (441.1x) | 223.444ms (1.4x) | 320.734ms (slowest) |
| `06-resolver-records` | 11.706ms (30.3x) | 238.577ms (1.5x) | 354.671ms (slowest) |
| `07-registrations` | 8.169ms (96.9x) | 791.757ms (slowest) | 331.595ms (2.4x) |
| `08-name-history` | 4.620ms (71.0x) | 227.947ms (1.4x) | 328.274ms (slowest) |
| `09-event-scan` | 24.628ms (169.9x) | unsupported | 4185.512ms (slowest) |
| `10-relationship-filter` | 7.810ms (43.0x) | unsupported | 335.809ms (slowest) |
| `11-text-search` | 79.856ms (4.7x) | 355.761ms (1.0x) | 371.400ms (slowest) |

Relative speed is calculated against the slowest supported numeric result in each row. Unsupported, timeout, and error cells are excluded from the numeric baseline.