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
        .stdout(predicate::str::contains("ml"))
        .stdout(predicate::str::contains("Supported domains"))
        .stdout(predicate::str::contains("customfolio create"));
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
        .success()
        .stdout(predicate::str::contains("Portfolio app ready"))
        .stdout(predicate::str::contains("pnpm install"));

    assert!(out.join("package.json").is_file());
    assert!(out.join("app/layout.tsx").is_file());
    let portfolio = fs::read_to_string(out.join("content/portfolio.json")).unwrap();
    assert!(portfolio.contains("devops") || portfolio.contains("\"Jordan\""));
    assert!(portfolio.contains("j@example.com"));
    assert!(portfolio.contains("github.com/jordan"));
    let pkg = fs::read_to_string(out.join("package.json")).unwrap();
    assert!(pkg.contains("\"site\"") || pkg.contains("site"));
}

#[test]
fn create_minimal_skips_sample_experience() {
    let dir = tempdir().unwrap();
    let out = dir.path().join("min");
    cargo_bin_cmd!("customfolio")
        .args([
            "create",
            "min",
            "--output",
            out.to_str().unwrap(),
            "--domain",
            "frontend",
            "--full-name",
            "Min User",
            "--yes",
            "--minimal",
        ])
        .assert()
        .success();

    let portfolio = fs::read_to_string(out.join("content/portfolio.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&portfolio).unwrap();
    assert!(v["experience"].as_array().unwrap().is_empty());
    assert_eq!(v["projects"].as_array().unwrap().len(), 1);
    assert_eq!(v["blog"]["enabled"], false);
}

#[test]
fn create_all_domains_with_yes() {
    for domain in [
        "frontend",
        "backend",
        "fullstack",
        "ml",
        "mobile",
        "devops",
        "data",
        "security",
        "game",
        "general",
    ] {
        let dir = tempdir().unwrap();
        let out = dir.path().join(domain);
        cargo_bin_cmd!("customfolio")
            .args([
                "create",
                domain,
                "--output",
                out.to_str().unwrap(),
                "--domain",
                domain,
                "--full-name",
                "Domain Tester",
                "--yes",
            ])
            .assert()
            .success();
        let portfolio = fs::read_to_string(out.join("content/portfolio.json")).unwrap();
        assert!(
            portfolio.contains(&format!("\"{domain}\"")) || portfolio.contains(domain),
            "portfolio should mention domain {domain}"
        );
    }
}

#[test]
fn create_force_overwrites() {
    let dir = tempdir().unwrap();
    let out = dir.path().join("force-me");
    fs::create_dir_all(&out).unwrap();
    fs::write(out.join("stale.txt"), b"old").unwrap();

    cargo_bin_cmd!("customfolio")
        .args([
            "create",
            "force-me",
            "--output",
            out.to_str().unwrap(),
            "--domain",
            "general",
            "--yes",
            "--force",
        ])
        .assert()
        .success();

    assert!(!out.join("stale.txt").exists());
    assert!(out.join("package.json").is_file());
}

#[test]
fn create_non_empty_without_force_fails() {
    let dir = tempdir().unwrap();
    let out = dir.path().join("blocked");
    fs::create_dir_all(&out).unwrap();
    fs::write(out.join("stale.txt"), b"old").unwrap();

    cargo_bin_cmd!("customfolio")
        .args([
            "create",
            "blocked",
            "--output",
            out.to_str().unwrap(),
            "--yes",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not empty").or(predicate::str::contains("error")));
}

#[test]
fn create_domain_alias() {
    let dir = tempdir().unwrap();
    let out = dir.path().join("alias");
    cargo_bin_cmd!("customfolio")
        .args([
            "create",
            "alias",
            "--output",
            out.to_str().unwrap(),
            "--domain",
            "fe",
            "--full-name",
            "Alias User",
            "--yes",
        ])
        .assert()
        .success();
    let portfolio = fs::read_to_string(out.join("content/portfolio.json")).unwrap();
    assert!(portfolio.contains("frontend") || portfolio.contains("Frontend"));
}

#[test]
fn create_theme_and_primary() {
    let dir = tempdir().unwrap();
    let out = dir.path().join("themed");
    cargo_bin_cmd!("customfolio")
        .args([
            "create",
            "themed",
            "--output",
            out.to_str().unwrap(),
            "--domain",
            "security",
            "--full-name",
            "Sec",
            "--primary",
            "#abcdef",
            "--theme",
            "dark",
            "--yes",
        ])
        .assert()
        .success();
    let portfolio = fs::read_to_string(out.join("content/portfolio.json")).unwrap();
    assert!(portfolio.contains("#abcdef"));
    assert!(portfolio.contains("\"dark\"") || portfolio.contains("dark"));
}

#[test]
fn create_invalid_domain_fails() {
    cargo_bin_cmd!("customfolio")
        .args(["create", "x", "--domain", "not-a-domain", "--yes"])
        .assert()
        .failure();
}

#[test]
fn help_works() {
    cargo_bin_cmd!("customfolio")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Scaffold"));
}

#[test]
fn create_help_works() {
    cargo_bin_cmd!("customfolio")
        .args(["create", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("domain"))
        .stdout(predicate::str::contains("yes"));
}

#[test]
fn version_flag_works() {
    cargo_bin_cmd!("customfolio")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("customfolio"));
}

#[test]
fn csv_to_ndjson_streams_example_shape() {
    let dir = tempdir().unwrap();
    let csv = dir.path().join("people.csv");
    let ndjson = dir.path().join("people.ndjson");
    fs::write(
        &csv,
        "slug,domain,name,title,email\n\
         ada-lovelace,frontend,Ada Lovelace,Frontend Engineer,ada@example.com\n\
         grace-hopper,backend,Grace Hopper,Backend Engineer,grace@example.com\n",
    )
    .unwrap();

    cargo_bin_cmd!("customfolio")
        .args([
            "csv-to-ndjson",
            csv.to_str().unwrap(),
            "-o",
            ndjson.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Conversion complete"))
        .stdout(predicate::str::contains("succeeded: 2"));

    let text = fs::read_to_string(&ndjson).unwrap();
    let lines: Vec<_> = text.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("\"slug\":\"ada-lovelace\""));
    assert!(lines[0].contains("\"domain\":\"frontend\""));
}
