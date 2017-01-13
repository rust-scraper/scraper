extern crate scraper;
use scraper::html::Html;
use scraper::selector::Selector;
#[test]
fn test_html_to_text() {
    let html = Html::parse_document("<html>
        <p>p1</p>
        <script>script 1</script>
        <div>
            <script>script 2</script>
        </div>
        <p>p2</p>
    </html>");
    let selector = Selector::parse("html").expect("Unable to parse css selector");
    let elements = html.select(
        &selector
    );
    for element in elements {        
        let s: String = element.content(Default::default()).into();
        assert_eq!("p1\n        \n        \n            \n        \n        p2\n    ", s);
    }
}
