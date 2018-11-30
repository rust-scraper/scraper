use super::Html;
use super::Selector;

#[test]
fn test_html_root_element_ref() {
    let html = Html::parse_fragment(r#"<a href="http://github.com">1</a>"#);
    let root_ref = html.root_element_ref();
    let href = root_ref.select(&Selector::parse("a").unwrap()).next().unwrap();
    assert_eq!(href.inner_html(), "1");
    assert_eq!(href.value().attr("href").unwrap(), "http://github.com");
}