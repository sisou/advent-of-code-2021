fn main() {
    let input = utils::load_input("./input.txt");

    let mut fishes: Vec<Lanternfish> = input[0].split(",").map(|str| {
        let days = u8::from_str_radix(str, 10).unwrap();
        Lanternfish::from_days(days)
    }).collect();

    for _ in 0..80 {
        let mut number_of_newborns = 0;

        for fish in &mut fishes {
            let reproduced = fish.age();
            if reproduced {
                number_of_newborns += 1;
            }
        }

        for _ in 0..number_of_newborns {
            fishes.push(Lanternfish::newborn());
        }
    }

    println!("Number of lanternfish after 80 days: {}", fishes.len());
}

struct Lanternfish {
    days_until_reproduction: u8,
}

impl Lanternfish {
    fn from_days(days: u8) -> Self {
        Lanternfish {
            days_until_reproduction: days,
        }
    }

    fn newborn() -> Self {
        Lanternfish {
            days_until_reproduction: 8,
        }
    }

    fn age(&mut self) -> bool {
        if self.days_until_reproduction == 0 {
            self.days_until_reproduction = 6;
            true
        } else {
            self.days_until_reproduction -= 1;
            false
        }
    }
}
