use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn step_size(begin:isize, end:isize) -> isize
{
	if begin < end
	{
		return 1;
	}
	else if begin > end
	{
		return -1;
	}
	return 0;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut lines:HashMap<(isize, isize), usize> = HashMap::new();
	let mut straight_lines:HashMap<(isize, isize), usize> = HashMap::new();
	
	for row in contents.lines()
	{
		let values:Vec<&str> = row.split(" -> ").collect();
		let mut beginning:Vec<isize> = values[0].split(',').map(|value| value.parse().unwrap()).collect();
		let end:Vec<isize> = values[1].split(',').map(|value| value.parse().unwrap()).collect();
		let x_difference = step_size(beginning[0], end[0]);
		let y_difference = step_size(beginning[1], end[1]);
		loop
		{
			let value = lines.entry((beginning[0], beginning[1])).or_insert(0);
			*value += 1;
			if x_difference == 0 || y_difference == 0
			{
				let straight = straight_lines.entry((beginning[0], beginning[1])).or_insert(0);
				*straight += 1;
			}
			if beginning[0] == end[0] && beginning[1] == end[1]
			{
				break;
			}
			beginning[1] += y_difference;
			beginning[0] += x_difference;
		}
	}
	
	let mut result_1 = 0;
	for val in straight_lines.values()
	{
		if val > &1
		{
			result_1 += 1;
		}
	}
	let mut result_2 = 0;
	for val in lines.values()
	{
		if val > &1
		{
			result_2 += 1;
		}
	}
	
	println!("First star: {}", result_1);
	println!("Second star: {}", result_2);
}
