#[macro_use]
extern crate clap;

mod contest;
mod file_utils;
mod sample_cases;

use std::io;
use clap::{App, Arg, SubCommand};
use contest::Contest;

fn main() -> Result<(), io::Error>{
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
    if let Some(ref matches) = matches.subcommand_matches("gen") {
        let contest_id = matches.value_of("contest");
        let problem_id = matches.value_of("problem");

        // Problem is specified (such as "a", "b", "c"...).
        if let Some(contest_id) = contest_id {
            let mut contest_info = Contest::new(contest_id, problem_id);
            contest_info.fetch_sample_cases();
            contest_info.create_sample_cases_files(problem_id)?;
        }
        Ok(())
    } else {
        Ok(())
    }
}
