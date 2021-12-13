// d13.rs
use std::fs;

const DAY: &'static str = "d13";

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

	// Day 13 Challenge : ??
	let mut points = std::collections::HashSet::new();
	let mut lines = content.lines();
	
	for line in lines.by_ref().take_while(|&l| l != "") {
		let mut numbers = line.split(",");
		let x = numbers.next().unwrap().parse::<u32>().unwrap();
		let y = numbers.next().unwrap().parse::<u32>().unwrap();

		points.insert((x,y));
	}

	//parse the folds
	for (i,line) in lines.enumerate() {
		let mut input = line.split("=");
		let orientation = input.next().unwrap().chars().last().unwrap();
		let value = input.next().unwrap().parse::<u32>().unwrap();

		if orientation == 'x' {
			fold_vertical(&mut points, value);
		} else {
			fold_horizontal(&mut points, value);
		}

		if i == 0 {
			// Count the first fold
			println!("First Challenge - result: {}", points.len());
		}
	}

	//second part : Fold everything
	let (mut xmax, mut ymax): (u32, u32) = (0,0);

	for &(x,y) in &points {
		if x > xmax {
			xmax = x;
		}
		if y > ymax {
			ymax = y;
		}
	}

	println!("Part 2 : 8 capital letters");

	for y in 0..(ymax+1) {
		for x in 0..(xmax+1) {
			print!("{}", if points.contains(&(x, y)) {"#"} else {" "});
		}
		println!("");
	}

}

fn fold_vertical(points: &mut std::collections::HashSet<(u32,u32)>, axis: u32) -> () {
	// remove the points over the axis and collect them
	// Best solution but experimental api : 
	//let moved_points = points.drain_filter(|&(_,y)| y > axis).collect();

	let moved_points: Vec<(u32,u32)> = points.iter().filter(|&&(x,_)| x > axis).cloned().collect();
	for el in &moved_points {
		points.remove(el);
	}

	// move them back with new coordinates
	for &(x,y) in &moved_points {
		points.insert((2*axis - x, y));
	}

}

fn fold_horizontal(points: &mut std::collections::HashSet<(u32,u32)>, axis: u32) -> () {
	// remove the points over the axis and collect them
	
	// Best solution but experimental api : 
	//let moved_points = points.drain_filter(|&(x,_)| x > axis).collect();

	let moved_points: Vec<(u32,u32)> = points.iter().filter(|&&(_,y)| y > axis).cloned().collect();
	for el in &moved_points {
		points.remove(el);
	}

	// move them back with new coordinates
	for &(x,y) in &moved_points {
		points.insert((x, axis - (y-axis)));
	}
	
}