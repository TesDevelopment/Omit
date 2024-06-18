use std::{env, fs, path::PathBuf};
use components::linker::read_linker;
use soft_aes::aes::{aes_enc_ecb, aes_dec_ecb};

mod components;
mod commands;
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

    if !PathBuf::from("/.omit").exists() {
        fs::create_dir(".omit").expect("Unable to create .omit directory");
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

        "key" => {
            if args.len() < 3 {
                commands::key::run_key("");
            } else {
                commands::key::run_key(&args[2]);
            }
        },

        "version" => {
            println!("Omit version 0.1.0");
        },

        "ensure" => commands::ensure::run_ensure(),
        
        _ => commands::default::run_default(subcommand),
    }
}
