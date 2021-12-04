#[derive(Debug)]
struct Field {
    number: i32,
    is_drawn: bool,
}

impl Field {
    fn mark(&self) -> Field {
        Field {
            number: self.number,
            is_drawn: true,
        }
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/kmuV3FLV")?.text()?;

    let mut lines: Vec<&str> = input.lines().collect();

    let numbers: Vec<i32> = lines
        .remove(0)
        .split(",")
        .map(|s| s.parse().expect("Not a number"))
        .collect();

    let mut boards: Vec<Vec<Vec<Field>>> = lines.iter().fold(Vec::new(), |mut acc, line| {
        if line.trim().len() == 0 {
            acc.push(Vec::new());

            acc
        } else {
            let line_number: Vec<i32> =
                line.trim()
                    .split(" ")
                    .fold(Vec::new(), |mut acc, s| match s.trim().parse() {
                        Ok(num) => {
                            acc.push(num);

                            acc
                        }
                        Err(_) => acc,
                    });

            let mut board = acc.remove(acc.len() - 1);
            let mut board_line = Vec::new();

            for number in line_number.iter() {
                board_line.push(Field {
                    number: number.to_owned(),
                    is_drawn: false,
                })
            }

            board.push(board_line);
            acc.push(board);

            acc
        }
    });

    for number in numbers.into_iter() {
        boards = boards
            .into_iter()
            .map(|board| {
                board
                    .into_iter()
                    .map(|line| {
                        line.into_iter()
                            .map(|field| {
                                if field.number == number {
                                    field.mark()
                                } else {
                                    field
                                }
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        let mut break_it = false;

        for (index, board) in boards.iter().enumerate() {
            for (y, line) in board.iter().enumerate() {
                let mut is_win = true;

                for field in line.iter() {
                    if field.is_drawn == false {
                        is_win = false;
                    }
                }

                if is_win {
                    break_it = true;

                    break;
                }

                is_win = true;

                for y in 0..board[0].len() {
                    for x in 0..board[y].len() {
                        if board[x][y].is_drawn == false {
                            is_win = false;
                        }
                    }
                }

                if is_win {
                    println!("{} {:?}", index, board[y]);
                    break_it = true;

                    break;
                }
            }

            if break_it {
                let mut sum = 0;

                for y in 0..board[0].len() {
                    for x in 0..board[y].len() {
                        if board[x][y].is_drawn == false {
                            sum += board[x][y].number;
                        }
                    }
                }

                println!("{}", sum * number);

                break;
            }
        }

        if break_it {
            break;
        }
    }

    Ok(())
}
