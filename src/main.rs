mod simulator;

use clap::Parser;
use simulator::ElectionConfig;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    elections: PathBuf,
}

fn get_elections_file(filepath: PathBuf) -> Result<Vec<ElectionConfig>, ()> {
    let mut reader = csv::Reader::from_path(filepath).unwrap();
    let mut rounds = vec![];

    for result in reader.deserialize() {
        let round: ElectionConfig = result.unwrap();
        rounds.push(round)
    }

    Ok(rounds)
}

fn main() {
    let args = Args::parse();

    let elections = get_elections_file(args.elections).unwrap();

    for election in elections {
        simulator::run_simulation(election);
    }
}
