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
	let mut numbers:Vec<usize> = Vec::new();
	let mut start_index = 0;
	let mut invalid = 0;
	
	for row in contents.lines()
	{	
		let number:usize = row.trim().parse().unwrap();
		if numbers.len() >= 25 && invalid == 0
		{
			let mut result = false;
			'outer: for i in start_index..numbers.len()
			{
				for j in start_index..numbers.len()
				{
					if numbers[i] != numbers[j] && numbers[i] + numbers[j] == number
					{
						result = true;
						break 'outer;
					}
				}
			}
			if !result
			{
				invalid = number;
				println!("First star: {}", number);
			}
			start_index += 1;
		}
		numbers.push(number);
	}
	
	'looping: for i in 0..numbers.len()
	{
		let mut index = i;
		let mut result = 0;
		while index < numbers.len() && result < invalid
		{
			result += numbers[index];
			if result == invalid
			{
				println!("Second star: {}", numbers[i..index+1].iter().min().unwrap() + numbers[i..index+1].iter().max().unwrap());
				break 'looping;
			}
			index += 1;
		}
	}
}
