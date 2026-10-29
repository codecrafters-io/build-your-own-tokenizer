#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "bytes" {
        let data = fs::read(&args[2]).unwrap();
        let text = data
            .iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("[{text}]");
    } else {
        eprintln!("unknown command: {}", args[1]);
    }
}
