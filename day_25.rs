use std::collections::HashMap;

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/HeBbGMHw")?.text()?;

    let mut cucumbers = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, item) in line.trim().chars().enumerate() {
            cucumbers.insert((x, y), item);
        }
    }

    let size = (
        input.lines().next().unwrap().trim().len(),
        input.lines().collect::<Vec<&str>>().len(),
    );
    let herds = ['>', 'v'];
    let mut step = 0;

    loop {
        step += 1;
        let mut did_move = false;

        for herd in herds.iter() {
            let mut new_cucumbers = cucumbers.clone();

            for ((x, y), cucumber) in cucumbers.iter() {
                if cucumber != herd {
                    continue;
                }

                let move_position = if *herd == '>' {
                    ((x + 1) % size.0, *y)
                } else {
                    (*x, (y + 1) % size.1)
                };

                let check_cucumber = cucumbers[&move_position];
                let can_move = check_cucumber == '.';

                if can_move {
                    new_cucumbers.insert((*x, *y), '.');
                    new_cucumbers.insert(move_position, *cucumber);
                    did_move = true;
                }
            }

            cucumbers = new_cucumbers;
        }

        if !did_move {
            break;
        }
    }

    println!("{}", step);

    Ok(())
}
