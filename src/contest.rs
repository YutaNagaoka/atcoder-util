extern crate reqwest;
extern crate scraper;

use crate::file_utils;
use crate::sample_cases::SampleCases;
use reqwest::Url;
use scraper::Html;
use std::io;

/// Struct to contain `Html` and `SampleCases` for each problems.
///
/// If you tell this struct to fetch all problems in a contest, `i`-th element of `problem_htmls`
/// or `problem_sample_cases` will be of `i`-th problem in a contest.
///
/// Or if you tell this struct to fetch a specific problem in a contest, `problem_htmls` and
/// `problem_sample_cases` will have one element of the problem.
pub struct Contest {
    pub problem_htmls: Vec<Html>,
    pub problem_sample_cases: Vec<SampleCases>,
}

impl Contest {
    /// Construct a new `Contest`.
    pub fn new(contest_id: &str, problem_id: Option<&str>) -> Contest {
        let mut contest = Contest {
            problem_htmls: Vec::new(),
            problem_sample_cases: Vec::new(),
        };
        match problem_id {
            Some(problem_id) => {
                contest.fetch_problem_html(contest_id, &problem_id.chars().nth(0).unwrap());
            }
            None => {
                contest.fetch_all_problems_html(contest_id);
            }
        }
        contest
    }

    /// Fetch `Html` of a specific problem based on `contest_id` and `problem_id`.
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

    /// Fetch `Html` of all problems in a contest specified by `contest_id`.
    fn fetch_all_problems_html(&mut self, contest_id: &str) -> &mut Self {
        let alphabets = (b'a'..=b'z').map(|c| c as char).collect::<Vec<char>>();
        for problem_id in alphabets {
            self.fetch_problem_html(contest_id, &problem_id);
        }
        self
    }

    /// Create `SampleCases` for each fetched `Html`.
    pub fn fetch_sample_cases(&mut self) -> &mut Self {
        for html in self.problem_htmls.clone() {
            let sc = SampleCases::new(&html);
            self.problem_sample_cases.push(sc);
        }
        self
    }

    /// Create files of sample case(s) for each problems.
    pub fn create_sample_cases_files(&self, problem_id: Option<&str>) -> Result<(), io::Error> {
        file_utils::create_directory("io_examples".to_string())?;

        if let Some(problem_id) = problem_id {
            file_utils::create_test_files(
                &self.problem_sample_cases[0],
                &problem_id.chars().nth(0).unwrap(),
            )?;
        } else {
            let alphabets = (b'a'..=b'z').map(|c| c as char).collect::<Vec<char>>();
            for (sc, alphabet) in self.problem_sample_cases.iter().zip(alphabets.iter()) {
                file_utils::create_test_files(sc, alphabet)?;
            }
        }

        Ok(())
    }
}

#[test]
fn test_problem_links() {
    ()
}
