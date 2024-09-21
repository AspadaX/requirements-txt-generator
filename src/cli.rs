use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Your python project folder path
    #[arg(short, long)]
    pub path: String,

    /// Your virtual environment for this project, for now it only supports Conda
    #[arg(short, long, default_value = "Conda")]
    pub virtual_environment: Option<String>,

    /// The Python version in the virtual environment
    #[arg(short, long)]
    pub python_version: Option<String>,

    /// The virtual environment name
    #[arg(short, long)]
    pub env: Option<String>,
    
    /// specificly for Windows, you will have to specify where your 
    /// virtual environment is located at
    #[arg(short, long)]
    pub env_path: Option<String>,
}

pub trait Validation {
	/// a function to validate the data structure
	fn validate(&self) -> Result<bool, Box<dyn std::error::Error>>;
}

impl Validation for Arguments {
	fn validate(&self) -> Result<bool, Box<dyn std::error::Error>> {
		let mut is_validated: bool = false;
		if cfg!(target_os = "windows") {
	    	if self.env_path.is_none() {
	     		panic!(
					"You have to specify `--env-path` as you seem to use Windows."
				);
			} else if self.env.is_some() || self.python_version.is_some() || self.virtual_environment.is_some() {
				panic!(
					"You don't need to specify `--env`, `--python-version`, or `--virtual-environment` on Windows."
				);
			}
		} else {
		   	if self.env_path.is_some() && (self.env.is_some() || self.python_version.is_some() || self.virtual_environment.is_some()) {
		    	panic!(
					"You don't need to specify `--env`, `--python-version`, or `--virtual-environment`, if you have already specified `--env-path`."
				);
			}
		}
		
		// change the validation to true if all logics above passed
		is_validated = true;
		return Ok(is_validated);
	}
}