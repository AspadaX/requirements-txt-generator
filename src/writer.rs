use std::io::Write;

pub fn write_to_file(
    save_path: std::path::PathBuf,
    python_modules: std::collections::HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create_new(save_path)?;

    for python_module in python_modules {
        writeln!(file, "{}", python_module)?;
    }

    return Ok(());
}
