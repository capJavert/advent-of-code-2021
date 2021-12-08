#[derive(Debug)]
struct Entry {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/ggBHmFuX")?.text()?;

    let entries: Vec<Entry> = input
        .trim()
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(" | ").collect();
            let patterns = split[0].split(" ").map(|item| String::from(item)).collect();
            let outputs = split[1].split(" ").map(|item| String::from(item)).collect();

            Entry { patterns, outputs }
        })
        .collect();

    let mut unique_output_count = 0;

    for entry in entries.iter() {
        for output in entry.outputs.iter() {
            match output.len() {
                2 => unique_output_count += 1,
                4 => unique_output_count += 1,
                3 => unique_output_count += 1,
                7 => unique_output_count += 1,
                _ => (),
            }
        }
    }

    println!("{}", unique_output_count);

    Ok(())
}
