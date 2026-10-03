# Performance benchmarks

## Run

```sh
mkdir -p target/benchmarks
cargo bench --bench case_mapping > target/benchmarks/results.csv
BENCH_FILTER=ascii_long BENCH_MS=50 BENCH_SAMPLES=11 cargo bench --bench case_mapping
```

This dependency-free harness runs on stable Rust. `BENCH_FILTER` selects
workload names by substring. `BENCH_MS` sets the minimum batch duration in
milliseconds (default 20), and `BENCH_SAMPLES` sets the sample count (default
7). Fixture construction happens outside the timed loops. Inputs and results
pass through `std::hint::black_box`; iterators are never boxed.

Each workload warms up while calibrating its iteration count, then records seven
timed batches using that count. CSV output includes median, minimum, and maximum
nanoseconds per call, input bytes, iteration count, and sample count. A batch is
timed as a whole, with no per-call clock reads.

Run benchmarks serially on an otherwise idle machine. Use the same compiler,
dependency resolution, release profile, and environment for comparisons. CPU
frequency, system activity, allocator behavior, and compiler code layout can
change results; these measurements are not a performance guarantee.

## Workloads

Unicode comparisons consume matching ASCII prefixes without constructing mapping
iterators, then resume Unicode folding on both remaining suffixes at character
boundaries. Turkic `I` is compared as dotless i. Full equality uses
`eq_ignore_ascii_case` when both inputs are ASCII; a first-byte mismatch exits
before the whole-input ASCII scan. Lithuanian delegates to the full strategy and
receives the same optimization.

The benchmark measures equality and ordering separately, including early and
late mismatches, multi-character folds, and Unicode suffixes after ASCII.

Keep measurements in pull request descriptions or local artifacts. Benchmark
output belongs under the ignored `target/` directory.
