use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
struct Vent {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

impl Vent {
    fn is_diagonal(&self) -> bool {
        self.x1 != self.x2 && self.y1 != self.y2
    }

    fn to_x_range(&self) -> Range<usize> {
        if self.x1 < self.x2 {
            self.x1..self.x2 + 1
        } else {
            self.x2..self.x1 + 1
        }
    }

    fn to_y_range(&self) -> Range<usize> {
        if self.y1 < self.y2 {
            self.y1..self.y2 + 1
        } else {
            self.y2..self.y1 + 1
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    intersections: usize,
}

impl Point {
    fn cover(&mut self) {
        self.intersections += 1;
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/GP6e1p3x")?.text()?;

    let command_match =
        Regex::new(r"^(?P<x1>[0-9]{1,}),(?P<y1>[0-9]{1,}) -> (?P<x2>[0-9]{1,}),(?P<y2>[0-9]{1,})$")
            .unwrap();

    let vents: Vec<Vent> = input
        .trim()
        .lines()
        .map(|s| {
            let matches = command_match.captures(s.trim()).unwrap();
            let command = Vent {
                x1: matches
                    .name("x1")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("should be a number"),
                y1: matches
                    .name("y1")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("should be a number"),
                x2: matches
                    .name("x2")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("should be a number"),
                y2: matches
                    .name("y2")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("should be a number"),
            };

            command
        })
        .collect();

    let mut diagram: HashMap<String, Point> = HashMap::new();

    for (_, vent) in vents.iter().enumerate() {
        if vent.is_diagonal() {
            continue;
        }

        for x in vent.to_x_range() {
            for y in vent.to_y_range() {
                let mut key = String::from("x");
                key.push_str(&x.to_string());
                key.push_str("y");
                key.push_str(&y.to_string());

                let point = diagram.entry(key).or_insert(Point {
                    x,
                    y,
                    intersections: 0,
                });
                point.cover();
            }
        }
    }

    println!(
        "{}",
        diagram
            .values()
            .fold(0, |acc, point| if point.intersections > 1 {
                acc + 1
            } else {
                acc
            })
    );

    Ok(())
}
