mod sample_cases;

fn main() {
    let url = "https://atcoder.jp/contests/abc144/tasks/abc144_e";
    let sc = sample_cases::SampleCases::new(url);
    for input in sc.input {
        println!("INPUT:\n{}", input);
    }
    for output in sc.output {
        println!("OUTPUT:\n{}", output);
    }
}
