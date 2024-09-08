MainObjective: automatically generate a requirements.txt for python projects
	
Objectives {
	- get all paths of the py files under a python project folder
	- iterate over the paths, and get the parent imports of each py files contain
	- put the parent imports into a requirements.txt
}

Functions {
	function CollectPythonFilepaths(path_to_project: String) -> List[String] {
		- recursively get all the py file's path
			- if the path is a directory, extract all filepaths undernearth it
			- else, we return an empty Vec<String>
		
		return: paths
	}
	
	function ParseParentImports(filepath: String) -> List[String] {
		- open the py file
		- collect lines that are import statements
		- get the parent imports
		
		return: parent imports
	}
	
	function GenerateRequirements(parent_imports: List[List[String]]) -> None {
		- iterate over each parent imports
		- save the unique imports to a `requirements.txt` file
	}
}