#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(clippy::multiple_crate_versions)]
//! Decompose a compound word into its constituents.

use std::collections::BTreeSet;

use log::trace;
use unicode_titlecase::StrTitleCase;

/// Decompose a compound word into its constituents.
pub fn decompound<T: AsRef<str>>(
    word: T,
    is_valid_single_word: &impl Fn(&str) -> bool,
    titlecase_suffix: bool,
) -> Option<Vec<String>> {
    let word = word.as_ref();
    let mut constituents = vec![];

    if _is_compound_word(
        word,
        is_valid_single_word,
        titlecase_suffix,
        &mut constituents,
    ) {
        debug_assert!(
            !constituents.is_empty(),
            "Compound word should have constituents"
        );

        Some(constituents)
    } else {
        None
    }
}

fn _is_compound_word(
    word: &str,
    is_valid_single_word: &impl Fn(&str) -> bool,
    titlecase_suffix: bool,
    constituents: &mut Vec<String>,
) -> bool {
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

    match greediest_split {
        Some((prefix, suffix)) => {
            constituents.push(prefix.to_owned());

            trace!(
                "Prefix '{}' found to be valid, seeing if suffix '{}' is valid.",
                prefix,
                suffix
            );

            let suffix_candidates = {
                // Dedupe so no unnecessary work is done, but keep order for determinism
                let mut set = BTreeSet::from_iter(vec![suffix.to_owned()]);

                if titlecase_suffix {
                    set.insert(suffix.to_titlecase_lower_rest());
                }

                set
            };

            for suffix in suffix_candidates {
                if is_valid_single_word(&suffix) {
                    trace!("Suffix '{}' is valid: valid single word", suffix);
                    constituents.push(suffix);
                    return true;
                }

                if _is_compound_word(
                    &suffix,
                    is_valid_single_word,
                    titlecase_suffix,
                    constituents,
                ) {
                    trace!("Suffix '{}' is valid: valid compound word", suffix);
                    // Not pushing to constituents, that's already been done in the
                    // recursion step
                    return true;
                }
            }

            trace!("Suffix '{}' is not valid", suffix);
            false
        }
        None => {
            if is_valid_single_word(word) {
                trace!("Word '{}' is valid: valid single word", word);

                debug_assert!(
                    constituents.is_empty(),
                    "Single word should be the only constituent"
                );
                constituents.push(word.to_owned());

                true
            } else {
                trace!("Word '{}' is not valid", word);
                false
            }
        }
    }
}
