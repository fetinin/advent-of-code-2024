mod tools;

// https://adventofcode.com/2024/day/1#part2
fn main() {
    // read lines
    let (first_list, second_list) = tools::read_pairs_from_stdin().expect("Failed to read lines");

    // count occurrence of each element
    let mut occurences = std::collections::HashMap::new();
    for id in second_list.iter() {
        let counter = occurences.entry(id).or_insert(0);
        *counter += 1;
    }

    // count similarity score
    let mut similarity_score = 0;
    for a in first_list.iter() {
        similarity_score += a * occurences.get(a).unwrap_or(&0);
    }

    println!("{}", similarity_score);
}

