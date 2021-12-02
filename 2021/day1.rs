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
	let mut result = 0;
	let mut result_2 = -1;
	let mut previous = 0;
	let mut entries:Vec<usize> = Vec::new();
	
	for row in contents.lines()
	{
		entries.push(row.parse().unwrap());
	}
	for i in 1..entries.len()
	{
		if entries[i] > entries[i-1]
		{
			result += 1;
		}
		if i >= 2
		{
			let sum = entries[i] + entries[i-1] + entries[i-2];
			if sum > previous
			{
				result_2 += 1;
			}
			previous = sum;
		}
	}
	println!("First star: {}", result);
	println!("Second star: {}", result_2);
}
