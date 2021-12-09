use itertools::Itertools;
use std::collections::HashMap;

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
            let patterns: Vec<String> = split[0]
                .split(" ")
                .map(|item| {
                    let mut chars: Vec<char> = item.chars().collect();
                    chars.sort();

                    String::from_iter(chars)
                })
                .collect();
            let outputs: Vec<String> = split[1]
                .split(" ")
                .map(|item| {
                    let mut chars: Vec<char> = item.chars().collect();
                    chars.sort();

                    String::from_iter(chars)
                })
                .collect();

            Entry { patterns, outputs }
        })
        .collect();
    let mut output_count = 0;
    let segments: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g"];
    let digits: Vec<&str> = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    for entry in entries.iter() {
        for permutation in segments.iter().permutations(segments.len()).unique() {
            let map: HashMap<String, String> =
                permutation.iter().fold(HashMap::new(), |mut acc, item| {
                    acc.insert(
                        String::from(&segments[acc.len()][..]),
                        String::from(&item[..]),
                    );

                    acc
                });

            let patterns: Vec<String> = digits
                .iter()
                .map(|original_pattern| {
                    let mut pattern: Vec<String> = original_pattern
                        .chars()
                        .map(|original_letter| String::from(&map[&original_letter.to_string()]))
                        .collect();
                    pattern.sort();

                    pattern.join("")
                })
                .collect();

            if patterns
                .iter()
                .all(|pattern| entry.patterns.contains(pattern))
            {
                let map: HashMap<String, String> =
                    permutation.iter().fold(HashMap::new(), |mut acc, item| {
                        acc.insert(
                            String::from(&item[..]),
                            String::from(&segments[acc.len()][..]),
                        );

                        acc
                    });

                let displayed_number =
                    entry.outputs.iter().fold(String::from(""), |acc, output| {
                        let mut pattern: Vec<String> = output
                            .chars()
                            .map(|original_letter| String::from(&map[&original_letter.to_string()]))
                            .collect();
                        pattern.sort();

                        let digit = pattern.join("");
                        let number = digits
                            .iter()
                            .position(|item| item == &digit)
                            .unwrap()
                            .to_string();

                        acc + &number
                    });

                let result: usize = displayed_number.parse().expect("parse failed");

                output_count += result;

                break;
            }
        }
    }

    println!("{}", output_count);

    Ok(())
}
