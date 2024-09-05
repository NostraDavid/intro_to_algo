use clap::Parser;
use intro_to_algo::algorithms;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Algorithm to run
    #[arg(short, long)]
    algorithm: String,
}

/// Run the specified algorithm
///
/// Run like `cargo run -- --algorithm insertion_sort` if you don't feel like
/// figuring out what the executable is called.
fn main() {
    let args = Args::parse();

    // Call the specified algorithm
    match args.algorithm.as_str() {
        "insertion_sort" => algorithms::insertion_sort::run(),
        _ => println!("Unknown algorithm"),
    }
}
