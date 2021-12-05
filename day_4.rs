#[derive(Debug, Clone)]
struct Field {
    number: usize,
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

    let mut numbers: Vec<usize> = lines
        .remove(0)
        .split(",")
        .map(|s| s.parse().expect("Not a number"))
        .collect();

    let mut board_id = 0;

    let mut boards: Vec<Vec<Vec<Field>>> = lines.iter().fold(Vec::new(), |mut acc, line| {
        if line.trim().len() == 0 {
            acc.push(Vec::new());
            board_id += 1;

            acc
        } else {
            let line_number: Vec<usize> =
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

    let mut board_wins: Vec<usize> = Vec::new();
    let mut last_number = 0;

    while numbers.len() > 0 && board_wins.len() < boards.len() {
        let number = numbers.remove(0);

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

        for (index, board) in boards.to_vec().iter().enumerate() {
            if board_wins.contains(&index) {
                continue;
            }

            let mut is_win = true;

            for y in 0..board[0].len() {
                is_win = true;

                for x in 0..board[y].len() {
                    if board[y][x].is_drawn == false {
                        is_win = false;
                    }
                }

                if is_win {
                    break;
                }
            }

            if !is_win {
                for y in 0..board[0].len() {
                    is_win = true;
                    for x in 0..board[y].len() {
                        if board[x][y].is_drawn == false {
                            is_win = false;
                        }
                    }

                    if is_win {
                        break;
                    }
                }
            }

            if is_win {
                board_wins.push(index);
            }
        }

        last_number = number;
    }

    let mut sum = 0;
    let board = boards.get(board_wins.pop().unwrap()).unwrap();

    for y in 0..board[0].len() {
        for x in 0..board[y].len() {
            if board[x][y].is_drawn == false {
                sum += board[x][y].number;
            }
        }
    }

    println!("{}", sum * last_number);

    Ok(())
}
