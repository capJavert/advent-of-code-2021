fn check_chunk_open(character: char, stack: &mut Vec<char>) -> Option<char> {
    stack.push(character);

    None
}

fn check_chunk_close(
    character: char,
    check_character: char,
    stack: &mut Vec<char>,
) -> Option<char> {
    match stack.pop() {
        Some(chunk_character) => {
            if chunk_character != check_character {
                Some(character)
            } else {
                None
            }
        }
        None => Some(character),
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/AMq24KMw")?.text()?;

    let mut invalid_characters: Vec<char> = vec![];

    for line in input.lines() {
        let mut stack: Vec<char> = vec![];

        for character in line.trim().chars() {
            let checked_character = match character {
                '(' => check_chunk_open(character, &mut stack),
                '[' => check_chunk_open(character, &mut stack),
                '{' => check_chunk_open(character, &mut stack),
                '<' => check_chunk_open(character, &mut stack),
                ')' => check_chunk_close(character, '(', &mut stack),
                ']' => check_chunk_close(character, '[', &mut stack),
                '}' => check_chunk_close(character, '{', &mut stack),
                '>' => check_chunk_close(character, '<', &mut stack),
                _ => {
                    panic!("Invalid character {}", character);
                }
            };

            match checked_character {
                Some(c) => {
                    invalid_characters.push(c);
                    break;
                }
                None => (),
            };
        }

        // TODO incomplete lines
        // if stack.len() > 0 {
        //     println!("Invalid line trailing characters {:?}", stack);
        // }
    }

    let error_score = invalid_characters
        .iter()
        .fold(0, |acc, character| match character {
            ')' => acc + 3,
            ']' => acc + 57,
            '}' => acc + 1197,
            '>' => acc + 25137,
            _ => {
                panic!("Invalid character {}", character);
            }
        });

    println!("{}", error_score);

    Ok(())
}
