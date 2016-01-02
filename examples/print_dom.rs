extern crate html5ever;
extern crate scraper;
extern crate tendril;

use std::io;

use tendril::{ByteTendril, ReadExt};
use scraper::dom::Dom;

fn main() {
    let mut input = ByteTendril::new();
    io::stdin().read_to_tendril(&mut input).unwrap();
    let input = input.try_reinterpret().unwrap();
    let dom: Dom = html5ever::parse(html5ever::one_input(input), Default::default());

    println!("errors: {:#?}", dom.errors);
    print!("{}", dom.document);
}
