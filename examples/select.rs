extern crate scraper;

use std::io;

use scraper::{Html, Selector};

static HTML: &'static str = r##"
<!DOCTYPE html>
<title>Hello, world!</title>
<meta charset="utf-8">
<header id="header">
  <h1>Title</h1>
  <p class="tagline">Tagline</p>
  <nav>
    <ul>
      <li><a href="#">Nav Link</a></li>
    </ul>
  </nav>
</header>
<main>
  <p>Content</p>
</main>
"##;

fn main() {
    let html = Html::parse_document(HTML);
    println!("{:#?}", html);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let selector = Selector::parse(&input).unwrap();
    for node in html.select(&selector) {
        println!("{:?}", node.value());
    }
}
