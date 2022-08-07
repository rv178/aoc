use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut input = input.split("\n").collect::<Vec<&str>>();
    let ilen = input[0].len();
    input.remove(input.len() - 1);

    let mut gamma: Vec<char> = Vec::new();
    for item in input {
        for char in item.chars() {
            gamma.push(char);
        }
    }

    let mut gstr: Vec<String> = Vec::new();
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    let mut cindex = 0;
    let gamma = vec_to_2d_vec(gamma, ilen);

    for _i in 0..gamma[0].len() {
        for (y, row) in gamma.iter().enumerate() {
            if cindex > row.len() - 1 {
                cindex = 0;
            }
            gstr.push(row[cindex].to_string());
            if y == gamma.len() - 1 {
                cindex += 1;
            }
        }
        gamma_rate.push_str(&recurring(&gstr, true).to_string());
        epsilon_rate.push_str(&recurring(&gstr, false).to_string());
        gstr.clear();
    }

    let gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("Epsilon: {}", epsilon_rate);
    println!("Gamma: {}", gamma_rate);
    println!("Power consumption: {}", gamma_rate * epsilon_rate);
}

fn recurring(array: &Vec<String>, is_gamma: bool) -> u8 {
    let mut rmap = HashMap::new();
    let mut count_0 = 0;
    let mut count_1 = 0;
    for item in array {
        let item = item.parse::<u8>().unwrap();
        match item {
            0 => {
                rmap.insert(item, count_0);
                count_0 += 1
            }
            1 => {
                rmap.insert(item, count_1);
                count_1 += 1
            }
            _ => {}
        }
    }

    if is_gamma {
        match rmap.get(&0).cmp(&rmap.get(&1)) {
            Ordering::Equal => 1,
            Ordering::Less => 1,
            Ordering::Greater => 0,
        }
    } else {
        match rmap.get(&0).cmp(&rmap.get(&1)) {
            Ordering::Equal => 0,
            Ordering::Less => 0,
            Ordering::Greater => 1,
        }
    }
}

fn vec_to_2d_vec(vec: Vec<char>, len: usize) -> Vec<Vec<char>> {
    let mut vec_2d: Vec<Vec<char>> = Vec::new();
    let mut vec_1d: Vec<char> = Vec::new();
    for item in vec {
        vec_1d.push(item);
        if vec_1d.len() == len {
            vec_2d.push(vec_1d);
            vec_1d = Vec::new();
        }
    }
    vec_2d
}
