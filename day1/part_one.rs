mod tools;

// https://adventofcode.com/2024/day/1#part1
fn main() {
    // read lines
    let (mut first_list, mut second_list) = tools::read_pairs_from_stdin().expect("Failed to read lines");
    // sort vectors
    first_list.sort();
    second_list.sort();

    // find distance between elements
    let mut sum = 0;
    for (a, b) in first_list.iter().zip(second_list.iter()) {
        sum += (a - b).abs();
    }
    println!("{}", sum);
}

