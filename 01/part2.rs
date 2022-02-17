fn main() {
    let input = utils::load_input("./input.txt");

    let mut last_sum = 0;
    let mut sum_increase_count = 0;

    let mut previous_depth = 0;
    let mut second_previous_depth = 0;

    for line in input {
        let depth: i32 = line.parse().unwrap();

        let mut sum = 0;

        if previous_depth > 0 && second_previous_depth > 0 {
            sum = depth + previous_depth + second_previous_depth;
        }

        if sum > 0 && last_sum > 0 {
            if sum > last_sum {
                sum_increase_count += 1;
            }
        }

        second_previous_depth = previous_depth;
        previous_depth = depth;
        last_sum = sum;
    }

    println!("Number of times sum of depths increases: {}", sum_increase_count);
}
