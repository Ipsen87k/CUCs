use std::error::Error;

use assert_cmd::{cargo::CommandCargoExt, Command};



const MP3_1:&str = "./tests/res/j.mp3";
const MP3_2:&str = "./tests/res/g.mp3";
const WAV_1:&str = "./tests/res/m.wav";

type CResult<T> = Result<T,Box<dyn Error>>;

fn run(args:&[&str])->CResult<()>{
    let output = Command::cargo_bin("cplay")?
        .args(args)
        .output()
        .expect("fail");

    assert!(output.status.success());
    Ok(())
}

#[test]
fn play_mp3()->CResult<()>{
    run(&[MP3_1])
}

#[test]
fn play_wav()->CResult<()>{
    run(&[WAV_1])
}

#[test]
fn play_musics()->CResult<()>{
    run(&[MP3_1,MP3_2,WAV_1])
}