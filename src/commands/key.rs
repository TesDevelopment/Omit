use std::{fs, path::PathBuf};

use rand::Rng;

fn generate_key() ->  [u8; 32]{
    rand::thread_rng().gen::<[u8; 32]>()
}

pub fn run_key(key: &str) {
    let key_path = ".omit/key";
    let key_path_buf = PathBuf::from(key_path);

    if key == "" {
        let key = generate_key();
        fs::write(&key_path_buf, key).expect("Unable to write key");
    } else {
        fs::write(&key_path_buf, key).expect("Unable to write key");
    }
}