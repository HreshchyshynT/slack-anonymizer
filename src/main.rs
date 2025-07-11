use clap::Parser;
use std::fs;
use std::io::{self, Read};

use slack_anonymizer::{Options, anonymize_text, format_legend};

#[derive(Parser)]
#[command(name = "slack-anonymizer")]
#[command(
    about = "Anonymize Slack text by replacing usernames, channels, emails, and custom keywords"
)]
#[command(version)]
struct Args {
    /// Input file path. If not provided, reads from stdin
    input: Option<String>,

    /// Anonymize URLs
    #[arg(long)]
    urls: bool,

    /// Comma-separated list of keywords to replace with anonymous data
    #[arg(long)]
    replace: Option<String>,

    /// Print anonymization legend after output
    #[arg(long)]
    legend: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Read input
    let input = match args.input {
        Some(file_path) => fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    // Parse keywords
    let keywords = if let Some(keyword_str) = args.replace {
        keyword_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        Vec::new()
    };

    // Set up options
    let options = Options::new(args.urls, keywords);

    // Anonymize text
    let (anonymized, map) = anonymize_text(&input, &options)?;

    // Output result
    print!("{}", anonymized);

    // Output legend if requested
    if args.legend {
        let legend = format_legend(&map)?;
        if !legend.is_empty() {
            print!("{}", legend);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::process::Command;
    use tempfile::NamedTempFile;

    #[test]
    fn test_cli_basic() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hey @john, check #general").unwrap();

        let output = Command::new("cargo")
            .args(&["run", "--", temp_file.path().to_str().unwrap()])
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("@user"));
        assert!(stdout.contains("#ch"));
    }

    #[test]
    fn test_cli_with_legend() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hey @john, check #general").unwrap();

        let output = Command::new("cargo")
            .args(&["run", "--", temp_file.path().to_str().unwrap(), "--legend"])
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("=== ANONYMIZATION LEGEND ==="));
        assert!(stdout.contains("@john → @user"));
        assert!(stdout.contains("#general → #ch"));
    }

    #[test]
    fn test_cli_with_urls() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Visit https://company.com").unwrap();

        let output = Command::new("cargo")
            .args(&["run", "--", temp_file.path().to_str().unwrap(), "--urls"])
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("https://example"));
    }

    #[test]
    fn test_cli_with_keywords() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "ProjectX is ready").unwrap();

        let output = Command::new("cargo")
            .args(&[
                "run",
                "--",
                temp_file.path().to_str().unwrap(),
                "--replace",
                "ProjectX",
            ])
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("keyword1"));
    }
}
