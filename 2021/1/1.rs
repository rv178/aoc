use std::{cmp::Ordering, fs, process::exit};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input = input.split("\n").collect::<Vec<&str>>();

    for i in 0..input.len() {
        if input[i].is_empty() {
            exit(1);
        }

        let num: i32 = input[i].parse().expect("Unable to parse");

        if i == 0 {
            println!("{} (N/A - no previous measurement)", num);
        } else {
            let prev_num: i32 = input[i - 1].parse().expect("Unable to parse");

            match num.cmp(&prev_num) {
                Ordering::Greater => {
                    println!("{} (increased)", num);
                }
                Ordering::Less => {
                    println!("{} (decreased)", num);
                }
                Ordering::Equal => {
                    println!("{} (equal)", num);
                }
            }
        }
    }
}
