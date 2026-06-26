use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

fn setup_env() -> tempfile::TempDir {
    let dir = tempdir().unwrap();
    // Sentinel CLI looks for .sentinel/knowledge.db in the current directory.
    let sentinel_dir = dir.path().join(".sentinel");
    fs::create_dir_all(&sentinel_dir).unwrap();
    dir
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("sentinel-cli").unwrap();
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sentinel Arc"));
}

#[test]
fn test_cli_init() {
    let dir = setup_env();
    let mut cmd = Command::cargo_bin("sentinel-cli").unwrap();
    cmd.current_dir(dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Workspace successfully initialized!"));

    assert!(dir.path().join(".sentinel").join("knowledge.db").exists());
}

#[test]
fn test_cli_doctor_uninitialized() {
    let dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("sentinel-cli").unwrap();
    // No .sentinel directory here
    cmd.current_dir(dir.path())
        .arg("doctor")
        .assert()
        .success() // Fails because it's not initialized
        .stdout(predicate::str::contains("Workspace not initialized"));
}

#[test]
fn test_cli_doctor_initialized() {
    let dir = setup_env();

    // Init first
    let mut cmd_init = Command::cargo_bin("sentinel-cli").unwrap();
    cmd_init.current_dir(dir.path()).arg("init").assert().success();

    // Doctor
    let mut cmd_doctor = Command::cargo_bin("sentinel-cli").unwrap();
    cmd_doctor
        .current_dir(dir.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Database found"));
}
