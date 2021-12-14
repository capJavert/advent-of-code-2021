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

    for _ in 1..11 {
        let template_pairs =
            template
                .iter()
                .enumerate()
                .step_by(2)
                .fold(vec![], |mut acc, (index, item)| {
                    if index > 0 {
                        match template.get(index - 1) {
                            Some(prev_item) => {
                                acc.push(String::from(prev_item) + item);
                            }
                            None => (),
                        };
                    }

                    match template.get(index + 1) {
                        Some(next_item) => {
                            acc.push(String::from(item) + next_item);
                        }
                        None => (),
                    };

                    acc
                });

        let mut insertions = vec![];

        for pair in pairs.iter() {
            for (index, template_pair) in template_pairs.iter().enumerate() {
                if template_pair == &pair.0 {
                    insertions.push((index + 1, pair.1.to_string()));
                }
            }
        }

        insertions.sort_by(|a, b| b.0.cmp(&a.0));

        for (index, item) in insertions {
            template.insert(index, item);
        }
    }

    let counts =
        template
            .into_iter()
            .fold(HashMap::new(), |mut acc: HashMap<String, usize>, item| {
                *acc.entry(item).or_insert(0) += 1;

                acc
            });

    println!(
        "{}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    );

    Ok(())
}
