use clap::Parser;
use serde::{Deserialize, Serialize};
use frate_registry::fetch;
use frate_registry::registry::{generate_registry, save_registry};
use crate::cli::{Cli, Commands};
use rayon::prelude::*;


pub mod cli;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { name, repo, out, max } => {
            generate(name, repo, out, max)?;
        }
        Commands::FromList { file} => {
            println!("Generating from: {}", file);
            let file_str = std::fs::read_to_string(file)?;
            let generations: Vec<GenerationFile> = serde_json::from_str(&file_str)?;
            generations.par_iter().for_each(|generation| {
                if let Err(err) = generate(
                    generation.name.clone(),
                    generation.repo.clone(),
                    generation.out.clone(),
                    generation.max,
                ) {
                    eprintln!("Error generating {}: {}", generation.name, err);
                }
            });
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerationFile {
    name: String,
    repo: String,
    out: Option<String>,
    max: Option<usize>
}

fn generate(
    name: String,
    repo: String,
    out: Option<String>,
    max: Option<usize>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating registry for {} from {}", &name, &repo);
    let releases = fetch::fetch_releases(&repo)?;
    let tool = generate_registry(releases, &repo, max)?;
    if let Some(out) = out {
        save_registry(&tool, &out)?;
        println!("written to {}", &out)
    }
    else {
        let out = format!("{}.json", &name);
        save_registry(&tool, &out)?;
        println!("written to {}", &out)
    }
    Ok(())
}
