use std::collections::HashMap;

#[derive(Debug)]
enum FoldDirection {
    X,
    Y,
}

#[derive(Debug)]
struct Dot {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Fold {
    direction: FoldDirection,
    value: usize,
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/RwZqCHPF")?.text()?;

    let mut dots: HashMap<(usize, usize), Dot> = HashMap::new();
    let mut folds: Vec<Fold> = vec![];

    for line in input.lines() {
        let item = line.trim();

        if item == "" {
            continue;
        }

        if item.starts_with("fold along") {
            let clear_item = item.replace("fold along ", "");
            let fold: Vec<&str> = clear_item.split("=").collect();

            folds.push(Fold {
                direction: match fold[0] {
                    "x" => FoldDirection::X,
                    "y" => FoldDirection::Y,
                    _ => {
                        panic!("invalid fold");
                    }
                },
                value: fold[1].parse().expect("parse failed"),
            })
        } else {
            let split: Vec<&str> = item.split(",").collect();
            let x = split[0].parse().expect("parse failed");
            let y = split[1].parse().expect("parse failed");

            dots.insert((x, y), Dot { x, y });
        }
    }

    for fold in folds {
        dots = dots.into_iter().fold(
            HashMap::new(),
            |mut acc: HashMap<(usize, usize), Dot>, (id, dot)| {
                let should_fold = match fold.direction {
                    FoldDirection::X => dot.x > fold.value,
                    FoldDirection::Y => dot.y > fold.value,
                };

                if should_fold {
                    let x = match fold.direction {
                        FoldDirection::X => fold.value - (dot.x - fold.value),
                        FoldDirection::Y => dot.x,
                    };
                    let y = match fold.direction {
                        FoldDirection::X => dot.y,
                        FoldDirection::Y => fold.value - (dot.y - fold.value),
                    };

                    acc.insert((x, y), Dot { x, y });
                } else {
                    acc.insert(id, dot);
                }

                acc
            },
        );

        break;
    }

    println!("{}", dots.len());

    Ok(())
}
