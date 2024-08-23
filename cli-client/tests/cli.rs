use assert_cmd::prelude::*;
use assert_fs::fixture::FileWriteStr;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn it_should_recognize_missing_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("strikes")?;

    cmd.arg("guenther");
    cmd.assert().failure().stderr(predicate::str::contains(
        "unrecognized subcommand 'guenther'",
    ));

    Ok(())
}

#[test]
fn it_should_add_strike() -> Result<(), Box<dyn std::error::Error>> {
    let db_file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
    let config_file = assert_fs::NamedTempFile::new("./tests/fixtures/configuration.yaml")?;
    config_file.write_str(
        format!(
            "{{\"local\": {{\"db_path\": \"{}\"}}}}",
            db_file.path().to_str().unwrap()
        )
        .as_str(),
    )?;

    let mut cmd = Command::cargo_bin("strikes")?;

    cmd.arg("--config-path")
        .arg(config_file.path())
        .arg("strike")
        .arg("guenther");
    cmd.assert().success();

    Ok(())
}

#[test]
fn it_should_list_strikes_in_descending_order() -> Result<(), Box<dyn std::error::Error>> {
    let db_file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
    let config_file = assert_fs::NamedTempFile::new("./tests/fixtures/configuration.yaml")?;
    config_file.write_str(
        format!(
            "{{\"local\": {{\"db_path\": \"{}\"}}}}",
            db_file.path().to_str().unwrap()
        )
        .as_str(),
    )?;

    db_file.write_str("{\"guenther\": 1, \"heinz\": 2}")?;

    let mut cmd = Command::cargo_bin("strikes")?;

    cmd.arg("--config-path").arg(config_file.path()).arg("ls");

    let expected_output = "Tarnished  | Strikes    |\n\
                           heinz      | 2          |\n\
                           guenther   | 1          |\n";

    cmd.assert().success().stdout(expected_output);

    Ok(())
}

#[test]
fn it_should_clear_all_strikes() -> Result<(), Box<dyn std::error::Error>> {
    let db_file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
    let config_file = assert_fs::NamedTempFile::new("./tests/fixtures/configuration.yaml")?;
    config_file.write_str(
        format!(
            "{{\"local\": {{\"db_path\": \"{}\"}}}}",
            db_file.path().to_str().unwrap()
        )
        .as_str(),
    )?;

    let mut cmd = Command::cargo_bin("strikes")?;
    cmd.arg("--config-path")
        .arg(config_file.path())
        .arg("strike")
        .arg("guenther");
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("strikes")?;
    cmd.arg("--config-path").arg(config_file.path()).arg("ls");
    cmd.assert()
        .success()
        .stdout("Tarnished  | Strikes    |\nguenther   | 1          |\n");

    let mut cmd = Command::cargo_bin("strikes")?;
    cmd.arg("--config-path")
        .arg(config_file.path())
        .arg("clear");
    cmd.assert()
        .success()
        .stdout("All strikes have been cleared!\n");

    let mut cmd = Command::cargo_bin("strikes")?;
    cmd.arg("--config-path").arg(config_file.path()).arg("ls");
    cmd.assert()
        .success()
        .stdout("No one has been tarnished yet!\n");

    Ok(())
}
