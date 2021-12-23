use colored::*;
use std::collections::HashMap;
use std::io;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn move_amphipod(
    direction: Direction,
    current: (usize, usize),
    amphipods: &mut HashMap<(usize, usize), char>,
    area: &mut HashMap<(usize, usize), char>,
    score: &mut usize,
) -> (usize, usize) {
    let new_position = match direction {
        Direction::Up => (current.0, current.1 - 1),
        Direction::Right => (current.0 + 1, current.1),
        Direction::Down => (current.0, current.1 + 1),
        Direction::Left => (current.0 - 1, current.1),
    };

    let is_free = match area.get(&new_position) {
        Some(field) => *field == '.',
        None => false,
    };

    if !is_free {
        return current;
    }

    area.insert(current, '.');
    area.remove(&new_position);
    let amphipod = amphipods.remove(&current).unwrap();
    amphipods.insert(new_position, amphipod);

    *score += match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("invalid amphipod"),
    };

    new_position
}

fn is_amphipod_in_place(position: (usize, usize), amphipod: char) -> bool {
    let (x, y) = position;

    match amphipod {
        'A' => [(3, 2), (3, 3)].contains(&(x, y)),
        'B' => [(5, 2), (5, 3)].contains(&(x, y)),
        'C' => [(7, 2), (7, 3)].contains(&(x, y)),
        'D' => [(9, 2), (9, 3)].contains(&(x, y)),
        _ => panic!("invalid amphipod"),
    }
}

fn is_amphipod_in_hallway(position: (usize, usize)) -> bool {
    let (x, y) = position;

    y == 1 && (1..12).contains(&x)
}

fn is_amphipod_in_room(position: (usize, usize)) -> bool {
    [
        (3, 2),
        (3, 3),
        (5, 2),
        (5, 3),
        (7, 2),
        (7, 3),
        (9, 2),
        (9, 3),
    ]
    .contains(&position)
}

fn did_win(amphipods: &HashMap<(usize, usize), char>) -> bool {
    let mut did_win = true;

    for ((x, y), amphipod) in amphipods.iter() {
        did_win = is_amphipod_in_place((*x, *y), *amphipod);

        if !did_win {
            break;
        }
    }

    did_win
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/pmTGCZr2")?.text()?;

    let mut area = HashMap::new();
    let mut amphipods = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, field) in line.chars().enumerate() {
            match field {
                '#' => area.insert((x, y), field),
                '.' => area.insert((x, y), field),
                'A' => amphipods.insert((x, y), field),
                'B' => amphipods.insert((x, y), field),
                'C' => amphipods.insert((x, y), field),
                'D' => amphipods.insert((x, y), field),
                ' ' => Option::None,
                _ => panic!("invalid field"),
            };
        }
    }

    let mut selected_amphipod = *amphipods.keys().next().unwrap();
    let mut score = 0;
    let mut last_state = (area.clone(), amphipods.clone(), score, selected_amphipod);
    let mut error_message = "";

    loop {
        print!("\x1B[2J\x1B[1;1H");

        let did_win = did_win(&amphipods);

        for y in 0..5 {
            for x in 0..13 {
                match area.get(&(x, y)) {
                    Some(field) => print!("{}", field.to_string()),
                    None => match amphipods.get(&(x, y)) {
                        Some(field) => {
                            if !did_win && selected_amphipod == (x, y) {
                                print!("{}", field.to_string().red().bold())
                            } else {
                                if is_amphipod_in_place((x, y), *field) {
                                    print!("{}", field.to_string().bold().green())
                                } else {
                                    print!("{}", field.to_string().bold())
                                }
                            }
                        }
                        None => print!(" "),
                    },
                };
            }

            println!();
        }

        println!();
        println!("Score: {}", score);

        if did_win {
            println!();
            println!("You win!!! Congratzz!!");
            break;
        }

        if error_message.len() > 0 {
            println!("Move invalidated: '{}'", error_message.red());

            error_message = "";
        }

        println!("Use (W,A,S,D) keys to select amphipods");
        println!("Press Enter to switch active amphipod");

        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        match selection.to_uppercase().as_str().trim() {
            "" => {
                if [(3, 1), (5, 1), (7, 1), (9, 1)].contains(&selected_amphipod) {
                    error_message =
                        "Amphipods will never stop on the space immediately outside any room"
                }

                if score != last_state.2
                    && is_amphipod_in_room(selected_amphipod)
                    && !is_amphipod_in_place(selected_amphipod, amphipods[&selected_amphipod])
                {
                    error_message =
                        "Amphipods will never move from the hallway into a room unless that room is their destination room and that room contains no amphipods which do not also have that room as their own destination."
                }

                if score != last_state.2
                    && is_amphipod_in_hallway(last_state.3)
                    && is_amphipod_in_hallway(selected_amphipod)
                {
                    error_message ="Once an amphipod stops moving in the hallway, it will stay in that spot until it can move into a room."
                }

                let current = amphipods
                    .keys()
                    .position(|key| *key == selected_amphipod)
                    .unwrap();

                let is_valid_move = error_message.len() == 0;

                if is_valid_move {
                    last_state = (area.clone(), amphipods.clone(), score, selected_amphipod);
                } else {
                    area = last_state.0.clone();
                    amphipods = last_state.1.clone();
                    score = last_state.2;
                }

                selected_amphipod = *amphipods
                    .keys()
                    .enumerate()
                    .find(|(index, ..)| *index == (current + 1) % amphipods.len())
                    .unwrap()
                    .1;

                last_state.3 = selected_amphipod
            }
            "W" => {
                selected_amphipod = move_amphipod(
                    Direction::Up,
                    selected_amphipod,
                    &mut amphipods,
                    &mut area,
                    &mut score,
                );
            }
            "D" => {
                selected_amphipod = move_amphipod(
                    Direction::Right,
                    selected_amphipod,
                    &mut amphipods,
                    &mut area,
                    &mut score,
                );
            }
            "S" => {
                selected_amphipod = move_amphipod(
                    Direction::Down,
                    selected_amphipod,
                    &mut amphipods,
                    &mut area,
                    &mut score,
                );
            }
            "A" => {
                selected_amphipod = move_amphipod(
                    Direction::Left,
                    selected_amphipod,
                    &mut amphipods,
                    &mut area,
                    &mut score,
                );
            }
            "U" => {
                area = last_state.0.clone();
                amphipods = last_state.1.clone();
                score = last_state.2;
                selected_amphipod = last_state.3
            }
            _ => (),
        }
    }

    Ok(())
}
