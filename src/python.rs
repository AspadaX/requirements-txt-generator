use tokio::io::AsyncReadExt;

pub fn recursively_get_py_files(
	directory: &std::path::PathBuf
) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
	
	// see if it is a directory or a file
	let target = std::fs::File::open(directory)?;
	
	// if it is a directory, we will be saving the filepaths underneath
	let mut filepaths: Vec<std::path::PathBuf> = Vec::new();
	
	// case if it is a directory
	if target.metadata().unwrap().is_dir() {
		// get all targets under this directory
		let sub_targets = std::fs::read_dir(
			directory
		)?;
		
		// create an OsStr for matching the extension we parsed
		let desired_extension = std::ffi::OsStr::new("py");
		
		for item in sub_targets {
			
			let filepath = item.unwrap().path().clone();
			
			match filepath.is_dir() {
				true => {
					let mut recursive_filepaths = recursively_get_py_files(
						&filepath
					)?;
					filepaths.append(&mut recursive_filepaths);
				},
				false => {
					if filepath.extension() == Some(desired_extension) {
						filepaths.push(filepath);
					}
				}
			};
			
		}
	}
	
	return Ok(filepaths);
}

/// get the py files' content under the specified path 
pub async fn get_py_files_content(
	filepaths: Vec<std::path::PathBuf>
) -> std::io::Result<Vec<String>> {
    
	// concurrently open files in this directory
	let mut open_files_tasks: tokio::task::JoinSet<
		Result<
			tokio::fs::File, 
			std::io::Error
		>
	> = tokio::task::JoinSet::new();
	for (_, item) in filepaths.into_iter().enumerate() {
		// we only open the .py files
		if &item.extension().unwrap() == &"py" {
			open_files_tasks.spawn(
				tokio::fs::File::open(item)
			);
		}
    }
    
    // retrieve file open results
    let mut py_files_content: Vec<String> = Vec::new();
    while let Some(result) = open_files_tasks
    	.join_next()
     	.await {
      		// read the content of the py file
	    	let mut py_file = result??;
			
			// read the py file content into a String
			let mut py_file_content: String = String::new();
			let _ = py_file.read_to_string(&mut py_file_content).await?;
			
		  	py_files_content.push(py_file_content);
    	}
    
    return Ok(py_files_content);
}

pub fn collect_py_parent_imports(
	py_file_contents: Vec<String>
) -> std::collections::HashSet<String> {
	// store all parent imports found
	let mut all_parent_imports: std::collections::HashSet<String> = std::collections::HashSet::new();
	for py_file_content in py_file_contents {
		for line in py_file_content.lines() {
			// python imports always start with either `import` or `from`
			// what we really care about is the first element after the 
			// `import` or `from` key word
			if line.starts_with("import") || line.starts_with("from") {
				let mut splited_line = line.split(" ");
				let parent_imports = splited_line
					.nth(1)
					.unwrap_or("");
				
				if parent_imports != "" {
					// python imports are splited by dots
					let mut splited_parent_imports = parent_imports
						.split(".");
					
					let parent_import = splited_parent_imports
				        .nth(0)
				        .unwrap_or("");
					
					all_parent_imports.insert(parent_import.to_string());
				}
			}
		}
	}
	return all_parent_imports;
}