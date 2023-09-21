use crate::TakeValue::*;
use clap::{App, Arg};
use std::{error::Error, fs::File, io::BufReader, num::ParseIntError};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tailr")
        .version("0.1.0")
        .author("Leo Lin <wtlin1228@gmail.com>")
        .about("Rust tail")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Suppress headers")
                .takes_value(false),
        )
        .get_matches();

    let to_take_num = |val: &str| match val {
        "+0" => Ok(PlusZero),
        _ => {
            let num = val.parse::<i64>()?;
            if val.starts_with("+") {
                Ok(TakeNum(num))
            } else {
                Ok(TakeNum(num * -1))
            }
        }
    };

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches
            .value_of("lines")
            .map(to_take_num)
            .transpose()
            .map_err(|_: ParseIntError| {
                format!(
                    "illegal line count -- {}",
                    matches.value_of("lines").unwrap()
                )
            })?
            .unwrap(),
        bytes: matches
            .value_of("bytes")
            .map(to_take_num)
            .transpose()
            .map_err(|_: ParseIntError| {
                format!(
                    "illegal byte count -- {}",
                    matches.value_of("bytes").unwrap()
                )
            })?,
        quiet: matches.is_present("quiet"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match File::open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

fn count_lines_bytes(filename: &str) -> MyResult<(i64, i64)> {
    let file = BufReader::new(File::open(&filename).unwrap());
    file.into_iter()
}

#[cfg(test)]
mod tests {
    use super::count_lines_bytes;

    #[test]
    fn test_count_lines_bytes() {
        let res = count_lines_bytes("tests/inputs/one.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (1, 24));

        let res = count_lines_bytes("tests/inputs/ten.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (10, 49));
    }
}
