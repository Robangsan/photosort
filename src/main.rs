use std::path::PathBuf;

use clap::Parser;
use photosort::{gather_entries, process_files};
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(version, about="A quick tool to sort your photo/video files into folders based on their creation date", long_about="")]
struct Cli {
    #[arg(short='p', long="path", help="The path where the files to sort are located")]
    path: PathBuf,
    #[arg(short='o', long="output", help="Specify the output folder where the sorted folders will be created")]
    output_folder: Option<PathBuf>,
    #[arg(short='s', long="subfolder", help="Specify if an additional subfolder will be created for each month")]
    subfolder: bool,
    #[arg(short='r', long="recursive", help="Process folders recursively")]
    recursive: bool,
    #[arg(short='c', long="copy", help="Copies the files instead of moving them (takes longer)")]
    copy: bool,
    #[arg(default_values=["jpg", "jpeg", "png", "tiff", "tif", "mp4", "avi"], short='e', long="extensions", help="Specify the extensions of the files to be sorted")]
    exts: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let cli =  Cli::parse();
    let entries = gather_entries(&cli.path, cli.recursive, &cli.exts).context("Error while reading folder")?;

    if !entries.is_empty() {
        process_files(entries, cli.subfolder, cli.path, cli.output_folder, cli.copy)?;
    }

    Ok(())
}