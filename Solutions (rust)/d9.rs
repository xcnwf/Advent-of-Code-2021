// d9.rs
use std::fs;

const DAY: &'static str = "d9";

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

	// Day 9 Challenge : Count the low points
    let mut matrix: Vec<Vec<(usize,bool)>> = Vec::new();
    for line in content.lines() {
        let mut numbers = Vec::new();
        for c in line.chars() {
            numbers.push((c.to_digit(10).unwrap() as usize, false));
        }
        matrix.push(numbers);
    }
    //Parsed!
    let n = matrix.len();
    let m = matrix[0].len();

    //part1
    let mut lowest_points = Vec::new(); //prep for part 2
    let mut total_risk : usize = 0;
    for i in 0..n {
        for j in 0..m {
            let curr_point: usize = matrix[i][j].0;
            let mut lowest: bool = true;
            if i != 0 {
                lowest = lowest && (matrix[i-1][j].0 > curr_point); 
            } if i != n-1 {
                lowest = lowest && (matrix[i+1][j].0 > curr_point);
            } if j != 0 {
                lowest = lowest && (matrix[i][j-1].0 > curr_point);
            } if j != m-1 {
                lowest = lowest && (matrix[i][j+1].0 > curr_point);
            }

            // if it is still the lowest point, add its risk
            if lowest {
                total_risk += curr_point + 1;
                
                // so we note every points
                lowest_points.push((i,j));
            }
        }
    }

    println!("First challenge - result: {}", total_risk);

    //part2 - find bassins
    // for this part, we need to start from each lowest point and 
    // recursively extends in all four directions until a 9 is found
    let mut bassins: Vec<usize> = Vec::new();
    for point in lowest_points {
        bassins.push(bassin_size(&mut matrix, point, n, m));
    }

    bassins.sort_unstable();

    println!("Second challenge - result: {}", bassins.iter().rev().take(3).fold(1, |i,&e| i*e));
}

/**
 * This function returns the size of the bassin in a n * m matrix obtained by starting from point 
 */
fn bassin_size (matrix: &mut Vec<Vec<(usize, bool)>>, point: (usize, usize), n:usize, m:usize) -> usize {
    let (i,j) = point;
    let mut size = 1;
    let (curr_point, visited) = matrix[i].get_mut(j).unwrap();
    
    // if it is a 9 or a visited point, we dont want to visit again.
    if *curr_point == 9 || *visited {
        return 0;
    }

    //else we flag it as visited
    *visited = true;


    //and we (try to) visit the next points
    if i != 0 {
        size += bassin_size(matrix, (i-1,j), n, m);
    } if i != n-1 {
        size += bassin_size(matrix, (i+1, j), n, m);
    } if j != 0 {
        size += bassin_size(matrix, (i, j-1), n, m);
    } if j != m-1 {
        size += bassin_size(matrix, (i, j+1), n, m);
    }

    return size;
}