fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/TJUTpQ13")?.text()?;

    let mut depth_changes = 0;
    let mut last_measure: &i32 = &0;
    let measures: Vec<i32> = input
        .trim()
        .lines()
        .map(|s| s.parse().expect("Parse failed"))
        .collect();

    for (index, measure) in measures.iter().enumerate() {
        if index > 0 {
            if measure > last_measure {
                depth_changes += 1;
            }
        }

        last_measure = measure
    }

    println!("{}", depth_changes);

    Ok(())
}
