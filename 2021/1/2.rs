use std::{cmp::Ordering, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let input = input.split("\n").collect::<Vec<&str>>();
    let mut sums: Vec<i32> = Vec::new();

    for i in 0..input.len() {
        if i < input.len() - 3 {
            let num: i32 = input[i].parse().expect("Unable to parse");

            let next_num: i32 = input[i + 1].parse().expect("Unable to parse");
            let next_next_num: i32 = input[i + 2].parse().expect("Unable to parse");

            sums.push(num + next_num + next_next_num);

            if i == 0 {
                println!("{} (N/A - no previous measurement)", sums[0]);
            }
        }
    }

    for i in 0..sums.len() {
        if i > 0 {
            match sums[i].cmp(&sums[i - 1]) {
                Ordering::Greater => {
                    println!("{} (increased)", sums[i]);
                }
                Ordering::Less => {
                    println!("{} (decreased)", sums[i]);
                }
                Ordering::Equal => {
                    println!("{} (equal)", sums[i]);
                }
            }
        }
    }
}
