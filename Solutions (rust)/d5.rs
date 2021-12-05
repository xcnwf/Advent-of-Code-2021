// d5.rs
use std::fs;

const DAY: &'static str = "d5";

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


//vertical or horizontal lines
//type Line = (usize, (usize, usize));

// Point = (coordinate x, coordinate y)
type Point = (usize, usize);

fn main() {
	println!("Day {}\n", DAY);

	let content = read_file();

	// Day 5 Challenge : Count the number of times the next int is bigger than the current one

	//Parsing input

	//overlaps is the list of points with their overlap count > 0
	let mut overlaps: std::collections::HashMap<Point, usize> = std::collections::HashMap::new();
	for line in content.lines() {
		let mut points = line.split(" -> ");
		let mut end_points: [(usize, usize);2] = [(0,0); 2];
		for i in 0..2 {
			let mut coordinate_iter = points.next().unwrap().split(',');
			let c1 = coordinate_iter.next().unwrap().parse::<usize>().unwrap();
			let c2 = coordinate_iter.next().unwrap().parse::<usize>().unwrap();
			end_points[i] = (c1,c2);
		}

		//Vertical lines
		if end_points[0].0 == end_points[1].0 {
			let mut c1 = end_points[0].1;
			let mut c2 = end_points[1].1;
			if c1 > c2 {let ctemp = c1; c1 = c2; c2 = ctemp;}
			for i in c1..(c2+1) {
				let point = (end_points[0].0,i);
				if !overlaps.contains_key(&point) {
					overlaps.insert(point, 1);
				} else {
					*overlaps.get_mut(&point).unwrap() += 1;
				}
			}
		
		// Horizontal lines
		} else if end_points[0].1 == end_points[1].1 {
			let mut c1 = end_points[0].0;
			let mut c2 = end_points[1].0;
			if c1 > c2 {let ctemp = c1; c1 = c2; c2 = ctemp;}
			for i in c1..(c2+1) {
				let point = (i,end_points[0].1);
				if !overlaps.contains_key(&point) {
					overlaps.insert(point, 1);
				} else {
					*overlaps.get_mut(&point).unwrap() += 1;
				}
			}
		}
	}

	//PARSING DONE!

	//part1 : For each overlap point, check if overlap > 1

	let n_overlaps = overlaps.values().filter(|&c| (*c)>1).count();
	println!("First challenge - Result: {}", n_overlaps);
}