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

    let expected_file_content = vec![
        ("b_input", ["6\nabcabc\n", "6\nabcadc\n", "1\nz\n"].to_vec()),
        ("b_output", ["Yes\n", "No\n", "No\n"].to_vec())
    ]
        .into_iter()
        .collect::<HashMap<&str, Vec<&str>>>();

    let mut dir_to_test = PathBuf::new();
    dir_to_test.push("io_examples/b");
    for dir_res in fs::read_dir(dir_to_test).unwrap() {
        let dir_name = dir_res.unwrap().path();
        for (i, file_res) in fs::read_dir(&dir_name).unwrap().into_iter().enumerate() {
            let file = file_res.unwrap().path();
            let file_content = fs::read_to_string(file).unwrap();
            let s = dir_name.file_name().unwrap();
            assert_eq!(
                expected_file_content.get(s.to_str().unwrap()).unwrap()[i],
                file_content
            );
        }
    }
}

#[test]
fn test_fetch_all_problem() {
    let contest_id = "abc145";
    gen::execute_fetching_problems_in_contest(&contest_id).unwrap();

    let expected_file_content = vec![
        ("a_input", vec!["2\n", "100\n"].to_vec()),
        ("a_output", vec!["4\n", "10000\n"].to_vec()),
        ("b_input", ["6\nabcabc\n", "6\nabcadc\n", "1\nz\n"].to_vec()),
        ("b_output", ["Yes\n", "No\n", "No\n"].to_vec()),
        (
            "c_input",
            [
                "3\n0 0\n1 0\n0 1\n",
                "2\n-879 981\n-866 890\n",
                "8\n-406 10\n512 859\n494 362\n-955 -475\n128 553\n-986 -885\n763 77\n449 310\n",
            ]
                .to_vec(),
        ),
        (
            "c_output",
            ["2.2761423749\n", "91.9238815543\n", "7641.9817824387\n"].to_vec(),
        ),
        ("d_input", ["3 3\n", "2 2\n", "999999 999999\n"].to_vec()),
        ("d_output", ["2\n", "0\n", "151840682\n"].to_vec()),
        (
            "e_input",
            [
                "2 60\n10 10\n100 100\n",
                "3 60\n10 10\n10 20\n10 30\n",
                "3 60\n30 10\n30 20\n30 30\n",
                "10 100\n15 23\n20 18\n13 17\n24 12\n18 29\n19 27\n23 21\n18 20\n27 15\n22 25\n",
            ]
                .to_vec(),
        ),
        ("e_output", ["110\n", "60\n", "50\n", "145\n"].to_vec()),
        (
            "f_input",
            [
                "4 1\n2 3 4 1\n",
                "6 2\n8 6 9 1 2 1\n",
                "10 0\n1 1000000000 1 1000000000 1 1000000000 1 1000000000 1 1000000000\n",
            ]
                .to_vec(),
        ),
        ("f_output", ["3\n", "7\n", "4999999996\n"].to_vec()),
    ]
        .into_iter()
        .collect::<HashMap<&str, Vec<&str>>>();

    for &problem_id in ["a", "b", "c", "d", "e", "f"].iter() {
        let mut dir_to_test = PathBuf::new();
        dir_to_test.push(format!("io_examples/{}", problem_id));
        for dir_res in fs::read_dir(dir_to_test).unwrap() {
            let dir_name = dir_res.unwrap().path();
            for (i, file_res) in fs::read_dir(&dir_name).unwrap().into_iter().enumerate() {
                let file = file_res.unwrap().path();
                let file_content = fs::read_to_string(file).unwrap();
                let s = dir_name.file_name().unwrap();
                assert_eq!(
                    expected_file_content.get(s.to_str().unwrap()).unwrap()[i],
                    file_content
                );
            }
        }
    }

    fs::remove_dir_all("io_examples").expect("Failed to deleted directory.");
}
