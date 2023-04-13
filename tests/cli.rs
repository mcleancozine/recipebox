use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn one_name_one_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").arg("-s").arg("test_source").assert();
    assert.success();

    Ok(())
}

#[test]
fn not_enough_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").assert();
    assert.failure().stderr(predicate::str::contains("the following required arguments were not provided"));

    Ok(())
}

#[test]
fn too_many_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").arg("-s").arg("test_source").arg("extra_arg").assert();
    assert.failure().stderr(predicate::str::contains("unexpected argument"));

    Ok(())
}

#[test]
fn multi_word_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test name").arg("-s").arg("test_source").assert();
    assert.success();

    Ok(())
}

#[test]
fn multi_word_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").arg("-s").arg("test source").assert();
    assert.success();

    Ok(())
}