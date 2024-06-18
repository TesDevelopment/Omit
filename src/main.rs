use std::{env, fs, path::PathBuf};
use soft_aes::aes::{aes_enc_ecb, aes_dec_ecb};

mod components;
/*
fn get_random_key32() ->  [u8; 32]{
    let mut arr = [0u8; 32];
    thread_rng().try_fill(&mut arr[..]).expect("Ooops!");
    return arr;
} */
fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: omit <command>, use omit help for more information");
        return;
    }

    let subcommand = &args[1];

    match subcommand.as_str() {
        "help" => {
            println!("!! Omit is a program built for integration into your git pipline, bin installation is required for that use case !!");
            println!("omit pull: Decrypts the secrets and writes them to the file system (Automatically done on git pull if hooks are installed)");
            println!("omit commit: Encrypts secrets and writes them to the .omit directory (Automatically done on git commit if hooks are installed)");

            println!("omit ensure: Writes all secrets to your gitignore file");
            println!("omit help: Displays this message");
            println!("omit version: Displays the version of omit installed");

            println!("omit init: Initalizes omit hooks if the directory is home to a repository");
            println!("omit key [key]: Generates a new key for the repository or implements the provided key");

            println!("omit <file_path>");
        },
        
        _ => {
            let mut path_buf = PathBuf::from(subcommand);

            if path_buf.is_relative() {
                path_buf = env::current_dir().unwrap().join(path_buf);
            }

            match path_buf.try_exists() {
                Ok(exists) => {

                    if !exists {
                        eprintln!("Expected subcommand or file, found '{}' instead", subcommand);
                        return;
                    }

                    if path_buf.is_dir() {
                        eprintln!("Expected subcommand or file, found '{}' instead", subcommand);
                        return;
                    }

                    let file_contents = fs::read_to_string(&path_buf).expect("Unable to read file");
                    
                    let encryption_key = components::key::get_key();

                    match encryption_key {
                        Ok(encryption_key) => {
                            let encrypted_content = components::key::encrypt_file(file_contents, encryption_key).expect("Unable to encrypt file");

                            fs::wri
                        },
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    
                    }
                },
                Err(_) => {
                    eprintln!("Expected subcommand or file, found '{}' instead", subcommand);
                }
            }
        },
    }
}
