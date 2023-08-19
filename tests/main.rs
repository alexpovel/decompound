fn convert_to_owned(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(|s| s.to_owned()).collect()
}

fn custom_check(word: &str) -> bool {
    word.chars().all(|c| c.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use decompound::{
        decompound, DecompositionError, DecompositionError::*, DecompositionOptions as Opt,
    };
    use rstest::rstest;

    use crate::{convert_to_owned, custom_check};

    type DecompositionTestResult<'a> = Result<Vec<&'a str>, DecompositionError>;

    #[rstest]
    #[case("A", Opt::empty(), Err(SingleWord("A".into())))]
    #[case("A", Opt::TRY_TITLECASE_SUFFIX, Err(SingleWord("A".into())))]
    #[case("A", Opt::SPLIT_HYPHENATED, Err(SingleWord("A".into())))]
    #[case("A", Opt::all(), Err(SingleWord("A".into())))]
    //
    #[case("AA", Opt::empty(), Ok(vec!["A", "A"]))]
    #[case("AA", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "A"]))]
    #[case("AA", Opt::SPLIT_HYPHENATED, Ok(vec!["A", "A"]))]
    #[case("AA", Opt::all(), Ok(vec!["A", "A"]))]
    //
    #[case("AAA", Opt::empty(), Ok(vec!["A", "A", "A"]))]
    #[case("AAA", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "A", "A"]))]
    #[case("AAA", Opt::SPLIT_HYPHENATED, Ok(vec!["A", "A", "A"]))]
    #[case("AAA", Opt::all(), Ok(vec!["A", "A", "A"]))]
    //
    #[case("AB", Opt::empty(), Ok(vec!["A", "B"]))]
    #[case("AB", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "B"]))]
    #[case("AB", Opt::SPLIT_HYPHENATED, Ok(vec!["A", "B"]))]
    #[case("AB", Opt::all(), Ok(vec!["A", "B"]))]
    //
    #[case("ABC", Opt::empty(), Ok(vec!["A", "B", "C"]))]
    #[case("ABC", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "B", "C"]))]
    #[case("ABC", Opt::SPLIT_HYPHENATED, Ok(vec!["A", "B", "C"]))]
    #[case("ABC", Opt::all(), Ok(vec!["A", "B", "C"]))]
    //
    #[case("Aa", Opt::empty(), Err(NothingValid))]
    #[case("Aa", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "A"]))]
    #[case("Aa", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Aa", Opt::all(), Ok(vec!["A", "A"]))]
    //
    #[case("AaA", Opt::empty(), Err(NothingValid))]
    #[case("AaA", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "A", "A"]))]
    #[case("AaA", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("AaA", Opt::all(), Ok(vec!["A", "A", "A"]))]
    //
    #[case("AaAa", Opt::empty(), Err(NothingValid))]
    #[case("AaAa", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "A", "A", "A"]))]
    #[case("AaAa", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("AaAa", Opt::all(), Ok(vec!["A", "A", "A", "A"]))]
    //
    // We titlecase the suffix, not the initial prefix
    #[case("a", Opt::empty(), Err(NothingValid))]
    #[case("a", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("a", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("a", Opt::all(), Err(NothingValid))]
    //
    #[case("ab", Opt::empty(), Err(NothingValid))]
    #[case("ab", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("ab", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("ab", Opt::all(), Err(NothingValid))]
    //
    #[case("aB", Opt::empty(), Err(NothingValid))]
    #[case("aB", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("aB", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("aB", Opt::all(), Err(NothingValid))]
    //
    #[case("Ab", Opt::empty(), Err(NothingValid))]
    #[case("Ab", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "B"]))]
    #[case("Ab", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Ab", Opt::all(), Ok(vec!["A", "B"]))]
    //
    #[case("Abc", Opt::empty(), Err(NothingValid))]
    #[case("Abc", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "B", "C"]))]
    #[case("Abc", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Abc", Opt::all(), Ok(vec!["A", "B", "C"]))]
    //
    #[case("ABc", Opt::empty(), Err(NothingValid))]
    #[case("ABc", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "B", "C"]))]
    #[case("ABc", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("ABc", Opt::all(), Ok(vec!["A", "B", "C"]))]
    //
    #[case("AbC", Opt::empty(), Err(NothingValid))]
    #[case("AbC", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["A", "B", "C"]))]
    #[case("AbC", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("AbC", Opt::all(), Ok(vec!["A", "B", "C"]))]
    fn test_decompound_basic(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &["A", "B", "C"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("Süßwasserschwimmbäder", Opt::empty(), Err(NothingValid))]
    #[case("Süßwasserschwimmbäder", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["Süßwasser", "schwimm", "Bäder"]))]
    #[case("Süßwasserschwimmbäder", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Süßwasserschwimmbäder", Opt::all(), Ok(vec!["Süßwasser", "schwimm", "Bäder"]))]
    //
    #[case("Süßwasserbäderbäder", Opt::empty(), Err(NothingValid))]
    #[case("Süßwasserbäderbäder", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["Süßwasser", "Bäder", "Bäder"]))]
    #[case("Süßwasserbäderbäder", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Süßwasserbäderbäder", Opt::all(), Ok(vec!["Süßwasser", "Bäder", "Bäder"]))]
    //
    #[case("süßwasserschwimmbäder", Opt::empty(), Err(NothingValid))]
    #[case("süßwasserschwimmbäder", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("süßwasserschwimmbäder", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("süßwasserschwimmbäder", Opt::all(), Err(NothingValid))]
    //
    // Valid word but not contained in the dictionary
    #[case("Süßwasserfisch", Opt::empty(), Err(NothingValid))]
    #[case("Süßwasserfisch", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("Süßwasserfisch", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Süßwasserfisch", Opt::all(), Err(NothingValid))]
    //
    #[case("Bäder", Opt::empty(), Err(SingleWord("Bäder".into())))]
    #[case("Bäder", Opt::TRY_TITLECASE_SUFFIX, Err(SingleWord("Bäder".into())))]
    #[case("Bäder", Opt::SPLIT_HYPHENATED, Err(SingleWord("Bäder".into())))]
    #[case("Bäder", Opt::all(), Err(SingleWord("Bäder".into())))]
    //
    #[case("bäder", Opt::empty(), Err(NothingValid))]
    #[case("bäder", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("bäder", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("bäder", Opt::all(), Err(NothingValid))]
    fn test_decompound_german(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &["Süßwasser", "schwimm", "Bäder"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("Fußball", Opt::empty(), Err(SingleWord("Fußball".into())))]
    #[case("Fußball", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["Fuß", "Ball"]))]
    #[case("Fußball", Opt::SPLIT_HYPHENATED, Err(SingleWord("Fußball".into())))]
    #[case("Fußball", Opt::all(), Ok(vec!["Fuß", "Ball"]))]
    //
    // Suffix is verb, works without titlecasing
    #[case("Fernsehen", Opt::empty(), Ok(vec!["Fern", "sehen"]))]
    #[case("Fernsehen", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["Fern", "sehen"]))]
    #[case("Fernsehen", Opt::SPLIT_HYPHENATED, Ok(vec!["Fern", "sehen"]))]
    #[case("Fernsehen", Opt::all(), Ok(vec!["Fern", "sehen"]))]
    //
    // Prefix not in dictionary
    #[case("Hellsehen", Opt::empty(), Err(NothingValid))]
    #[case("Hellsehen", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("Hellsehen", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Hellsehen", Opt::all(), Err(NothingValid))]
    fn test_decompound_german_already_in_dictionary(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &[
            "Fuß",
            "Ball",
            "Fußball",
            //
            "Fern",
            "sehen",
            "Fernsehen",
            //
            "hell",
        ];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("Fußball", Opt::empty(), Err(NothingValid))]
    #[case("Fußball", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["Fuß", "Ball"]))]
    #[case("Fußball", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Fußball", Opt::all(), Ok(vec!["Fuß", "Ball"]))]
    //
    // Suffix is verb, works without titlecasing
    #[case("Fernsehen", Opt::empty(), Ok(vec!["Fern", "sehen"]))]
    #[case("Fernsehen", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["Fern", "sehen"]))]
    #[case("Fernsehen", Opt::SPLIT_HYPHENATED, Ok(vec!["Fern", "sehen"]))]
    #[case("Fernsehen", Opt::all(), Ok(vec!["Fern", "sehen"]))]
    //
    // Prefix not in dictionary
    #[case("Hellsehen", Opt::empty(), Err(NothingValid))]
    #[case("Hellsehen", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("Hellsehen", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Hellsehen", Opt::all(), Err(NothingValid))]
    fn test_decompound_german_not_already_in_dictionary(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &["Fuß", "Ball", "Fern", "sehen", "hell"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    // Doesn't make the most sense as English dictionaries will also contains 'football'
    // and 'cupcake'...
    #[case("football", Opt::empty(), Ok(vec!["foot", "ball"]))]
    // It only TRIES, *additionally*, it doesn't *break* the non-titlecase versions:
    #[case("football", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["foot", "ball"]))]
    #[case("football", Opt::SPLIT_HYPHENATED, Ok(vec!["foot", "ball"]))]
    #[case("football", Opt::all(), Ok(vec!["foot", "ball"]))]
    //
    #[case("cupcake", Opt::empty(), Ok(vec!["cup", "cake"]))]
    #[case("cupcake", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["cup", "cake"]))]
    #[case("cupcake", Opt::SPLIT_HYPHENATED, Ok(vec!["cup", "cake"]))]
    #[case("cupcake", Opt::all(), Ok(vec!["cup", "cake"]))]
    fn test_decompound_english(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &["foot", "ball", "cup", "cake"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    // Still finds compound word, as prefix only ever extends to `footbal`, which is
    // invalid, so it falls back to the last valid split at `foot` and `ball`.
    #[case("football", Opt::empty(), Ok(vec!["foot", "ball"]))]
    #[case("football", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["foot", "ball"]))]
    #[case("football", Opt::SPLIT_HYPHENATED, Ok(vec!["foot", "ball"]))]
    #[case("football", Opt::all(), Ok(vec!["foot", "ball"]))]
    fn test_decompound_english_word_already_in_dictionary(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &["foot", "ball", "football"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("self-esteem", Opt::empty(), Err(NothingValid))]
    #[case("self-esteem", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("self-esteem", Opt::SPLIT_HYPHENATED, Ok(vec!["self", "esteem"]))]
    #[case("self-esteem", Opt::all(), Ok(vec!["self", "esteem"]))]
    fn test_decompound_hyphenated_word_without_word_in_list(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &["self", "esteem"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("self-esteem", Opt::empty(), Err(SingleWord("self-esteem".into())))]
    #[case("self-esteem", Opt::TRY_TITLECASE_SUFFIX, Err(SingleWord("self-esteem".into())))]
    #[case("self-esteem", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("self-esteem", Opt::all(), Err(NothingValid))]
    fn test_decompound_hyphenated_word_with_word_in_list_only(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &["self-esteem"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("self-esteem", Opt::empty(), Err(SingleWord("self-esteem".into())))]
    #[case("self-esteem", Opt::TRY_TITLECASE_SUFFIX, Err(SingleWord("self-esteem".into())))]
    #[case("self-esteem", Opt::SPLIT_HYPHENATED, Ok(vec!["self", "esteem"]))]
    #[case("self-esteem", Opt::all(), Ok(vec!["self", "esteem"]))]
    fn test_decompound_hyphenated_word_with_word_in_list_among_others(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &["self", "esteem", "self-esteem"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("Küchenfühler-Tiger", Opt::empty(), Err(NothingValid))]
    #[case("Küchenfühler-Tiger", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("Küchenfühler-Tiger", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Küchenfühler-Tiger", Opt::all(), Ok(vec!["Küchen", "Fühler", "Tiger"]))]
    //
    #[case("Türangel-Gerätprüfer", Opt::empty(), Err(NothingValid))]
    #[case("Türangel-Gerätprüfer", Opt::TRY_TITLECASE_SUFFIX, Err(NothingValid))]
    #[case("Türangel-Gerätprüfer", Opt::SPLIT_HYPHENATED, Err(NothingValid))]
    #[case("Türangel-Gerätprüfer", Opt::all(), Ok(vec!["Tür", "Angel", "Gerät", "Prüfer"]))]
    //
    #[case(
        "Schwingschleifer-Überlast-Schutzhören",
        Opt::empty(),
        Err(NothingValid)
    )]
    #[case(
        "Schwingschleifer-Überlast-Schutzhören",
        Opt::TRY_TITLECASE_SUFFIX,
        Err(NothingValid)
    )]
    #[case(
        "Schwingschleifer-Überlast-Schutzhören",
        Opt::SPLIT_HYPHENATED,
        Err(NothingValid)
    )]
    #[case("Schwingschleifer-Überlast-Schutzhören", Opt::all(), Ok(vec!["Schwing", "Schleifer", "Überlast", "Schutz", "hören"]))]
    fn test_decompound_complex_german_hyphenated_words(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        const WORDS: &[&str] = &[
            "Küchen",
            "Fühler",
            "Tiger",
            //
            "Tür",
            "Angel",
            "Gerät",
            "Prüfer",
            //
            "Schwing",
            "Schleifer",
            "Überlast",
            "Schutz",
            "hören",
        ];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    // Greedy prefix fetching. When titlecasing, all suffix candidates are tried in
    // ascending order. Uppercase is first and matches immediately.
    #[case("football", Opt::empty(), Ok(vec!["footbal", "l"]))]
    #[case("football", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["footbal", "L"]))]
    #[case("football", Opt::SPLIT_HYPHENATED, Ok(vec!["footbal", "l"]))]
    #[case("football", Opt::all(), Ok(vec!["footbal", "L"]))]
    fn test_decompound_custom_check(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        assert_eq!(
            decompound(word, &custom_check, options),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("🦀", Opt::empty(), Err(SingleWord("🦀".into())))]
    #[case("🦀", Opt::TRY_TITLECASE_SUFFIX, Err(SingleWord("🦀".into())))]
    #[case("🦀", Opt::SPLIT_HYPHENATED, Err(SingleWord("🦀".into())))]
    #[case("🦀", Opt::all(), Err(SingleWord("🦀".into())))]
    //
    #[case("🦀🦀", Opt::empty(), Ok(vec!["🦀", "🦀"]))]
    #[case("🦀🦀", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["🦀", "🦀"]))]
    #[case("🦀🦀", Opt::SPLIT_HYPHENATED, Ok(vec!["🦀", "🦀"]))]
    #[case("🦀🦀", Opt::all(), Ok(vec!["🦀", "🦀"]))]
    //
    // Arabic
    #[case("العربية", Opt::empty(), Ok(vec!["العربي", "ة"]))]
    #[case("العربية", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["العربي", "ة"]))]
    #[case("العربية", Opt::SPLIT_HYPHENATED, Ok(vec!["العربي", "ة"]))]
    #[case("العربية", Opt::all(), Ok(vec!["العربي", "ة"]))]
    //
    // Chinese
    #[case("中文", Opt::empty(), Ok(vec!["中", "文"]))]
    #[case("中文", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["中", "文"]))]
    #[case("中文", Opt::SPLIT_HYPHENATED, Ok(vec!["中", "文"]))]
    #[case("中文", Opt::all(), Ok(vec!["中", "文"]))]
    //
    // Japanese
    #[case("日本語", Opt::empty(), Ok(vec!["日本", "語"]))]
    #[case("日本語", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["日本", "語"]))]
    #[case("日本語", Opt::SPLIT_HYPHENATED, Ok(vec!["日本", "語"]))]
    #[case("日本語", Opt::all(), Ok(vec!["日本", "語"]))]
    //
    // Korean
    #[case("한국어", Opt::empty(), Ok(vec!["한국", "어"]))]
    #[case("한국어", Opt::TRY_TITLECASE_SUFFIX, Ok(vec!["한국", "어"]))]
    #[case("한국어", Opt::SPLIT_HYPHENATED, Ok(vec!["한국", "어"]))]
    #[case("한국어", Opt::all(), Ok(vec!["한국", "어"]))]
    //
    // Special characters
    #[case("\n", Opt::empty(), Err(SingleWord("\n".into())))]
    #[case("\n", Opt::TRY_TITLECASE_SUFFIX, Err(SingleWord("\n".into())))]
    #[case("\n", Opt::SPLIT_HYPHENATED, Err(SingleWord("\n".into())))]
    #[case("\n", Opt::all(), Err(SingleWord("\n".into())))]
    //
    #[case(" ", Opt::empty(), Err(SingleWord(" ".into())))]
    #[case(" ", Opt::TRY_TITLECASE_SUFFIX, Err(SingleWord(" ".into())))]
    #[case(" ", Opt::SPLIT_HYPHENATED, Err(SingleWord(" ".into())))]
    #[case(" ", Opt::all(), Err(SingleWord(" ".into())))]
    //
    #[case("", Opt::empty(), Err(SingleWord("".into())))]
    #[case("", Opt::TRY_TITLECASE_SUFFIX, Err(SingleWord("".into())))]
    #[case("", Opt::SPLIT_HYPHENATED, Err(SingleWord("".into())))]
    #[case("", Opt::all(), Err(SingleWord("".into())))]
    fn test_decompound_unicode_edge_cases(
        #[case] word: &str,
        #[case] options: Opt,
        #[case] expected: DecompositionTestResult,
    ) {
        let anything_goes = |_: &str| true;
        assert_eq!(
            decompound(word, &anything_goes, options),
            expected.map(convert_to_owned)
        );
    }
}
