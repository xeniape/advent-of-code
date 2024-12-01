use std::fs::read_to_string;
use std::str;

fn main() {
    println!("--- Day 1: Historian Hysteria ---");
    let mut total_distance = 0;
    let mut similarity_score = 0;

    // Read and parse input from file
    // Create two separate lists, one for each group
    let (mut list_one, mut list_two) = convert_lines_to_lists("input.txt");

    // Sort lists
    list_one.sort();
    list_two.sort();

    // Compare entries from each list and calculate distance
    // Calculate sum of distances
    for (index, entry) in list_one.iter().enumerate() {
        total_distance += i32::abs(entry - list_two.get(index).unwrap());
    }

    // Output solution
    println!("Total distance: {}", total_distance);

    // Calculate similarity score between lists
    for entry_one in list_one.iter() {
        let mut multiplicator = 0;

        for entry_two in list_two.iter() {
            if entry_one == entry_two {
                multiplicator += 1;
            }
        }

        similarity_score += multiplicator * entry_one;
    }

    // Output solution
    println!("Similarity score: {}", similarity_score);
}

fn convert_lines_to_lists(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let mut split_string = str::split_whitespace(line);

        if let Some(first) = split_string.next() {
            list_one.push(first.to_string().parse().unwrap());
        }
        if let Some(second) = split_string.next() {
            list_two.push(second.to_string().parse().unwrap());
        }
    }

    (list_one, list_two)
}
