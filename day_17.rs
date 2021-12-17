use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() -> Result<(), reqwest::Error> {
    let input = "target area: x=217..240, y=-126..-69";
    let command_match = Regex::new(
        r"^.*=(?P<x1>[0-9]{1,})..(?P<x2>[0-9]{1,}).*=(?P<y1>[0-9-]{1,})..(?P<y2>[0-9-]{1,})$",
    )
    .unwrap();
    let matches = command_match.captures(input.trim()).unwrap();
    let x_target: (isize, isize) = (
        matches
            .name("x1")
            .unwrap()
            .as_str()
            .parse()
            .expect("parse failed"),
        matches
            .name("x2")
            .unwrap()
            .as_str()
            .parse()
            .expect("parse failed"),
    );
    let y_target: (isize, isize) = (
        matches
            .name("y1")
            .unwrap()
            .as_str()
            .parse()
            .expect("parse failed"),
        matches
            .name("y2")
            .unwrap()
            .as_str()
            .parse()
            .expect("parse failed"),
    );

    let mut velocities = HashMap::new();

    let x_range = x_target.0..x_target.1 + 1;
    let y_range = y_target.0..y_target.1 + 1;

    for x in 0..1000 {
        for y in -1000..1000 {
            let mut position: (isize, isize) = (0, 0);
            let mut velocity: (isize, isize) = (x, y);

            loop {
                position.0 += velocity.0;
                position.1 += velocity.1;

                velocity.0 = match velocity.0.cmp(&0) {
                    Ordering::Less => velocity.0 + 1,
                    Ordering::Greater => velocity.0 - 1,
                    Ordering::Equal => velocity.0,
                };
                velocity.1 -= 1;

                if position.0 > x_target.1 || position.1 < y_target.0 {
                    break;
                }

                if x_range.contains(&position.0) && y_range.contains(&position.1) {
                    velocities.insert((x, y), true);
                }
            }
        }
    }

    println!("{}", velocities.len());

    Ok(())
}
