use std::path::PathBuf;

use clap::Parser;
use photosort::{gather_entries, process_files};

#[derive(Parser)]
#[command(version, about="A quick tool to sort your photo/video files into folders based on their creation date", long_about="")]
struct Cli {
    #[arg(default_value=".", help="The path where the files to sort are located")]
    path: PathBuf,
    #[arg(short='s', long="subfolder", help="Specify if an additional subfolder will be created for each month")]
    subfolder: bool,
    #[arg(short='r', long="recursive", help="Process folders recursively")]
    recursive: bool,
    #[arg(default_values=["jpg", "jpeg", "png", "tiff", "tif", "mp4", "avi"], short='e', long="extensions", help="Specify the extensions of the files to be sorted")]
    exts: Vec<String>,
}

fn main() -> Result<(), std::io::Error> {
    let cli =  Cli::parse();
    let entries = gather_entries(cli.path, cli.recursive, &cli.exts).expect("Error while reading folder");

    process_files(entries, cli.subfolder)?;

    Ok(())
}