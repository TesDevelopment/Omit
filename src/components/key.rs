use std::io::Error;
use std::io;

use soft_aes::aes::{aes_dec_ecb, aes_enc_ecb};

pub fn get_key() -> Result<String, io::Error> {
    let mut dot_omit = std::env::current_dir().unwrap();
    dot_omit.push("/.omit");

    dot_omit.try_exists()?;

    match dot_omit.try_exists() {
        Err(e) => Err(e),

        Ok(found) => {
            if !found {
                return Err(Error::new(std::io::ErrorKind::NotFound, "No key found, please run 'omit key' to generate a key"))
            }

            let encryption_key = std::fs::read_to_string(dot_omit.join("/.omit_key"))?;

            Ok(encryption_key)
        }
    }
}

pub fn decrypt_file(file_contents: String, encryption_key: String) -> Result<String, io::Error> {
    let encrypted_content = aes_dec_ecb(file_contents.as_bytes(), encryption_key.as_bytes(), Some("PKCS7"));

    Ok(String::from_utf8(encrypted_content.unwrap()).unwrap())
}

pub fn encrypt_file(file_contents: String, encryption_key: String) -> Result<String, io::Error> {
    let encrypted_content = aes_enc_ecb(file_contents.as_bytes(), encryption_key.as_bytes(), Some("PKCS7"));

    Ok(String::from_utf8(encrypted_content.unwrap()).unwrap())
}
