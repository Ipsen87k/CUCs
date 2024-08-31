use std::{error::Error, fs};

use assert_cmd::Command;

type R = Result<(),Box<dyn Error>>;

fn run(args:&[&str],expected_file:&str)->R{
    let mut expected = fs::read_to_string(expected_file)?;
    expected.push_str("\n");

    Command::cargo_bin("sbc")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn string_byte_j_test()->R{
    run(&["-b","日本語"], "tests/expected/japan.out")?;
    Ok(())
}

#[test]
fn strign_byte_e_test()->R{
    run(&["-b","Rust"], "tests/expected/eng.out")?;
    Ok(())
}

#[test]
fn strign_byte_many_args_test()->R{
    run(&["-b","どらくえ","dragon"], "tests/expected/manyargs.b.out")?;
    Ok(())
}

#[test]
fn string_count_test()->R{
    run(&["-c","tabletennis"], "tests/expected/count.out")?;
    Ok(())
}

#[test]
fn strign_count_many_args_test()->R{
    run(&["-c","東京都千代田区","nintendo"], "tests/expected/manyargs.c.out")?;
    Ok(())
}

#[test]
fn string_byte_count_args_test()->R{
    run(&["-bc","わたぼう"], "tests/expected/b.c.out")?;
    Ok(())
}

#[test]
fn string_byte_count_many_args_test()->R{
    run(&["-bc","司馬遷","import"], "tests/expected/manyargs.b.c.out")?;
    Ok(())
    
}