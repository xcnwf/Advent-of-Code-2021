// d10.rs
use std::fs;

const DAY: &'static str = "d10";

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

	// Day 10 Challenge : Find the incompletes and corrupted strings.

	/*
    ): 3 points.
    ]: 57 points.
    }: 1197 points.
    >: 25137 points.
	*/
	let mut corrupted_lookup = std::collections::HashMap::new();
	corrupted_lookup.insert(')', 3);
	corrupted_lookup.insert(']', 57);
	corrupted_lookup.insert('}', 1197);
	corrupted_lookup.insert('>', 25137);


	/*
    ): 1 point.
    ]: 2 points.
    }: 3 points.
    >: 4 points.
	*/
	let mut incomplete_lookup = std::collections::HashMap::new();
	incomplete_lookup.insert('(', 1);
	incomplete_lookup.insert('[', 2);
	incomplete_lookup.insert('{', 3);
	incomplete_lookup.insert('<', 4);

	let mut incomplete_scores = Vec::new();

	let mut score = 0;
	for line in content.lines() {
		let mut chunk_starts: Vec<char> = Vec::new();
		let mut corrupted = false;
		for c in line.chars() {
			if ['(', '{', '[', '<'].contains(&c) {
				chunk_starts.push(c);
			} else {
				let last_open = chunk_starts.pop().unwrap();
				match (last_open, c) {
					('(',')') => (),
					('{','}') => (),
					('[',']') => (),
					('<','>') => (),
					_ => {
						score += corrupted_lookup[&c]; 
						corrupted = true;
						break;
					},
				}
			}
		}
		if !corrupted { //incomplete
			incomplete_scores.push( chunk_starts.iter().rev().fold(0 as u64,|score, &c| (score * 5 as u64) + incomplete_lookup[&c] as u64) )
		}
	}

	println!("Fist challenge - result: {}", score);

	incomplete_scores.sort();

	println!("Second challenge - result: {}", incomplete_scores[incomplete_scores.len() / 2])
}