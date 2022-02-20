fn main() {
    let input = utils::load_input("./input.txt");

    let positions: Vec<i32> = input[0].split(",").map(|str| {
        i32::from_str_radix(str, 10).unwrap()
    }).collect();

    let target = (positions.iter().sum::<i32>() as f32 / positions.len() as f32).floor() as i32;

    let fuel = positions.iter().fold(0, |acc, pos| {
        acc + fuel_use(i32::abs(target - pos))
    });

    println!("The best position is {} and costs {} fuel", target, fuel);
}

fn fuel_use(distance: i32) -> i32 {
    let mut used = 0;

    for step in 1..(distance + 1) {
        used += step;
    }

    used
}
