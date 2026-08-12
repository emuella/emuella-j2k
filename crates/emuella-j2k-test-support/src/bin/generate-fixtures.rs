use emuella_j2k_test_support::generate_grayscale_j2k;
use std::env;
use std::fs;
use std::path::PathBuf;

fn run() -> Result<(), String> {
    let output = PathBuf::from(
        env::args_os()
            .nth(1)
            .ok_or_else(|| "usage: generate-fixtures EMPTY_OUTPUT_DIRECTORY".to_owned())?,
    );
    if output.exists() {
        let mut entries = fs::read_dir(&output)
            .map_err(|error| format!("failed to inspect {}: {error}", output.display()))?;
        if entries.next().is_some() {
            return Err(format!(
                "output directory is not empty: {}",
                output.display()
            ));
        }
    }
    fs::create_dir_all(&output)
        .map_err(|error| format!("failed to create {}: {error}", output.display()))?;

    let codestream = generate_grayscale_j2k(17, 19)?;
    fs::write(output.join("gray-gradient-17x19.j2k"), codestream)
        .map_err(|error| format!("failed to write generated codestream: {error}"))?;
    fs::write(
        output.join("PROVENANCE.toml"),
        concat!(
            "schema_version = 1\n",
            "generator = \"emuella-j2k-test-support\"\n",
            "recipe = \"grayscale-gradient-v1\"\n",
            "dimensions = [17, 19]\n",
            "license = \"Apache-2.0\"\n",
            "timestamped = false\n",
        ),
    )
    .map_err(|error| format!("failed to write provenance: {error}"))?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("generate-fixtures: {error}");
        std::process::exit(2);
    }
}
