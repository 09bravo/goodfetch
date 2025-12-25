use std::env;
use crate::process;
pub fn flags() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
    match args[1].as_str() {
        "--help" | "-h" | "-help" | "--h" =>  {
        println!("W.I.P");
        process::exit(0);
        }
        _ => {
        println!("goodfetch: Unknown flag");
        process::exit(1);
        }
        }
    }
}
