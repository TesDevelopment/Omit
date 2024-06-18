use std::{env, fs, path::PathBuf};

use crate::components::{self, linker::read_linker};

pub fn run_default(subcommand: &str) {
    let mut path_buf = PathBuf::from(subcommand);

    if path_buf.is_relative() {
        path_buf = env::current_dir().unwrap().join(path_buf);
    }


    println!("{:?}", path_buf);
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

                    let linker_page = read_linker().expect("Unable to read linker file");

                    let mut linked_objects = linker_page.linked_objects;
                    let path_str = path_buf.to_str().unwrap();

                    let found_object = linked_objects.iter().find(|linked_object| linked_object.path == path_str);
                    
                    match found_object {
                        Some(found_object) => {
                            let encrypted_path = format!(".omit/{}", found_object.identifier);
                            let encrypted_path_buf = PathBuf::from(encrypted_path);

                            fs::write(&encrypted_path_buf, encrypted_content).expect("Unable to write encrypted file");
                        },

                        None => {

                            let generated_identifier = cuid::cuid2();

                            let encrypted_path = format!(".omit/{}", generated_identifier);
                            let encrypted_path_buf = PathBuf::from(encrypted_path);
                            fs::write(encrypted_path_buf, encrypted_content).expect("Unable to write encrypted file");

                            linked_objects.push(components::linker::LinkedObject::new(generated_identifier, path_str.to_string()));
                        }
                    }

                    
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
}