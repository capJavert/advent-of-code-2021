use std::collections::HashMap;

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/1KzMjj2U")?.text()?;

    let fish_school: Vec<usize> = input
        .split(",")
        .map(|item| item.trim().parse().expect("parse failed"))
        .collect();
    let simulation_days = 256;
    let mut spawns: HashMap<usize, usize> = HashMap::new();

    for fish in fish_school.iter() {
        let spawn = spawns.entry(*fish + 1).or_insert(0);

        *spawn += 1;
    }

    let mut school_size = fish_school.len();

    for day in 1..simulation_days + 1 {
        let mut new_spawns = spawns.clone();

        match spawns.get(&day) {
            Some(new_fish) => {
                let spawn = new_spawns.entry(day).or_insert(0);
                *spawn = 0;

                let spawn = new_spawns.entry(day + 1 + 6).or_insert(0);
                *spawn += *new_fish;
                school_size += new_fish;

                let spawn = new_spawns.entry(day + 1 + 8).or_insert(0);
                *spawn += new_fish;
            }
            None => (),
        }

        spawns = new_spawns;
    }

    println!("{}", school_size);

    Ok(())
}
