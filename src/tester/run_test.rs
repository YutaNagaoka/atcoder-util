use crate::sample_cases::SampleCases;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, PartialEq)]
pub enum JudgeStatus {
    AC,
    WA,
}

/// Run a code and check if the output is the same to each output example.
pub fn run_test(problem_id: &str) -> JudgeStatus {
    let program_to_execute = format!("./{}.exe", problem_id);
    let sc = SampleCases::new_from_files(problem_id);

    for i in 0..sc.input.len() {
        let mut process = Command::new(&program_to_execute)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let process_stdin = process.stdin.as_mut().unwrap();
        let input_example = sc.input.iter().nth(i).unwrap();
        process_stdin.write_all(input_example.as_bytes());

        let mut output = String::from_utf8(process.wait_with_output().unwrap().stdout).unwrap();
        output.retain(|c| c != '\r');
        let output_example = sc.output.iter().nth(i).unwrap();
        println!("{}", *output_example);
        println!("{}", output);

        if *output_example != output {
            return JudgeStatus::WA;
        }
    }
    JudgeStatus::AC
}
