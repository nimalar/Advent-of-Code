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
	let mut second_result = 0;
	
	for mass in contents.split_whitespace()
	{
		let mut i:i32 = mass.parse().unwrap();
		let mut fuel = i / 3 - 2;
		result += fuel;
		while fuel > 0
		{
			second_result += fuel;
			fuel = fuel / 3 - 2;
		}
	}
	
	println!("First star: {}", result);
	println!("Second star: {}", second_result);
}
