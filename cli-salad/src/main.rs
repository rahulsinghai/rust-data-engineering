use clap::Parser;
use cli_salad::create_fruit_salad;

// Derives the Parser trait for the Opts struct, enabling it to parse command-line arguments.
// The `#[derive(Parser)]` attribute generates the necessary code to parse the arguments.
// This struct defines the command-line options for the application.
#[derive(Parser)]
// The `#[clap]` attribute provides additional information about the application.
// Provides metadata for the command-line application, including version, author, and a brief description.
#[clap(
    // The `version` and `author` attributes provide information about the application.
    version = "1.0",
    author = "Rahul Singhai <singrahu@gmail.com>",
    // The `about` attribute provides a description of the argument.
    about = "Number of fruits to include in the salad"
)]
// Defines a struct to hold the command-line options.
struct Opts {
    // It uses the `clap` crate to parse command-line arguments.
    // This option can be specified with `-n` or `--number`.
    // The `short` and `long` attributes specify the short and long versions of the argument.
    #[clap(short, long)]
    // The `number` field represents the number of fruits to include in the salad.
    // The `usize` type indicates that the argument should be parsed as an unsigned integer.
    number: usize,
}

// The main function is the entry point of the application.
// cargo run -- -n 5
// -- will pass anything after -- to the command line framework
fn main() {
    // Parse the command-line arguments into the `Opts` struct.
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create the fruit salad
    create_fruit_salad(num_fruits);

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits,
        create_fruit_salad(num_fruits)
    );
}
