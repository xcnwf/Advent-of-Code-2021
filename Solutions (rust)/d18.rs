// d18.rs
use std::fs;

const DAY: &'static str = "d18";

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

	// Day 18 Challenge : ??
    let mut lines = content.lines();

    let first_line = lines.next().unwrap();

    let (mut snail_num,_) = parse(first_line);

    while let Some(line) = lines.next() {
        snail_num = addition(snail_num, parse(line).0);
    }

    println!("First challenge - result: {}", get_magnitude(&snail_num));


    //Part2 : Max magnitude from sum of 2 snail nums.
    let mut max_mag = 0;
    let mut i = 0;
    loop {
        let mut lines = content.lines();
        if let Some(line_start) = lines.nth(i) {
            while let Some(line) = lines.next() {
                let (snail_num,_) = parse(line_start);
                let num2 = parse(line).0;
                let mag = get_magnitude(&addition(snail_num, num2));
                max_mag = u32::max(max_mag, mag);
            }
        } else {
            break;
        }
        i += 1;
    }
    // loop on y + x
    loop {
        let mut lines = content.lines();
        if let Some(line_start) = lines.nth(i) {
            while let Some(line) = lines.next() {
                let (snail_num,_) = parse(line_start);
                let num = parse(line).0;
                let mag = get_magnitude(&addition(num, snail_num));
                max_mag = u32::max(max_mag, mag);
            }
        } else {
            break
        }
        i += 1;
    }

    println!("Second challenge - result : {}", max_mag);
}


// So this will be either an uint or a pair
enum Element {
    Value(u32),
    Pair(Box<Element>, Box<Element>)
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Value(n) => write!(f, "{}", n),
            Element::Pair(bel1,bel2) => write!(f, "[{},{}]", *bel1, *bel2),
        }
    }
}

fn parse(line: &str) -> (Element, &str) {
    if line.chars().next().unwrap() == '[' {
        let (el1, line) = parse(&line[1..]);
        //skip the comma
        let (el2, line) = parse(&line[1..]);
        return (Element::Pair(Box::new(el1), Box::new(el2)), &line[1..]);
    } else {
        let mut i = 0usize;
        let mut chars = line.chars();
        while chars.next().unwrap().is_digit(10) {
            i+=1;
        }
        return (Element::Value((&line[..i]).parse::<u32>().unwrap()), &line[i..]);
    }
}

fn addition(e1: Element, e2: Element) -> Element {
    let mut ne = Element::Pair(Box::new(e1), Box::new(e2));

    while reduce(&mut ne, 1) {};

    ne
}

fn reduce(e: &mut Element, depth: usize) -> bool {
    // to reduce, first we explode
    explode(e, depth).is_some()
    // Then we split if we didn't explode
        || split(e)
}

fn split(e: &mut Element) -> bool {
    match e {
        Element::Value(x) => if *x >= 10 {
            *e = Element::Pair(
                Box::new(Element::Value(*x/2)),
                Box::new(Element::Value((*x+1)/2))
            ); true} else {false},
        // No need to dereference, it is done automatically with a box (impl DerefMut)
        Element::Pair(e1, e2) => split(e1) || split(e2),
    }
}

fn explode(e: &mut Element, depth: usize) -> Option<(u32, u32)> {
    match e {
        Element::Value(_) => None,
        Element::Pair(e1, e2) => if depth >= 5 {
            match (&**e1, &**e2) { 
                // Store the result.
                (&Element::Value(n1), &Element::Value(n2)) => {
                    //And delete the pair.
                    *e = Element::Value(0);
                    Some((n1,n2))
                }
                _ => None
            }
        } else {
            // Get the result of explode.
            if let Some((n1,n2)) = explode(e1, depth+1) {
                //If something did explode and was not already treated,
                //we try to add the value to the right (as we are left)
                if n2 != 0 && increase_value(e2, n2, false) {
                    // If we could add the value, then we change it to 0
                    Some((n1,0))
                } else {Some((n1,n2))}
            // If it didn't, try the other side.
            } else if let Some((n1,n2)) = explode(e2, depth+1) {
                if n1 != 0 && increase_value(e1, n1, true) {
                    Some((0,n2))
                } else { Some((n1,n2)) }
            } else {
                None
            }
        },
    }
}

fn increase_value(e: &mut Element, val: u32, from_right_side: bool) -> bool {
    match e {
        // If we are a value, add the value and return true
        Element::Value(x) => {
            *e = Element::Value(*x+val);
            true
        },
        // Else, try a side first then the other, depending on the boolean value
        Element::Pair(e1, e2) => {
            if from_right_side {
                increase_value(e2, val, from_right_side)
                    || increase_value(e1, val, from_right_side)
            } else {
                increase_value(e1, val, from_right_side)
                    || increase_value(e2, val, from_right_side)
            }
        }
    }
}

fn get_magnitude(e: &Element) -> u32 {
    match e {
        // If we are a value, add the value and return true
        Element::Value(x) => { *x
        },
        // Else, try a side first then the other, depending on the boolean value
        Element::Pair(e1, e2) => {
            3*get_magnitude(e1) + 2*get_magnitude(e2)
        }
    }
}