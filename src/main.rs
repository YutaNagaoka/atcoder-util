#[macro_use]
extern crate clap;

mod sample_cases;
mod file_utils;

use clap::{ App, Arg, SubCommand };

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("gen")
            .about("Generate input/output format example fetched from AtCoder website.")
            .arg(Arg::with_name("contest")
                .help("Specify which contest to fetch.")
                .short("c")
                .long("contest")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("problem")
                .help("Specify which problem to fetch.")
                .short("p")
                .long("problem")
                .takes_value(true)
            )
        );

    let matches = app.get_matches();


    if let Some(ref matches) = matches.subcommand_matches("gen") {
        let contest_id = matches.value_of("contest").unwrap();
        let problem_id = matches.value_of("problem").unwrap();
        let url = format!("https://atcoder.jp/contests/{}/tasks/{}_{}", contest_id, contest_id, problem_id);
        let sc = sample_cases::SampleCases::new(&url);
        file_utils::create_test_files(sc).expect("Failed to execution.")
    }
    else {
        ()
    }
}
