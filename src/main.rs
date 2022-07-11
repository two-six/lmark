mod parser;

use std::collections::HashMap;

use comrak::{markdown_to_html, ComrakOptions};

fn main() {
    println!(
        "{}",
        markdown_to_html("Hello, **世界**!", &ComrakOptions::default())
    );
    println!("{}", parser::parse()["another text"].0);
}

#[allow(dead_code)]
fn get_vars(text: &str) -> HashMap<String, HashMap<String, String>> {
    let mut x1 = HashMap::new();
    let x2 = HashMap::new();
    x1.insert(text.to_owned(), x2);
    x1
}

// [category]
// - [ ] phone - ($300)
// - [ ]
// [.]
//
//
//
//
//
//
