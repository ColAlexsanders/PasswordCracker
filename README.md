A password cracker written in Rust that utilizes multithreading to increase efficiency during the comparison process.

This project was originally started for one of my university courses, and I enjoyed it so much that I decided to work on it further.

# Requirements
+ Latest version of Cargo

# Running The Program
In your command line terminal of choice, change directory to the one you just cloned/unzipped and then cd to the `src` directory. Since the `rockyou.txt` file is currently hardcoded, make sure to unzip the `rockyou.zip` archive into the `src` directory (the .txt file was too big to upload on both the github website and the github CLI). After that, use the `cargo run` command and it should create a directory called `target` in the repo's root directory, and the program's binary should then be contained in target's `debug` subdirectory. That same command will load up the CLI for the program. Just make sure you have a hash that you want to use against the entries in rockyou.txt.

If anyone who happens to come across this has any issues or functionality suggestions, please do not hesitate to post an issue to this repository. Thank you.

# My plans for this:
+ add both dictionary and brute force attack functionality for the following algorithms:
  + argon2 (**dictionary support currently implemented**)
  + Salted SHA-256 (**dictionary support currently implemented**)
  + Bcrypt (**dictionary support currently implemented**)
  + Salted MD5
  + phppass SHA-512
  + phppass MD5
  + Salted HMAC SHA-256
  + Salted PBKDF2 HMAC SHA-256
  + Salted PBKDF2 HMAC SHA-512
+ crack zip files
+ option to edit the wordlist
+ GUI(?) 
