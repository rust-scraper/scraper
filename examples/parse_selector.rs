extern crate scraper;

use std::io::{self, Read};

use scraper::Selector;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let selector = Selector::parse(&input);
    println!("{:#?}", selector);
}
