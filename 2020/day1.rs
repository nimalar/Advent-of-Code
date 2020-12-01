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
	let mut entries:Vec<usize> = Vec::new();
	let mut first_found = false;
	let mut second_found = false;
	
	for value in contents.split_whitespace()
	{
		entries.push(value.parse().unwrap());
	}
		
	'outer: for i in 0..entries.len()
	{
		for j in 0..entries.len()
		{
			if !first_found && i != j && entries[i] + entries[j] == 2020
			{
				println!("First star: {}", entries[i] * entries[j]);
				first_found = true;
				if second_found
				{
					break 'outer;
				}
			}
			if !second_found
			{
				for k in 0..entries.len()
				{
					if i != j && i != k && j != k && entries[i] + entries[j] + entries[k] == 2020
					{
						println!("Second star: {}", entries[i] * entries[j] * entries[k]);
						second_found = true;
						if first_found
						{
							break 'outer;
						}
					}
				}
			}
		}
	}
}
