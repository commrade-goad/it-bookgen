use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub synopsys: String,
    pub path: String,
    pub image: String,
    pub tag: Vec<String>,
    pub rating: u8,
}

impl Book {
    pub fn parse_json(data: &str) -> Option<Vec<Book>> {
        let parsed: Vec<Book> = match serde_json::from_str(data) {
            Ok(val) => val,
            Err(_) => return None,
        };
        return Some(parsed);
    }

    pub fn to_json(self) -> Option<String> {
        match serde_json::to_string(&self) {
            Ok(val) => return Some(val),
            Err(_) => return None,
        };
    }

    pub fn vec_to_json(obj: &Vec<Book>) -> Option<String> {
        match serde_json::to_string(&obj) {
            Ok(val) => return Some(val),
            Err(_) => return None,
        };
    }
}
