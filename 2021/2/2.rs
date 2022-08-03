use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input = input.split("\n").collect::<Vec<&str>>();
    let mut stats = [0, 0, 0];

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
                    stats[0] = stats[0] + v;
                    stats[1] = stats[1] + stats[2] * v;
                }
                "down" => {
                    stats[2] = stats[2] + v;
                }
                "up" => {
                    stats[2] = stats[2] - v;
                }
                _ => {}
            }
            println!(
                "Hz Pos: {}, Depth: {}, Aim: {}",
                stats[0], stats[1], stats[2]
            );
        }
    }
}
