use std::path::PathBuf;
use std::process::Command;

pub fn run_commit() {
    let linker: crate::components::linker::LinkerPage = crate::components::linker::read_linker().unwrap();
    match crate::components::key::get_key() {
        Ok(encryption_key) => {
            for linked_object in linker.linked_objects {
                let file = std::fs::read_to_string(&linked_object.path);
        
                match file {
                    Ok(file) => {
                        let encrypted = crate::components::key::encrypt_file(file, &encryption_key).unwrap();
        
                        let mut path = PathBuf::from(".omit");
                        path.push(&linked_object.identifier);
                
                        std::fs::write(path, encrypted).unwrap(); 
                    },
                    Err(_) => {
                        continue;
                    }
                };  
            }
        
            let output = Command::new("git")
                .arg("add")
                .arg(".omit")
                .output()
                .expect("Failed to stage omit files");
        
        
            if !output.status.success() {
                let error_message = String::from_utf8_lossy(&output.stderr);
                println!("Failed to stage omit files, you're modified secret's won't be commited...: {}", error_message);
            }
        
            println!("Omit Commit complete");
        },
        Err(_) => {
            eprintln!("No key found, please run 'omit key [key]' to generate a key");
        }
    }
}