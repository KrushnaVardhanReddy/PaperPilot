# PaperPilot — System Benchmark Report
Generated: 2026-09-30 12:26:36
Platform: Linux 6.8.0, Intel(R) Xeon(R) Processor @ 2.30GHz, 7 GB

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
| Merge | 0.00 µs | 6.28 ms | 6.28 ms | 16845.01 |
| Split | 0.00 µs | 11.20 ms | 11.20 ms | 45594.31 |
| ExtractText | 0.00 µs | 124.60 ms | 124.60 ms | 4032.73 |
| Compress | 0.00 µs | 30.49 ms | 30.49 ms | 16534.32 |
| Watermark | 0.00 µs | 4.79 ms | 4.79 ms | 105719.87 |
| Encrypt | 0.00 µs | 3.61 ms | 3.61 ms | 144578.61 |
| OCR | 0.00 µs | 922.08 µs | 922.08 µs | 108942.02 |
| Render | 0.00 µs | 557.07 µs | 557.07 µs | 90938.56 |

## Cold Start vs Warm Start
| Operation | Cold Start | Warm (avg) | Delta |
|---|---|---|---|
| ExtractText | 9.37 ms | 9.18 ms | 194.00 µs |
| Merge | 5.63 ms | 5.08 ms | 547.00 µs |

## Memory Leak Detection (1000 Iterations)
| Operation | Iteration | RSS (MB) |
|---|---|---|
| Merge | 100 | 39.05 MB |
| Merge | 200 | 39.05 MB |
| Merge | 300 | 39.05 MB |
| Merge | 400 | 39.05 MB |
| Merge | 500 | 39.05 MB |
| Merge | 600 | 39.05 MB |
| Merge | 700 | 39.05 MB |
| Merge | 800 | 39.05 MB |
| Merge | 900 | 39.05 MB |
| Merge | 1000 | 39.05 MB |
| ExtractText | 100 | 39.05 MB |
| ExtractText | 200 | 39.05 MB |
| ExtractText | 300 | 39.05 MB |
| ExtractText | 400 | 39.05 MB |
| ExtractText | 500 | 39.05 MB |
| ExtractText | 600 | 39.05 MB |
| ExtractText | 700 | 39.05 MB |
| ExtractText | 800 | 39.05 MB |
| ExtractText | 900 | 39.05 MB |
| ExtractText | 1000 | 39.05 MB |

## Concurrent Operations
| Mode | Total Time |
|---|---|
| Sequential | 20.14 ms |
| Concurrent | 14.69 ms |

## NLP Resolver Throughput (10,000 queries)
| Metric | Value |
|---|---|
| Average Latency (per query) | 0.00 µs |
| P99 Latency (batch) | 14.49 ms |
| Throughput | 690578.88 queries/s |