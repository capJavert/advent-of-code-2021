use std::collections::HashMap;

fn get_rotations_for_scanner(
    scanner: &Vec<(isize, isize, isize)>,
    orientation: usize,
) -> Vec<(isize, isize, isize)> {
    let direction = orientation / 4;
    let rotation = orientation % 4;

    scanner
        .to_vec()
        .into_iter()
        .map(|(x, y, z)| {
            let adjusted = match direction {
                1 => (x, z, -y),  // up
                2 => (x, -z, y),  // down
                3 => (-z, y, x),  // left
                4 => (z, y, -x),  // right
                5 => (-x, y, -z), // behind
                _ => (x, y, z),
            };

            adjusted
        })
        .map(|(x, y, z)| {
            let adjusted = match rotation {
                1 => (-y, x, z),  // 90deg right
                2 => (-x, -y, z), // 180deg right
                3 => (y, -x, z),  // 270deg right
                _ => (x, y, z),
            };

            adjusted
        })
        .collect::<Vec<(isize, isize, isize)>>()
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/j7nsaj5f")?.text()?;
    let mut scanners: Vec<Vec<(isize, isize, isize)>> = vec![];

    for line in input.lines() {
        let item = line.trim();

        if item.len() == 0 {
            continue;
        }

        if item.starts_with("--- scanner") {
            scanners.push(vec![]);
        } else {
            let mut split = item.split(",");

            scanners.last_mut().unwrap().push((
                split.next().unwrap().parse().expect("parse failed"),
                split.next().unwrap().parse().expect("parse failed"),
                split.next().unwrap().parse().expect("parse failed"),
            ))
        }
    }

    let mut adjusted_scanners = HashMap::new();
    let mut checked_scanners = HashMap::new();

    adjusted_scanners.insert(0, ((0, 0, 0), scanners[0].to_vec()));

    while adjusted_scanners.len() != scanners.len() {
        for (index, _) in scanners.iter().enumerate() {
            if !adjusted_scanners.contains_key(&index) {
                continue;
            }

            if checked_scanners.contains_key(&index) {
                continue;
            } else {
                checked_scanners.insert(index, true);
            }

            'find_rotations: for (index2, scanner2) in scanners.iter().enumerate() {
                if adjusted_scanners.contains_key(&index2) {
                    continue;
                }

                let beacons = adjusted_scanners.get(&index).unwrap().1.to_vec();

                for rotation in 0..24 {
                    let rotations = get_rotations_for_scanner(scanner2, rotation);
                    let mut keys: HashMap<(isize, isize, isize), usize> = HashMap::new();

                    for beacon in beacons.to_vec().iter() {
                        for option in rotations.iter() {
                            let zero = (
                                beacon.0 - option.0,
                                beacon.1 - option.1,
                                beacon.2 - option.2,
                            );
                            *keys.entry(zero).or_insert(0) += 1;

                            if keys.get(&zero).unwrap() >= &12 {
                                let position = adjusted_scanners.get(&index).unwrap().0;
                                adjusted_scanners.insert(
                                    index2,
                                    (
                                        (
                                            position.0 + zero.0,
                                            position.1 + zero.1,
                                            position.2 + zero.2,
                                        ),
                                        rotations,
                                    ),
                                );

                                continue 'find_rotations;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut unique_beacons: HashMap<(isize, isize, isize), bool> = HashMap::new();

    for (position, beacons) in adjusted_scanners.values() {
        for beacon in beacons.into_iter() {
            unique_beacons.insert(
                (
                    beacon.0 + position.0,
                    beacon.1 + position.1,
                    beacon.2 + position.2,
                ),
                true,
            );
        }
    }

    println!("{}", unique_beacons.len());

    Ok(())
}
