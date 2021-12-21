use regex::Regex;

fn roll_distribution(roll: usize) -> usize {
    match roll {
        3 => 1, // 111
        4 => 3, // 112, 121, 211
        5 => 6, // 113, 131, 311, 122, 212, 221
        6 => 7, // 123, 132, 213, 231, 312, 321, 222
        7 => 6, // 223, 232, 322, 133, 313, 331
        8 => 3, // 233, 323, 332
        9 => 1, // 333
        _ => panic!("invalid sum"),
    }
}

fn play_game(player1: (usize, usize), player2: (usize, usize), turn: bool) -> (usize, usize) {
    if player1.1 >= 21 {
        (1, 0)
    } else if player2.1 >= 21 {
        (0, 1)
    } else {
        let mut wins = (0, 0);

        for roll in 3..10 {
            let mut new_player1 = (player1.0, player1.1);
            let mut new_player2 = (player2.0, player2.1);

            if turn {
                new_player1.0 = ((new_player1.0 - 1 + roll) % 10) + 1;
                new_player1.1 += new_player1.0;
            } else {
                new_player2.0 = ((new_player2.0 - 1 + roll) % 10) + 1;
                new_player2.1 += new_player2.0;
            }

            let universes = roll_distribution(roll);

            let game_result = play_game(new_player1, new_player2, !turn);
            wins.0 += universes * game_result.0;
            wins.1 += universes * game_result.1;
        }

        wins
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/1HXUkXy1")?.text()?;

    let position_regex = Regex::new(r"^.*(?P<position>[0-9]{1,})$").unwrap();
    let players: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let matches = position_regex.captures(line.trim()).unwrap();

            let position: usize = matches
                .name("position")
                .unwrap()
                .as_str()
                .parse()
                .expect("parse failed");
            (position, 0)
        })
        .collect();

    let wins = play_game(players[0], players[1], true);

    println!("{:?}", std::cmp::max(wins.0, wins.1));

    Ok(())
}
