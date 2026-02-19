use clap::Parser;
use std::io::Read;
use whitespace_sifter::WhitespaceSifter;

#[derive(Parser)]
#[command(
    name = "whitespace-sifter",
    about = "Sift duplicate whitespaces away!",
    author = "JumperBot_",
    version
)]
struct Args {
    /// Reads from stdin if omitted
    input: Option<String>,

    /// Preserve newlines
    #[arg(long)]
    preserve_newlines: bool,
}

fn main() {
    let args: Args = Args::parse();

    let input: String = if let Some(val) = args.input {
        val
    } else {
        let mut buf = String::new();
        if let Err(err) = std::io::stdin().read_to_string(&mut buf) {
            eprintln!("Error reading from stdin: {err}");
            std::process::exit(1);
        }
        buf
    };

    if args.preserve_newlines {
        print!("{}", input.sift_preserve_newlines());
        return;
    }

    print!("{}", input.sift());
}
