use std::collections::HashMap;

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/PVnh4C5R")?.text()?;

    let crabs: Vec<i32> = input
        .split(",")
        .map(|item| item.parse().expect("parse failed"))
        .collect();

    let max_position = *crabs.iter().max().unwrap();
    let min_position = *crabs.iter().min().unwrap();
    let mut min_fuel_cost = i32::MAX;
    let mut cache: HashMap<i32, i32> = HashMap::new();

    for crab in min_position..max_position + 1 {
        let mut fuel_cost = 0;
        for crab2 in crabs.iter() {
            let move_steps = (crab - crab2).abs();
            let move_fuel_cost: i32 = match cache.get(&move_steps) {
                Some(cached_cost) => *cached_cost,
                None => {
                    let calculated_cost = (1..move_steps + 1).sum();
                    cache.insert(move_steps, calculated_cost);

                    calculated_cost
                }
            };

            fuel_cost += move_fuel_cost
        }

        if fuel_cost < min_fuel_cost {
            min_fuel_cost = fuel_cost;
        }
    }

    println!("{}", min_fuel_cost);

    Ok(())
}
