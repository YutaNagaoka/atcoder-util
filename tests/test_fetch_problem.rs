extern crate atcoder_util;

use atcoder_util::gen;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_fetch_one_problem() {
    let contest_id = "abc145";
    let problem_id = "b";
    gen::execute_fetching_problem(&contest_id, &problem_id).unwrap();

    assert_eq!(
        fs::read_to_string("io_examples/b/b_input/input1.txt").unwrap(),
        "6\nabcabc\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/b/b_input/input2.txt").unwrap(),
        "6\nabcadc\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/b/b_input/input3.txt").unwrap(),
        "1\nz\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/b/b_output/output1.txt").unwrap(),
        "Yes\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/b/b_output/output2.txt").unwrap(),
        "No\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/b/b_output/output3.txt").unwrap(),
        "No\n"
    );
}

#[test]
fn test_fetch_all_problem() {
    let contest_id = "abc145";
    gen::execute_fetching_problems_in_contest(&contest_id).unwrap();

    assert_eq!(
        fs::read_to_string("io_examples/a/a_input/input2.txt").unwrap(),
        "100\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/b/b_input/input1.txt").unwrap(),
        "6\nabcabc\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/c/c_output/output2.txt").unwrap(),
        "91.9238815543\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/d/d_output/output3.txt").unwrap(),
        "151840682\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/e/e_input/input3.txt").unwrap(),
        "3 60\n30 10\n30 20\n30 30\n"
    );
    assert_eq!(
        fs::read_to_string("io_examples/f/f_input/input3.txt").unwrap(),
        "10 0\n1 1000000000 1 1000000000 1 1000000000 1 1000000000 1 1000000000\n"
    );

    fs::remove_dir_all("io_examples").expect("Failed to deleted directory.");
}
