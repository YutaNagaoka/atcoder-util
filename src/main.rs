extern crate reqwest;
extern crate scraper;

use reqwest::Url;
use scraper::{Html, Selector};

fn main() {
    let url = "https://atcoder.jp/contests/abc144/tasks/abc144_e";
    let sample_cases = parse_sample_case(url);
    for case in sample_cases {
        println!("{}", case);
    }
}

fn parse_sample_case(url: &str) -> Vec<String> {
    let result = fetch_html(url);
    let html = Html::parse_document(&(result.unwrap()));

    let selector_lang_ja = Selector::parse("span.lang-ja").unwrap();
    let html_lang_ja = html.select(&selector_lang_ja).nth(0).unwrap().html();
    let selector_sample_case = Selector::parse("pre").unwrap();
    let html_sample_case = Html::parse_fragment(&html_lang_ja);
    let elem_sample_case = html_sample_case.select(&selector_sample_case);

    let mut sample_cases: Vec<String> = Vec::new();
    for node in elem_sample_case {
        // Skip element of input example.
        if node.children().count() > 1 {
            continue;
        }
        let text = node.text().collect::<Vec<&str>>().join("");
        sample_cases.push(text);
    }
    sample_cases
}

fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let parsed_url = Url::parse(url).unwrap();
    let mut res = reqwest::get(parsed_url).unwrap();
    res.text()
}
