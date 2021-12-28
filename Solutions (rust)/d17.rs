// d17.rs
use std::fs;

const DAY: &'static str = "d17";

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

	// Day 17 Challenge : ??
    let line = content.lines().next().unwrap();

    let area = line.split_once(": ").unwrap().1;
    let (x_str, y_str) : (&str, &str) = area.split_once(", ").unwrap();
    let mut x_lmao = (&x_str[2..]).split("..");
    let x = (x_lmao.next().unwrap().parse::<i32>().unwrap(), x_lmao.next().unwrap().parse::<i32>().unwrap());
    let mut y_lmao = (&y_str[2..]).split("..");
    let y = (y_lmao.next().unwrap().parse::<i32>().unwrap(), y_lmao.next().unwrap().parse::<i32>().unwrap());

    let y_min = i32::min(y.0,y.1);

    // Due to the physics rules, the projectile, if thrown up, will always go back to y=0 with 
    // a velocity of -(vy_0 + 1).
    // This means that we want to reach maximal hight without ever overshooting the lowest y boundary
    // This is done by picking a vy_0 of -(y_min + 1), this way we will attain the lowest y boundary in
    // one shot.

    let vy_0 = -(y_min + 1);

    // Then the highest point reached has an altitude of vy_0 * (vy_0 + 1) / 2
    // (see the sum of consecutive naturals)
    // sum (0..=n) (inclusive) => n*(n+1)/2 

    println!("First challenge - result: {}", (vy_0*(vy_0+1)) / 2);


    // part2

    //some trajectories can be the same, so we just need to keep track of all the initial velocities.
    let mut velocities = std::collections::HashSet::new();

    let x_max = i32::max(x.0,x.1);
    let mut artithmetic_progressions = std::collections::HashMap::new();
    let mut i = 1;
    let mut n = 1;
    while n <= x_max {
        artithmetic_progressions.insert(n,i);
        i += 1;
        n += i;
    }

    println!("{}..{} - {}..{}",x.0,x.1,y.0,y.1);
    for i in (x.0)..=(x.1) {
        for j in (y.0)..=(y.1) {
            initial_velocities(i, j, artithmetic_progressions.contains_key(&i), &mut velocities);
        }
    }


    println!("Second challenge - result: {}", velocities.len());
}

fn initial_velocities(x: i32, y: i32, is_arithmetic: bool, vels: &mut std::collections::HashSet<(i32,i32)>) {
    vels.insert((x,y)); //at least direct with step = 1
    
    let mut step = 2;
    print!("\n{} {}: ",x,y);


    /* Because for n steps, x must be of the form 
            x = n*k + arithmetic_sum(n-1), with k natural
            (eg. 15 = 2*7 + 1 so it could be reached in 2 steps with initial velocity 8 (=k+2-1))
            (    15 = 3*4 + 3 so it could be reached in 3 steps with ---------------- 6 (=k+3-1))
            (    15 = 6*0 + 15 so it could be reached in 6 steps with initial velocity 5 (=k+6-1))
       So while x - arithmetic_sum(n-1) >= 0, there could exist a k
    */
    while x >= arithmetic_sum(step-1) /*step * (step - 1)*/ {
        //if we can reach this x in 'step' steps.
        if (x-arithmetic_sum(step-1))%step == 0 {
            let vx_0 = (x-arithmetic_sum(step-1))/step + step - 1; //x diminishes so we need to compensate
            //same for y but the relation is reversed.
            if (y+arithmetic_sum(step-1))%step == 0 {
                let vy_0 = (y+arithmetic_sum(step-1))/step; // y increases so we don't touch it
                vels.insert((vx_0,vy_0));
                print_debug(vx_0, vy_0, step);
            }
        }
        step += 1;
    }

    //(step-1) - 1 to account for the =
    let min_step_x = step-2;

    step = 1;
    if (is_arithmetic) {
        //for x in arithmetic progression, we want to also consider the cases where x stabilizes
        while -y >= arithmetic_sum(step - 1) {
            // if y can be reached.
            if (y+arithmetic_sum(step-1))%step == 0 {
                let vy_0 = -(y+arithmetic_sum(step-1))/step - 1;
                
                // if it takes no less than the number of steps needed for x to stabilize
                if 2*vy_0 + step + 1 > min_step_x {
                    vels.insert((min_step_x,vy_0));
                    print_debug(min_step_x, vy_0, 2*vy_0+step+1);
                }
            }
            step += 1;
        }
    }
}

fn print_debug(vx:i32,vy:i32,step:i32) {
    print!("({};{};{}) ",vx,vy,step);
}

fn arithmetic_sum(n: i32) -> i32 {
    n*(n+1)/2
}

fn is_valid(x: (i32,i32), y: (i32,i32), mut vx: i32, mut vy: i32) -> bool {
    let (mut x_pos, mut y_pos) = (0,0);
    let init_vx = vx;
    let init_vy = vy;

    while y_pos >= y.0 {
        if x_pos >= x.0 && x_pos <= x.1 && y_pos >= y.0 && y_pos <= y.1 {
            return true
        } 
        x_pos += vx;
        y_pos += vy;

        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;
    }
    print!("({};{}) ",init_vx,init_vy);
    return false;
}