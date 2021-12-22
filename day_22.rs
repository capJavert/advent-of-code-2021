use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone)]
struct Cube {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cube {
    fn contains(&self, cube: &Cube) -> bool {
        !(self.x.1 < cube.x.0
            || self.x.0 > cube.x.1
            || self.y.1 < cube.y.0
            || self.y.0 > cube.y.1
            || self.z.1 < cube.z.0
            || self.z.0 > cube.z.1)
    }
}

#[derive(Debug, Copy, Clone)]
struct Step {
    state: bool,
    cube: Cube,
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/Qk56RHNW")?.text()?;

    let step_regex = Regex::new(r"^(?P<state>on|off) x=(?P<x>-?[0-9]{1,}..-?[0-9]{1,}),y=(?P<y>-?[0-9]{1,}..-?[0-9]{1,}),z=(?P<z>-?[0-9]{1,}..-?[0-9]{1,})$").unwrap();

    let steps: Vec<Step> = input
        .lines()
        .filter(|line| line.trim().len() > 0)
        .map(|line| {
            let matches = step_regex.captures(line.trim()).unwrap();

            let mut coords = [
                matches.name("x").unwrap().as_str(),
                matches.name("y").unwrap().as_str(),
                matches.name("z").unwrap().as_str(),
            ]
            .into_iter()
            .map(|item| {
                let mut split = item.split("..");

                (
                    split.next().unwrap().parse().expect("parse failed"),
                    split.next().unwrap().parse().expect("parse failed"),
                )
            })
            .collect::<Vec<(isize, isize)>>();

            Step {
                state: if matches.name("state").unwrap().as_str() == "on" {
                    true
                } else {
                    false
                },
                cube: Cube {
                    x: coords.remove(0),
                    y: coords.remove(0),
                    z: coords.remove(0),
                },
            }
        })
        .collect();

    let mut cubes = HashMap::new();

    let valid_area = Cube {
        x: (-50, 50),
        y: (-50, 50),
        z: (-50, 50),
    };

    for step in steps.iter() {
        if !valid_area.contains(&step.cube) {
            continue;
        }

        let Step { cube, .. } = step;

        for x in RangeInclusive::new(cube.x.0, cube.x.1) {
            for y in RangeInclusive::new(cube.y.0, cube.y.1) {
                for z in RangeInclusive::new(cube.z.0, cube.z.1) {
                    cubes.insert((x, y, z), step.state);
                }
            }
        }
    }

    println!(
        "{}",
        cubes
            .values()
            .fold(0, |acc, item| if *item { acc + 1 } else { acc })
    );

    Ok(())
}
