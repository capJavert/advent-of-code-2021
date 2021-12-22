use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Cube {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cube {
    fn size(&self) -> isize {
        ((self.x.0 - self.x.1).abs() + 1)
            * ((self.y.0 - self.y.1).abs() + 1)
            * ((self.z.0 - self.z.1).abs() + 1)
    }

    fn intersection(&self, cube: &Cube) -> Cube {
        if self.is_intersecting(cube) {
            Cube {
                x: (
                    isize::max(self.x.0, cube.x.0),
                    isize::min(self.x.1, cube.x.1),
                ),
                y: (
                    isize::max(self.y.0, cube.y.0),
                    isize::min(self.y.1, cube.y.1),
                ),
                z: (
                    isize::max(self.z.0, cube.z.0),
                    isize::min(self.z.1, cube.z.1),
                ),
            }
        } else {
            Cube {
                x: (0, 0),
                y: (0, 0),
                z: (0, 0),
            }
        }
    }

    fn is_intersecting(&self, cube: &Cube) -> bool {
        if self.x.1 < cube.x.0 || self.x.0 > cube.x.1 {
            false
        } else if self.y.1 < cube.y.0 || self.y.0 > cube.y.1 {
            false
        } else if self.z.1 < cube.z.0 || self.z.0 > cube.z.1 {
            false
        } else {
            true
        }
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

    let mut cubes: Vec<Step> = vec![];

    for step in steps.iter() {
        let Step { cube, .. } = step;
        let mut intersections = vec![];

        for step2 in cubes.iter() {
            let Step { cube: cube2, .. } = step2;

            // to account for intersection we switch
            // the state of the step so that "on" intersection
            // becomse "off" step (because it cuts the part of the
            // original step cube)
            if cube.is_intersecting(cube2) {
                intersections.push(Step {
                    state: !step2.state,
                    cube: cube.intersection(cube2),
                });
            }
        }

        cubes.append(&mut intersections);

        if step.state {
            cubes.push(step.clone());
        }
    }

    println!(
        "{}",
        cubes.iter().fold(0, |acc, item| acc
            + item.cube.size() * if item.state { 1 } else { -1 })
    );

    Ok(())
}
