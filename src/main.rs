use decompound::{decompound, DecompositionOptions};
use std::{collections::HashSet, env, io::stdin};

fn main() -> Result<(), &'static str> {
    let (options, word) = parse()?;

    eprintln!("Reading list of valid (single) words from stdin...");

    let valid_words: HashSet<String> = stdin()
        .lines()
        .map(|l| l.expect("Failed to read line from stdin"))
        .map(|l| l.trim().to_owned())
        .collect();

    eprintln!("Read {} words.", valid_words.len());
    eprintln!("Constituents of '{}' are:", word);

    match decompound(word, &|w| valid_words.contains(&w.to_owned()), options) {
        Ok(words) => {
            for word in words {
                println!("{}", word);
            }
            Ok(())
        }
        Err(decompound::DecompositionError::SingleWord(_)) => {
            Err("Word is valid as a single word, but not as a compound")
        }
        Err(_) => Err("Failed to split word"),
    }
}

/// Parses command line arguments. Super ugly and hacky, as it's done manually since
/// `lib` and `bin` dependencies cannot be separated, and we don't want the `lib` part
/// to depend on `clap`, for example.
///
/// See also:
///
/// https://github.com/rust-lang/cargo/issues/1982
///
/// https://users.rust-lang.org/t/whats-the-convention-for-handling-a-hybrid-library-and-binary-crates-dependencies/84174
fn parse() -> Result<(DecompositionOptions, String), &'static str> {
    let mut args: Vec<String> = env::args().collect();
    eprintln!("Args: {:?}", args);

    args.remove(0); // Program name

    let mut options = DecompositionOptions::empty();
    let mut word = None;

    for arg in args {
        eprintln!("Arg: {}", arg);

        match arg.as_str() {
            "-t" | "--try-titlecase-suffix" => {
                eprintln!("Will try titlecasing suffix");
                options |= DecompositionOptions::TRY_TITLECASE_SUFFIX
            }
            "-s" | "--split-hyphenated" => {
                eprintln!("Will split hyphenated words");
                options |= DecompositionOptions::SPLIT_HYPHENATED
            }
            "--shatter" => {
                eprintln!("Will shatter words");
                options |= DecompositionOptions::SHATTER
            }
            a if a.starts_with('-') => {
                eprintln!("Unknown option: {}", a);
                return Err("Unknown option");
            }
            a => {
                eprintln!("Setting word to: {}", a);
                word = Some(a.to_owned());
            }
        }
    }

    let word = word.expect("No word detected");

    Ok((options, word))
}
