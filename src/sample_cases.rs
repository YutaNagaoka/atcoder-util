extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};
use std::fs;
use std::path::PathBuf;

/// Struct to contain input/output examples of a problem.
pub struct SampleCases {
    pub input: Vec<String>,
    pub output: Vec<String>,
}

impl SampleCases {
    /// Construct a new `SampleCases`.
    pub fn new(html: &Html) -> SampleCases {
        let mut sc = SampleCases {
            input: Vec::new(),
            output: Vec::new(),
        };
        let io_examples = SampleCases::parse_io_examples(html);
        sc.extract_io_example(io_examples);
        sc
    }

    /// Construct a new `SampleCases` from input/output example files.
    /// This method is called in `atcoder-util test`.
    pub fn new_from_files(problem_id: &str) -> Self {
        let mut sc = SampleCases {
            input: Vec::new(),
            output: Vec::new(),
        };

        let mut dir_io_examples = PathBuf::new();
        dir_io_examples.push(format!("io_examples/{}", problem_id));

        for dir_res in fs::read_dir(dir_io_examples).unwrap() {
            let dir_name = dir_res.unwrap().path();
            // Sort i/o example files because an order of `fs::read_dir()` returns directory depends on environment.
            let mut dir_sample_case: Vec<_> = fs::read_dir(&dir_name)
                .unwrap()
                .map(|d| d.unwrap())
                .collect();
            dir_sample_case.sort_by_key(|dir| dir.path());

            for file in dir_sample_case {
                let file_content = fs::read_to_string(file.path()).unwrap();
                if dir_name.ends_with(&format!("{}_input", problem_id)) {
                    sc.input.push(file_content);
                } else {
                    sc.output.push(file_content);
                }
            }
        }
        sc
    }

    /// Find elements of i/o examples in html and get them.
    fn parse_io_examples(html: &Html) -> Vec<String> {
        let selector_lang_ja = Selector::parse("span.lang-ja").unwrap();
        let selector_io_example = Selector::parse("pre").unwrap();

        let html_lang_ja = html.select(&selector_lang_ja).nth(0).unwrap().html();
        let html_io_example = Html::parse_fragment(&html_lang_ja);

        let io_examples: Vec<String> = html_io_example
            .select(&selector_io_example)
            .filter(|example| example.children().count() == 1)
            .map(|example| example.text().collect::<Vec<&str>>().join(""))
            .collect();

        io_examples
    }

    /// Push i/o examples to vector in a struct itself.
    fn extract_io_example(&mut self, io_examples: Vec<String>) {
        for (i, io_example) in io_examples.iter().enumerate() {
            // IO example of even index is input.
            if i % 2 == 0 {
                self.input.push(io_example.to_string());
            } else {
                self.output.push(io_example.to_string());
            }
        }
    }
}
