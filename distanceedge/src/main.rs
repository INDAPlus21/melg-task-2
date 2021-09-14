use std::cmp;

fn main() {
    let mut line = String::new();
    // Input
    std::io::stdin().read_line(&mut line).unwrap();
    let values: Vec<&str> = line.split_whitespace().collect();
    let height = values[0].parse::<i64>().unwrap();
    let width = values[1].parse::<i64>().unwrap();

    // Loop a grid
    for y in 0..height {
        for x in 0..width {
            let x_distance = cmp::min(x, width - x - 1) + 1;
            let y_distance = cmp::min(y, height - y - 1) + 1;
            let min_distance = cmp::min(x_distance, y_distance);

            if min_distance > 9 {
                print!(".");
            } else {
                print!("{}", cmp::min(x_distance, y_distance));
            }
        }

        println!();
    }
}
