mod render;
mod schema;
mod style;

use clap::Parser;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "yaml-resume", about = "Convert YAML to a beautiful HTML resume")]
struct Args {
    /// Path to the YAML resume file
    input: PathBuf,

    /// Output HTML file path (default: resume.html)
    #[arg(short, long, default_value = "resume.html")]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    let yaml = match std::fs::read_to_string(&args.input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {e}", args.input.display());
            process::exit(1);
        }
    };

    let resume: schema::Resume = match serde_yaml::from_str(&yaml) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error parsing YAML: {e}");
            process::exit(1);
        }
    };

    let html = render::render(&resume);

    if let Err(e) = std::fs::write(&args.output, &html) {
        eprintln!("Error writing {}: {e}", args.output.display());
        process::exit(1);
    }

    println!("Wrote {}", args.output.display());
}
