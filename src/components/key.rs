use std::{io::Error};
use std::{fs, io};

use base64::{engine::general_purpose, Engine as _};
use soft_aes::aes::{aes_dec_ecb, aes_enc_ecb};

pub fn get_key() -> Result<Vec<u8>, io::Error> {
    let mut dot_omit = std::env::current_dir()?;
    dot_omit.push(".omit");

    match dot_omit.try_exists() {
        Err(e) => Err(e),

        Ok(found) => {
            if !found {
                return Err(Error::new(std::io::ErrorKind::NotFound, "No key found, please run 'omit key' to generate a key"))
            }

            dot_omit.push(".omit_key");

            let encryption_key = general_purpose::STANDARD.decode(std::fs::read_to_string(dot_omit)?).unwrap();

            Ok(encryption_key)
        }
    }
}

pub fn get_key_base() -> Result<String, io::Error> {
    let mut dot_omit = std::env::current_dir().unwrap();
    dot_omit.push(".omit");

    match dot_omit.try_exists() {
        Err(_) => {
            return Err(Error::new(std::io::ErrorKind::NotFound, "No key found, please run 'omit key' to generate a key"));
        }

        Ok(found) => {
            if !found {
                return Err(Error::new(std::io::ErrorKind::NotFound, "No key found, please run 'omit key' to generate a key"));
            }

            dot_omit.push(".omit_key");

            Ok(fs::read_to_string(dot_omit)?)
        }
    }
}

pub fn decrypt_file(file_contents: String, encryption_key: &Vec<u8>) -> Result<String, io::Error> {
    let debased = general_purpose::STANDARD.decode(file_contents).unwrap();

    let encrypted_content = aes_dec_ecb(&debased, encryption_key, Some("PKCS7"));

    Ok(String::from_utf8(encrypted_content.unwrap()).unwrap())
}

pub fn encrypt_file(file_contents: String, encryption_key: &Vec<u8>) -> Result<String, io::Error> {
    let encrypted_content = aes_enc_ecb(file_contents.as_bytes(), encryption_key, Some("PKCS7"));

    let based = general_purpose::STANDARD.encode(encrypted_content.unwrap());

    Ok(based)
}
