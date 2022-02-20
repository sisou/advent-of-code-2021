fn main() {
    let input = utils::load_input("./input.txt");

    let mut positions: Vec<i32> = input[0].split(",").map(|str| {
        i32::from_str_radix(str, 10).unwrap()
    }).collect();

    positions.sort();

    let target = positions[positions.len() / 2];
    let fuel = positions.iter().fold(0, |acc, pos| {
        acc + i32::abs(target - pos)
    });

    println!("The best position is {} and costs {} fuel", target, fuel);
}
