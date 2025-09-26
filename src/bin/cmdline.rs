use clap::{Parser, Subcommand};
use saves_sync::config::Config;
use saves_sync::utils;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Track,
    List,
    Pull,
    Push,
}

fn main() {
    let contents: String = match utils::get_config() {
        Ok(mut f) => {
            let mut contents: String = String::new();
            let _ = f.read_to_string(&mut contents);

            contents
        }
        Err(e) => panic!("{}", e),
    };

    let config: Config = toml::from_str(&contents).expect("Failed to parse config file");

    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            for i in config.list() {
                println!("{}", i)
            }
        }
        Commands::Track => config.track(),
        Commands::Pull => config.pull(),
        Commands::Push => config.push(),
    };
}
