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
	let mut result_1 = 0;
	let mut result_2 = 0;
	
	for row in contents.lines()
	{
		let mut parts:Vec<&str> = row.split(": ").collect();
		let mut rule:Vec<&str> = parts[0].split(" ").collect();
		let mut numbers:Vec<&str> = rule[0].split("-").collect();
		
		let min:usize = numbers[0].parse().unwrap();
		let max:usize = numbers[1].parse().unwrap();
		let counter = parts[1].matches(rule[1]).count();
		if counter >= min && counter <= max
		{
			result_1 += 1;
		}
		let characters:Vec<_> = parts[1].chars().collect();
		if (characters[min - 1].to_string() == rule[1]) != (characters[max - 1].to_string() == rule[1])
		{
			result_2 += 1;
		}
		
	}
	println!("First star: {}", result_1);
	println!("Second star: {}", result_2);
}
