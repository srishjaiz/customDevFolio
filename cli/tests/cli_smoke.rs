use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn domains_lists_presets() {
    cargo_bin_cmd!("customfolio")
        .arg("domains")
        .assert()
        .success()
        .stdout(predicate::str::contains("frontend"))
        .stdout(predicate::str::contains("ml"));
}

#[test]
fn create_yes_scaffolds_app() {
    let dir = tempdir().unwrap();
    let out = dir.path().join("site");
    cargo_bin_cmd!("customfolio")
        .args([
            "create",
            "site",
            "--output",
            out.to_str().unwrap(),
            "--domain",
            "devops",
            "--full-name",
            "Jordan",
            "--email",
            "j@example.com",
            "--github",
            "jordan",
            "--yes",
        ])
        .assert()
        .success();

    assert!(out.join("package.json").is_file());
    assert!(out.join("app/layout.tsx").is_file());
    let portfolio = fs::read_to_string(out.join("content/portfolio.json")).unwrap();
    assert!(portfolio.contains("devops") || portfolio.contains("\"Jordan\""));
    let pkg = fs::read_to_string(out.join("package.json")).unwrap();
    assert!(pkg.contains("\"site\"") || pkg.contains("site"));
}

#[test]
fn help_works() {
    cargo_bin_cmd!("customfolio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Scaffold"));
}
