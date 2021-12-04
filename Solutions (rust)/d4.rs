use std::fs;

const DAY: &'static str = "d4"; 

fn read_file() -> String {
	print!("Reading input...");

	let content: std::io::Result<String> = fs::read_to_string("../Inputs/".to_owned()+DAY+".txt");

	println!("Done!");

	content.unwrap()
}

type bingo_case = (u8, bool);

const SIZE: usize = 5;

type bingo_board = [bingo_case; SIZE*SIZE];

fn sum_unchecked(board: &bingo_board) -> usize {
	let mut total: usize = 0;
	for (value, check) in board {
		if !check {
			total += *value as usize;
		}
	}
	total
}

fn update_board(board: &mut bingo_board, n: u8) {
	for i in 0..(SIZE*SIZE) {
		let (value, _) = (*board)[i];
		if value == n {
			(*board)[i] = (value, true);
		}
	}
}

fn check_board(board: &bingo_board) -> bool {
	let mut chk: bool = false;
	for i in 0..SIZE {
		
		let mut column = true;
		let mut line = true;
		for j in 0..SIZE {
			line = line && board[SIZE*i+j].1;
			column = column && board[SIZE*j+i].1;
		}
		chk = chk || line || column;
	}
	chk
}

fn main() {
	println!("Day {}\n", DAY);

	let content = read_file();

	// Day 4 Challenge : Calculate values based on bingo boards

	let mut numbers: Vec<u8> = Vec::new();
	let mut boards: Vec<bingo_board> = Vec::new();

	//Parse input:
	let mut enum_lines = content.lines();
	let mut line;
	line = enum_lines.next();
	for value in line.unwrap().split(',') {
		let n : u8 = value.parse::<u8>().unwrap();
		numbers.push(n);
	}

	//skip empty line
	enum_lines.next();

	line = enum_lines.next();

	while line != None {
		let mut board: bingo_board = [(0,false); SIZE*SIZE];
		
		//for each bingo line
		for i in 0..SIZE {
			//for each number
			for (j,n) in line.unwrap().split_ascii_whitespace().enumerate() {
				let val:u8 = n.parse::<u8>().unwrap();
				board[SIZE*i+j] = (val, false);
			}
			line = enum_lines.next();
		}

		boards.push(board);

		//skip empty line
		line = enum_lines.next();
	}

	//INPUT PARSED!!
	

	//part 1

	'part1: for n in numbers.iter() {
		for board in boards.iter_mut() {
			//update
			update_board(board, *n);

			//check if winner
			if check_board(board) {
				//print sum * n
				let sum: usize = sum_unchecked(board);
				println!("First Challenge - Result: {}", (*n) as usize*sum);
				break 'part1;
			}
		}
	}

	//part2 - No need to reset board state : we look for the losers
	let mut n_boards: Vec<bingo_board>;
	'part2: for n in numbers.iter() {
		//update all boards
		for board in boards.iter_mut() {
			update_board(board, *n);
		}

		//prune boards: Keep only the uncompletes.
		n_boards = boards.iter().filter(|&b| !check_board(b)).cloned().collect();

		//If there are no longer uncompleted boards (ie the last board got completed)
		if n_boards.len() == 0 {

			//get the last board
			let board = boards.first().unwrap();
			
			//print sum * n
			let sum: usize = sum_unchecked(board);
			println!("Second Challenge - Result: {}", (*n) as usize*sum);
			break 'part2;
		}

		boards = n_boards;
	}
}