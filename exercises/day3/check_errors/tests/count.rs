use assert_cmd::Command;

#[test]
fn exit_no_count() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("check_errors")?;

    cmd.write_stdin("");

    cmd.assert().success();

    let out_vec = cmd.output()?.stdout;
    let out = std::str::from_utf8(&out_vec)?;

    let counts = &out.split_at(out.find("Counts:").unwrap()).1[7..].trim();
    if counts != &"" {
        panic!("bad counts value: {}", counts);
    }

    Ok(())
}


#[test]
fn exit_some_counts() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("check_errors")?;

    cmd.write_stdin("1\n2\n3\n2\n1\n\n");

    cmd.assert().success();

    let out_vec = cmd.output()?.stdout;
    let out = std::str::from_utf8(&out_vec)?;

    let counts = &out.split_at(out.find("Counts:").unwrap()).1[7..].trim();
    if counts != &"1: 2\n2: 2\n3: 1" {
        panic!("bad counts value: {}", counts);
    }

    Ok(())
}


#[test]
fn handle_negative_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("check_errors")?;

    cmd.write_stdin("1\n-2\n3\n-2\n1\n\n");

    cmd.assert().success();

    let out_vec = cmd.output()?.stdout;
    let out = std::str::from_utf8(&out_vec)?;

    let counts = &out.split_at(out.find("Counts:").unwrap()).1[7..].trim();
    if counts != &"-2: 2\n1: 2\n3: 1" {
        panic!("bad counts value: {}", counts);
    }

    Ok(())
}
