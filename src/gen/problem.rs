extern crate reqwest;
extern crate scraper;

use crate::gen::file_utils;
use crate::sample_cases::SampleCases;
use reqwest::Url;
use scraper::Html;
use std::io;


/// Contains information of a problem in a contest.
pub struct Problem<'a> {
    pub contest_id: &'a str,
    pub problem_id: &'a str,
    pub sample_cases: SampleCases,
}

impl<'a> Problem<'a> {
    /// Create a new instance.
    pub fn new(contest_id: &'a str, problem_id: &'a str) -> Option<Problem<'a>> {
        let sc = Problem::fetch_sample_cases(contest_id, problem_id);
        if let Some(sc) = sc {
            return Some(Problem {
                contest_id,
                problem_id,
                sample_cases: sc,
            });
        }
        None
    }

    /// Create `SampleCases` of a specific problem.
    fn fetch_sample_cases(contest_id: &'a str, problem_id: &'a str) -> Option<SampleCases> {
        let html = Problem::fetch_problem_html(contest_id, problem_id);
        if let Some(html) = html {
            let sc = SampleCases::new(&html);
            return Some(sc);
        }
        None
    }

    /// Fetch `Html` of a problem.
    fn fetch_problem_html(contest_id: &'a str, problem_id: &'a str) -> Option<Html> {
        let url = format!(
            "https://atcoder.jp/contests/{}/tasks/{}_{}",
            contest_id, contest_id, problem_id
        );
        let parsed_url = Url::parse(&url).unwrap();
        let mut res = reqwest::get(parsed_url).unwrap();

        if res.status().as_u16() == 200 {
            let html_text = res.text().unwrap();
            let document = Html::parse_document(&html_text);
            return Some(document);
        }
        None
    }

    /// Create files which contains information of sample cases.
    pub fn create_sample_cases_files(&self) -> Result<(), io::Error> {
        file_utils::create_directory("io_examples".to_string())?;
        file_utils::create_test_files(&self.sample_cases, self.problem_id)?;
        Ok(())
    }
}
