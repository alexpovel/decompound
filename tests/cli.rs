#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use rstest::rstest;

    #[rstest]
    fn test_cli() {
        // Should rebuild the binary to `target/debug/<name>`. This works if running as an
        // integration test (insides `tests/`), but not if running as a unit test (inside
        // `src/main.rs` etc.).
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

        const WORDS: &[&str] = &["Affen", "Gruppen", "Überfall"];

        cmd.args([
            "--try-titlecase-suffix",
            "--split-hyphenated",
            "Affengruppen-Überfall",
        ])
        .write_stdin(WORDS.join("\n"));

        let raw_output = cmd.output().unwrap().stdout;
        let output = String::from_utf8(raw_output).unwrap();

        assert_eq!(output.trim(), "Affen\nGruppen\nÜberfall");
    }
}
