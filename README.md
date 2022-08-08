# focaccia

[![GitHub Actions](https://github.com/artichoke/focaccia/workflows/CI/badge.svg)](https://github.com/artichoke/focaccia/actions)
[![Discord](https://img.shields.io/discord/607683947496734760)](https://discord.gg/QCe2tp2)
[![Twitter](https://img.shields.io/twitter/follow/artichokeruby?label=Follow&style=social)](https://twitter.com/artichokeruby)
<br>
[![Crate](https://img.shields.io/crates/v/focaccia.svg)](https://crates.io/crates/focaccia)
[![API](https://docs.rs/focaccia/badge.svg)](https://docs.rs/focaccia)
[![API trunk](https://img.shields.io/badge/docs-trunk-blue.svg)](https://artichoke.github.io/focaccia/focaccia/)

Unicode case folding methods for case-insensitive string comparisons. Used to
implement case folding operations on the [`Symbol`] and [`String`] classes in
the Ruby Core implementation in [Artichoke Ruby][artichoke].

[`symbol`]: https://ruby-doc.org/core-3.1.2/Symbol.html
[`string`]: https://ruby-doc.org/core-3.1.2/String.html
[artichoke]: https://github.com/artichoke/artichoke

Focaccia supports full, ASCII, and Turkic [Unicode case folding] equality and
ordering comparisons.

[unicode case folding]: https://www.w3.org/International/wiki/Case_folding

> One of the most common things that software developers do is "normalize" text
> for the purposes of comparison. And one of the most basic ways that developers
> are taught to normalize text for comparison is to compare it in a "case
> insensitive" fashion. In other cases, developers want to compare strings in a
> case sensitive manner. Unicode defines upper, lower, and title case properties
> for characters, plus special cases that impact specific language's use of
> text. (W3C, Case Folding)

_focaccia_ is a flat Italian bread. The focaccia crate compares UTF-8 strings by
flattening them to folded downcase. Artichoke goes well with focaccia.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
focaccia = "1.2.0"
```

Then make case insensitive string comparisons like:

```rust
use core::cmp::Ordering;
use focaccia::CaseFold;

let fold = CaseFold::Full;
assert_eq!(fold.casecmp("MASSE", "Maße"), Ordering::Equal);
assert_eq!(fold.casecmp("São Paulo", "Sao Paulo"), Ordering::Greater);

assert!(fold.case_eq("MASSE", "Maße"));
assert!(!fold.case_eq("São Paulo", "Sao Paulo"));
```

For text known to be ASCII, Focaccia can make a more performant comparison
check:

```rust
use core::cmp::Ordering;
use focaccia::CaseFold;

let fold = CaseFold::Ascii;
assert_eq!(fold.casecmp("Crate: focaccia", "Crate: FOCACCIA"), Ordering::Equal);
assert_eq!(fold.casecmp("Fabled", "failed"), Ordering::Less);

assert!(fold.case_eq("Crate: focaccia", "Crate: FOCACCIA"));
assert!(!fold.case_eq("Fabled", "failed"));
```

ASCII case comparison can be checked on a byte slice:

```rust
use core::cmp::Ordering;
use focaccia::{ascii_casecmp, ascii_case_eq};

assert_eq!(ascii_casecmp(b"Artichoke Ruby", b"artichoke ruby"), Ordering::Equal);
assert!(ascii_case_eq(b"Artichoke Ruby", b"artichoke ruby"));
```

Turkic case folding is similar to full case folding with additional mappings for
[dotted and dotless I]:

[dotted and dotless i]: https://en.wikipedia.org/wiki/Dotted_and_dotless_I

```rust
use core::cmp::Ordering;
use focaccia::CaseFold;

let fold = CaseFold::Turkic;
assert_eq!(fold.casecmp("İstanbul", "istanbul"), Ordering::Equal);
assert_ne!(fold.casecmp("İstanbul", "Istanbul"), Ordering::Equal);

assert!(fold.case_eq("İstanbul", "istanbul"));
assert!(!fold.case_eq("İstanbul", "Istanbul"));
```

## Implementation

Focaccia generates conversion tables from Unicode data files. Focaccia
implements case folding as defined in the [Unicode standard][casemap] (see
[`CaseFolding.txt`]).

[casemap]: https://unicode.org/faq/casemap_charprop.html#casemap
[`casefolding.txt`]: CaseFolding.txt

## `no_std`

Focaccia is `no_std` compatible with an optional and enabled by default
dependency on `std`. Focaccia does not link to `alloc` in its `no_std`
configuration.

## Crate features

All features are enabled by default.

- **std** - Enable linking to the [Rust Standard Library]. Enabling this feature
  adds [`Error`] implementations to error types in this crate.

[rust standard library]: https://doc.rust-lang.org/stable/std/index.html
[`error`]: https://doc.rust-lang.org/stable/std/error/trait.Error.html

### Minimum Supported Rust Version

This crate requires at least Rust 1.52.0. This version can be bumped in minor
releases.

## Unicode Version

Focaccia implements Unicode case folding with the Unicode 14.0.0 case folding
ruleset.

Each new release of Unicode may bring updates to the `CaseFolding.txt` which is
the source for the folding mappings in this crate. Updates to the case folding
rules will be accompanied with a minor version bump.

## License

`focaccia` is licensed under the [MIT License](LICENSE) (c) Ryan Lopopolo.

`focaccia` includes Unicode Data Files which are subject to the [Unicode Terms
of Use] and [Unicode Data Files and Software License](LICENSE-UNICODE) (c)
1991-2022 Unicode, Inc.

[unicode terms of use]: https://www.unicode.org/copyright.html

Generated files in this repository are marked with `// @generated` comments and
the Unicode copyright. These generated files incorporate data derived from the
Unicode Data Files. More details about the generation process can be found in
[`scripts/gen_case_lookups.rb`]. The generated sources created by this script
are subject to both the MIT License contained in this repository and the Unicode
Data Files and Software License.

[`scripts/gen_case_lookups.rb`]: scripts/gen_case_lookups.rb
