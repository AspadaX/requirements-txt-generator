# requirements-txt-generator
This project aims to reduce the painful experience after done writing a python project, and then find out that you forgot to keep a `requirements.txt`. The project is still at its early stage, but it can now get the basic works done. Feel free to try out.

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

# Contributions
All contributions are welcomed. 

# License
MIT.
