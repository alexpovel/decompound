#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_results)]
#![warn(unused_lifetimes)]
#![warn(unused)]
#![warn(missing_docs)]
#![allow(clippy::multiple_crate_versions)]
#![doc = include_str!("../README.md")]

use std::{collections::BTreeSet, error::Error, fmt::Display};

use bitflags::bitflags;
use log::trace;
use unicode_titlecase::StrTitleCase;

/// Error cases for the [`Result`] of [`decompound`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DecompositionError {
    /// Result was *not* a compound word, but a valid *single* word.
    /// Whether this is a hard error is subjective: in any case, *decomposition failed*.
    SingleWord(String),
    /// Nothing valid was found (neither a compound word nor a single, non-compound
    /// word).
    NothingValid,
}

impl Display for DecompositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecompositionError::SingleWord(word) => {
                write!(f, "Not a compound, but valid single word: {word}")
            }
            DecompositionError::NothingValid => write!(f, "No valid decomposition found"),
        }
    }
}

impl Error for DecompositionError {}

bitflags! {
    /// Options for [`decompound`], configuring its search. Available options are found
    /// below, as `pub const`s.
    ///
    /// Use [`DecompositionOptions::empty()`] to set *no* options. See the [`bitflags`
    /// docs](https://docs.rs/bitflags/latest/bitflags/#working-with-flags-values) for
    /// more.
    #[derive(Clone)]
    pub struct DecompositionOptions: u32 {
        /// *In addition* to the original suffix being tried, try its titlecased version
        /// as well. Does nothing if suffix is already titlecased.
        const TRY_TITLECASE_SUFFIX = 1;
        /// Treat hyphenated words as compound words. Its constituents will be returned
        /// as a collection of *all* constituents of the hyphenated word, *without* any
        /// hyphens.
        const SPLIT_HYPHENATED = 1 << 1;
    }
}

impl AsRef<DecompositionOptions> for DecompositionOptions {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// [`Result`] of a [`decompound`] operation.
pub type DecompositionResult = Result<Vec<String>, DecompositionError>;

/// Docs...
///
/// ## Errors
///
/// ...
pub fn decompound(
    word: impl AsRef<str>,
    is_valid_single_word: &impl Fn(&str) -> bool,
    options: impl AsRef<DecompositionOptions>,
) -> DecompositionResult {
    let mut constituents = vec![];
    let word = word.as_ref();
    let options = options.as_ref();

    if options.contains(DecompositionOptions::SPLIT_HYPHENATED) {
        // Avoid reentry on upcoming recursive call
        let options = options.clone() - DecompositionOptions::SPLIT_HYPHENATED;

        for subword in word.split('-') {
            match decompound(subword, is_valid_single_word, &options) {
                Ok(words) => constituents.extend(words),
                // Actually allowed in this mode: words like 'string-concatenation' are
                // valid, where each part is only a 'single' word, not again a compound
                // word in itself.
                Err(DecompositionError::SingleWord(word)) => constituents.push(word),
                _ => break,
            };
        }

        return match &constituents[..] {
            [] => Err(DecompositionError::NothingValid),
            [w] => Err(DecompositionError::SingleWord(w.into())),
            _ => Ok(constituents),
        };
    }

    if is_valid_compound_word(word, is_valid_single_word, options, &mut constituents) {
        debug_assert!(
            !constituents.is_empty(),
            "Compound word must have constituents"
        );

        Ok(constituents)
    } else {
        trace!("Word is not a valid compound word");

        if is_valid_single_word(word) {
            Err(DecompositionError::SingleWord(word.to_owned()))
        } else {
            Err(DecompositionError::NothingValid)
        }
    }
}

fn is_valid_compound_word(
    word: impl AsRef<str>,
    is_valid_single_word: &impl Fn(&str) -> bool,
    options: &DecompositionOptions,
    constituents: &mut Vec<String>,
) -> bool {
    let word = word.as_ref();
    trace!("Checking if word is valid compound word: '{}'", word);

    // Greedily fetch the longest possible prefix. Otherwise, we short-circuit and
    // might end up looking for (for example) "He" of "Heizölrechnung" and its
    // suffix "izölrechnung" (not a word), whereas we could have found "Heizöl" and
    // "Rechnung" instead.
    let greediest_split = {
        let mut split = None;

        for (i, _) in word.char_indices().skip(1) {
            let (prefix, suffix) = word.split_at(i);

            debug_assert!(!prefix.is_empty(), "Prefix should never be empty");
            debug_assert!(!suffix.is_empty(), "Suffix should never be empty");

            if is_valid_single_word(prefix) {
                split = Some((prefix, suffix));
            }
        }

        split
    };

    if let Some((prefix, suffix)) = greediest_split {
        constituents.push(prefix.to_owned());

        trace!(
            "Prefix '{}' found to be valid, seeing if suffix '{}' is valid.",
            prefix,
            suffix
        );

        let suffix_candidates = {
            // Dedupe so no unnecessary work is done, but keep order for determinism
            let mut set = BTreeSet::from_iter(vec![suffix.to_owned()]);

            if options.contains(DecompositionOptions::TRY_TITLECASE_SUFFIX) {
                let _ = set.insert(suffix.to_titlecase_lower_rest());
            }

            set
        };

        debug_assert!(
            !suffix_candidates.is_empty(),
            "Suffix candidates should never be empty"
        );

        for suffix in suffix_candidates {
            if is_valid_single_word(&suffix) {
                trace!("Suffix '{}' is valid: valid single word", suffix);
                constituents.push(suffix);
                return true;
            }

            if is_valid_compound_word(&suffix, is_valid_single_word, options, constituents) {
                trace!("Suffix '{}' is valid: valid compound word", suffix);
                // Not pushing to constituents, that's already been done in the
                // recursion step
                return true;
            }
        }
    }

    false
}
