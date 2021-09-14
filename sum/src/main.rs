use std::collections::BinaryHeap;

fn main() {
    // Input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<i32>().unwrap();

    input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let values: Vec<&str> = input.split_whitespace().collect();

    // Insert everything into a binary heap
    let mut heap = BinaryHeap::new();
    for s in values {
        heap.push(s.parse::<i32>().unwrap());
    }

    // Calulate total from half of the values
    let mut total = 0;
    for _x in 0..((heap.len() + 1) / 2) {
        total += heap.pop().unwrap();
    }

    // Print
    println!("{}", total);
}
