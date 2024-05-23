use clap::{builder::PossibleValue, Arg,  Command, ValueEnum};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use crate::{psgen_core::Password, CResult};
use std::io::Write;

const DEFAULT_PASS_GENERATION_COUNTS: u32 = 10;

#[derive(Debug)]
pub struct Config {
    number: Option<u32>,
    pass_level: PasswordStrength,
    generate_counts: u32,
    no_value:NoValue,
}

#[derive(Debug,PartialEq, Eq,Clone,Default)]
pub enum NoValue {
    #[default]
    None,
    Symbol,
    Number,
    NumberAndSymbol,
}

impl ValueEnum for NoValue{
    fn value_variants<'a>() -> &'a [Self] {
        &[
            NoValue::None,
            NoValue::Symbol,
            NoValue::Number,
            NoValue::NumberAndSymbol,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            NoValue::None => PossibleValue::new("default").alias("none"),
            NoValue::Symbol => PossibleValue::new("s"),
            NoValue::Number => PossibleValue::new("nu").alias("un"),
            NoValue::NumberAndSymbol => PossibleValue::new("ns").alias("sn"),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum PasswordStrength {
    Weak,
    #[default]
    Medium,
    Strong,
    VeryStrong,
}

impl PasswordStrength {
    pub fn get_pass_length_from_level(&self) -> u32 {
        match self {
            PasswordStrength::Weak => 8,
            PasswordStrength::Medium => 12,
            PasswordStrength::Strong => 18,
            PasswordStrength::VeryStrong => 24,
        }
    }
}

impl ValueEnum for PasswordStrength {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            PasswordStrength::Weak,
            PasswordStrength::Medium,
            PasswordStrength::Strong,
            PasswordStrength::VeryStrong,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            PasswordStrength::Weak => PossibleValue::new("w"),
            PasswordStrength::Medium => PossibleValue::new("m"),
            PasswordStrength::Strong => PossibleValue::new("s"),
            PasswordStrength::VeryStrong => PossibleValue::new("e"),
        })
    }
}

pub fn create_args() -> Config {
    let matches = Command::new("psgen")
        .version("0.1.0")
        .author("Ipsen87k")
        .about("generate password")
        .arg(
            Arg::new("passlength")
                .value_name("PASSLENGTH")
                .help("password length")
                .short('l')
                .long("length")
                .value_parser(clap::value_parser!(u32))
        )
        .arg(
            Arg::new("passstrength")
                .value_name("PASSSTRENGTH")
                .short('s')
                .long("strength")
                .help("Password strength")
                .value_parser(clap::value_parser!(PasswordStrength))
        )
        .arg(
            Arg::new("generationcounts")
                .value_name("GENERATIONCOUNTS")
                .help("generation password counts")
                .short('c')
                .long("count")
                .value_parser(clap::value_parser!(u32))
        )
        .arg(
            Arg::new("novalue")
                .value_name("NOVALUE")
                .help("except value")
                .short('n')
                .long("novalue")
                .value_parser(clap::value_parser!(NoValue))
        )
        .get_matches();

    Config {
        number: matches.get_one("passlength").cloned(),
        pass_level: matches.get_one("passstrength").cloned().unwrap_or_default(),
        generate_counts: matches
            .get_one("generationcounts")
            .cloned()
            .unwrap_or(DEFAULT_PASS_GENERATION_COUNTS),
        no_value:matches
            .get_one("novalue")
            .cloned()
            .unwrap_or_default(),
    }
}

pub fn run(config: Config) -> CResult<()> {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);

    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Green));
    stdout.set_color(&color_spec)?;

    let psgen = Password::new(config.number, config.pass_level,config.no_value);

    for i in 0..config.generate_counts {
        if i % 5 == 0 && i != 0{
            write!(&mut stdout,"\n\t{}",psgen.generate())?;
        }else{
            write!(&mut stdout,"\t{}",psgen.generate())?;
        }
    }

    stdout.reset()?;

    Ok(())
}
