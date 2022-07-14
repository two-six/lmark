mod parser;

use std::collections::HashMap;

fn main() {
    let mut categories = Vec::new();
    categories.push(HashMap::from([
        (
            "Expense::Grocery::Walmart".to_uppercase().to_owned(),
            HashMap::from([
                ("EUR".to_uppercase().to_owned(), 107.25),
                ("USD".to_uppercase().to_owned(), 20.3),
            ]),
        ),
        (
            "Assets::Bank::Bank of america".to_uppercase().to_owned(),
            HashMap::from([("USD".to_uppercase().to_owned(), 3450.0)]),
        ),
    ]));
    println!("{:#?}", categories);
}

#[allow(dead_code)]
fn get_vars(text: &str) -> HashMap<String, HashMap<String, String>> {
    let mut x1 = HashMap::new();
    let x2 = HashMap::new();
    x1.insert(text.to_owned(), x2);
    x1
}

// [category] 16.07.2022
// - [ ] phone - (300 USD)
// - [ ]
// [.]
//
// [expenses.phone] - (300 USD)
// [assets.bank]
//
//
//
