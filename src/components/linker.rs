use serde::{Deserialize, Serialize};

use super::key::{encrypt_file, get_key};

// Technically you can parse json without a struct but a struct makes validation a lot more streamlined

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkerPage {
    pub linked_objects: Vec<LinkedObject>,
    pub key_check: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkedObject {
    pub identifier: String,
    pub path: String,
    pub signiture: String,
}

impl LinkedObject {
    pub fn new(identifier: String, path: String,signiture: String) -> LinkedObject {
        LinkedObject {
            identifier,
            path,
            signiture
        }
    }

}
pub fn write_linker(linker: LinkerPage) -> Result<(), std::io::Error> {
    let linker_file = serde_json::to_string(&linker)?;
    std::fs::write(".omit/linker.json", linker_file)
}

pub fn read_linker() -> Result<LinkerPage, std::io::Error> {
    let linker_file = std::fs::read_to_string(".omit/linker.json");

    match linker_file {
        Err(_) => {
            match super::key::get_key_base() {
                Ok(encryption_key_base) => {
                    let linker = LinkerPage {
                        linked_objects: Vec::new(),
                        key_check: encrypt_file(encryption_key_base, &get_key()?)?
                    };

                    write_linker(linker.clone())?;

                    Ok(linker)
                },
                Err(e) => return Err(e)
            }
            
        }
        file => {
            let linker: LinkerPage = serde_json::from_str(&file.unwrap())?;
            Ok(linker)
        }
    }

    
}