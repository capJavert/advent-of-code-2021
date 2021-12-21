use regex::Regex;

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/1HXUkXy1")?.text()?;

    let position_regex = Regex::new(r"^.*(?P<position>[0-9]{1,})$").unwrap();
    let mut players: Vec<(usize, usize)> = input
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

    let mut dice = 0;
    let mut roll_count = 0;

    'game: while players.iter().all(|player| player.1 < 1000) {
        for (index, player) in players.to_vec().iter().enumerate() {
            let moves = (0..3).fold(0, |acc, _| {
                dice += 1;
                dice = if dice % 100 == 0 { 100 } else { dice % 100 };

                acc + dice
            });
            roll_count += 3;

            let new_position = player.0 + moves;
            let position = if new_position % 10 == 0 {
                10
            } else {
                new_position % 10
            };
            let points = player.1 + position;

            players[index] = (position, points);

            if points >= 1000 {
                break 'game;
            }
        }
    }

    println!(
        "{}",
        roll_count * players.iter().find(|player| player.1 < 1000).unwrap().1
    );

    Ok(())
}
