A password cracker written in Rust that utilizes multithreading to increase efficiency during the comparison process.

This was originally for a project for one of my university courses, and now I decided that I want to work on it further. Right now it only reliably supports dictionary attacks against Bcrypt and SHA256 hashes, but I am planning on adding a lot more hashing algorithms to the program. I also want to implement functional zip file cracking through brute force and many other features that I believe can enhance the user's experience using the tool. 

# Requirements
+ Latest version of Cargo (as of 05/19/2025)

# Running The Program
In your command line terminal of choice, change directory to the one you just cloned/unzipped and then cd to the `src` directory. Since the `rockyou.txt` file is currently hardcoded, make sure to unzip the `rockyou.zip` archive into the `src` directory (the .txt file was too big to upload on both the github website and the github CLI). After that, run the `cargo run` command and it should create a directory called `target` in the repo's root directory, and the program's binary should then be contained in target's `debug` subdirectory. That same command will load up the CLI for the program. Just make sure you have a hash that you want to use against the entries in rockyou.txt.

If anyone who happens to come across this has any issues or functionality suggestions, please do not hesitate to post an issue to this repository. Thank you.
