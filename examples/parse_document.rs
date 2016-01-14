extern crate scraper;

use std::io::{self, Read};

use scraper::Html;

fn main() {
    let mut document = String::new();
    io::stdin().read_to_string(&mut document).unwrap();
    let html = Html::parse_document(&document);
    println!("{:#?}", html);
}
