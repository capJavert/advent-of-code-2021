fn hex_to_binary(hex: &str) -> &str {
    match hex {
        "0" => "0000",
        "1" => "0001",
        "2" => "0010",
        "3" => "0011",
        "4" => "0100",
        "5" => "0101",
        "6" => "0110",
        "7" => "0111",
        "8" => "1000",
        "9" => "1001",
        "A" => "1010",
        "B" => "1011",
        "C" => "1100",
        "D" => "1101",
        "E" => "1110",
        "F" => "1111",
        _ => panic!("invalid hex string '{}'", hex),
    }
}

fn binary_to_decimal(binary: &str) -> usize {
    isize::from_str_radix(binary, 2).unwrap() as usize
}

fn parse_packet<'a>(packet: &'a str, version_numbers: &mut Vec<usize>) -> &'a str {
    let mut extra = "";

    let version = binary_to_decimal(&packet[0..3]);
    version_numbers.push(version);
    let type_id = binary_to_decimal(&packet[3..6]);

    match type_id {
        4 => {
            let rest = &packet[6..];
            let mut literal = String::new();

            for (index, character) in rest.chars().enumerate().step_by(5) {
                match character.to_string().parse().expect("parse failed") {
                    1 => {
                        literal += &rest[index + 1..index + 5];
                    }
                    0 => {
                        literal += &rest[index + 1..index + 5];
                        extra = &rest[index + 5..];
                        break;
                    }
                    _ => panic!("invalid literal group prefix '{}'", character),
                }
            }

            if extra.len() >= 6 {
                parse_packet(extra, version_numbers);
            }
        }
        _ => {
            if packet.len() >= 7 {
                let length_type_id: usize = packet[6..]
                    .chars()
                    .next()
                    .unwrap()
                    .to_string()
                    .parse()
                    .expect("parse failed");

                match length_type_id {
                    0 => {
                        if packet.len() >= 22 {
                            let length = binary_to_decimal(&packet[7..22]);
                            parse_packet(&packet[22..22 + length], version_numbers);
                            extra = &packet[22 + length..];
                            if extra.len() >= 6 {
                                parse_packet(extra, version_numbers);
                            }
                        }
                    }
                    1 => {
                        if packet.len() >= 18 {
                            parse_packet(&packet[18..], version_numbers);
                        }
                    }
                    _ => panic!("invalid length type_id '{}'", length_type_id),
                }
            }
        }
    }

    extra
}

fn version_checksum(input: &str) -> usize {
    let mut packet = String::new();
    let mut version_numbers = vec![];

    for character in input.chars() {
        packet += hex_to_binary(&character.to_string());
    }

    parse_packet(&packet, &mut version_numbers);

    version_numbers.iter().fold(0, |acc, item| acc + item)
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/jtiP9GMV")?.text()?;

    println!("{}", version_checksum(&input));

    Ok(())
}
