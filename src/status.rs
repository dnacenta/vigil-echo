use crate::state::{self, AlertLevel, Trend};

const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

pub fn run() -> Result<(), String> {
    let config = state::load_config()?;
    let history = state::load_signals()?;
    let analysis = state::load_analysis()?;

    println!("\n{BOLD}vigil-echo{RESET} — cognitive health dashboard\n");

    // Signal history
    println!("  {BOLD}Signal History{RESET}");
    if history.is_empty() {
        println!("    No signals collected yet. Run `vigil-echo collect` after a session.");
    } else {
        println!(
            "    {} data points (max {})",
            history.len(),
            config.max_history
        );
        if let Some(first) = history.first() {
            println!("    First: {}", &first.timestamp[..19]);
        }
        if let Some(last) = history.last() {
            println!("    Latest: {}", &last.timestamp[..19]);
            println!();
            println!("  {BOLD}Latest Signals{RESET}");
            print_signal(
                "    vocabulary_diversity",
                last.signals.vocabulary_diversity,
            );
            print_signal("    question_generation", last.signals.question_generation);
            print_signal("    thought_lifecycle", last.signals.thought_lifecycle);
            print_signal("    evidence_grounding", last.signals.evidence_grounding);
        }
    }

    // Analysis
    println!();
    if let Some(analysis) = analysis {
        let level_str = match &analysis.alert_level {
            AlertLevel::Healthy => format!("{GREEN}HEALTHY{RESET}"),
            AlertLevel::Watch => format!("{YELLOW}WATCH{RESET}"),
            AlertLevel::Concern => format!("{RED}CONCERN{RESET}"),
            AlertLevel::Alert => format!("{BOLD}{RED}ALERT{RESET}"),
        };
        println!("  {BOLD}Analysis{RESET}");
        println!("    Status: {level_str}");
        println!(
            "    {} improving, {} stable, {} declining",
            analysis.improving_count, analysis.stable_count, analysis.declining_count
        );

        if !analysis.signals.is_empty() {
            println!();
            println!("  {BOLD}Trends{RESET}");
            for (name, trend) in &analysis.signals {
                let (arrow, color) = match trend.trend {
                    Trend::Improving => ("↑", GREEN),
                    Trend::Stable => ("→", DIM),
                    Trend::Declining => ("↓", RED),
                };
                let val = trend
                    .current
                    .map(|v| format!("{:.2}", v))
                    .unwrap_or("—".to_string());
                println!(
                    "    {color}{arrow}{RESET} {:<24} {} ({:+.2})",
                    friendly_name(name),
                    val,
                    trend.delta
                );
            }
        }

        if !analysis.watch_messages.is_empty() {
            println!();
            println!("  {BOLD}Alerts{RESET}");
            for msg in &analysis.watch_messages {
                println!("    {YELLOW}⚡{RESET} {msg}");
            }
        }

        if let Some(highlight) = &analysis.highlight {
            println!();
            println!("  {GREEN}✦{RESET} {highlight}");
        }
    } else {
        println!("  {DIM}No analysis yet — need at least one collection.{RESET}");
    }

    // Config summary
    println!();
    println!("  {BOLD}Config{RESET}");
    println!("    Window size: {}", config.window_size);
    println!("    Max history: {}", config.max_history);
    println!("    Cooldown: {}s", config.cooldown_seconds);
    println!();

    Ok(())
}

fn print_signal(label: &str, value: Option<f64>) {
    match value {
        Some(v) => println!("{label}: {v:.2}"),
        None => println!("{label}: {DIM}—{RESET}"),
    }
}

fn friendly_name(name: &str) -> &str {
    match name {
        "vocabulary_diversity" => "vocabulary diversity",
        "question_generation" => "question generation",
        "thought_lifecycle" => "thought lifecycle",
        "evidence_grounding" => "evidence grounding",
        _ => name,
    }
}
