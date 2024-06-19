use std::{fs, path::PathBuf};
use base64::{engine::general_purpose, Engine as _};

use rand::Rng;

use crate::components::{key::encrypt_file, linker::{read_linker, write_linker}};

fn generate_key() ->  [u8; 32]{
    rand::thread_rng().gen::<[u8; 32]>()
}

pub fn run_key(key: &str) {
    let key_path = ".omit/.omit_key";
    let key_path_buf = PathBuf::from(key_path);

    if key == "" {
        let generated_key = generate_key();
        let output = general_purpose::STANDARD.encode(&generated_key);

        fs::write(&key_path_buf, &output).expect("Unable to write key");

        println!("Modifying linker");
        let mut linker = read_linker().expect("Unable to read linker file");

        linker.key_check = encrypt_file(output, &generated_key.iter().map(|&i|i).collect()).unwrap();

        write_linker(linker).expect("Unable to write linker file");
    } else {
        println!("Using buff");
        fs::write(&key_path_buf, key).expect("Unable to write key");
    }
}