use std::io::{self, BufRead};

fn main() {
    let _stdin = io::stdin();
    let lines = _stdin.lock().lines();

    for line in lines.map(Result::unwrap) {
        // Calculate difference
        let values: Vec<&str> = line.split_whitespace().collect();

        let number1 = values[0].trim().parse::<i64>().unwrap();
        let number2 = values[1].trim().parse::<i64>().unwrap();

        println!("{}", (number1 - number2).abs());
    }
}