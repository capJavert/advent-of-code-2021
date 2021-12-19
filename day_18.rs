use regex::Regex;

fn check_explode(number: &str) -> bool {
    let mut level = 0;

    for c in number.chars() {
        if c == '[' {
            level += 1;
        }

        if c == ']' {
            level -= 1;
        }

        if level == 5 {
            break;
        }
    }

    level >= 5
}

fn calc_magnitude(number: &json::JsonValue) -> u64 {
    let a = if number[0].len() == 0 {
        number[0].as_fixed_point_u64(0).unwrap()
    } else {
        calc_magnitude(&number[0])
    };
    let b = if number[1].len() == 0 {
        number[1].as_fixed_point_u64(0).unwrap()
    } else {
        calc_magnitude(&number[1])
    };

    3 * a + 2 * b
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/Rs1FL7ux")?.text()?;

    let split_regex = Regex::new(r"[0-9]{2,}").unwrap();
    let number_regex = Regex::new(r"[0-9]{1,}").unwrap();

    let numbers: Vec<&str> = input.lines().map(|line| line.trim()).collect();
    let mut largest_magnitude = 0;

    for number1 in numbers.iter() {
        for number2 in numbers.iter() {
            if number1 == number2 {
                continue;
            }

            let mut number = String::from("[") + &number1 + "," + number2 + "]";

            loop {
                let is_explode = check_explode(&number);
                let is_split = match split_regex.captures(&number) {
                    Some(_) => true,
                    None => false,
                };

                if is_explode {
                    let mut explode_index_start = 0;
                    let mut explode_index_end = 0;
                    let mut level = 0;

                    let mut string_pair = String::new();

                    for (index, c) in number.chars().enumerate() {
                        if c == '[' {
                            level += 1;
                        }

                        if c == ']' {
                            level -= 1;

                            if explode_index_start > 0 {
                                explode_index_end = index - 1;
                                break;
                            }
                        }
                        if explode_index_start > 0 {
                            string_pair += &c.to_string();
                        }

                        if level == 5 && explode_index_start == 0 {
                            explode_index_start = index + 1;
                        }
                    }

                    let pair: Vec<usize> = string_pair
                        .split(",")
                        .map(|s| s.to_string().parse::<usize>().expect("parse failed"))
                        .collect();
                    let pair = (pair[0], pair[1]);

                    let mut replace_index_left = None;
                    let mut replace_index_right = None;

                    for group in number_regex.find_iter(&number) {
                        if group.start() > explode_index_end {
                            replace_index_right = Option::from(group);

                            break;
                        }

                        if group.end() < explode_index_start {
                            replace_index_left = Option::from(group);
                        }
                    }

                    let new_number = match replace_index_right {
                        Some(group) => {
                            let left = String::from(&number[..group.start()]);
                            let middle = String::from(&number[group.start()..group.end()]);
                            let right = String::from(&number[group.end()..]);

                            let replace_number = group
                                .as_str()
                                .replace("[", "")
                                .replace("]", "")
                                .replace(",", "")
                                .parse::<usize>()
                                .expect("parse failed");

                            let value = String::from(
                                left + &middle.replace(
                                    &replace_number.to_string(),
                                    &(replace_number + pair.1).to_string(),
                                ) + &right,
                            );

                            value
                        }
                        None => number.to_string(),
                    };

                    let left = String::from(&new_number[..explode_index_start - 1]);
                    let middle =
                        String::from(&new_number[explode_index_start - 1..explode_index_end + 2]);
                    let right = String::from(&new_number[explode_index_end + 2..]);

                    let value = String::from(
                        left + &middle
                            .replace(&String::from(String::from("[") + &string_pair + "]"), "0")
                            + &right,
                    );

                    let new_number = value;

                    let new_number = match replace_index_left {
                        Some(group) => {
                            let left = String::from(&new_number[..group.start()]);
                            let middle = String::from(&new_number[group.start()..group.end()]);
                            let right = String::from(&new_number[group.end()..]);

                            let replace_number = group
                                .as_str()
                                .replace("[", "")
                                .replace("]", "")
                                .replace(",", "")
                                .parse::<usize>()
                                .expect("parse failed");

                            let value = String::from(
                                left + &middle.replace(
                                    &replace_number.to_string(),
                                    &(replace_number + pair.0).to_string(),
                                ) + &right,
                            );

                            value
                        }
                        None => new_number.to_string(),
                    };

                    number = new_number
                } else if is_split {
                    let group = split_regex.find(&number).unwrap();
                    let replace_number: f32 = group.as_str().parse().expect("parse failed");
                    let pair: (usize, usize) = (
                        (replace_number / 2.0).floor() as usize,
                        (replace_number / 2.0).ceil() as usize,
                    );

                    let left = String::from(&number[..group.start()]);
                    let middle = String::from(&number[group.start()..group.end()]);
                    let right = String::from(&number[group.end()..]);

                    let new_number = String::from(
                        left + &middle.replace(
                            &group.as_str(),
                            &String::from(
                                String::from("[")
                                    + &pair.0.to_string()
                                    + ","
                                    + &pair.1.to_string()
                                    + "]",
                            ),
                        ) + &right,
                    );

                    number = new_number
                } else {
                    break;
                }
            }

            let magnitude = calc_magnitude(&json::parse(&number).unwrap());

            if largest_magnitude < magnitude {
                largest_magnitude = magnitude;
            }
        }
    }

    println!("{}", largest_magnitude);

    Ok(())
}
