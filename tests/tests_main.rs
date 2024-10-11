#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command; // Run programs

    #[test]
    fn test_main_wrong_path() {
        let mut cmd = Command::cargo_bin("detox").unwrap();

        cmd.arg("READ ME.md");
        cmd.assert()
            .stdout(predicate::str::contains("annot find path"));
    }

    #[test]
    fn test_main_normal() {
        let mut cmd = Command::cargo_bin("detox").unwrap();

        cmd.arg("README.md");
        cmd.assert()
            .stdout(predicate::str::contains("1 file checked"));
    }

    #[test]
    fn test_main_quiet() {
        let mut cmd = Command::cargo_bin("detox").unwrap();

        cmd.arg("README.md").arg("-q");
        cmd.assert().stdout(predicate::str::is_empty());
    }

    #[test]
    fn test_main_version() {
        let mut cmd = Command::cargo_bin("detox").unwrap();

        cmd.arg("README.md").arg("-v");
        cmd.assert()
            .failure()
            .stdout(predicate::str::contains("detox"));
    }
}
