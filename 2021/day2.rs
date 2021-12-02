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
	let mut depth_aka_aim = 0;
	let mut horizontal = 0;
	let mut new_depth = 0;
	
	for row in contents.lines()
	{
		let parts:Vec<&str> = row.split_whitespace().collect();
		let value:usize = parts[1].parse().unwrap();
		match parts[0]
		{
			"forward" => {
				horizontal += value;
				new_depth += depth_aka_aim * value;
			},
			"down" => depth_aka_aim += value,
			"up" => depth_aka_aim -= value,
			_ => println!("parse error"),
		}
	}
	
	println!("First star: {}", depth_aka_aim * horizontal);
	println!("Second star: {}", new_depth * horizontal);
}
