mod cli;
mod python;
mod virtual_envs;
mod writer;

use std::collections::HashSet;

use clap::Parser;
use cli::Validation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	//load the standard libraries list, which will need to be filtered out
	//, after done parsing the pacakges
	let stds: HashSet<String> = python::std::load_standard_libraries_list()?;
	
    // user input arguments from the command line
    let arguments = cli::Arguments::parse();
    // validate the arguments
    let _ = arguments.validate()?;
    
    let anaconda = virtual_envs::conda::Anaconda::new()?;
    
    let mut environment_path = String::new();
    if !cfg!(target_os = "windows") {
	    environment_path = anaconda.get_environment_path(
		    arguments.env.clone().unwrap(), 
			arguments.python_version.clone().unwrap()
	    )?;
    } else {
    	environment_path = arguments.env_path.unwrap();
    }
    
    if arguments.virtual_environment.unwrap() == "Conda" {
        // record the start time for benchmarking
        let start_time = std::time::Instant::now();
        println!(
            "Start generating requirements.txt ...for {}",
            &arguments.path
        );

        println!("{}", &environment_path);
        let packages = anaconda
            .get_packages(
            	&environment_path
            )
            	.unwrap();

        // used to convert the project path into a `PathBuf`
        let mut path_buffer = std::path::PathBuf::from(
        	arguments.path
        );
        let filepaths = python::package::recursively_get_py_files(
        	&path_buffer
        )?;

        let py_files_contents = python::package::get_py_files_content(
        	filepaths
        ).await?;

        let mut results = python::package::collect_py_parent_imports(
        	py_files_contents
        );

        let python_packages = python::package::get_packages(
	        &mut results, 
			stds,
	        packages
        )?;

        let mut python_packages_string = HashSet::new();
        for python_package in python_packages {
            python_packages_string.insert(python_package.to_string());
        }

        path_buffer.push("requirements.txt");
        writer::write_to_file(path_buffer, python_packages_string)?;

        // display the benchmark
        let end = start_time.elapsed().as_secs_f64();
        println!("Generation finished in {} secs", end);
    }

    return Ok(());
}
