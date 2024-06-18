use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct LinkerPage {
    linked_objects: Vec<LinkedObject>,
}


struct LinkedObject {
    identifier: String,
    path: String
}