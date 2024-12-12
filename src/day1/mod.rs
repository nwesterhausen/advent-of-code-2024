use std::fs;

pub fn main() {
    println!("\nDay 1\n");
    // Read input file
    let (left_numbers, right_numbers) = read_input_file();

    // Complete PART 1:
    let total_distance = total_distance(&left_numbers, &right_numbers);
    println!("Part 1 - Total distance: {}", total_distance);

    // Complete PART 2:
    let similarity_score = similarity_score(&left_numbers, &right_numbers);
    println!("Part 2 - Similarity score: {}", similarity_score);
}

/// Figure out exactly how often each number from the left list appears in the right list
/// Calculate a total similarity score by adding up each number in the left list after 
/// multiplying it by the number of times that number appears in the right list.
fn similarity_score(left_numbers: &Vec<i32>, right_numbers: &Vec<i32>) -> i32 {
    let mut similarity_score = 0;

    for number in left_numbers {
        let appearances = right_numbers.iter().filter(|&n| *n == *number).count() as i32;
        similarity_score += number * appearances;
    }

    similarity_score
}

/// Add up the total distance between both arrays when they are sorted, smallest to largest
fn total_distance(left_numbers: &Vec<i32>, right_numbers: &Vec<i32>) -> i32 {
    // Sort both arrays
    let mut left_numbers = left_numbers.clone();
    left_numbers.sort();
    let mut right_numbers = right_numbers.clone();
    right_numbers.sort();

    // Line them up smallest to largest, then find the absolute difference between the two
    let mut total_distance = 0;
    for i in 0..left_numbers.len() {
        total_distance += (left_numbers[i] - right_numbers[i]).abs();
    }

    total_distance
}

fn read_input_file() -> (Vec<i32>, Vec<i32>)
{
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();


    let contents = fs::read_to_string("inputs/day1.txt")
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        left_numbers.push(numbers[0].parse::<i32>().unwrap());
        right_numbers.push(numbers[1].parse::<i32>().unwrap());
    }

    (left_numbers, right_numbers)
}