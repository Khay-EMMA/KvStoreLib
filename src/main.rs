use std::{env::Args, iter::Skip};

fn main() {
    let mut arguments: Skip<Args> = std::env::args().skip(1);
    let key: String = arguments.next().unwrap();
    let value: String = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let content: String = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", content).unwrap();
}
