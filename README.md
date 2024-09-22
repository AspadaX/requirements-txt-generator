# requirements-txt-generator
This project aims to reduce the painful experience after done writing a python project, and then find out that you forgot to keep a `requirements.txt`. The project is still at its early stage, but it can now get the basic works done. Feel free to try out.

# Why to use
While the Python script finished a generation in 0.04 seconds and `pipreqs` used more than 1 seconds, the Rust implementation of a requirements txt generator completed a generation in 0.01 seconds with cleaner result, as it does the following: 
- The Rust implementation is inherently faster than garbage collected languages. 
- It collects python packages without connecting to third party API written in Python, for example, Python codes or Conda API. Rather, it achieves this by analyzing the Conda environment folder and the project files.
- The Rust implementation relis on the local packages information without the need to reqeust the Internet. For example, in `pipreqs`, it retrieves the latest package version via http requests which comes with two disadvantages:
	- the network requests are not as efficient as the local operations. 
	- the network requests can be unstable in certain regions where the Internet is supervised or limited. 
	- since the package version adheres to the latest one, it is possible that the latest releases my contain bugs that were not presented in the former version that is used in the developer's environment. 

In addition to the efficiency augmentations, the Rust implementation also does not need to install additional dependecies to your Python project environment, which further prevents polluting your environment and making dependency management harder. 

# how to use
If you have Rust installed, you can pretty much use the following command to install this tool:
```
cargo install requirements-txt-generator
```
then, use this command to generate your `requirements.txt`
```
requirements-txt-generator --path `your python project path` --python-version `your python version, for example, 3.11.4 is python3.11` --env `your conda enviornment name for your project`
```
there you go, after the program finishes, you will see the `requirements.txt` under your project folder.

However, if you are Windows user, you will have to specify your Conda environment path. You can run this tool with the following command:
```
requirements-txt-generator.exe --path `your python project path`  --env-path `your conda enviornment path`
```

# Contributions
All contributions are welcomed. 

# License
MIT.
