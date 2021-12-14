use std::collections::HashMap;

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/bFd6DrPw")?.text()?;

    let mut template = vec![];
    let mut pairs: Vec<(String, String)> = vec![];

    for line in input.lines() {
        let item = line.trim();

        if item == "" {
            continue;
        }

        if item.contains("->") {
            let split: Vec<&str> = item.split(" -> ").collect();

            pairs.push((String::from(split[0]), String::from(split[1])));
        } else {
            template = item.chars().map(|c| c.to_string()).collect();
        }
    }

    let mut pair_counts: HashMap<String, usize> = HashMap::new();
    for (index, item) in template.iter().enumerate().step_by(2) {
        if index > 0 {
            match template.get(index - 1) {
                Some(prev_item) => {
                    let pair = String::from(prev_item) + item;

                    *pair_counts.entry(pair).or_insert(0) += 1;
                }
                None => (),
            };
        }

        match template.get(index + 1) {
            Some(next_item) => {
                let pair = String::from(item) + next_item;

                *pair_counts.entry(pair).or_insert(0) += 1;
            }
            None => (),
        };
    }

    let mut item_counts: HashMap<String, usize> = HashMap::new();
    for item in template.iter() {
        *item_counts.entry(item.to_string()).or_insert(0) += 1;
    }

    for _ in 1..41 {
        let mut new_pair_counts = pair_counts.clone();

        for pair in pairs.iter() {
            let pair_count = pair_counts.entry(pair.0.to_string()).or_insert(0);
            *item_counts.entry(pair.1.to_string()).or_insert(0) += *pair_count;

            let split: Vec<String> = pair.0.chars().map(|c| c.to_string()).collect();
            *new_pair_counts.entry(pair.0.to_string()).or_insert(0) -= *pair_count;

            let prev_pair = split[0].to_string() + &pair.1;
            *new_pair_counts.entry(prev_pair).or_insert(0) += *pair_count;

            let next_pair = pair.1.to_string() + &split[1];
            *new_pair_counts.entry(next_pair).or_insert(0) += *pair_count;
        }

        pair_counts = new_pair_counts;
    }

    println!(
        "{}",
        item_counts.values().max().unwrap() - item_counts.values().min().unwrap()
    );

    Ok(())
}
