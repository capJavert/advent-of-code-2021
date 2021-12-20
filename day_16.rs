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

fn parse_packet<'a>(packet: &'a str) -> (usize, (usize, usize)) {
    let mut value = 0;
    let mut end_index = 0;

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
                        end_index = 6 + index + 5;
                        value = binary_to_decimal(&literal);
                        break;
                    }
                    _ => panic!("invalid literal group prefix '{}'", character),
                }
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
                let mut sub_packets = vec![];

                match length_type_id {
                    0 => {
                        if packet.len() >= 22 {
                            let length = binary_to_decimal(&packet[7..22]);
                            let mut index = 22;

                            while index < 22 + length {
                                let (sub_index, sub_packet) = parse_packet(&packet[index..]);
                                index += sub_index;
                                sub_packets.push(sub_packet);
                            }

                            end_index = index;
                        }
                    }
                    1 => {
                        let length = binary_to_decimal(&packet[7..18]);
                        let mut index = 18;
                        let mut count = 0;

                        while count < length {
                            let (sub_index, sub_packet) = parse_packet(&packet[index..]);
                            index += sub_index;
                            count += 1;
                            sub_packets.push(sub_packet);
                        }

                        end_index = index;
                    }
                    _ => panic!("invalid length type_id '{}'", length_type_id),
                }

                value = match type_id {
                    0 => sub_packets.iter().map(|s| s.1).sum(),
                    1 => sub_packets.iter().map(|s| s.1).product(),
                    2 => sub_packets.iter().map(|s| s.1).min().unwrap(),
                    3 => sub_packets.iter().map(|s| s.1).max().unwrap(),
                    5 => {
                        if sub_packets[0].1 > sub_packets[1].1 {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if sub_packets[0].1 < sub_packets[1].1 {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if sub_packets[0].1 == sub_packets[1].1 {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("invalid type_id {}", type_id),
                };
            }
        }
    }

    (end_index, (type_id, value))
}

fn packet_value(input: &str) -> usize {
    let mut packet = String::new();

    for character in input.chars() {
        packet += hex_to_binary(&character.to_string());
    }

    parse_packet(&packet).1 .1
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/jtiP9GMV")?.text()?;
    println!("{}", packet_value(&input));

    Ok(())
}
