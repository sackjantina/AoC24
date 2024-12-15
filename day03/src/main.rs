use std::io::Error;
use regex::Regex;

fn part1(filename: &str) -> Result<(), Error> {
    let text = std::fs::read_to_string(filename)?;

    let re = Regex::new(r"(mul\((?<X>[0-9]{1,3}),(?<Y>[0-9]{1,3})\))").unwrap();

    let instructions: Vec<(i32, i32)> = re.captures_iter(&text).map(|inst| {
        let x = inst.name("X").unwrap().as_str().parse().expect("Failed to parse number");
        let y = inst.name("Y").unwrap().as_str().parse().expect("Failed to parse number");
        (x,y)
    }).collect();

    let mut sum = 0;
    println!("Number of valid instructions: {}", instructions.len()); 
    for instruction in instructions {
        // println!("{:?}", instruction);
        sum += instruction.0 * instruction.1;
    }

    println!("Part 1 Sum: {}", sum);

    Ok(())
}

fn part2(filename: &str) -> Result<(), Error> {
    let text = std::fs::read_to_string(filename)?;

    let mul_pattern = r"(?<MUL>mul\((?<X>[0-9]{1,3}),(?<Y>[0-9]{1,3})\))";
    let do_pattern = r"(?<DO>do\(\))";
    let dont_pattern = r"(?<DONT>don\'t\(\))";
    let combined_pattern = format!("{}|{}|{}", mul_pattern, do_pattern, dont_pattern);
    let re = Regex::new(&combined_pattern).unwrap();
    // println!("Regex pattern: {}", combined_pattern);

    let mut instructions: Vec<(i32, i32)> = Vec::new();
    let mut enabled: bool = true;

    for inst in re.captures_iter(&text) {
        // println!("{}", inst.get(0).unwrap().as_str());
        if let Some(do_inst) = inst.name("DO") {
            // println!("{}", do_inst.as_str());    
            enabled = true;
        } else if let Some(dont_inst) = inst.name("DONT") {
            // println!("{}", dont_inst.as_str());
            enabled = false;
        } else if let Some(mul_inst) = inst.name("MUL") {
            let x: i32 = inst.name("X").unwrap().as_str().parse().expect("Failed to parse number");
            let y: i32 = inst.name("Y").unwrap().as_str().parse().expect("Failed to parse number");
            if enabled {
                // println!("({},{})", x,y);
                instructions.push((x,y));
            }
        }
    }
    
    let mut sum = 0;
    println!("Number of valid instructions: {}", instructions.len()); 
    for instruction in instructions {
        // println!("{:?}", instruction);
        sum += instruction.0 * instruction.1;
    }

    println!("Part 2 Sum: {}", sum);

    Ok(())
}

fn main() {
    let filename = "input.txt";
    let _ = part1(filename);
    let _ = part2(filename);
}
