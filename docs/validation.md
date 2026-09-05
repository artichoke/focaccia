# Correctness validation

The independent audit in `src/folding/mapping/oracle.rs` parses
`CaseFolding.txt` directly. It does not use `scripts/gen_case_lookups.rb` or the
generated reference tables in `src/exhaustive`. Full folding uses C/F records;
Turkic folding applies T overrides. Simple (S) records are excluded.

Run the normal suite, including the scalar and expansion checks:

```sh
cargo test
```

Run the larger pairwise audit explicitly in release mode:

```sh
cargo test --release every_pair_of_mapping_sources_and_outputs -- --ignored --nocapture
```

The large audit is ignored by default to avoid adding its pairwise workload to
every debug test run. It was explicitly executed during this validation.

## Coverage

- Every one of the 1,112,064 Unicode scalar values: exact full and Turkic lookup
  output, iterator count, size hints, and exhaustion. These compare actual
  output code points, so accidentally mapping two different characters to the
  same value cannot pass merely because both sides fold equally.
- Every scalar through all four public strategies and their free functions:
  equality with the independently folded output and ordering against empty.
- All 65,536 ordered byte pairs and all 65,536 two-byte inputs: ASCII equality,
  ordering, folding, and prefix behavior, including non-UTF-8 bytes.
- All ordered pairs of the 3,113 distinct mapping sources, mapping outputs,
  empty string, NUL, and maximum scalar: 9,690,769 pairs per strategy, or
  38,763,076 comparisons across Full, Turkic, Lithuanian, and ASCII. Both enum
  methods and free functions are checked against the reference result.
- Expanding mappings with NUL, ASCII, combining acute, combining dot above, and
  maximum-scalar suffixes; all ordered pairs of that corpus; and every
  concatenation of two expanding source characters. These exercise comparisons
  where expansion output crosses input-character boundaries.

This is exhaustive over the finite domains listed above, not over all possible
strings or all pairs of Unicode scalars. Arbitrary-length strings form an
infinite domain. The implementation composes per-scalar mappings with Rust's
iterator equality and lexicographic comparison; the audit checks all mappings
and separately exercises that composition.

## Results (2026-09-05)

No implementation errors were found. The normal suite passed on Rust 1.83.0 (the
MSRV), with 75 tests passing and the large audit ignored. The large audit passed
separately on stable Rust in release mode. Clippy with warnings denied also
passed.

The checked-in data was compared byte-for-byte with the official [Unicode 17.0
case-folding data]. Both had SHA-256:

```text
ff8d8fefbf123574205085d6714c36149eb946d717a0c585c27f0f4ef58c4183
```

An independent runtime sweep used CRuby 3.4.1 via the official ruby.wasm npm
package `@ruby/3.4-wasm-wasi@2.10.1`. This binary reports Unicode 15.0.0; it is
not the repository's pinned Ruby 3.4.10 runtime.

For every scalar, `String#downcase(:fold)` was compared with the Unicode 17
reference mapping. There were 55 differences. Every difference exactly matched
an entry changed between the official [Unicode 15.0 case-folding data] and
Unicode 17.0 data; there were no unexplained differences. Thus Focaccia's
Unicode 17 behavior is not byte-for-byte identical to this older Ruby's Unicode
15 behavior. Supporting a particular Ruby Unicode version would be a separate
compatibility decision.

`String#downcase(:lithuanian)` matched default downcasing for every scalar. The
29 Lithuanian comparison vectors in `tests/lithuanian.rs` were also verified
against this runtime, including accented and combining-character sequences. Ruby
does not accept `:fold` together with `:lithuanian`; the reference comparison
uses `downcase(:fold)`, while the no-op Lithuanian mapping option is checked
separately. This does not establish parity for every Ruby encoding or Ruby's
ASCII-only `String#casecmp` method.

[Unicode 17.0 case-folding data]:
  https://www.unicode.org/Public/17.0.0/ucd/CaseFolding.txt
[Unicode 15.0 case-folding data]:
  https://www.unicode.org/Public/15.0.0/ucd/CaseFolding.txt
