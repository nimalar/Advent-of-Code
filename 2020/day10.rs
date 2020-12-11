use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("sozething went wrong reading the file");
	let mut adapters:Vec<usize> = Vec::new();
	let mut differences:Vec<usize> = vec![0; 3];
	
	for row in contents.lines()
	{	
		adapters.push(row.trim().parse().unwrap());
	}
	adapters.push(0);
	adapters.sort();
	let built_in = adapters.last().unwrap() + 3;
	adapters.push(built_in);
	
	for i in 1..adapters.len()
	{
		differences[adapters[i] - adapters[i - 1] - 1] += 1;
	}

	let mut option_vec:Vec<usize> = vec![0; adapters.len()];
	option_vec[0] = 1;
	for i in 0..adapters.len() - 1
	{
		for j in 1..4
		{
			if i + j < adapters.len() && adapters[i + j] - adapters[i] <= 3
			{
				option_vec[i + j] += option_vec[i];
			}
		}
	}
	
	println!("First star: {}", differences[0] * differences[2]);
	println!("Second star: {}", *option_vec.last().unwrap());
}
