extern crate scraper;

use std::io::{self, Read};

use scraper::Html;

fn main() {
    let mut fragment = String::new();
    io::stdin().read_to_string(&mut fragment).unwrap();
    let html = Html::parse_fragment(&fragment);
    println!("{:#?}", html);
}
