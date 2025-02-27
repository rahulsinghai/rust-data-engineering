// A command line tool to play the game of Marco Polo.
// The game is played by two players. The first player says "Marco" and the
// second player responds with "Polo". The first player then says "Marco" again
// and the second player responds with "Polo" again. This continues until the
// first player says "Marco" and the second player responds with "Polo" for the
// third time. The game then ends and the first player is declared the winner.

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    name = "marco-polo",
    author = "Rahul Singhai",
    about = "A command line tool to play the game of Marco Polo."
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(author = "Rahul Singhai")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = marco_polo::marco_polo(&name);
            println!("{}", result);
        }
        None => println!("No command given"),
    }
}
