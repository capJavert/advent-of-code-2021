fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/4inKsU3B")?.text()?;

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    let report: Vec<&str> = input.lines().map(|s| s.trim()).collect();
    let report_length = report.len();
    let report_item_length = report[0].len();

    for index in (0..report_item_length).step_by(1) {
        let mut zero_occurrences_count = 0;

        for item in report.iter() {
            let mut number: char = '0';

            for (char_index, char) in item.char_indices() {
                if char_index == index {
                    number = char
                }
            }

            if number == '0' {
                zero_occurrences_count += 1;
            }
        }

        if zero_occurrences_count > (report_length / 2) {
            gamma.push_str("0");
            epsilon.push_str("1");
        } else {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
    }

    let gamma_number = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_number = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", gamma_number * epsilon_number);

    Ok(())
}
