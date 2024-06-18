use serde::{Deserialize, Serialize};

// Technically you can parse json without a struct but a struct makes validation a lot more streamlined

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkerPage {
    pub linked_objects: Vec<LinkedObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkedObject {
    pub identifier: String,
    pub path: String
}

impl LinkedObject {
    pub fn new(identifier: String, path: String) -> LinkedObject {
        LinkedObject {
            identifier,
            path
        }
    }

}

pub fn read_linker() -> Result<LinkerPage, std::io::Error> {
    let linker_file = std::fs::read_to_string(".omit/linker.json")?;

    let linker: LinkerPage = serde_json::from_str(&linker_file)?;
    Ok(linker)
}

pub fn read_linker_raw() -> String {
    std::fs::read_to_string(".omit/linker.json").unwrap()
}

#[cfg(tests)]
mod tests {

    #[test]
    pub fn test_read_linker() {
        let linker = read_linker().unwrap();
        assert_eq!(linker.linked_objects.len(), 1);
    }
}