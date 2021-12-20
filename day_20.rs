use std::collections::HashMap;

const PIXELS: [(isize, isize); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn binary_to_decimal(binary: &str) -> usize {
    isize::from_str_radix(binary, 2).unwrap() as usize
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/crNGi71m")?.text()?;

    let mut algorithm = vec![];
    let mut image: HashMap<(isize, isize), String> = HashMap::new();

    for (index, line) in input.lines().enumerate() {
        let item = line.trim();

        if index == 0 {
            algorithm = item.chars().map(|c| c.to_string()).collect();
            continue;
        }

        if item.len() == 0 {
            continue;
        }

        for (x, c) in item.chars().enumerate() {
            image.insert((x as isize, (index - 2) as isize), c.to_string());
        }
    }

    for step in 1..51 {
        let mut temp_image = image.clone();

        for pixel in image.clone().keys() {
            let pixels = PIXELS.iter().map(|direction| {
                let adjacent_x = pixel.0 + direction.1;
                let adjacent_y = pixel.1 + direction.0;
                (adjacent_x, adjacent_y)
            });
            for (x, y) in pixels.into_iter() {
                let binary = PIXELS.iter().fold(String::new(), |mut acc, direction| {
                    let adjacent_x = x + direction.1;
                    let adjacent_y = y + direction.0;

                    let pixel = image
                        .entry((adjacent_x, adjacent_y))
                        .or_insert(String::from(if step % 2 == 0 { "#" } else { "." }));

                    if pixel == "#" {
                        acc += "1";
                    } else {
                        acc += "0";
                    };
                    acc
                });
                let enhancement_index = binary_to_decimal(&binary);
                let enhancement_pixel = String::from(&algorithm[enhancement_index]);
                temp_image.insert((x, y), enhancement_pixel);
            }
        }

        image = temp_image;
    }

    println!(
        "{}",
        image.values().fold(0, |acc, pixel| {
            if pixel == "#" {
                acc + 1
            } else {
                acc
            }
        })
    );

    Ok(())
}
