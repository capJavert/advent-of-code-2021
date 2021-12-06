fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/1KzMjj2U")?.text()?;

    let mut fish_school: Vec<isize> = input
        .split(",")
        .map(|item| item.trim().parse().expect("parse failed"))
        .collect();
    let simulation_days = 80;

    for _ in 0..simulation_days {
        let last_fish_school = fish_school.clone();

        for (index, fish) in last_fish_school.iter().enumerate() {
            match fish {
                0 => {
                    fish_school[index] = 6;
                    fish_school.push(8);
                }
                _ => {
                    fish_school[index] -= 1;
                }
            }
        }
    }

    println!("{}", fish_school.len());

    Ok(())
}
