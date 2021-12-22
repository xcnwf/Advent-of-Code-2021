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
    let mut m = std::collections::HashMap::new();
    let mut i = 1;
    let mut n = 1;
    while n <= x_max {
        m.insert(n,i);
        i += 1;
        n += i;
    }

    n = i*(i-1)/2;

    for i in (x.0)..=(x.1) {
        for j in (y.0)..=(y.1) {
            initial_velocities(i, j, m.contains_key(&i), &mut velocities);
        }
    }


    println!("Second challenge - result: {}", velocities.len());
}

// max steps a case could be reached in
fn max_steps(x:i32, y:i32) -> i32 {
    i32::max(x, i32::abs(y))
}

fn initial_velocities(x: i32, y: i32, is_arithmetic: bool, vels: &mut std::collections::HashSet<(i32,i32)>) {
    vels.insert((x,y)); //at least direct with step = 1
    
    let mut step = 2;
    print!("\n{} {}: ",x,y);
    while x >= step * (step - 1) {
        //if we can reach this x in 'step' steps.
        if (x-arithmetic_sum(step-1))%step == 0 {
            let vx_0 = (x-arithmetic_sum(step-1))/step + step - 1; //x diminishes so wee need to compensate
            //same for y but the relation is reversed.
            if (y+arithmetic_sum(step-1))%step == 0 {
                let vy_0 = (y+arithmetic_sum(step-1))/step; // y doesn't so we don't touch it
                vels.insert((vx_0,vy_0));
                print_debug(vx_0, vy_0, step);
            }
        }
        step += 1;
    }

    let min_step_x = step;

    step = 1;
    while -y > step * (step - 1) {
        // if y can be reached.
        if (y+arithmetic_sum(step-1))%step == 0 {
            let vy_0 = -(y+arithmetic_sum(step-1))/step - 1;
            
            // if it takes no less than the number of steps needed for x to stabilize
            if 2*vy_0 + step + 1 >= min_step_x {
                vels.insert((min_step_x,vy_0));
                print_debug(min_step_x, vy_0, 2*vy_0+step+1);
            }
        }
        step += 1;
    }
}

fn up_shoot(x:i32, y_min:i32, y_max:i32, steps:i32) -> u32{
    //if it is an arithmetic progression, then we can reach this case by firing up
    // and higher than the time it takes to have vx = 0
    print!("ALERT: {} ", steps);
    
    // We need to consider only the different vy
    let mut vys = std::collections::HashSet::new();
    for y in y_max..=y_min {
        let mut step = 1;
        while -y > step * (step - 1) {
            // if y can be reached.
            if (y+arithmetic_sum(step-1))%step == 0 {
                let vy_0 = -(y+arithmetic_sum(step-1))/step - 1;
                
                // if it takes no less than the number of steps needed for x to stabilize
                if 2*vy_0 + step + 1 >= steps {
                    vys.insert(vy_0);
                    print_debug(steps, vy_0, 2*vy_0+step+1);
                }
            }
        step += 1;
        }
    }
    return vys.len() as u32;
}

fn print_debug(vx:i32,vy:i32,step:i32) {
    print!("({};{};{}) ",vx,vy,step);
}

fn arithmetic_sum(n: i32) -> i32 {
    n*(n+1)/2
}
