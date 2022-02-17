fn main() {
    let input = utils::load_input("./input.txt");

    let mut last_known_depth = 0;
    let mut depth_increase_count = 0;

    for line in input {
        let depth: i32 = line.parse().unwrap();

        if last_known_depth > 0 && depth > last_known_depth {
            depth_increase_count += 1;
        }

        last_known_depth = depth;
    }

    println!("Number of times depth increases: {}", depth_increase_count);
}
