/*
 * Adapted from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
 */

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut depth = 0;
    let mut horizontal_position = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(command) = line {
                let split_command: Vec<&str> = command.split(" ").collect();
                let direction = split_command[0];
                let distance: i32 = split_command[1].parse().unwrap();

                match direction {
                    "forward" => horizontal_position += distance,
                    "up" => depth -= distance,
                    "down" => depth += distance,
                    _ => panic!("Unknown direction {}", direction),
                }
            } else {
                panic!("Could not read line");
            }
        }
    } else {
        panic!("Could not read file");
    }

    println!("Horizontal position: {}", horizontal_position);
    println!("Depth: {}", depth);
    println!("Multiplied: {}", horizontal_position * depth);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
