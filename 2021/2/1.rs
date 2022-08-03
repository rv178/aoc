use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input = input.split("\n").collect::<Vec<&str>>();
    let mut coords = [0, 0];

    for i in input {
        let mut commands: HashMap<&str, i32> = HashMap::new();
        let mut cmds: Vec<&str> = Vec::new();
        let split = i.split(|c: char| c.is_whitespace());

        for s in split {
            cmds.push(s);
        }

        for i in 0..cmds.len() {
            if i + 1 < cmds.len() {
                if i % 2 == 0 {
                    commands.insert(
                        cmds[i],
                        cmds[i + 1].parse::<i32>().expect("Failed to parse"),
                    );
                }
            }
        }

        for (k, v) in commands {
            match k {
                "forward" => {
                    coords[0] = coords[0] + v;
                }
                "down" => {
                    coords[1] = coords[1] + v;
                }
                "up" => {
                    coords[1] = coords[1] - v;
                }
                _ => {}
            }
            println!("Hz Pos: {}, Depth: {}", coords[0], coords[1]);
        }
    }
}
