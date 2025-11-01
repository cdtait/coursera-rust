mod reuters_code;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <underlying_name> <contract_month>", args[0]);
        std::process::exit(1);
    }

    let underlying_name = &args[1];
    let contract_month = &args[2];

    let reuters_code = reuters_code::generate_reuters_code(underlying_name, contract_month);
    println!("Reuters Code: {}", reuters_code);
}