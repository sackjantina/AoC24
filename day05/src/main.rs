use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn read_input(filename: &str) -> Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut reading_rules = true;
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            reading_rules = false;
            continue;
        }
        if reading_rules {
            let rule: Vec<i32> = line.split("|").map(|n| n.trim().parse().unwrap()).collect();
            rules.entry(rule[0]).or_insert_with(Vec::new).push(rule[1]);
            // println!("{:?}", rule);
        } else {
            updates.push(line.split(',').map(|n| n.trim().parse().unwrap()).collect());
        }
    }

    // println!("{:?}", rules);
    // println!("{:?}", updates);

    Ok((rules, updates))
}

fn part1(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) {
    let mut correct: bool;
    let mut correct_orders = vec![];

    for update in updates {
        // println!("Update: {:?}", update);
        correct = true;
        '_check_update: for (i, page) in update.iter().enumerate() {
            // print!("page {}: ", page);
            for p in &update[..i] {
                // print!("{} " , p);
                if let Some(rule) = rules.get(page) {
                    if rule.contains(p) {
                        // println!("Broken Rule");
                        correct = false;
                        break '_check_update;
                    }
                }
            }
            // println!();
        }
        if correct { correct_orders.push(update.clone()) };
    }
    
    let mut sum = 0;
    for update in correct_orders {
        // println!("{:?}", update);
        sum += update[update.len()/2];
    }

    println!("Part 1 Sum: {}", sum);
}

fn part2(rules: HashMap<i32, Vec<i32>>, mut updates: Vec<Vec<i32>>) {
    let mut corrected_updates = Vec::new();

    for (x, update) in updates.iter_mut().enumerate() {
        // println!("Update: {:?}", update);

        let mut swaps;
        let mut correct = false;
        let mut corrected = false;    

        while !correct {
            correct = true;
            swaps = Vec::new();

            // Iterate over each page in an update
            '_check_update: for (i, page) in update.iter().enumerate() {
                // print!("page {}: ", page);

                // Back iterate over all previous pages up to that one checking the rules on each
                '_check_page: for (j, p) in update[..i].iter().enumerate() {
                    // print!("{} " , p);

                    if let Some(rule) = rules.get(page) {
                        if rule.contains(p) {
                            // println!("\nBroken Rule: {}|{:?}", page, rule);
                            swaps.push((i,j, *page));
                            correct = false;
                            corrected = true;
                            break '_check_update;
                        }
                    }
                }
                // println!();
            }

            for (i,j,page) in &swaps {
                update.remove(*i);
                update.insert(*j, *page);
                // println!("New Update: {:?}", update);
            }

        }
        if corrected {
            corrected_updates.push(update.clone());
        }
    }

    let mut sum = 0;
    // println!("Corrected Updates:");
    for c_up in corrected_updates {
        // println!("{:?}", c_up);
        sum += c_up[c_up.len()/2];
    }
    println!("Part 2 Sum: {}", sum);
}

fn main() {
    let Ok((rules, updates)) = read_input("input.txt") else {return;};

    part1(&rules, &updates);
    part2(rules, updates);
}
