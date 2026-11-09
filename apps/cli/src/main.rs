//! UpdrageGuard command-line interface.
//!
//! Provides local workflows for analyzing and validating Soroban contract
//! upgrades without the web UI.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "updrageguard")]
#[command(about = "Test before you upgrade. Migrate with confidence.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compare two contract versions.
    Compare {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
    },
    /// Assess the risk of a proposed upgrade.
    Assess {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
    },
    /// Print version information.
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compare { from, to } => {
            println!("Comparing contract versions:");
            println!("  from: {from}");
            println!("  to:   {to}");
            // TODO: invoke version comparator on parsed contract versions.
        }
        Commands::Assess { from, to } => {
            println!("Assessing upgrade risk:");
            println!("  from: {from}");
            println!("  to:   {to}");
            // TODO: run full analysis pipeline and render a report.
        }
        Commands::Version => {
            println!("updrageguard {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
