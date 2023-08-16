# Decompound

Decomposes a compound word into its constituent parts. Works in any language, as you
provide the rules around what constitutes a (*single*) word. The algorithm is
Unicode-aware.

## Usage

Usage is very straightforward. The party piece is a closure, deciding whether a *single*
word is valid. As this can be highly dynamic and language-specific, this decision is
left to the user.

```rust
use decompound::{decompound, DecompositionOptions};

let is_valid_single_word = |w: &str| ["jet", "ski",].contains(&w);

assert_eq!(
    decompound(
        "jetski",
        &is_valid_single_word,
        DecompositionOptions::empty(),
    ).unwrap(),
    vec!["jet", "ski"]
);
```

Candidates for validity checks are simple dictionary lookups (for example, using
[`std::collections::HashSet`], [`phf`](https://crates.io/crates/phf), [Finite State
Transducers](https://docs.rs/fst/latest/fst/), [binary
search](https://docs.rs/b4s/latest/b4s/), ...), or any elaborate algorithm of your
choice.

### Configuration

Configuration is exposed as a [bit field](https://en.wikipedia.org/wiki/Bit_field) via
[`DecompositionOptions`]. It affords more complex use cases, freely combinable.
Usefulness depends on the natural language at hand. Some, for example German, might require:

```rust
use decompound::{decompound, DecompositionError, DecompositionOptions};

let is_valid_single_word = |w: &str| ["Haus", "Boot", "Küche"].contains(&w);

assert_eq!(
    decompound(
        "Hausboot-Küche",
        &is_valid_single_word,
        // Wouldn't find anything without titlecasing `boot` to `Boot`,
        // and splitting on hyphens.
        DecompositionOptions::SPLIT_HYPHENATED | DecompositionOptions::TRY_TITLECASE_SUFFIX
    ).unwrap(),
    vec!["Haus", "Boot", "Küche"]
);
```

This covers all currently available options already, see also:

```rust
use decompound::DecompositionOptions;

assert!(
    (
        DecompositionOptions::SPLIT_HYPHENATED
        | DecompositionOptions::TRY_TITLECASE_SUFFIX
    ).is_all()
);
```

### Failure modes

If the word cannot be decomposed, a [`DecompositionError`] is returned.

```rust
use decompound::{decompound, DecompositionError, DecompositionOptions};

let is_valid_single_word = |w: &str| ["jet", "ski"].contains(&w);

assert_eq!(
    decompound(
        "snowball",
        &is_valid_single_word,
        DecompositionOptions::empty(),
    ),
    Err(DecompositionError::NothingValid)
);
```

#### Overeager validity checks

Nothing prevents you from providing a closure *which itself accepts compound words*.
Compound words being included in a lookup dictionary (instead of *only* root words) is
an example 'pathological' case. Accommodating compound words yourself is precisely what
this crate is [supposed to alleviate](#motivation). If you already have that capability,
the crate at hand is not needed (hence "overeager checks").

Although [`decompound`] always prefers splits if possible,

```rust
use decompound::{decompound, DecompositionError, DecompositionOptions};

// Contains a compound word *and* its root words.
let is_valid_single_word = |w: &str| ["railroad", "rail", "road"].contains(&w);

assert_eq!(
    decompound(
        "railroad",
        &is_valid_single_word,
        DecompositionOptions::empty(),
    ).unwrap(),
    vec!["rail", "road"]
);
```

if root words are missing but the compound itself is present, decomposition technically
*fails*. This case is considered an error, and marked as such. That is [more
ergonomic](https://web.archive.org/web/20230815000654/https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
than being returned a [`Vec`] of constituents of length 1, requiring more awkward error
handling at the call site.

```rust
use decompound::{decompound, DecompositionError, DecompositionOptions};

// *Only* contains a compound word.
let is_valid_single_word = |w: &str| ["railroad"].contains(&w);

assert_eq!(
    decompound(
        "railroad",
        &is_valid_single_word,
        DecompositionOptions::empty(),
    ),
    Err(DecompositionError::SingleWord("railroad".to_string()))
);
```

## Motivation

...
