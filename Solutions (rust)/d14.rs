// d14.rs
use std::fs;
use std::collections::HashMap;

const DAY: &'static str = "d14_example";

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
	let mut lines = content.lines();
    let mut polymer : Vec<char> = lines.next().unwrap().chars().collect();

    //skip empty line
    lines.next();

    //parse rules
    let mut rules = HashMap::new();
    for line in lines {
        let mut chars = line.chars();
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();
        let nc = chars.last().unwrap();
        rules.insert((c1,c2), nc);
    }

    for _ in 0..10 {
        round(&mut polymer, &rules);
        println!("{:?}", polymer);
        break;
    }
}

fn round (polymer: &mut Vec<char>, rules: &HashMap<(char, char), char>) {
    let mut new_chars = Vec::new();

    for i in 0..(polymer.len()-1) {
        match rules.get(&(polymer[i], polymer[i+1])) {
            None => (),
            Some(&nc) => new_chars.push((i+1, nc)),
        }
    }

    //insert the new chars
    polymer.reserve(new_chars.len());
    for (j, &(i, nc)) in new_chars.iter().enumerate() {
        polymer.insert(i+j, nc);
    }

}

fn frequency(polymer: &Vec) -> HashMap<char, int> {
    let mut freq = HashMap::new();
    for c in polymer
}