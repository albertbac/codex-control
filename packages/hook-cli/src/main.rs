use std::io::{self, Read};

use clap::{Parser, Subcommand};
use hook_cli::{run_doctor, run_ingest, run_policy};

#[derive(Parser)]
#[command(name = "codex-control-hook")]
#[command(about = "Codex Control hook ingestion and policy CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ingest {
        #[arg(long)]
        emit_json_response: bool,
    },
    Policy,
    Doctor,
}

fn main() {
    let cli = Cli::parse();
    let output = match cli.command {
        Commands::Ingest { emit_json_response } => run_ingest(&read_stdin(), emit_json_response),
        Commands::Policy => run_policy(&read_stdin()),
        Commands::Doctor => run_doctor(),
    };

    if !output.stdout.is_empty() {
        print!("{}", output.stdout);
    }
    if !output.stderr.is_empty() {
        eprintln!("{}", output.stderr);
    }
    std::process::exit(output.exit_code);
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    if io::stdin().read_to_string(&mut buffer).is_err() {
        return String::new();
    }
    buffer
}
