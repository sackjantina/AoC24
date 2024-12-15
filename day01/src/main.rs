use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::io::Error;

fn read_input(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let inputs: Vec<&str> = line.split_whitespace().collect();

        if inputs.len() == 2 {
            let in1: i32 = inputs[0].parse().expect("Failed to parse first number");
            let in2: i32 = inputs[1].parse().expect("Failed to parse second number");

            list1.push(in1);
            list2.push(in2);
          }
    }

    Ok((list1, list2))
}

fn part1(mut list1: Vec<i32>, mut list2: Vec<i32>) {
    list1.sort();
    list2.sort();

    let mut total_dist = 0;
    for (num1, num2) in list1.iter().zip(list2.iter()) {
        let dist = (num1 - num2).abs();
        total_dist += dist;
    }

    println!("Total Distance: {}", total_dist);
}

fn part2(list1: &Vec<i32>, list2: &Vec<i32>) {
    let mut map = HashMap::new();

    for num in list2.iter() {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut similarity = 0;
    for num in list1.iter() {
        let occurences = map.get(num).unwrap_or(&0);
        similarity += num * occurences;
    }

    println!("Similarity: {}", similarity);
}

fn main() {
    let file_path = "input.txt";
    let Ok((list1, list2)) = read_input(file_path) else {return};
    part1(list1.clone(), list2.clone());
    part2(&list1, &list2);
}
