use std::collections::HashSet;

fn main() {
    // Input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<i32>().unwrap();

    // Save all names in a vector
    let mut names = Vec::new();

    for _i in 0..count * 2 {
        input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        names.push(input);
    }

    let mut hash_set = HashSet::new();

    // Add all to set
    for i in 0..count {
        let full_name = format!(
            "{} {}",
            names[i as usize].trim(),
            names[(i + count) as usize].trim()
        );
        hash_set.insert(full_name);
    }

    // Print set size
    println!("{}", hash_set.len());
}
