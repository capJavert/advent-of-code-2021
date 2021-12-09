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

    let mut risk_levels_sum = 0;

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
                let risk_level = 1 + point;
                risk_levels_sum += risk_level;
            }
        }
    }

    println!("{}", risk_levels_sum);

    Ok(())
}
