use regex::Regex;

#[derive(Debug)]
struct Command {
    name: String,
    arg1: i32,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    debug: bool,
}

impl Position {
    fn init(debug: bool) -> Position {
        Position { x: 0, y: 0, debug }
    }

    fn forward(&mut self, arg1: i32) {
        self.x += arg1;

        if self.debug {
            println!("forward {} => {}", arg1, self.x)
        }
    }

    fn down(&mut self, arg1: i32) {
        self.y += arg1;

        if self.debug {
            println!("down {} => {}", arg1, self.y)
        }
    }

    fn up(&mut self, arg1: i32) {
        self.y -= arg1;

        if self.debug {
            println!("up {} => {}", arg1, self.y)
        }
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/DM51vs8P")?.text()?;
    let command_match = Regex::new(r"^(?P<name>[a-z]{1,})\s(?P<arg1>[0-9]{1,})$").unwrap();

    let commands: Vec<Command> = input
        .trim()
        .lines()
        .map(|s| {
            let matches = command_match.captures(s.trim()).unwrap();
            let command = Command {
                name: String::from(matches.name("name").unwrap().as_str()),
                arg1: matches
                    .name("arg1")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("arg1 should be a number"),
            };

            command
        })
        .collect();

    let mut position = Position::init(false);

    for command in commands.iter() {
        match command.name.trim() {
            "forward" => {
                position.forward(command.arg1);
            }
            "down" => {
                position.down(command.arg1);
            }
            "up" => {
                position.up(command.arg1);
            }
            _ => {
                panic!("Command '{}' is not supported", command.name)
            }
        }
    }

    println!("{}", position.x * position.y);

    Ok(())
}
