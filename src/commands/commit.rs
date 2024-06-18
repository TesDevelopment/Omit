use std::path::PathBuf;

pub fn run_commit() {
    let linker: crate::components::linker::LinkerPage = crate::components::linker::read_linker().unwrap();
    let encryption_key = crate::components::key::get_key().unwrap();

    for linked_object in linker.linked_objects {
        let file = std::fs::read_to_string(&linked_object.path).unwrap();
        let encrypted = crate::components::key::encrypt_file(file, &encryption_key).unwrap();

        let mut path = PathBuf::from(".omit");
        path.push(&linked_object.identifier);

        std::fs::write(path, encrypted).unwrap();
    }

    println!("Commit complete");
}