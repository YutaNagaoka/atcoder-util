extern crate reqwest;
extern crate scraper;

use crate::sample_cases::{ SampleCases };
use scraper::Html;
use reqwest::Url;
use crate::file_utils;


pub struct Contest {
    pub problem_htmls: Vec<Html>,
    pub problem_sample_cases: Vec<SampleCases>,
}

impl Contest {
    pub fn new(contest_id: &str) -> Contest {
        let mut c = Contest{ problem_htmls: Vec::new(), problem_sample_cases: Vec::new() };
        c.fetch_problem_url(contest_id);
        c
    }

    fn fetch_problem_url(&mut self, contest_id: &str) -> &mut Self {
        for problem_id in 'a' as u8..='z' as u8 {
            let url =
                format!("https://atcoder.jp/contests/{}/tasks/{}_{}", contest_id, contest_id, problem_id as char);
            let parsed_url = Url::parse(&url).unwrap();
            let mut res = reqwest::get(parsed_url).unwrap();
            match res.status().as_u16() {
                200 => {
                    let html_text = res.text().unwrap();
                    self.problem_htmls.push(Html::parse_document(&html_text));
                },
                _ => break,
            }
        }
        self
    }

    pub fn fetch_sample_cases(&mut self) -> &mut Self {
        for html in self.problem_htmls.clone() {
            let sc = SampleCases::new(&html);
            self.problem_sample_cases.push(sc);
        }
        self
    }

    pub fn create_sample_cases_files(&self) {
        let alphabets = (b'a' ..= b'z')
            .map(|c| c as char)
            .collect::<Vec<char>>();
        for (sc, alphabet) in self.problem_sample_cases.iter().zip(alphabets.iter()) {
            file_utils::create_test_files(sc, alphabet)
                .expect("Failed to create files of sample cases.");
        }
    }
}

#[test]
fn test_problem_links() {
    ()
}
