use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let max_val = 30000;
	let mut jumble:Vec<Vec<Vec<usize>>> = vec![vec![vec![0;3]; max_val]; max_val];
	let start_val = 10000;
	
	let mut num = 0;
	for wire in contents.split_whitespace()
	{
		num += 1;
		let mut x = start_val;
		let mut y = start_val;
		let mut steps = 0;
		for direction in wire.split(',')
		{
			let values = direction.split_at(1);
			let length:usize = values.1.parse().unwrap();
			if values.0 == "R"
			{
				for i in x+1..x+length+1
				{
					jumble[y][i][0] += num;
					steps += 1;
					if jumble[y][i][num] == 0
					{
						jumble[y][i][num] = steps;
					}
				}	
				x += length;
			}
			else if values.0 == "L"
			{
				for i in (x-length..x).rev()
				{
					jumble[y][i][0] += num;
					steps += 1;
					if jumble[y][i][num] == 0
					{
						jumble[y][i][num] = steps;
					}
				}	
				x -= length;
			}			
			else if values.0 == "D"
			{
				for j in y+1..y+length+1
				{
					jumble[j][x][0] += num;
					steps += 1;
					if jumble[j][x][num] == 0
					{
						jumble[j][x][num] = steps;
					}
				}	
				y += length;
			}
			else if values.0 == "U"
			{
				for j in (y-length..y).rev()
				{
					jumble[j][x][0] += num;
					steps += 1;
					if jumble[j][x][num] == 0
					{
						jumble[j][x][num] = steps;
					}
				}	
				y -= length;
			}
		}
	}
	
	let mut result = 0;
	let mut second_result = 0;
	
	for j in 0..jumble.len()
	{
		for i in 0..jumble[j].len()
		{
			if jumble[j][i][0] == 3
			{
				let manhattan = (i as i32 - start_val as i32).abs() + (j as i32 - start_val as i32).abs();
				let step_count = jumble[j][i][1] + jumble[j][i][2];
				if result == 0 || manhattan < result
				{
					result = manhattan;
				}
				if second_result == 0 || step_count < second_result
				{
					second_result = step_count;
				}
			}
		}
	}
	println!("First star: {}", result);
	println!("Second star: {}", second_result);
}
