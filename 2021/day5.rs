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

fn calculate_result(ref hash:HashMap<(isize, isize), usize>) -> usize
{
	let mut result = 0;
	for val in hash.values()
	{
		if val > &1
		{
			result += 1;
		}
	}
	result
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
			*lines.entry((beginning[0], beginning[1])).or_insert(0) += 1;
			if x_difference == 0 || y_difference == 0
			{
				*straight_lines.entry((beginning[0], beginning[1])).or_insert(0) += 1;
			}
			if beginning[0] == end[0] && beginning[1] == end[1]
			{
				break;
			}
			beginning[1] += y_difference;
			beginning[0] += x_difference;
		}
	}
	
	println!("First star: {}", calculate_result(straight_lines));
	println!("Second star: {}", calculate_result(lines));
}
