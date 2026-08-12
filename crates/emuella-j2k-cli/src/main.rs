use emuella_j2k_core::{InspectOptions, inspect};
use std::env;
use std::fs;
use std::path::PathBuf;

fn usage() -> &'static str {
    "usage: emuella-j2k inspect INPUT"
}

fn run() -> Result<(), String> {
    let mut arguments = env::args_os().skip(1);
    let command = arguments.next().ok_or_else(|| usage().to_owned())?;
    let input = PathBuf::from(arguments.next().ok_or_else(|| usage().to_owned())?);
    if arguments.next().is_some() || command != "inspect" {
        return Err(usage().to_owned());
    }

    let bytes =
        fs::read(&input).map_err(|error| format!("failed to read {}: {error}", input.display()))?;
    let metadata = inspect(&bytes, &InspectOptions::default())
        .map_err(|error| format!("failed to inspect {}: {error}", input.display()))?;
    println!("{metadata:#?}");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("emuella-j2k: {error}");
        std::process::exit(2);
    }
}
