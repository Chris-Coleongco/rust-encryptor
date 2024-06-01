use openssl::symm::{decrypt, encrypt, Cipher};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

fn encrypt_handler(algorithm: Cipher, key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    println!("encrypting files");
    encrypt(algorithm, key, Some(iv), plaintext).expect("Encryption failed")
}

fn decrypt_handler(algorithm: Cipher, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    println!("decrypting files");
    let cipher = Cipher::aes_128_cbc();
    decrypt(algorithm, key, Some(iv), ciphertext).expect("Decryption failed")
}

fn read_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("you are a FAILUER");

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("failed to read");

    return buffer;
}

fn write_file(text: Vec<u8>, destination: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // Truncate the file before writing
        .open(destination)
        .expect("Failed to open file for writing");
    file.write(&text).expect("Failed to write data to file");
}

fn main() {
    println!("encrypt | decrypt");

    let mut input = String::new();

    let key = b"0915de3caa66326168b39437ca958196";

    let iv = b"1760474e6a75254a";

    let conf_file = File::open("trucy_config.txt");

    let mut file = match conf_file {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error reading fileee: {}", error);
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

    let algo = Cipher::aes_256_cbc();

    let file2 = read_file("dumdum");

    if input.trim() == "encrypt" {
        if config == "status: masked" {
            println!("already encrypted, if you wish to encrypt with a different key, decrypt then encrypt again");
        } else {
            let ciphertext = encrypt_handler(algo, key, iv, &file2);

            write_file(ciphertext, "dumdum");
            let update_conf = "status: masked";
            write_file(update_conf.as_bytes().to_vec(), "trucy_config.txt")
        }
    } else if input.trim() == "decrypt" {
        if config == "status: plain" {
            println!("already plain");
        } else {
            let plaintext = decrypt_handler(algo, key, iv, &file2);
            write_file(plaintext, "dumdum");
            let update_conf = "status: plain";
            write_file(update_conf.as_bytes().to_vec(), "trucy_config.txt")
        }
        // USE OPENSSL https://docs.rs/openssl/latest/openssl/
    } else {
        println!("that- that's not an option... | usage: encrypt OR decrypt");
    }
}
