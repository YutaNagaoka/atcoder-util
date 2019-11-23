extern crate colored;

use crate::sample_cases::SampleCases;
use colored::Colorize;
use std::io::Write;
use std::process::{Command, Stdio};


/// Code test status.
#[derive(PartialEq)]
enum JudgeStatus {
    AC,
    WA,
}

/// This struct represents code test result.
struct ExecResult {
    status: JudgeStatus,
    expected_output: String,
    actual_output: String,
}

impl ExecResult {
    /// Print whether accepted or wrong for each sample cases.
    fn print_judge_status(&self, case_number: usize) {
        println!("Case: {}", case_number + 1);
        print!("Expected Output: {}", self.expected_output);
        print!("Actual Output:   {}", self.actual_output);
        if self.status == JudgeStatus::AC {
            println!("{}", "AC".green());
        } else {
            println!("{}", "WA".red());
        }
        println!("");
    }
}

/// Run a program and validate answer for each sample cases.
pub fn run_test_all(problem_id: &str) {
    let program_to_execute = format!("./{}.exe", problem_id);
    let sc = SampleCases::new_from_files(problem_id);

    for i in 0..sc.input.len() {
        let result = run_test(
            &program_to_execute,
            sc.input.iter().nth(i).unwrap(),
            sc.output.iter().nth(i).unwrap(),
        );
        result.print_judge_status(i);
    }
}

/// Run program and check if its output is correct in one sample case.
fn run_test(program: &str, expected_input: &str, expected_output: &str) -> ExecResult {
    let mut process = Command::new(program)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run program.");

    let process_stdin = process.stdin.as_mut().unwrap();
    process_stdin.write_all(expected_input.as_bytes()).unwrap();
    let mut actual_output = String::from_utf8(process.wait_with_output().unwrap().stdout).unwrap();
    actual_output.retain(|c| c != '\r');

    let status = if *expected_output == actual_output {
        JudgeStatus::AC
    }
    else {
        JudgeStatus::WA
    };
    ExecResult {
        status,
        expected_output: expected_output.to_string(),
        actual_output: actual_output,
    }
}
