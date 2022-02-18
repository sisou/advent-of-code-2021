fn main() {
    let input = utils::load_input("./input.txt");

    let mut board_rows = vec![];
    let mut boards = vec![];
    let mut draws = vec![];

    for i in 0..input.len() {
        let line = &input[i];

        if i == 0 {
            // First line is drawn_numbers
            let numbers = line.split(",");
            draws = numbers.map(|num| {
                i32::from_str_radix(num, 10).unwrap()
            }).collect();
            continue;
        }

        if line.len() == 0 {
            // Every empty line signals a new board
            continue;
        }

        let row = vec2array(line.split_whitespace().map(|num| {
            let value = i32::from_str_radix(num, 10).unwrap();
            Field::new(value)
        }).collect::<Vec<Field>>());

        board_rows.push(row);

        if board_rows.len() == 5 {
            let rows = vec2array(board_rows);
            boards.push(Board::new(rows));
            board_rows = vec![];
        }
    }

    // Call numbers and check boards for bingo
    for draw in draws {
        for board in &mut boards {
            board.call(draw);

            if board.has_bingo() {
                let unmarked_sum = board.unmarked_sum();
                println!("Board unmarked sum: {}", unmarked_sum);
                println!("Drawn number: {}", draw);
                println!("Multiplied: {}", unmarked_sum * draw);
                return;
            }
        }
    }

    println!("No board had bingo");
}

fn vec2array<T>(v: Vec<T>) -> [T; 5] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", 5, v.len()))
}

struct Field {
    value: i32,
    marked: bool,
}

impl Field {
    fn new(value: i32) -> Self {
        Field {
            value,
            marked: false,
        }
    }

    fn set_marked(&mut self) -> () {
        self.marked = true;
    }
}

struct Board {
    rows: [[Field; 5]; 5],
}

impl Board {
    fn new(rows: [[Field; 5]; 5]) -> Self {
        Board {
            rows,
        }
    }

    fn call(&mut self, number: i32) -> () {
        for row in &mut self.rows {
            for field in row {
                if field.value == number {
                    field.set_marked();
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            if self.check_row(i) || self.check_column(i) {
                return true;
            }
        }

        false
    }

    fn check_row(&self, i: usize) -> bool {
        let mut bingo = true;

        for field in &self.rows[i] {
            if !field.marked {
                bingo = false;
                break;
            }
        }

        bingo
    }

    fn check_column(&self, i: usize) -> bool {
        let mut bingo = true;

        for fields in &self.rows {
            if !fields[i].marked {
                bingo = false;
                break;
            }
        }

        bingo
    }

    fn unmarked_sum(&self) -> i32 {
        let mut sum = 0;

        for row in &self.rows {
            for field in row {
                if !field.marked {
                    sum += field.value;
                }
            }
        }

        sum
    }
}
