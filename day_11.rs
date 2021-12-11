use std::collections::HashMap;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/mM9RcCKy")?.text()?;

    let mut octopuses: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|point| {
                    let height = point.to_string().parse().expect("parse failed");

                    height
                })
                .collect()
        })
        .collect();

    let mut flashes_count = 0;

    for _ in 0..100 {
        octopuses = octopuses
            .into_iter()
            .map(|row| row.into_iter().map(|octopus| octopus + 1).collect())
            .collect();

        let mut flashes: HashMap<(usize, usize), bool> = HashMap::new();

        'flashing: loop {
            let mut did_flash = false;

            for y in 0..octopuses.len() {
                for x in 0..octopuses[0].len() {
                    match flashes.get(&(y, x)) {
                        Some(_) => continue,
                        None => (),
                    };

                    let octopus = octopuses.get(y).unwrap().get(x).unwrap();

                    if *octopus > 9 {
                        flashes.insert((y, x), true);
                        did_flash = true;
                        flashes_count += 1;

                        let adjacent_octopuses =
                            DIRECTIONS.iter().fold(vec![], |mut acc, direction| {
                                flashes.insert((y, x), true);
                                did_flash = true;

                                let octopus_y = (y as isize) + direction.0;
                                let octopus_x = (x as isize) + direction.1;

                                if octopus_y.is_negative() || octopus_x.is_negative() {
                                    acc
                                } else {
                                    match octopuses.get(octopus_y.unsigned_abs()).and_then(
                                        |octopus_row| octopus_row.get(octopus_x.unsigned_abs()),
                                    ) {
                                        Some(_) => acc.push((
                                            octopus_y.unsigned_abs(),
                                            octopus_x.unsigned_abs(),
                                        )),
                                        None => (),
                                    }
                                    acc
                                }
                            });

                        octopuses = octopuses
                            .into_iter()
                            .enumerate()
                            .map(|(row_y, row)| {
                                row.into_iter()
                                    .enumerate()
                                    .map(|(octopus_x, octopus)| {
                                        if adjacent_octopuses.contains(&(row_y, octopus_x)) {
                                            octopus + 1
                                        } else {
                                            octopus
                                        }
                                    })
                                    .collect()
                            })
                            .collect();
                    }
                }
            }

            if !did_flash {
                break 'flashing;
            }
        }

        octopuses = octopuses
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|octopus| if octopus > 9 { 0 } else { octopus })
                    .collect()
            })
            .collect();
    }

    println!("{}", flashes_count);

    Ok(())
}
