fn convert_to_owned(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(|s| s.to_owned()).collect()
}

fn custom_check(word: &str) -> bool {
    word.chars().all(|c| c.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use decompound::decompound;
    use rstest::rstest;

    use crate::{convert_to_owned, custom_check};

    #[rstest]
    #[case("A", true, Some(vec!["A"]))]
    #[case("AB", true, Some(vec!["A", "B"]))]
    #[case("ABC", true, Some(vec!["A", "B", "C"]))]
    //
    #[case("AA", true, Some(vec!["A", "A"]))]
    #[case("AAA", true, Some(vec!["A", "A", "A"]))]
    //
    #[case("AA", false, Some(vec!["A", "A"]))]
    #[case("AAA", false, Some(vec!["A", "A", "A"]))]
    //
    #[case("Aa", false, None)]
    #[case("AaA", false, None)]
    #[case("AaAa", false, None)]
    //
    // We titlecase the suffix, not the initial prefix
    #[case("a", true, None)]
    #[case("ab", true, None)]
    #[case("aB", true, None)]
    #[case("abc", true, None)]
    #[case("aBc", true, None)]
    #[case("abC", true, None)]
    #[case("aBC", true, None)]
    //
    #[case("Ab", true, Some(vec!["A", "B"]))]
    #[case("Abc", true, Some(vec!["A", "B", "C"]))]
    #[case("ABc", true, Some(vec!["A", "B", "C"]))]
    #[case("AbC", true, Some(vec!["A", "B", "C"]))]
    //
    #[case("A", false, Some(vec!["A"]))]
    #[case("AB", false, Some(vec!["A", "B"]))]
    #[case("ABC", false, Some(vec!["A", "B", "C"]))]
    //
    #[case("a", false, None)]
    #[case("ab", false, None)]
    #[case("aB", false, None)]
    #[case("abc", false, None)]
    #[case("aBc", false, None)]
    #[case("abC", false, None)]
    #[case("aBC", false, None)]
    //
    #[case("Ab", false, None)]
    #[case("Abc", false, None)]
    #[case("ABc", false, None)]
    #[case("AbC", false, None)]
    fn test_decompound_basic(
        #[case] word: &str,
        #[case] titlecase_suffix: bool,
        #[case] expected: Option<Vec<&str>>,
    ) {
        const WORDS: &[&str] = &["A", "B", "C"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), titlecase_suffix),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("Süßwasserschwimmbäder", true, Some(vec!["Süßwasser", "schwimm", "Bäder"]))]
    #[case("Süßwasserbäderbäder", true, Some(vec!["Süßwasser", "Bäder", "Bäder"]))]
    #[case("Mauerdübel", true, Some(vec!["Mauer", "Dübel"]))]
    #[case("Mauerdübelkübel", true, Some(vec!["Mauer", "Dübel", "Kübel"]))]
    //
    #[case("süßwasserschwimmbäder", true, None)]
    //
    #[case("Süßwasserschwimm", false, Some(vec!["Süßwasser", "schwimm"]))]
    #[case("Süßwasserbäderbäder", false, None)]
    #[case("Mauerdübel", false, None)]
    #[case("Mauerdübelkübel", false, None)]
    //
    #[case("süßwasserschwimmbäder", false, None)]
    //
    // Valid word but not contained in the dictionary
    #[case("Süßwasserfisch", false, None)]
    //
    // Single words are fine if contained in the dictionary
    #[case("Mauer", true, Some(vec!["Mauer"]))]
    #[case("Mauer", false, Some(vec!["Mauer"]))]
    // But not if they are not
    #[case("Haus", true, None)]
    #[case("Haus", false, None)]
    fn test_decompound_german(
        #[case] word: &str,
        #[case] titlecase_suffix: bool,
        #[case] expected: Option<Vec<&str>>,
    ) {
        const WORDS: &[&str] = &["Süßwasser", "schwimm", "Bäder", "Mauer", "Dübel", "Kübel"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), titlecase_suffix),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("Fußball", true, Some(vec!["Fuß", "Ball"]))]
    #[case("Fußball", false, None)] // Suffix is noun, doesn't work
    //
    #[case("Fernsehen", true, Some(vec!["Fern", "sehen"]))]
    #[case("Fernsehen", false, Some(vec!["Fern", "sehen"]))] // Suffix is verb, works
    //
    #[case("Hellsehen", true, None)] // Prefix no in dictionary
    #[case("Hellsehen", false, None)] // Prefix no in dictionary
    fn test_decompound_german_already_in_dictionary(
        #[case] word: &str,
        #[case] titlecase_suffix: bool,
        #[case] expected: Option<Vec<&str>>,
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
            decompound(word, &|w| WORDS.contains(&w), titlecase_suffix),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    // Doesn't make the most sense as English dictionaries will also contains 'football'
    // and 'cupcake'...
    #[case("football", true, Some(vec!["foot", "ball"]))]
    #[case("cupcake", true, Some(vec!["cup", "cake"]))]
    //
    #[case("football", false, Some(vec!["foot", "ball"]))]
    #[case("cupcake", false, Some(vec!["cup", "cake"]))]
    fn test_decompound_english(
        #[case] word: &str,
        #[case] titlecase_suffix: bool,
        #[case] expected: Option<Vec<&str>>,
    ) {
        const WORDS: &[&str] = &["foot", "ball", "cup", "cake"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), titlecase_suffix),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    // Still finds compound word, as prefix only ever extends to `footbal`, which is
    // invalid, so it falls back to the last valid split at `foot` and `ball`.
    #[case("football", true, Some(vec!["foot", "ball"]))]
    #[case("football", false, Some(vec!["foot", "ball"]))]
    fn test_decompound_english_word_already_in_dictionary(
        #[case] word: &str,
        #[case] titlecase_suffix: bool,
        #[case] expected: Option<Vec<&str>>,
    ) {
        const WORDS: &[&str] = &["foot", "ball", "football"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), titlecase_suffix),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("self-esteem", true, Some(vec!["self", "esteem"]))]
    #[case("self-esteem", false, Some(vec!["self", "esteem"]))]
    fn test_decompound_hyphenation(
        #[case] word: &str,
        #[case] titlecase_suffix: bool,
        #[case] expected: Option<Vec<&str>>,
    ) {
        const WORDS: &[&str] = &["self", "esteem"];

        assert_eq!(
            decompound(word, &|w| WORDS.contains(&w), titlecase_suffix),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    // Greedy prefix fetching. Suffix is uppercase as all suffix candidates are tried in
    // ascending order. Uppercase is first and matches immediately.
    #[case("football", true, Some(vec!["footbal", "L"]))]
    #[case("cupcake", true, Some(vec!["cupcak", "E"]))]
    //
    #[case("football", false, Some(vec!["footbal", "l"]))] // Note: not uppercase
    #[case("cupcake", false, Some(vec!["cupcak", "e"]))] // Note: not uppercase
    fn test_decompound_custom_check(
        #[case] word: &str,
        #[case] titlecase_suffix: bool,
        #[case] expected: Option<Vec<&str>>,
    ) {
        assert_eq!(
            decompound(word, &custom_check, titlecase_suffix),
            expected.map(convert_to_owned)
        );
    }

    #[rstest]
    #[case("🦀", true, Some(vec!["🦀"]))]
    #[case("🦀🦀", true, Some(vec!["🦀", "🦀"]))]
    //
    #[case("🦀", false, Some(vec!["🦀"]))]
    #[case("🦀🦀", false, Some(vec!["🦀", "🦀"]))]
    //
    #[case("العربية", true, Some(vec!["العربي", "ة"]))] // Arabic
    //
    #[case("中文", true, Some(vec!["中", "文"]))] // Chinese
    //
    #[case("日本語", true, Some(vec!["日本", "語"]))] // Japanese
    //
    #[case("한국어", true, Some(vec!["한국", "어"]))] // Korean
    //
    // Special characters
    #[case("\n", true, Some(vec!["\n"]))]
    #[case("\n", false, Some(vec!["\n"]))]
    //
    #[case(" ", true, Some(vec![" "]))]
    #[case(" ", false, Some(vec![" "]))]
    //
    #[case("", true, Some(vec![""]))]
    #[case("", false, Some(vec![""]))]
    fn test_decompound_unicode_edge_cases(
        #[case] word: &str,
        #[case] titlecase_suffix: bool,
        #[case] expected: Option<Vec<&str>>,
    ) {
        let anything_goes = |_: &str| true;
        assert_eq!(
            decompound(word, &anything_goes, titlecase_suffix),
            expected.map(convert_to_owned)
        );
    }
}
