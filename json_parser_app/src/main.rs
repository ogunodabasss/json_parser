use std::{fs, path::PathBuf};

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    parser_json: ParserJson,

    /// Sets a custom config file
    #[arg(value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ParserJson {
    Strings,
    Colors,
}

use json_parser_lib::{parser, utils::json::Json};

fn main() {
    let cli: Cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        let path_str: String = config_path.display().to_string();

        eprintln!("CLI input Path: {}", path_str);
        let binding: Result<String, std::io::Error> = fs::read_to_string(path_str);
        match binding {
            Ok(json) => match cli.parser_json {
                ParserJson::Strings => {
                    eprintln!("CLI input Strings json: {}",json);
                    let vec: Vec<parser::strings::Strings> = parser::strings::Strings::parse(&json);
                    let valid: bool = parser::strings::Strings::validate_data(vec);
                    match valid {
                        true => {
                            eprintln!("Struct String Json Valid Success")
                        },
                        false => panic!("Struct String Not Json Valid"),
                    }
                }
                ParserJson::Colors => {
                    eprintln!("CLI input Colors json: {}",json);
                    let vec: Vec<parser::colors::Colors> = parser::colors::Colors::parse(&json);
                    let valid: bool = parser::colors::Colors::validate_data(vec);
                    match valid {
                        true => {
                            eprintln!("Struct Colors Json Valid Success")
                        },
                        false => panic!("Struct Colors Not Json Valid"),
                    }
                }
            },
            Err(err) => {
                panic!("{}", err);
            }
        }
    } else {
        panic!("config path is null")
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // Continued program logic goes here...
}
