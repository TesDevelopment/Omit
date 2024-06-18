use crate::components::{key::decrypt_file, linker::{write_linker, LinkerPage}};

pub fn run_pull() {
    let linker = crate::components::linker::read_linker().unwrap();
    let encryption_key = crate::components::key::get_key().unwrap();

    for linked_object in linker.linked_objects {
        let file = std::fs::read_to_string(&linked_object.path).unwrap();

        // Skip the file if it hasn't changed, could be optimized but this is fine for now
        if file[0..9].to_string() == linked_object.signiture {
            continue;
        }

        let decrypted = decrypt_file(file, &encryption_key).unwrap();
        std::fs::write(&linked_object.path, decrypted).unwrap();
    }

    println!("Pull complete");
}