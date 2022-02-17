fn main() {
    let input = utils::load_input("./input.txt");

    let mut depth = 0;
    let mut horizontal_position = 0;

    for line in input {
        let command: Vec<&str> = line.split(" ").collect();
        let direction = command[0];
        let distance: i32 = command[1].parse().unwrap();

        match direction {
            "forward" => horizontal_position += distance,
            "up" => depth -= distance,
            "down" => depth += distance,
            _ => panic!("Unknown direction {}", direction),
        }
    }

    println!("Horizontal position: {}", horizontal_position);
    println!("Depth: {}", depth);
    println!("Multiplied: {}", horizontal_position * depth);
}
