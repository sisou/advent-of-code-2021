/*
 * Adapted from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
 */

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_input(filename: &str) -> Vec<String> {
    let mut input = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(text) = line {
                input.push(text);
            } else {
                panic!("Could not read line");
            }
        }
    } else {
        panic!("Could not read file");
    }

    input
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
