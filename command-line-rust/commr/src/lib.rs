use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("commr")
        .version("0.1.0")
        .author("Leo Lin <wtlin1228@gmail.com>")
        .about("Rust comm")
        .arg(
            Arg::with_name("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .required(true),
        )
        .arg(
            Arg::with_name("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .required(true),
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .help("Case-insensitive comparison of lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("col1")
                .short("1")
                .help("Suppress printing of column 1")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("col2")
                .short("2")
                .help("Suppress printing of column 2")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("col3")
                .short("3")
                .help("Suppress printing of column 3")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIM")
                .short("d")
                .long("output-delimiter")
                .help("Output delimiter")
                .default_value("\t"),
        )
        .get_matches();

    Ok(Config {
        file1: matches.value_of("file1").unwrap().to_owned(),
        file2: matches.value_of("file2").unwrap().to_owned(),
        show_col1: !matches.is_present("col1"),
        show_col2: !matches.is_present("col2"),
        show_col3: !matches.is_present("col3"),
        insensitive: matches.is_present("insensitive"),
        delimiter: matches.value_of("delimiter").unwrap().to_owned(),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let mut lines1 = open(file1)?.lines().filter_map(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().filter_map(Result::ok).map(case);

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    let print_col1 = |val: &str| {
        if config.show_col1 {
            println!("{}", val);
        }
    };

    let print_col2 = |val: &str| {
        if config.show_col2 {
            if config.show_col1 {
                println!("{}{}", config.delimiter, val)
            } else {
                println!("{}", val);
            }
        }
    };

    let print_col3 = |val: &str| {
        if config.show_col3 {
            if config.show_col1 && config.show_col2 {
                println!("{}{}{}", config.delimiter, config.delimiter, val)
            } else if !config.show_col1 && !config.show_col2 {
                println!("{}", val);
            } else {
                println!("{}{}", config.delimiter, val)
            }
        }
    };

    loop {
        match (&line1, &line2) {
            (None, None) => break,
            (None, Some(val2)) => {
                print_col2(val2);
                line2 = lines2.next();
            }
            (Some(val1), None) => {
                print_col1(val1);
                line1 = lines1.next();
            }
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                std::cmp::Ordering::Less => {
                    print_col1(val1);
                    line1 = lines1.next();
                }
                std::cmp::Ordering::Equal => {
                    print_col3(val1);
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                std::cmp::Ordering::Greater => {
                    print_col2(val2);
                    line2 = lines2.next();
                }
            },
        }
    }

    Ok(())
}
