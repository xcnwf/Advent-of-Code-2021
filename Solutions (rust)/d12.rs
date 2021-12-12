// d12.rs
use std::fs;

const DAY: &'static str = "d12";

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

	// Day 12 Challenge : ??

    let mut links: std::collections::HashMap<&str, Vec<&str>>  = std::collections::HashMap::new();

    for line in content.lines() {
        let mut spots = line.split("-");
        let start = spots.next().unwrap();
        if !links.contains_key(&start) {
            links.insert(start, Vec::new());
        }
        let end = spots.next().unwrap();
        if !links.contains_key(end) {
            links.insert(end, Vec::new());
        }

        links.get_mut(start).unwrap().push(end);
        links.get_mut(end).unwrap().push(start);
    }

    // part 1;
    let mut visits: Vec<Vec<&str>> = Vec::new();
    let start = vec!("start");
    visits.push(start);

    let mut total_paths: usize = 0;

    // while there are still things to visit.
    // /!\ no need to do further checks : big caves are never linked together,
    // removing any risk of infinite looping
    while visits.len() > 0 {
        let curr_visit: Vec<&str> = visits.pop().unwrap();
        for &cave in links[curr_visit.last().unwrap()].iter() {
            // if we reached the end, count the path
            if cave == "end" {
                total_paths += 1;
                continue;
            }

            // skip the small caves already visited
            if is_lowercase(cave) {
                if curr_visit.contains(&cave){
                    continue;
                }
            }

            //add all the new routes to visit.
            let mut new_visit = curr_visit.clone();
            new_visit.push(cave);
            visits.push(new_visit);
        }
    }

    println!("First challenge - result: {}",total_paths);



    //lets do part 2 separately
    //here, we store if we can visit one small cave twice
    let mut visits: Vec<(bool,Vec<&str>)> = Vec::new();
    let start = vec!("start");
    visits.push((true, start));

    let mut total_paths: usize = 0;

    while visits.len() > 0 {
        let (can_visit_twice, curr_visit): (bool, Vec<&str>) = visits.pop().unwrap();
        for &cave in links[curr_visit.last().unwrap()].iter() {
            
            let mut n_can_visit_twice: bool = can_visit_twice;

            // if we reached the end, count the path
            if cave == "end" {
                total_paths += 1;
                continue;
            }

            // we don't want to go twice to the start
            if cave == "start" {
                continue;
            }

            // skip the small caves already visited
            // if we already visited one twice
            if is_lowercase(cave) {
                if curr_visit.contains(&cave) {
                    if can_visit_twice {
                        n_can_visit_twice = false;
                    } else {
                        continue;
                    }
                }
            }

            //add the new route to visit.
            let mut new_visit = curr_visit.clone();
            new_visit.push(cave);
            visits.push((n_can_visit_twice, new_visit));
        }
    }

    println!("Second challenge - result: {}",total_paths);

}

fn is_lowercase<'a>(s: &'a str) -> bool {
    let mut result: bool = true;
    for c in s.chars() {
        result = result && c.is_ascii_lowercase();
    }
    result
}