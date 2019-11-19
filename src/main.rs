#[macro_use]
extern crate clap;

mod gen;

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
        .get_matches();

    // Fetch input/output examples and write each of them into text files.
    match matches.subcommand_matches("gen") {
        Some(ref matches) => {
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
            Ok(())
        }
        None => Ok(()),
    }
}
