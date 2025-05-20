/*  PasswordCracker v0.0.1 */
/* Written by: ColAlexsanders */

use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Instant;
use std::fs::File;
use sha2::{Sha256, Digest};
use bcrypt::verify;
use std::io::{self, BufRead, Write};
use std::path::Path;
//use zip::ZipArchive;
//use itertools::Itertools;

/* ---------------------------------------- FUNCTIONS FOR DICTIONARY ATTACK FEATURE --------------------------------------- */
 
fn read_from_list(file_path: &Path) -> Result<Vec<String>, io::Error> {
    let file = File::open(Path::new(file_path)).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(e) => eprintln!("Invalid UTF-8 data encountered: {}", e),
        }
    }
    Ok(lines)
}

fn compare_passwords_sha256(word: &str, target_hash: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(word);
    let result = hasher.finalize();
    let hex_result = format!("{:x}", result);
    hex_result == target_hash
}

fn compare_passwords_bcrypt(word: &str, target_hash: &str) -> bool {
    verify(word, target_hash).unwrap_or(false)
}

fn read_target_hash_sha256(prompt: &str, blacklist_sha256: &[char]) -> io::Result<String> {
    loop{
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new(); 
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();

        if has_blacklisted_chars(&input, blacklist_sha256) {
            println!("Input contains invalid characters. SHA256 entries don't contain some of the characters you have entered:\n \n {:?} \n \nThey either contain numbers 0-9 or lowercase letters. \n \n", blacklist_sha256);
            continue;
        }

        return Ok(input);
    }
}

fn read_target_hash_bcrypt(prompt: &str, blacklist_bcrypt: &[char]) -> io::Result<String> {
    loop{
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new(); 
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();

        if has_blacklisted_chars(&input, blacklist_bcrypt) {
            println!("Input contains invalid characters. Bcrypt entries don't contain some of the characters you have entered: \n \n {:?} \n \nThey either contain numbers 0-9, USD signs ($), uppercase/lowercase letters, and a period to separate the salt value and the hash. \n \n", blacklist_bcrypt);
            continue;
        }

        return Ok(input);
    }
}

fn has_blacklisted_chars(input: &str, blacklist: &[char]) -> bool {
    for c in input.chars() {
        if blacklist.contains(&c) {
            return true;
        }
    }
    false
}

/*fn edit_wordlist() {} */

/* ---------------------------------------- MENU CONFIG --------------------------------------- */

fn display_menu() -> io::Result<usize> {
    println!("Select an option:");
    println!("1. Crack SHA256 hash");
    println!("2. Crack bcrypt hash");
/*    println!("3. Crack zip file with dictionary attack");
    println!("4. Crack zip file with brute force attack"); */
    println!("100. Exit Program");
    print!("Enter your choice: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    match input.trim().parse::<usize>() {
        Ok(100) => {
            println!("Exiting program...");
            std::process::exit(0);
        },
        Ok(choice) if choice >= 1 && choice <= /*4*/ 2 => Ok(choice),
        _ => {
            eprintln!("Invalid choice. Please enter a number between 1 and 2 or the exit option.");
            display_menu()
        }
    }
}

/*fn crack_zip_with_dictionary(file_path: &Path, passwords: &[String]) -> Option<String> {
    let file = File::open(file_path).expect("Unable to open zip file");
    let mut archive = ZipArchive::new(file).expect("Failed to create archive");

    for password in passwords {
        if let Ok(mut _file) = archive.by_index_decrypt(0, password.as_bytes()) {
            println!("Found password: {}", password);
            return Some(password.clone());
        }
    }
    None
}*/

/*fn crack_zip_with_brute_force(file_path: &Path, charset: &[char], max_length: usize) -> Option<String> {
    let file = File::open(file_path).expect("Unable to open zip file");
    let mut archive = ZipArchive::new(file).expect("Failed to create archive");

    for length in 1..=max_length {
        for password in charset.iter().combinations_with_replacement(length) {
            let password: String = password.into_iter().collect();
            if let Ok(mut _file) = archive.by_index_decrypt(0, password.as_bytes()) {
                println!("Found password: {}", password);
                return Some(password);
            }
        }
    }
    None
}*/

/* ---------------------------------------- MAIN FUNCTION --------------------------------------- */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("rockyou.txt");
    let blacklist_sha256 = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=', '[', ']', '{', '}', ':', ';', '|', '"', ',', '.', '/', '~', '`', '>', '<', '?', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let blacklist_bcrypt = vec!['!', '@', '#', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=', '[', ']', '{', '}', ':', ';', '|', '"', ',', '~', '`', '>', '<', '?'];
    let choice = display_menu()?;
    let target_hash: String;
    let passwords: Vec<String>;
    
    match choice {
        1 => {
            target_hash = read_target_hash_sha256("Enter target hash: ", &blacklist_sha256)?;
            passwords = match read_from_list(file_path) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error reading wordlist: {}", e);
                    return Ok(());
                }
            };
        }    
        2 => {
            target_hash = read_target_hash_bcrypt("Enter target hash: ", &blacklist_bcrypt)?;
            passwords = match read_from_list(file_path) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error reading wordlist: {}", e);
                    return Ok(());
                }
            };
        }
        _ => {
            eprintln!("Invalid choice");
            return Ok(());
        }
    }
    
    // Now set up threading and processing (outside of the match statement)
    let start_time = Instant::now();
    let num_threads = 8;
    let chunk_size = passwords.len() / num_threads;
    let found_password = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];
    
    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 { 
            passwords.len() 
        } else { 
            start + chunk_size 
        };
        
        let words = passwords[start..end].to_vec();
        let target_hash_clone = target_hash.clone();
        let choice_clone = choice;
        let found_password_clone = Arc::clone(&found_password);
        
        let handle = thread::spawn(move || {
            for word in &words {
                if found_password_clone.load(Ordering::SeqCst) {
                    break;
                }
                
                match choice_clone {
                    1 => {
                        if compare_passwords_sha256(word, &target_hash_clone) {
                            println!("Found password: {}", word);
                            found_password_clone.store(true, Ordering::SeqCst);
                        }
                    }
                    2 => {
                        if compare_passwords_bcrypt(word, &target_hash_clone) {
                            println!("Found password: {}", word);
                            found_password_clone.store(true, Ordering::SeqCst);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let elapsed = start_time.elapsed();
    println!("Time taken: {:.2?}", elapsed);
    
    Ok(())
}   
        /*3 | 4 => {
            print!("Please enter the path to the zip file: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let zip_path = Path::new(input.trim_end());

            if choice == 3 {
                let passwords = match read_from_list(file_path) {
                    Ok(passwords) => passwords,
                    Err(e) => {
                        eprintln!("Error reading wordlist: {}", e);
                        return Ok(())
                    }
                };
                let start_time = Instant::now();
                if let Some(password) = crack_zip_with_dictionary(zip_path, &passwords) {
                    println!("Found password: {}", password);
                } else {
                    println!("Password not found.");
                }
                let elapsed = start_time.elapsed();
                println!("Time taken: {:.2?}", elapsed);
            } else {
                print!("Please enter the maximum length of the brute force attack: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let max_length: usize = input.trim_end().parse().expect("Invalid number");

                let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+}{:?></-=][{}}];'~`'|".chars().collect();
                let start_time = Instant::now();
                /*if let Some(password) = crack_zip_with_brute_force(zip_path, &charset, max_length) {
                    println!("Found password: {}", password);
                } else {
                    println!("Password not found.");
                }*/
                let elapsed = start_time.elapsed();
                println!("Time taken: {:.2?}", elapsed);
            } */
