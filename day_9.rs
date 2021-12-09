use std::collections::HashMap;

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/e2p42TMm")?.text()?;

    let heightmap: Vec<Vec<isize>> = input
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

    let mut low_points: Vec<(isize, isize)> = Vec::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            let point = heightmap[y][x];
            let adjacent_points = [
                heightmap
                    .get(y.wrapping_sub(1))
                    .and_then(|item| item.get(x)),
                heightmap
                    .get(y)
                    .and_then(|item| item.get(x.wrapping_add(1))),
                heightmap
                    .get(y.wrapping_add(1))
                    .and_then(|item| item.get(x)),
                heightmap
                    .get(y)
                    .and_then(|item| item.get(x.wrapping_sub(1))),
            ];

            let is_lowpoint = adjacent_points.into_iter().all(|option| match option {
                Some(adjacent_point) => adjacent_point > &point,
                None => true,
            });

            if is_lowpoint {
                low_points.push((y as isize, x as isize))
            }
        }
    }

    let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut basins: Vec<usize> = vec![];

    for point in low_points {
        fn find_basin(
            low_point: (isize, isize),
            directions: &[(isize, isize); 4],
            heightmap: &Vec<Vec<isize>>,
            basin: &mut usize,
            visited: &mut HashMap<(isize, isize), bool>,
        ) -> usize {
            for direction in directions {
                let (direction_y, direction_x) = direction;
                let (mut y, mut x) = low_point;

                loop {
                    let low_point = heightmap[y.unsigned_abs()][x.unsigned_abs()];
                    y += direction_y;
                    x += direction_x;
                    if y.is_negative() || x.is_negative() {
                        break;
                    } else {
                        match visited.get(&(y, x)) {
                            Some(_) => break,
                            None => (),
                        }

                        let moved_point = heightmap
                            .get(y.unsigned_abs())
                            .and_then(|item| item.get(x.unsigned_abs()));
                        match moved_point {
                            Some(point) => {
                                let is_lowpoint = low_point > *point;

                                if point == &9 || is_lowpoint {
                                    break;
                                } else {
                                    visited.insert((y, x), true);

                                    *basin += 1;
                                    let nested_basin =
                                        find_basin((y, x), directions, heightmap, &mut 0, visited);

                                    *basin += nested_basin;
                                }
                            }
                            None => break,
                        }
                    }
                }
            }

            *basin
        }

        let basin = find_basin(point, &directions, &heightmap, &mut 1, &mut HashMap::new());
        basins.push(basin);
    }

    basins.sort_by(|a, b| b.cmp(a));

    println!("{}", basins[0..3].iter().fold(1, |acc, basin| acc * basin));

    Ok(())
}
