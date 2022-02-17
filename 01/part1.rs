/*
 * Adapted from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
 */

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut last_known_depth = 0;
    let mut depth_increase_count = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(number) = line {
                let depth: i32 = number.parse().unwrap();

                if last_known_depth > 0 && depth > last_known_depth {
                    depth_increase_count += 1;
                }

                last_known_depth = depth;
            } else {
                panic!("Could not read line");
            }
        }
    } else {
        panic!("Could not read file");
    }

    println!("Number of times depth increases: {}", depth_increase_count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
