mod python;
mod writer;
mod cli;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	
	// user input arguments from the command line
	let arguments = cli::Arguments::parse();
	
	let mut path_buffer = std::path::PathBuf::from(
		arguments.path
	);
	let filepaths = python::recursively_get_py_files(
		&path_buffer
	)?;
	
	let py_files_contents = python::get_py_files_content(
		filepaths
	)
		.await?;
	
	let results = python::collect_py_parent_imports(
		py_files_contents
	);
	
	path_buffer.push("requirements.txt");
	writer::write_to_file(path_buffer, results)?;
	
	return Ok(());
}