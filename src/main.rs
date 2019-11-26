//! **atcoder-util** make it easy to test for your code in sample cases in competitive programming.
//!
//! ## Gen
//!
//! First, you have to fetch input/output example from AtCoder's website.
//! If you provide only contest name, sample cases of all problem in the contest will be downloaded.
//!
//! ```
//! $ atcoder-util gen abc145
//! ```
//!
//! Or if you specify a contest name and a problem, sample cases of the problem will be downloaded.
//! ```
//! $ atcoder-util gen abc145 c
//! ```
//!
//! ## Test
//!
//! After you fetched sample cases, you can run check whether its answer is correct in all at once.
//!
//! ```
//! $ atcoder-util test c
//! ```
//!

#[macro_use]
extern crate clap;
extern crate atcoder_util;

use atcoder_util::gen;
use atcoder_util::tester;
use clap::{App, Arg, SubCommand};
use std::io;

fn main() -> Result<(), io::Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("gen")
                .about("Generate input/output format example fetched from AtCoder's website.")
                .arg(
                    Arg::with_name("contest name")
                        .help("Specify which contest to fetch.")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("problem id")
                        .help("Specify which problem to fetch when a contest name is set.")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("Check whether your code will output right answer for each sample cases.")
                .arg(
                    Arg::with_name("problem id")
                        .help("Specify which problem to test.")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    // Fetch input/output examples and write each of them into text files.
    if let Some(ref matches) = matches.subcommand_matches("gen") {
        let contest_id = matches.value_of("contest name");
        let problem_id = matches.value_of("problem id");

        // Problem is specified (such as "a", "b", "c"...).
        match (contest_id, problem_id) {
            (Some(contest_id), Some(problem_id)) => {
                gen::execute_fetching_problem(contest_id, &problem_id)?;
            }
            (Some(contest_id), None) => {
                gen::execute_fetching_problems_in_contest(contest_id)?;
            }
            (_, _) => {}
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("test") {
        let problem_id = matches.value_of("problem id");
        if let Some(problem_id) = problem_id {
            let test_result = tester::run_test::run_test_all(problem_id);
            if test_result {
                println!("Passed all tests.");
            }
        }
    }
    Ok(())
}
