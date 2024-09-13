use super::traits::VirtualEnvironmentPackage;

pub struct Anaconda {
	path: String
}

#[derive(Debug)]
pub struct CondaPackage {
	name: String, 
	version: String
}

impl VirtualEnvironmentPackage for CondaPackage {
	fn get_name(&self) -> String {
		return self.name.clone();
	}
	
	fn get_version(&self) -> String {
		return self.version.clone();
	}
}

pub struct CondaEnvironment {
	pub name: String, 
	pub packages: Vec<CondaPackage>,
}

impl Anaconda {
	pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let result = std::env::var("PATH")?;
		
		let mut lines: Vec<&str> = Vec::new();
		if !cfg!(target_os = "windows") {
			// if the platform is Linux or macOS
			lines = result.split(":").collect();
		} else {
			// if the platform is Windows
			lines = result.split(";").collect();
		}
		
		let mut conda_path: String = String::new();
		
		for line in lines {
			if line.contains("anaconda3/bin") {
				println!("Anaconda 3 detected, path is: {}", &line);
				
				let processed_line = line.strip_suffix("/bin")
					.unwrap();
				conda_path.push_str(processed_line);
				
			} else if line.contains("miniconda/bin") {
				println!("Miniconda detected, path is: {}", &line);
				
				let processed_line = line.strip_suffix("/bin")
        			.unwrap();
				conda_path.push_str(processed_line);
				
			}
		}
		
		if &conda_path == &"" {
			panic!("Conda does not seem to be installed.");
		}
		
		return Ok(Self { path: conda_path });
	}
	
	pub fn get_packages(
		&self, 
		environment: String,
		python_version: String
	) -> Result<Vec<CondaPackage>, Box<dyn std::error::Error>> {
		// construct the environment path
		let environment_path = self.path
			.clone() + "/envs/" + environment.as_str() + "/lib/" + python_version.as_str() + "/site-packages";
		
		// the retrieved packages 
		let mut packages: Vec<CondaPackage> = Vec::new();
		
		let package_jsons = std::fs::read_dir(
			environment_path
		)?;
		
		// find out dist-info
		let mut dist_infos: Vec<String> = Vec::new();
		for package_json in package_jsons {
			let package = package_json
				.unwrap()
				.file_name();
			
			let elements_string = package
				.to_string_lossy();
			let mut elements: Vec<&str> = elements_string
				.split(".")
				.collect();
			
			if elements.last().unwrap() == &"dist-info" {
				dist_infos.push(package.into_string().unwrap());
			}
		}
		
		for item in dist_infos {
			// at this point, the filename will contain a dist-info string
			// at its end. so we will need to remove it first
			let filename: Vec<&str> = item.split("-").collect();
			let package_name = filename
				.get(0)
				.unwrap();
			
			let dist_info_unstripped = filename
				.get(1)
				.unwrap();
			let strings: Vec<&str> = dist_info_unstripped
				.split(".")
				.collect();
			let mut dist_info_stripped: Vec<&str> = Vec::new();
			for string in strings {
				if string != "dist" {
					dist_info_stripped.push(string);
				}
			}
			let package_version = dist_info_stripped
				.join(".");
			
			packages.push(
				CondaPackage {
					name: package_name.to_string(), 
					version: package_version
				}
			)
		}
		
		return Ok(packages);
	}
}

impl CondaEnvironment {
	pub fn new(
		environment_name: String
	) -> Result<Self, Box<dyn std::error::Error>> {
		let output = std::process::Command::new("conda")
			.arg("list")
			.arg("-n")
			.arg(environment_name)
			.output()
			.expect("conda command failed to start");
		
		let mut packages: Vec<CondaPackage> = Vec::new();
		let result = String::from_utf8(
			output.stdout
		)?;
		
		let lines = result.split("\n");
		for line in lines {
			if !line.starts_with("#") {
				let elements = line.split(" ");
				println!("{:?}", elements.collect::<Vec<&str>>());
			}
		}
		
		return Ok(Self { name: "".to_string(), packages: packages });
	}
}

pub fn lookup_conda_environments() -> Result<
	Vec<String>, 
	Box<dyn std::error::Error>
> {
	let output = std::process::Command::new("conda")
		.arg("info")
		.arg("--envs")
		.output()
		.expect("conda command failed to start");
	
	let result = String::from_utf8(
		output.stdout
	)?;
	
	let lines = result.lines();
	let mut conda_environments: Vec<String> = Vec::new();
	for line in lines {
		if !line.starts_with("#") {
			let mut conda_environment = String::new();
			for character in line.chars() {
				// the conda environment name usually is followed by a 
				// whitespace. 
				if &character != &' ' {
					conda_environment.push(character);
				} else {
					break;
				}
			}
			
			// put the environment name to the vec
			if &conda_environment != &"" {
				conda_environments.push(conda_environment);
			}
		}
	}
	
	return Ok(conda_environments);
	
}