mod fetch_sample_cases;

fn main() {
    let url = "https://atcoder.jp/contests/abc144/tasks/abc144_e";
    let sample_cases = fetch_sample_cases::parse_sample_cases(url);
    for case in sample_cases {
        println!("{}", case);
    }
}
