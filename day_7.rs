fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/PVnh4C5R")?.text()?;

    let crabs: Vec<i32> = input
        .split(",")
        .map(|item| item.parse().expect("parse failed"))
        .collect();

    let mut min_fuel_cost = i32::MAX;

    for crab in crabs.iter() {
        let mut fuel_cost = 0;
        for crab2 in crabs.iter() {
            fuel_cost += (crab - crab2).abs();
        }

        if fuel_cost < min_fuel_cost {
            min_fuel_cost = fuel_cost;
        }
    }

    println!("{}", min_fuel_cost);

    Ok(())
}
