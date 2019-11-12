extern crate reqwest;
extern crate scraper;

use crate::file_utils;
use crate::sample_cases::SampleCases;
use reqwest::Url;
use scraper::Html;

pub struct Contest {
    pub problem_htmls: Vec<Html>,
    pub problem_sample_cases: Vec<SampleCases>,
}

impl Contest {
    pub fn new(contest_id: &str, problem_id: Option<&str>) -> Contest {
        let mut c = Contest {
            problem_htmls: Vec::new(),
            problem_sample_cases: Vec::new(),
        };
        match problem_id {
            Some(problem_id) => {
                c.fetch_problem_html(contest_id, &problem_id.chars().nth(0).unwrap());
            }
            None => {
                c.fetch_all_problems_html(contest_id);
            }
        }
        c
    }

    fn fetch_problem_html(&mut self, contest_id: &str, problem_id: &char) -> &mut Self {
        let url = format!(
            "https://atcoder.jp/contests/{}/tasks/{}_{}",
            contest_id, contest_id, problem_id
        );
        let parsed_url = Url::parse(&url).unwrap();
        let mut res = reqwest::get(parsed_url).unwrap();
        if res.status().as_u16() == 200 {
            let html_text = res.text().unwrap();
            self.problem_htmls.push(Html::parse_document(&html_text));
        }
        self
    }

    fn fetch_all_problems_html(&mut self, contest_id: &str) -> &mut Self {
        let alphabets = (b'a'..=b'z').map(|c| c as char).collect::<Vec<char>>();
        for problem_id in alphabets {
            self.fetch_problem_html(contest_id, &problem_id);
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

    pub fn create_sample_cases_files(&self, problem_id: Option<&str>) {
        if let Some(problem_id) = problem_id {
            file_utils::create_test_files(
                &self.problem_sample_cases[0],
                &problem_id.chars().nth(0).unwrap(),
            )
            .expect("Failed to create files of sample cases.");
        } else {
            let alphabets = (b'a'..=b'z').map(|c| c as char).collect::<Vec<char>>();
            for (sc, alphabet) in self.problem_sample_cases.iter().zip(alphabets.iter()) {
                file_utils::create_test_files(sc, alphabet)
                    .expect("Failed to create files of sample cases.");
            }
        }
    }
}

#[test]
fn test_problem_links() {
    ()
}
