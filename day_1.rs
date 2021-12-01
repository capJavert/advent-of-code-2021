fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/TJUTpQ13")?.text()?;

    let mut depth_changes = 0;
    let mut last_measure: &i32 = &0;
    let measures: Vec<i32> = input
        .trim()
        .lines()
        .map(|s| s.parse().expect("Parse failed"))
        .collect();

    let mut measure_windows: Vec<i32> = Vec::new();
    let measure_window_size = 3;

    for (index, _) in measures.iter().enumerate() {
        let mut measure_window = 0;

        for (index2, measure2) in measures[index..].iter().enumerate() {
            if index2 == measure_window_size {
                break;
            }

            measure_window += measure2;
        }

        measure_windows.push(measure_window);
    }

    for (index, measure) in measure_windows.iter().enumerate() {
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
