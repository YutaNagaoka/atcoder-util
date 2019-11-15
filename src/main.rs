#[macro_use]
extern crate clap;

mod problem;
//mod contest;
mod file_utils;
mod sample_cases;

use clap::{App, Arg, SubCommand};
use std::io;
//use contest::Contest;
use problem::Problem;

fn main() -> Result<(), io::Error> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("gen")
                .about("Generate input/output format example fetched from AtCoder's website.")
                .arg(
                    Arg::with_name("contest")
                        .help("Specify which contest to fetch.")
                        .short("c")
                        .long("contest")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("problem")
                        .help("Specify which problem to fetch.")
                        .short("p")
                        .long("problem")
                        .takes_value(true),
                ),
        );

    let matches = app.get_matches();

    // Fetch input/output examples and write each of them into text files.
    match matches.subcommand_matches("gen") {
        Some(ref matches) => {
            let contest_id = matches.value_of("contest");
            let problem_id = matches.value_of("problem");

            // Problem is specified (such as "a", "b", "c"...).
            match (contest_id, problem_id) {
                (Some(contest_id), Some(problem_id)) => {
                    let problem = Problem::new(contest_id, problem_id);
                    if let Some(problem) = problem {
                        problem.create_sample_cases_files()?;
                    }
                }
                //(Some(contest_id), None) => {}
                (_, _) => {}
            }
            Ok(())
        }
        None => {
            Ok(())
        }
    }
}
