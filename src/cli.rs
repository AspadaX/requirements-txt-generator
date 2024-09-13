use std::default;

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
    pub virtual_environment: String,
    
    /// The Python version in the virtual environment
    #[arg(short, long)]
	pub python_version: String,
	
	/// The virtual environment name
	#[arg(short, long)]
	pub env: String,
}