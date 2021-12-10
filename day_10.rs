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

    let mut incomplete_lines: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let mut stack: Vec<char> = vec![];

        let mut invalid_character: Option<char> = None;

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
                    invalid_character = Option::from(c);
                    break;
                }
                None => (),
            };
        }

        match invalid_character {
            Some(_) => (),
            None => {
                if stack.len() > 0 {
                    incomplete_lines.push(stack)
                }
            }
        }
    }

    let mut completion_scores: Vec<usize> = incomplete_lines
        .into_iter()
        .map(|mut line| {
            line.reverse();

            line.iter().fold(0, |acc, character| {
                let completion_character = match character {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => {
                        panic!("Invalid character {}", character);
                    }
                };

                match completion_character {
                    ')' => (acc * 5) + 1,
                    ']' => (acc * 5) + 2,
                    '}' => (acc * 5) + 3,
                    '>' => (acc * 5) + 4,
                    _ => {
                        panic!("Invalid character {}", completion_character);
                    }
                }
            })
        })
        .collect();

    completion_scores.sort();

    println!("{:?}", completion_scores[completion_scores.len() / 2]);

    Ok(())
}
