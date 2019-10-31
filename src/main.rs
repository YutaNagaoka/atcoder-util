extern crate reqwest;
extern crate scraper;

use scraper::{Selector, Html};
use reqwest::Url;

fn main() {
    let url = "https://atcoder.jp/contests/abc144/tasks/abc144_e";
    let html = fetch_html(url);
    let doc = Html::parse_document(&(html.unwrap()));
    let selector = Selector::parse("pre").unwrap();
    for node in doc.select(&selector) {
        println!("---One iteration---");
        let text = node.text().collect::<Vec<_>>();
        println!("{:?}", text);
        println!("{:?}", node.value().id());
    }
}

fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let parsed_url = Url::parse(url).unwrap();
    let mut res = reqwest::get(parsed_url).unwrap();
    res.text()
}
