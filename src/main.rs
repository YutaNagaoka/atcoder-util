#[macro_use]
extern crate clap;

mod contest;
mod sample_cases;
mod file_utils;

use clap::{ App, Arg, SubCommand };
use contest::Contest;


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
        );

    let matches = app.get_matches();


    if let Some(ref matches) = matches.subcommand_matches("gen") {
        let contest_id = matches.value_of("contest").unwrap();
        let mut contest_info = Contest::new(contest_id);
        contest_info.fetch_sample_cases();
        contest_info.create_sample_cases_files();
        println!("Created files of sample cases.");
    }
    else {
        ()
    }
}


