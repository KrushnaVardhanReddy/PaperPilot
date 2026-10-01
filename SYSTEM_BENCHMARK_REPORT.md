# PaperPilot — System Benchmark Report
Generated: 2026-10-01 04:58:32
Platform: Linux 7.0.0-34-generic, 13th Gen Intel(R) Core(TM) i9-13900H, 15 GB

## Core Operations Scaling (Latency by Page Count)
| Operation | 1 page | 10 pages | 50 pages | 100 pages | 500 pages |
|---|---|---|---|---|---|
| Merge | - | - | - | 0.00 µs | - |
| Split | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs |
| ExtractText | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs |
| Compress | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs |
| Watermark | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs |
| Encrypt | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs | 0.00 µs |
| OCR | - | - | - | 0.00 µs | - |
| Render | - | - | 0.00 µs | - | - |

## Core Operations — P50/P95/P99 Latency (Max Size)
| Operation | P50 | P95 | P99 | Throughput (pages/s) |
|---|---|---|---|---|
| Merge | 0.00 µs | 1.42 ms | 1.42 ms | 75228.24 |
| Split | 0.00 µs | 5.27 ms | 5.27 ms | 105977.06 |
| ExtractText | 0.00 µs | 51.98 ms | 51.98 ms | 9991.97 |
| Compress | 0.00 µs | 2.05 ms | 2.05 ms | 261871.57 |
| Watermark | 0.00 µs | 3.12 ms | 3.12 ms | 201802.37 |
| Encrypt | 0.00 µs | 3.47 ms | 3.47 ms | 165258.52 |
| OCR | 0.00 µs | 3.34 ms | 3.34 ms | 32349.15 |
| Render | 0.00 µs | 1.55 ms | 1.55 ms | 38856.26 |

## Cold Start vs Warm Start
| Operation | Cold Start | Warm (avg) | Delta |
|---|---|---|---|
| ExtractText | 2.55 ms | 2.86 ms | -305.00 µs |
| Merge | 3.10 ms | 1.82 ms | 1.28 ms |

## Memory Leak Detection (1000 Iterations)
| Operation | Iteration | RSS (MB) |
|---|---|---|
| Merge | 100 | 99.73 MB |
| Merge | 200 | 99.73 MB |
| Merge | 300 | 99.73 MB |
| Merge | 400 | 99.73 MB |
| Merge | 500 | 99.73 MB |
| Merge | 600 | 99.73 MB |
| Merge | 700 | 99.73 MB |
| Merge | 800 | 99.73 MB |
| Merge | 900 | 99.73 MB |
| Merge | 1000 | 99.73 MB |
| ExtractText | 100 | 99.73 MB |
| ExtractText | 200 | 99.73 MB |
| ExtractText | 300 | 99.73 MB |
| ExtractText | 400 | 99.73 MB |
| ExtractText | 500 | 99.73 MB |
| ExtractText | 600 | 99.73 MB |
| ExtractText | 700 | 99.73 MB |
| ExtractText | 800 | 99.73 MB |
| ExtractText | 900 | 99.73 MB |
| ExtractText | 1000 | 99.73 MB |

## Concurrent Operations
| Mode | Total Time |
|---|---|
| Sequential | 8.84 ms |
| Concurrent | 5.35 ms |

## NLP Resolver Throughput (10,000 queries)
| Metric | Value |
|---|---|
| Average Latency (per query) | 0.00 µs |
| P99 Latency (batch) | 6.76 ms |
| Throughput | 1508326.06 queries/s |