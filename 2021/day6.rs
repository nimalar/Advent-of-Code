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
	let mut fish:Vec<usize> = vec![0; 9];
	
	for value in contents.split(',')
	{
		fish[value.parse::<usize>().unwrap()] += 1;
	}
	
	let mut current = 0;
	for day in 0..256
	{
		if day == 80
		{
			println!("First star: {}", fish.iter().sum::<usize>());
		}
		fish[(current + 7) % 9] += fish[current];
		current = (current + 1) % 9;
	}
	
	println!("Second star: {}", fish.iter().sum::<usize>());
}
