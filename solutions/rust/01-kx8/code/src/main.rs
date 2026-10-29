#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "bytes" {
        let data: Vec<u8> = fs::read(&args[2]).unwrap();
        println!("{}", serde_json::to_string(&data).unwrap());
    } else {
        eprintln!("unknown command: {}", args[1]);
    }
}
