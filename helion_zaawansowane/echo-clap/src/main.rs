use clap::{Parser};

/// Echo your input
#[derive(Parser, Debug)]
#[clap(author = "gawrmiko@gmail.com", version = "1.0.0", about = "Echo your input")]
struct Args {
    #[clap(required = true)]
    text: Vec<String>,

    #[clap(short = 'n', long = "no-newline")]
    omit_newline: bool,
}


fn main() {
    let args = Args::parse();

    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline {""} else {"\n"}
    );
}
