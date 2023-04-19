use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.assert();
    assert.success();

    Ok(())
}

#[test]
fn only_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").assert();
    assert.success();

    Ok(())
}

#[test]
fn only_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-s").arg("test_source").assert();
    assert.success();

    Ok(())
}

#[test]
fn one_name_one_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").arg("-s").arg("test_source").assert();
    assert.success();

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

#[test]
fn too_many_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").arg("-s").arg("test_source").arg("extra_arg").assert();
    assert.failure().stderr(predicate::str::contains("unexpected argument"));

    Ok(())
}

#[test]
fn extra_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").arg("-s").arg("test_source").arg("-n").arg("extra_name").assert();
    assert.failure().stderr(predicate::str::contains("cannot be used multiple times"));

    Ok(())
}

#[test]
fn extra_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").arg("-s").arg("test_source").arg("-s").arg("extra_source").assert();
    assert.failure().stderr(predicate::str::contains("cannot be used multiple times"));

    Ok(())
}

#[test]
fn empty_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("").arg("-s").arg("test_source").assert();
    assert.failure().stderr(predicate::str::contains("needs value"));

    Ok(())
}

#[test]
fn empty_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").arg("-s").arg("").assert();
    assert.failure().stderr(predicate::str::contains("needs value"));

    Ok(())
}

#[test]
fn space_for_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg(" ").arg("-s").arg("test_source").assert();
    assert.failure().stderr(predicate::str::contains("needs value"));

    Ok(())
}

#[test]
fn space_for_source() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recipe_box")?;

    let assert = cmd.arg("-n").arg("test_name").arg("-s").arg(" ").assert();
    assert.failure().stderr(predicate::str::contains("needs value"));

    Ok(())
}