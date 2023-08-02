mod simulator;

use clap::Parser;
use simulator::{ElectionConfig, ElectionResult};
use std::{path::PathBuf, result};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    elections: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
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

fn write_result_file(filepath: PathBuf, results: Vec<ElectionResult>) -> Result<(), ()> {
    let mut writer = csv::Writer::from_path(filepath).unwrap();

    for res in results {
        let res = writer.serialize(res).unwrap();
    }
    
    Ok(())    
}

fn main() {
    let args = Args::parse();

    let elections = get_elections_file(args.elections).unwrap();
    let mut results = Vec::new();

    for election in elections {
        let res = simulator::run_simulation(election);
        results.push(res);
    }

    if let Some(filepath) = args.output {
        write_result_file(filepath, results).unwrap();
    }

}
