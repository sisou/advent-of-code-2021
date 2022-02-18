fn main() {
    let input = utils::load_input("./input.txt");
    let input_nums: Vec<i32> = input.iter().map(|line| {
        i32::from_str_radix(line, 2).unwrap()
    }).collect();

    let oxygen = reduce(input_nums.iter().collect(), true);
    let co2 = reduce(input_nums.iter().collect(), false);

    println!("Oxygen: {}", oxygen);
    println!("CO2: {}", co2);
    println!("Life support rating: {}", oxygen * co2);
}

fn reduce(mut numbers: Vec<&i32>, most_common: bool) -> &i32 {
    for i in 0..12 {
        let position = 0b100000000000 >> i;

        let mut bit1_set = vec![];
        let mut bit0_set = vec![];

        for num in numbers {
            if num & position == position {
                bit1_set.push(num);
            } else {
                bit0_set.push(num);
            }
        }

        if bit1_set.len() >= bit0_set.len() {
            numbers = if most_common { bit1_set } else { bit0_set };
        } else {
            numbers = if most_common { bit0_set } else { bit1_set };
        }

        if numbers.len() == 1 {
            break;
        }
    }

    numbers[0]
}
