use std::cmp::Ordering;

enum Rating {
    Oxygen,
    CO2,
}

fn find_rating(mut report: Vec<&str>, rating_type: Rating) -> &str {
    let report_item_length = report[0].len();

    for index in (0..report_item_length).step_by(1) {
        let mut zero_occurrences_count: usize = 0;
        for item in report.iter() {
            let mut number: char = '0';
            for (char_index, character) in item.char_indices() {
                if char_index == index {
                    number = character
                }
            }
            if number == '0' {
                zero_occurrences_count += 1;
            }
        }

        let criteria = match rating_type {
            Rating::Oxygen => match zero_occurrences_count.cmp(&(report.len() / 2)) {
                Ordering::Less => '1',
                Ordering::Greater => '0',
                Ordering::Equal => '1',
            },
            Rating::CO2 => match zero_occurrences_count.cmp(&(report.len() / 2)) {
                Ordering::Less => '0',
                Ordering::Greater => '1',
                Ordering::Equal => '0',
            },
        };

        report = report
            .into_iter()
            .filter(|item| {
                let mut is_match = false;

                for (char_index, character) in item.char_indices() {
                    if char_index == index && character == criteria {
                        is_match = true;
                        break;
                    }
                }

                is_match
            })
            .collect();

        if report.len() == 1 {
            break;
        }
    }

    report[0]
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/4inKsU3B")?.text()?;

    let report: Vec<&str> = input.lines().map(|s| s.trim()).collect();

    let oxygen_number =
        isize::from_str_radix(&find_rating(report.to_vec(), Rating::Oxygen), 2).unwrap();
    let co2_number = isize::from_str_radix(&find_rating(report.to_vec(), Rating::CO2), 2).unwrap();

    println!("{}", oxygen_number * co2_number);

    Ok(())
}
