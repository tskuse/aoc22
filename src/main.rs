use clap::Parser;

mod problems;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let cli = Cli::parse();
    match cli.day {
        1 => problems::day1::execute(),
        _ => eprintln!("day {} unrecognized", cli.day),
    }
}
