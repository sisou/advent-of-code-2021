const DAYS: i32 = 256;

fn main() {
    let input = utils::load_input("./input.txt");

    let fishes: Vec<u8> = input[0].split(",").map(|str| {
        u8::from_str_radix(str, 10).unwrap()
    }).collect();

    // Create a schedule that covers newborns' time until reproduction
    let mut schedule: Vec<usize> = vec![0; 9];

    for fish in fishes {
        // Add each fish to the schedule
        schedule[usize::from(fish)] += 1;
    }

    for _ in 0..DAYS {
        let todays_fishes = schedule[0];

        // Rotate today's fishes to the end of the schedule as newborns
        schedule.rotate_left(1);

        // Add parent fishes again to the schedule in 7 days
        schedule[6] += todays_fishes;
    }

    let fish_count: usize = schedule.iter().sum();

    println!("Number of lanternfish after {} days: {}", DAYS, fish_count);
}
