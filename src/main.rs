mod sample_cases;
mod file_utils;

fn main() {
    let url = "https://atcoder.jp/contests/abc144/tasks/abc144_e";
    let sc = sample_cases::SampleCases::new(url);
    file_utils::create_test_files(sc).expect("Failed to execution.")
}
