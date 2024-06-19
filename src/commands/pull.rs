use std::path::PathBuf;

use crate::components::key::{decrypt_file, get_key_base};

pub fn run_pull() {
    let linker = crate::components::linker::read_linker().unwrap();
    match crate::components::key::get_key() {
        Ok(encryption_key) => {

            if(linker.key_check == "") {
                eprintln!("Run 'omit key' before pulling secrets");
                return;
            }

            if decrypt_file(linker.key_check, &encryption_key).unwrap() != get_key_base().unwrap() {
                eprintln!("Key check failed, please ensure you have the correct key");
                return;
            }

            for linked_object in linker.linked_objects {
                let mut path = PathBuf::from(".omit");
                path.push(&linked_object.identifier);
        
                let file = std::fs::read_to_string(&path).unwrap();
        
                // Skip the file if it hasn't changed, could be optimized but this is fine for now
                if file[0..9].to_string() == linked_object.signiture && PathBuf::from(&linked_object.path).exists() {
                    println!("Skipping {} because it has not changed... If this is incorrect run 'omit {}'", &linked_object.path, &linked_object.path);
                    continue;
                }
        
                let decrypted = decrypt_file(file, &encryption_key).unwrap();
        
                std::fs::write(&linked_object.path, decrypted).unwrap();
            }
        
            println!("Omit Pull complete");
        },

        Err(_) => {
            eprintln!("No key found, please run 'omit key [key]' to generate a key");
        }
    }
}