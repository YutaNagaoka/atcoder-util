pub mod file_utils;
pub mod problem;

use problem::Problem;
use std::io;

/// Fetch a specific problem in a contest.
pub fn execute_fetching_problem(contest_id: &str, problem_id: &str) -> Result<(), io::Error> {
    let problem = Problem::new(contest_id, &problem_id);
    if let Some(problem) = problem {
        problem.create_sample_cases_files()?;
    }
    Ok(())
}

/// Fetch all problem in a contest.
pub fn execute_fetching_problems_in_contest(contest_id: &str) -> Result<(), io::Error> {
    let alphabets = (b'a'..=b'z').map(|c| c as char).collect::<Vec<char>>();
    for alphabet in alphabets {
        let problem_id = format!("{}", alphabet);
        execute_fetching_problem(contest_id, &problem_id)?;
    }
    Ok(())
}
