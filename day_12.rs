use std::collections::HashMap;

fn find_path(
    next: &String,
    path: &mut Vec<String>,
    paths: &mut Vec<Vec<String>>,
    connections: &HashMap<String, Vec<String>>,
    did_use_shortcut: bool,
) {
    path.push(String::from(next));

    if next == "end" {
        paths.push(path.to_vec());
        ()
    } else {
        let next_paths = connections.get(next).unwrap();

        for next_path in next_paths {
            let mut will_use_shortcut = did_use_shortcut;

            if next_path == &String::from("start") && path.contains(&String::from("start")) {
                continue;
            }
            if &next_path.to_uppercase() != next_path && path.contains(next_path) {
                if did_use_shortcut {
                    continue;
                } else {
                    will_use_shortcut = true;
                }
            }

            find_path(
                next_path,
                &mut path.to_vec(),
                paths,
                connections,
                will_use_shortcut,
            );
        }
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
        false,
    );

    println!("{:?}", paths.len());

    Ok(())
}
