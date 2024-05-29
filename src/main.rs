use openssl::cipher::Cipher;
use std::fs::File;
use std::io::{self, Read};

fn encrypt() {
    println!("encrypting files");
    let key = aes_256_cbc();
    println!("{}", key);
}

fn decrypt() {
    println!("decrypting files");
}

fn main() {
    println!("encrypt | decrypt");
    let mut input = String::new();

    let file = File::open("trucy_config.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return;
        }
    };

    let mut config = String::new();

    io::stdin().read_line(&mut input).expect("FIREE");

    match file.read_to_string(&mut config) {
        Ok(_) => {
            println!("dumping config:");
            println!("{}", config)
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }
    }

    println!("selected: {}", input.trim());

    if input == "encrypt" {
        if config == "status: masked" {
            println!("already encrypted, if you wish to encrypt with a different key, decrypt then encrypt again");
        } else {
            encrypt();
        }
    } else if input == "decrypt" {
        if config == "status: plain" {
            println!("already plain");
        } else {
            decrypt();
        }
        // USE OPENSSL https://docs.rs/openssl/latest/openssl/
    } else {
        println!("that- that's not an option... | usage: encrypt OR decrypt");
    }
}
