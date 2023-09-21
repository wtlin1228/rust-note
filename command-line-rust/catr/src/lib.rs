use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Leo Lin <wtlin1228@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number the output lines, starting at 1")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number the non-blank output lines, starting at 1")
                .takes_value(false),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");

    if number_lines == true && number_nonblank_lines == true {
        return Err(
            "error: The argument '--number-nonblank' cannot be used with '--number'".into(),
        );
    }

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

// use Box because we don't know the size of BufReader at compile time
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Ok(file) => {
                let mut line_number = 1;
                for line_result in file.lines() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_number, line);
                        line_number += 1;
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!("");
                        } else {
                            println!("{:>6}\t{}", line_number, line);
                            line_number += 1;
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
            // Ok(mut file) => {
            //     let mut line_number = 1;
            //     loop {
            //         let mut line = String::new();
            //         match file.read_line(&mut line) {
            //             Ok(0) => break,
            //             Ok(_) => {
            //                 if config.number_lines {
            //                     print!("     {}\t{}", line_number, line);
            //                     line_number += 1;
            //                 } else if config.number_nonblank_lines {
            //                     if line == "\n" {
            //                         print!("{}", line);
            //                     } else {
            //                         print!("     {}\t{}", line_number, line);
            //                         line_number += 1;
            //                     }
            //                 } else {
            //                     print!("{}", line);
            //                 }
            //             }
            //             Err(_) => todo!(),
            //         }
            //     }
            // }
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
        }
    }

    Ok(())
}
