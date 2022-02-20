fn main() {
    let input = utils::load_input("./input.txt");

    let mut unique_segments_count = 0;

    for line in input {
        let sections: Vec<&str> = line.split(" | ").collect();
        let output_values: Vec<&str> = sections[1].split(" ").collect();

        for value in output_values {
            if value.len() == 2 || value.len() == 3 || value.len() == 4 || value.len() == 7 {
                unique_segments_count += 1;
            }
        }
    }

    println!("In the output values, the numbers 1, 4, 7 or 8 appear {} times", unique_segments_count);
}
