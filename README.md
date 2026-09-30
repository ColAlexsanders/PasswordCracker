A TUI password cracker written in Rust that does dictionary attacks on argon2, bcrypt, and sha256 hashes 

# Requirements
+ Latest version of Cargo

# Dependencies
+ cargo
+ rockyou.txt

# Running The Program
```bash
git clone https://github.com/ColAlexsanders/PasswordCracker.git
cd PasswordCracker
cargo run
```

# My plans for this:
+ support for 
  + Salted MD5 []
  + phppass SHA-512 []
  + phppass MD5 []
  + Salted HMAC SHA-256 []
  + Salted PBKDF2 HMAC SHA-256 []
  + Salted PBKDF2 HMAC SHA-512 []
+ brute force cracking []
+ crack zip files []
+ option to pass in wordlists as an argument []
