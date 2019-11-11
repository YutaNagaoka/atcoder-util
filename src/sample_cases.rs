extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};


pub struct SampleCases {
    pub input: Vec<String>,
    pub output: Vec<String>,
}

impl SampleCases {
    pub fn new(html: &Html) -> SampleCases {
        let mut sc = SampleCases { input: Vec::new(), output: Vec::new() };
        let io_examples = SampleCases::parse_io_examples(html);
        sc.extract_io_example(io_examples);
        sc
    }

    fn parse_io_examples(html: &Html) -> Vec<String> {
        let selector_lang_ja = Selector::parse("span.lang-ja").unwrap();
        let selector_io_example = Selector::parse("pre").unwrap();

        let html_lang_ja = html.select(&selector_lang_ja).nth(0).unwrap().html();
        let html_io_example = Html::parse_fragment(&html_lang_ja);

        let io_examples: Vec<String> = html_io_example.select(&selector_io_example)
            .filter(|example| example.children().count() == 1)
            .map(|example| example.text().collect::<Vec<&str>>().join(""))
            .collect();

        io_examples
    }

    pub fn extract_io_example(&mut self, io_examples: Vec<String>) {
        for (i, io_example) in io_examples.iter().enumerate() {
            // IO example of even index is input.
            if i % 2 == 0 {
                self.input.push(io_example.to_string());
            }
            else {
                self.output.push(io_example.to_string());
            }
        }
    }
}
