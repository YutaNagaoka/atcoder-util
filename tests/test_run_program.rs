extern crate atcoder_util;

use atcoder_util::gen;
use atcoder_util::tester::run_test;
use std::fs;
use std::process::Command;

#[test]
fn test_run_all_program() {
    gen::execute_fetching_problems_in_contest("abc120").expect("Failed to fetch problems.");
    test_run_one_program("a");
    test_run_one_program("b");
    test_run_one_program("c");
    test_run_one_program("d");
    fs::remove_dir_all("io_examples").expect("Failed to deleted a directory.");
}

fn test_run_one_program(problem_id: &str) {
    let compile_result = compile_program(problem_id);
    if let Some(source_file) = compile_result {
        let test_result = run_test::run_test_all(problem_id);
        fs::remove_file(source_file).expect("Failed to delete a file.");
        assert!(test_result);
    }
}

fn compile_program(problem_id: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    let exec_file = format!("./{}.exe", problem_id);
    #[cfg(not(target_os = "windows"))]
    let exec_file = format!("{}", problem_id);
    let source_file = format!("./tests/code/{}.cpp", problem_id);

    let status = Command::new("g++")
        .arg("-o")
        .arg(&exec_file)
        .arg(&source_file)
        .status()
        .expect("Failed to compile.");

    if status.success() {
        Some(exec_file)
    } else {
        None
    }
}
