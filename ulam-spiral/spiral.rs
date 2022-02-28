fn main() {
    let width: usize = 51;

    if width % 2 == 0 {
        panic!("Width must be an uneven number");
    }

    let mut data: Vec<i32> = vec![0; width.pow(2)];

    let mut x = ((width + 1) / 2) as i32;
    let mut y = ((width + 1) / 2) as i32;
    let mut direction = 1;
    let mut steps_until_turn = 1;
    let mut steps_since_turn = 0;
    let mut turn_count = 0;

    for i in 1..(width.pow(2) + 1) {
        write(&mut data, i as i32, x, y);

        match direction {
            1 => x += 1, // right
            2 => y -= 1, // up
            3 => x -= 1, // left
            4 => y += 1, // down
            _ => panic!("Invalid direction {}", direction),
        }

        steps_since_turn += 1;

        if steps_since_turn == steps_until_turn {
            // Turn
            direction = if direction < 4 { direction + 1} else { 1 };
            steps_since_turn = 0;
            turn_count += 1;

            // Every two turns, increase the steps until next turn
            if turn_count % 2 == 0 {
                steps_until_turn += 1;
            }
        }
    }

    render(data);
}

fn write(data: &mut Vec<i32>, i: i32, x: i32, y: i32) {
    let width = (data.len() as f32).sqrt() as usize;
    data[coords_to_index(x, y, width)] = i;
    // println!("Wrote number {} at ({}, {})", i, x, y);
}

fn coords_to_index(x: i32, y: i32, width: usize) -> usize {
    ((y - 1) * (width as i32) + (x - 1)) as usize
}

fn render(data: Vec<i32>) {
    let width = (data.len() as f32).sqrt();

    for (i, &field) in data.iter().enumerate() {
        let position = (i + 1) as f32;

        let is_prime = primes::is_prime(field as u64);

        if is_prime {
            print!("🟦");
        } else {
            print!("⬜");
        }

        if position % width == 0.0 {
            print!("\n");
        }
    }
}
