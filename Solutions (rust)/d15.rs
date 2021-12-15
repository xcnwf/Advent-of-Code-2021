// d14.rs
use std::fs;
use std::collections::HashMap;

const DAY: &'static str = "d15_example";

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

	// Day 14 Challenge : ??
    let mut matrix = Vec::new();
    for line in content.lines() {
        let vec_line: Vec<(u8,bool)> = line.chars().map(|&c| (c.to_digit(10),false)).collect();
        matrix.push(vec_line);
    }

    //part1 - Dijkstra
    let mut trail = vec![((0,0),0)]
    let mut next = std::collection::HashSet::new()

    loop {
        let (curr_pos, cost) = trail.last().unwrap();
        if curr == (99,99) {
            break;
        }

        for i in [-1,1] {
            next.insert(())
        }

    }
}