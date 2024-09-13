mod virtual_envs;
mod python;
mod writer;
mod cli;

use std::collections::HashSet;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	
	// user input arguments from the command line
	let arguments = cli::Arguments::parse();
	
	if arguments.virtual_environment == "Conda" {
		let anaconda = virtual_envs::conda::Anaconda::new()?;
		let packages = anaconda.get_packages(
			arguments.env,
			arguments.python_version
		).unwrap();
		
		// used to convert the project path into a `PathBuf`
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
		
		let python_packages = python::get_packages(
			results, 
			packages
		)?;
		
		let mut python_packages_string = HashSet::new();
		for python_package in python_packages {
			python_packages_string.insert(
				python_package.to_string()
			);
		}
		
		path_buffer.push("requirements.txt");
		writer::write_to_file(path_buffer, python_packages_string)?;
	}
	
	return Ok(());
}