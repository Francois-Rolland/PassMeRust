use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;

// Importing the entropy module
mod entropy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut password = String::new();

    if args.len() > 2 {
        println!("Usage: password-strength [password]");
        process::exit(1);
    }

    if args.len() == 2 {
        // Password provided as an argument
        password = args[1].clone();
    } else {
        // No argument: either prompt (interactive) or read piped stdin
        if atty::is(atty::Stream::Stdin) {
            // interactive terminal -> prompt
            print!("Enter a password: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut password).unwrap();
        } else {
            // stdin is piped -> read entire stdin
            io::stdin().read_to_string(&mut password).unwrap();
        }
    }

    let password = password.trim();

    if password.is_empty() {
        println!("Password cannot be empty.");
        process::exit(1);
    }

    let dictionary_path = Path::new("data/dictionary.txt");
    let common_passwords_path = Path::new("data/common-passwords.txt");

    if is_password_in_dictionary(password, dictionary_path) || is_password_common(password, common_passwords_path) {
        println!("Password is weak: It matches a common password or dictionary word.");
    } else {
        // Use the imported calculate_entropy function
        let entropy = entropy::calculate_entropy(password);
        println!("Password strength: {}", entropy::get_strength(entropy));
    }
}

fn is_password_in_dictionary(password: &str, dictionary_path: &Path) -> bool {
    let dictionary = fs::read_to_string(dictionary_path).unwrap_or_default();
    dictionary.lines().any(|line| line == password)
}

fn is_password_common(password: &str, common_passwords_path: &Path) -> bool {
    let common_passwords = fs::read_to_string(common_passwords_path).unwrap_or_default();
    common_passwords.lines().any(|line| line == password)
}
