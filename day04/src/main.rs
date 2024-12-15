use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Error;

fn read_input(filename: &str) -> Result<Vec<Vec<char>>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let map: Vec<Vec<_>> = reader.lines().map(|line| {
        line.expect("?").chars().collect()
    }).collect();    

    return Ok(map);
}

fn bounds_check(x: i32, y: i32, bounds: (i32, i32)) -> bool {
    return x >= 0 && x < bounds.0 && y >= 0 && y < bounds.1;
}

fn part1(map: &Vec<Vec<char>>) {
    let bounds: (i32, i32) = (map[0].len().try_into().unwrap(), map.len().try_into().unwrap());

    let pattern = ['M', 'A', 'S'];
    let directions = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

    let mut num_xmas = 0;
    let mut found_xmas;

    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == 'X' {
            //    println!("Found an X at ({},{})", x,y);
               for dir in directions {
                let mut new_x: i32 = x.try_into().unwrap();
                let mut new_y: i32 = y.try_into().unwrap();
                found_xmas = true;
                '_direction_check: for c in pattern {
                  new_x += dir.0;
                  new_y += dir.1;
                  if !bounds_check(new_x, new_y, bounds) || c != map[new_y as usize][new_x as usize] {
                    found_xmas = false;
                    break '_direction_check;
                  } 
                }
                if found_xmas { 
                    num_xmas += 1; 
                    // println!("Found XMAS at ({},{}) in {:?} direction", x,y,dir);
                }
               }
            }
        }
    }
    println!("Part 1: Number of XMAS = {}", num_xmas);
}

fn part2(map: &Vec<Vec<char>>) {
    let bounds = (map[0].len() as i32, map.len() as i32);
    let mut num_xmas = 0;
    let mut found_xmas;

    let patterns = [[('M', (-1, -1)), ('M', (-1, 1)), ('S', (1, 1)), ('S', (1, -1))],
                    [('M', (-1, 1)), ('M', (1, 1)), ('S', (1, -1)), ('S', (-1, -1))],
                    [('M', (1, 1)), ('M', (1, -1)), ('S', (-1, -1)), ('S', (-1, 1))],
                    [('M', (1, -1)), ('M', (-1, -1)), ('S', (-1, 1)), ('S', (1, 1))]];

    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == 'A' {
                // println!("Found 'A' at ({},{})", x,y);
                '_location_check: for pattern in patterns {
                    let cur_x = x as i32;
                    let cur_y = y as i32;
                    found_xmas = true;
                    '_pattern_check: for p in pattern {
                        let new_x = cur_x + p.1.0;
                        let new_y = cur_y + p.1.1;
                        // println!("Checking for '{}' at ({},{})", p.0, new_x, new_y);
                        if !bounds_check(new_x, new_y, bounds) || p.0 != map[new_y as usize][new_x as usize] {
                            // println!("poop");
                            found_xmas = false;
                            break '_pattern_check;
                        }
                    }
                    if found_xmas {
                        num_xmas += 1;
                        // println!("Found X-MAS at location: ({},{})", x,y);
                        break '_location_check;
                    }
                }
            }
        }
    }
    
    println!("Part 2: Number of X-MAS = {}", num_xmas);
}

fn main() {
    let map = read_input("input.txt").expect("Expecting a 2D Map of characters");

    // println!("  0 1 2 3 4 5 6 7 8 9");
    // for (i, row)in map.iter().enumerate() {
    //     print!("{} ", i);
    //     for c in row {
    //         print!("{} ",c);
    //     }
    //     println!();
    // }

    part1(&map);
    part2(&map);
}   
