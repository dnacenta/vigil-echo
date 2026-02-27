mod analyze;
mod collect;
mod init;
mod parser;
mod paths;
mod pulse;
mod signals;
mod state;
mod status;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "vigil-echo",
    about = "Metacognitive monitoring for AI self-evolution",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the metacognitive monitoring system
    Init,
    /// Collect signal vector from current document state
    Collect {
        /// What triggered this collection
        #[arg(long, default_value = "manual")]
        trigger: String,
    },
    /// Analyze signal trends over rolling window
    Analyze {
        /// Number of sessions to include in window
        #[arg(long, default_value = "10")]
        window: usize,
    },
    /// Inject cognitive health assessment at session start
    Pulse,
    /// Cognitive health dashboard
    Status,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Init) | None => init::run(),
        Some(Commands::Collect { trigger }) => collect::run(&trigger),
        Some(Commands::Analyze { window }) => {
            let config = match state::load_config() {
                Ok(mut c) => {
                    c.window_size = window;
                    c
                }
                Err(e) => {
                    eprintln!("\x1b[31m✗\x1b[0m {e}");
                    std::process::exit(1);
                }
            };
            let history = match state::load_signals() {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("\x1b[31m✗\x1b[0m {e}");
                    std::process::exit(1);
                }
            };
            let analysis = analyze::run(&history, &config);
            match state::save_analysis(&analysis) {
                Ok(()) => {
                    println!("Analysis complete: {:?}", analysis.alert_level);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Some(Commands::Pulse) => pulse::run(),
        Some(Commands::Status) => status::run(),
    };

    if let Err(e) = result {
        eprintln!("\x1b[31m✗\x1b[0m {e}");
        std::process::exit(1);
    }
}
