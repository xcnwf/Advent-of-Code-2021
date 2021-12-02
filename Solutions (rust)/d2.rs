// d1.rs
use std::fs;

const DAY: &'static str = "d2";

fn read_file() -> String {
    print!("Reading input...");

    let content: std::io::Result<String> = fs::read_to_string("../Inputs/".to_owned()+DAY+".txt");

    println!("Done!");

    if let Ok(s) = content {
        s
    } else {
        panic!("File Reading Error! Aborting!")
    }
}

fn main() {
    println!("Day {}\n", DAY);

    let content = read_file();

    // Day 2 Challenge : Multiply horizontal distance by depth.

    let mut h_dist: isize = 0;
    let mut depth: isize = 0;
    let mut depth2: isize = 0;
    for (i, l) in content.lines().enumerate() {
        let res = l.split(" ").collect::<Vec<&str>>();
        let value = res[1].parse::<isize>().unwrap();
        match res[0] {
            "forward" => {h_dist += value; depth2 += value*depth},
            "down" => {depth += value;},
            "up" => {depth -= value;},
            _ => {panic!("Bad input");},
        }
    }

    println!("Chall1 result is : {}", h_dist*depth);
    println!("Chall2 result is : {}", h_dist*depth2);
}