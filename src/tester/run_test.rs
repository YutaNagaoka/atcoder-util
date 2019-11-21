extern crate colored;

use colored::Colorize;
use crate::sample_cases::SampleCases;
use std::io::Write;
use std::process::{Command, Stdio};


/// Run a program and validate answer for each sample cases.
pub fn run_test_all(problem_id: &str) {
    let program_to_execute = format!("./{}.exe", problem_id);
    let sc = SampleCases::new_from_files(problem_id);

    for i in 0..sc.input.len() {
        run_test(
            &program_to_execute,
            i,
            sc.input.iter().nth(i).unwrap(),
            sc.output.iter().nth(i).unwrap(),
        );
    }
}

/// Run program and check if its output is correct in one sample case.
fn run_test(program: &str, case_number: usize, input_example: &str, output_example: &str) {
    let mut process = Command::new(program)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run program.");
    let process_stdin = process.stdin.as_mut().unwrap();
    process_stdin.write_all(input_example.as_bytes()).unwrap();

    let mut output = String::from_utf8(process.wait_with_output().unwrap().stdout).unwrap();
    output.retain(|c| c != '\r');

    print_judge_status(case_number, output_example, &output);
}

/// Print whether accepted or wrong for each sample cases.
fn print_judge_status(case_number: usize, output_example: &str, output: &str) {
    println!("Case: {}", case_number + 1);
    print!("Expected Output: {}", output_example);
    print!("Actual Output:   {}", output);
    if *output_example == *output {
        println!("{}", "AC".green());
    } else {
        println!("{}", "WA".red());
    }
    println!("");
}
