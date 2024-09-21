use std::collections::HashSet;

pub fn load_standard_libraries_list() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    // load the Python standard libraries list
    let python_standard_libraries_json = std::fs::read("./python_std_libraries.json")?;
    let python_standard_libraries: Vec<String> =
        serde_json::from_slice(&python_standard_libraries_json)?;
    
    // store the loaded std libraries list into a HashSet, which ensures
    // each name is unique
    let mut packages: HashSet<String> = HashSet::new();
    for package in python_standard_libraries
     	.into_iter() {
    	packages.insert(package);
    }
    
    return Ok(packages);
}
