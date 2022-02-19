use std::collections::HashMap;

fn main() {
    let input = utils::load_input("./input.txt");

    let mut lines = vec![];

    for line_definition in input {
        let coords: Vec<&str> = line_definition.split(" -> ").collect();
        let start_coords: Vec<i32> = coords[0].split(",").map(|num| {
            i32::from_str_radix(num, 10).unwrap()
        }).collect();
        let end_coords: Vec<i32> = coords[1].split(",").map(|num| {
            i32::from_str_radix(num, 10).unwrap()
        }).collect();

        lines.push(Line::new(
            Point::new(start_coords[0], start_coords[1]),
            Point::new(end_coords[0], end_coords[1]),
        ));
    }

    let mut map: HashMap<String, i32> = HashMap::new();

    // Insert line points
    for line in lines {
        for point in line.points() {
            let key = point.to_string();
            let &&current_value = &map.get(&key).unwrap_or(&0);
            map.insert(key, current_value + 1);
        }
    }

    // Count map points with multiple lines
    let count = map.values().fold(0, |acc, &value| {
        if value > 1 { acc + 1 } else { acc }
    });

    println!("Number of points where at least two lines overlap: {}", count);
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y,
        }
    }

    fn to_string(&self) -> String {
        format!("{}-{}", self.x, self.y)
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Line {
            start,
            end,
        }
    }

    fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        if self.start.x == self.end.x {
            // Vertical line
            let y_range = if self.start.y < self.end.y {
                self.start.y..(self.end.y + 1)
            } else {
                self.end.y..(self.start.y + 1)
            };

            for y in y_range {
                points.push(Point::new(self.start.x, y));
            }
        } else if self.start.y == self.end.y {
            // Horizontal line
            let x_range = if self.start.x < self.end.x {
                self.start.x..(self.end.x + 1)
            } else {
                self.end.x..(self.start.x + 1)
            };

            for x in x_range {
                points.push(Point::new(x, self.start.y));
            }
        } else {
            // Diagonal line
            let mut x = self.start.x;
            let mut y = self.start.y;

            loop {
                points.push(Point::new(x, y));

                if self.end.x == x && self.end.y == y {
                    break;
                }

                x += if self.start.x < self.end.x { 1 } else { -1 };
                y += if self.start.y < self.end.y { 1 } else { -1 };
            }
        }

        // println!(
        //     "Line from {}-{} to {}-{} has {} points",
        //     self.start.x,
        //     self.start.y,
        //     self.end.x,
        //     self.end.y,
        //     points.len(),
        // );

        points
    }
}
