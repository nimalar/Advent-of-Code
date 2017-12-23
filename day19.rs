use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);

    let mut vec:Vec<Vec<char>> = Vec::new();
    // get the input as vec
    for line in reader.lines()
    {
		let l = line.unwrap();
        let new_vec = l.chars().collect();
        vec.push(new_vec);
    }
    
    let mut y:i32 = 0;
    let mut x:i32 = 0;
    let mut dir = Direction{x: 0, y: 1};
    let mut result = String::new();
    let mut distance = 0;
    
    // find the starting place
    for j in 0..vec[0].len()
    {
        if vec[0][j] != ' '
        {
            x = j as i32;
            break;
        }
    }
    
    
    // go ahead until "+" is found
    loop
    {
        distance += 1;
        // find new direction
        if vec[y as usize][x as usize] == '+'
        {
            // previously moved in y-direction, now switch to x
            if dir.y != 0
            {
                dir.y = 0;
                if x > 0 && vec[y as usize][x as usize -1] != ' '
                {
                    dir.x = -1;
                }
                else
                {
                    dir.x = 1;
                }
            }
            else
            {
                dir.x = 0;
                if y > 0 && vec[y as usize -1][x as usize] != ' '
                {
                    dir.y = -1;
                }
                else
                {
                    dir.y = 1;
                }
            }
        }
        // continue same direction
        else
        {
            if vec[y as usize][x as usize] != '-' && vec[y as usize][x as usize] != '|' && vec[y as usize][x as usize] != ' '
            {
                // letter found, append to result
                result.push(vec[y as usize][x as usize]);
                
                // after letter check if it's possible to continue
                if vec[(y + dir.y) as usize][(x + dir.x) as usize] == ' '
                {
                    break;
                }
            }        
        }
        
        // new coordinate
        y += dir.y;
        x += dir.x;
    }
    println!("{}", result);
    println!("{}", distance);
}

struct Direction
{
    x : i32,
    y : i32,
}