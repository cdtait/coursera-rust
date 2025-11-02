mod reuters_code;
mod solutions_code;
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

    let mut test_array = [1, 3, 6, 4, 1, 2];
    let solution = solutions_code::solution1(&mut test_array);
    println!("Solution1 Result: {}", solution);
    let solution_hashset = solutions_code::solution3(&[1, 3, 6, 4, 1, 2]);
    println!("Solution3 Result: {}", solution_hashset);
}
