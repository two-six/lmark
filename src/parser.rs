use std::collections::HashMap;

pub fn parse() -> HashMap<String, (String, u32)> {
    let mut x1 = HashMap::new();
    x1.insert("another text".to_owned(), ("text".to_owned(), 32));
    x1
}
