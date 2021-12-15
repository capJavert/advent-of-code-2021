use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl State {
    fn get_neighbors(&self, grid: &HashMap<(usize, usize), usize>) -> Vec<(usize, usize)> {
        let (x, y) = self.position;
        let adjacent_cells = [
            (y.wrapping_sub(1), x),
            (y, x + 1),
            (y + 1, x),
            (y, x.wrapping_sub(1)),
        ];
        adjacent_cells.into_iter().fold(vec![], |mut acc, coords| {
            match grid.get(&(coords.1, coords.0)) {
                Some(_) => {
                    acc.push((coords.1, coords.0));
                }
                None => (),
            }
            acc
        })
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/w3L6EjTC")?.text()?;

    let mut rows: Vec<Vec<usize>> = vec![];

    for step in 0..5 {
        for line in input.lines() {
            let row: Vec<usize> = line
                .trim()
                .chars()
                .map(|c| {
                    let cell: usize = c.to_string().parse().expect("parse failed");
                    let mut incremented_cell = cell + step;

                    if incremented_cell > 9 {
                        incremented_cell -= 9;
                    }

                    incremented_cell
                })
                .collect();

            rows.push(row);
        }
    }

    for (y, row) in rows.to_vec().iter().enumerate() {
        for step in 1..5 {
            for cell in row {
                let mut incremented_cell = cell + step;

                if incremented_cell > 9 {
                    incremented_cell -= 9;
                }

                rows[y].push(incremented_cell)
            }
        }
    }

    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();

    for (y, row) in rows.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            grid.insert((x, y), *cell);
        }
    }

    let start_position = (0, 0);
    let start = State {
        cost: 0,
        position: start_position,
    };
    let end_position = (rows[0].len() - 1, rows.len() - 1);
    let end = State {
        cost: grid[&end_position],
        position: (rows[0].len() - 1, rows.len() - 1),
    };
    let mut frontier = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();

    frontier.push(start);
    came_from.insert(start.position, Option::None);
    cost_so_far.insert(start.position, 0);

    while frontier.len() > 0 {
        let current = frontier.pop().unwrap();

        if current.position == end.position {
            break;
        }

        for next in current.get_neighbors(&grid) {
            let cost = grid[&next];
            let new_cost = cost_so_far[&current.position] + cost;

            if !came_from.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost;

                frontier.push(State {
                    cost: priority,
                    position: next,
                });
                came_from.insert(next, Option::from(current));
            }
        }
    }

    let mut current = end.position;
    let mut cost = 0;

    while current != start.position {
        cost += grid[&current];
        current = came_from[&current].unwrap().position
    }

    println!("{}", cost);

    Ok(())
}
