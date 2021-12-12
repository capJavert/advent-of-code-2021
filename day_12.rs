use std::collections::HashMap;

fn find_path(
    next: &String,
    path: &mut Vec<String>,
    paths: &mut Vec<Vec<String>>,
    connections: &HashMap<String, Vec<String>>,
) {
    path.push(String::from(next));

    if next == "end" {
        paths.push(path.to_vec());
    }

    let next_paths = connections.get(next).unwrap();

    for next_path in next_paths {
        if &next_path.to_uppercase() != next_path && path.contains(next_path) {
            continue;
        }

        find_path(next_path, &mut path.to_vec(), paths, connections);
    }
}

fn main() -> Result<(), reqwest::Error> {
    let input = reqwest::blocking::get("https://pastebin.com/raw/vJRP4SEh")?.text()?;

    let paths: Vec<(String, String)> = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.trim().split("-").collect();

            (String::from(split[0]), String::from(split[1]))
        })
        .collect();

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for path in paths.iter() {
        let (start, end) = path;

        match connections.get_mut(start) {
            Some(items) => {
                items.push(String::from(end));
            }
            None => {
                connections.insert(String::from(start), vec![String::from(end)]);
            }
        }

        match connections.get_mut(end) {
            Some(items) => {
                items.push(String::from(start));
            }
            None => {
                connections.insert(String::from(end), vec![String::from(start)]);
            }
        }
    }

    let mut paths = vec![];

    find_path(
        &String::from("start"),
        &mut vec![],
        &mut paths,
        &connections,
    );

    println!("{:?}", paths.len());

    Ok(())
}
