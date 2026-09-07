mod html_report;
mod limits;
mod report;

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;

use report::CostRecord;

#[derive(Parser)]
#[command(name = "soroban-cost-cli", about = "Collect Soroban cost data from cargo test")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run cargo test, collect cost markers, and display results
    Report {
        /// Write an HTML report to this path
        #[arg(long = "html")]
        html: Option<PathBuf>,

        /// Path to the Cargo manifest directory (defaults to current dir)
        #[arg(long = "manifest-path")]
        manifest_path: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let sub = match cli.command {
        Commands::Report { html, manifest_path } => (html, manifest_path),
    };
    let (html, manifest_path) = sub;

    let manifest_dir = manifest_path.unwrap_or_else(|| PathBuf::from("."));
    let manifest_dir = std::fs::canonicalize(&manifest_dir)
        .with_context(|| format!("cannot resolve manifest path: {}", manifest_dir.display()))?;

    if !manifest_dir.join("Cargo.toml").exists() {
        anyhow::bail!(
            "no Cargo.toml found in {}",
            manifest_dir.display()
        );
    }

    println!(
        "{} running `cargo test` in {} ...",
        "=>".green().bold(),
        manifest_dir.display()
    );

    let output = Command::new("cargo")
        .args(["test", "--", "--nocapture"])
        .current_dir(&manifest_dir)
        .output()
        .context("failed to spawn cargo test")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let records: Vec<CostRecord> = stdout
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("##SOROBAN_COST_JSON##")?;
            serde_json::from_str(rest).ok()
        })
        .collect();

    if records.is_empty() {
        eprintln!(
            "{}",
            "No ##SOROBAN_COST_JSON## markers found in test output.".yellow()
        );
        eprintln!(
            "Make sure your tests call {}.",
            "soroban_cost_harness::record(...)".cyan()
        );
        if !output.status.success() {
            eprintln!("\n--- cargo test stderr ---\n{stderr}");
            anyhow::bail!("cargo test failed (see above)");
        }
        return Ok(());
    }

    println!(
        "\n{} collected {} cost record(s):\n",
        "=>".green().bold(),
        records.len()
    );

    report::print_report(&records, &limits::SorobanLimits::default());

    if let Some(html_path) = html {
        html_report::write_html_report(&records, &limits::SorobanLimits::default(), &html_path)?;
        println!(
            "\n{} HTML report written to {}",
            "=>".green().bold(),
            html_path.display()
        );
    }

    if !output.status.success() {
        eprintln!("\n--- cargo test stderr ---\n{stderr}");
        anyhow::bail!("cargo test failed (see above)");
    }

    Ok(())
}
