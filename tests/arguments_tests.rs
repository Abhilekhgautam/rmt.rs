use assert_cmd::prelude::*;
use core::time;
use predicates::prelude::*;
use rmt_lib::argument_errors::RmtArgumentErrors;
use std::{fs, process::Command};

fn unique_name() -> String {
    std::thread::sleep(time::Duration::from_nanos(1));
    sha256::digest(format!(
        "{}",
        chrono::offset::Local::now().timestamp_nanos()
    ))
}

#[test]
fn test_no_args() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let expected_output = format!("{}\n", RmtArgumentErrors::InvalidNumberOfArguments(0));

    cmd.assert()
        .failure()
        .stdout(predicate::str::diff(expected_output));
}

#[test]
fn test_only_flags() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("-f");

    let expected_output = format!("{}\n", RmtArgumentErrors::InvalidNumberOfArguments(0));
    cmd.assert()
        .failure()
        .stdout(predicate::str::diff(expected_output));
}

#[test]
fn test_not_existing_file_without_force() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let not_existing_file_name = unique_name();

    cmd.arg(&not_existing_file_name);

    let expected_output = format!(
        "{}\n",
        RmtArgumentErrors::InvalidPathWithoutForceFlags {
            element_name: not_existing_file_name
        }
    );
    cmd.assert()
        .failure()
        .stdout(predicate::str::diff(expected_output));
}

#[test]
fn test_not_existing_file_with_force() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let not_existing_file_name = unique_name();

    cmd.arg(&not_existing_file_name).arg("-f");

    cmd.assert()
        .success()
        .stdout(predicate::str::diff(""));
}

#[test]
fn test_delele_empty_folder_without_flags() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let not_existing_folder_name = unique_name();

    fs::create_dir_all(&not_existing_folder_name).unwrap();

    cmd.arg(&not_existing_folder_name);

    let expected_output = format!(
        "{}\n",
        RmtArgumentErrors::InvalidEmptyFolderFlags { 
            folder_name: format!("{}/{}", std::env::current_dir().unwrap().display(), not_existing_folder_name)
        }
    );
    cmd.assert()
        .failure()
        .stdout(predicate::str::diff(expected_output));

    fs::remove_dir_all(&not_existing_folder_name).unwrap();
}
