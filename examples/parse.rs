extern crate scraper;

use std::io::{self, Read};

use scraper::Html;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let html = Html::parse(&input);
    //println!("{:#?}", html);

    println!("{:#?}", html.css("div"));
}
