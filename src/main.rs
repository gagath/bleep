mod audio;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.1.0", author = "Agathe Porte <microjoe@microjoe.org>")]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    /// List all builtin sounds
    List,

    /// Run a command and bleep when finished
    Run,

    /// Play a builtin sound
    Play {
        /// Name of the builtin sound to play
        name: String,
    },

    /// Loop a builtin sound
    Loop {
        /// Name of the builtin sound to play
        name: String,

        /// Number of times to repeat the sound
        count: Option<u32>,
    },
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::List => {
            for e in audio::BUILTINS.keys() {
                println!("{}", e);
            }
        }
        SubCommand::Play { name } => {
            audio::play_builtin(&name).unwrap();
        }
        SubCommand::Loop { name, count } => {
            if let Some(c) = count {
                for _ in 0..c {
                    audio::play_builtin(&name).unwrap();
                }
            } else {
                loop {
                    audio::play_builtin(&name).unwrap();
                }
            }
        }
        SubCommand::Run => {}
    }
}
