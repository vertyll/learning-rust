use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use clap::{Arg, Parser};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum ParseValue {
    PlusZero,
    OtherNum(i64),
}

#[derive(Parser, Debug)]
#[command(author = "gawrmiko@gmail.com", version = "0.1.0", about = "Rust tail")]
pub struct Args {
    #[arg(required = true, value_name = "FILE")]
    files: Vec<String>,

    #[arg(short = 'n', long = "lines", value_name = "LINES", default_value = "10")]
    lines: String,

    #[arg(short = 'c', long = "bytes", value_name = "BYTES", conflicts_with = "lines")]
    bytes: Option<String>,

    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
}

pub fn run(config: Args) -> MyResult<()> {
    let num_files = config.files.len();
    let lines_val = parse_num(&config.lines)?;
    let bytes_val = match &config.bytes {
        Some(val) => Some(parse_num(val)?),
        None => None,
    };

    for (file_num, file_name) in config.files.iter().enumerate() {
        match File::open(file_name) {
            Err(err) => eprintln!("{} {}", file_name, err),
            Ok(file) => {
                if !config.quiet && num_files > 1 {
                    println!(
                        "{} ==> {} <==",
                        if file_num > 0 {"\n"} else {""},
                        file_name
                    )
                }
                let (total_lines, total_bytes) = count_lines_bytes(file_name)?;
                let file = BufReader::new(file);

                match &bytes_val {
                    Some(bytes_value) => print_bytes(file, bytes_value, total_bytes)?,
                    None => print_lines(file, &lines_val, total_lines)?,
                }
            }
        };
    }
    Ok(())
}