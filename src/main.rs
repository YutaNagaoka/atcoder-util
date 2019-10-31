extern crate reqwest;

fn main() {
    let mut res = reqwest::get("https://atcoder.jp/contests/abc144/tasks/abc144_a").unwrap();
    let body = res.text().unwrap();
    println!("{}", body);
}
