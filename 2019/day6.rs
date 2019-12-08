use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut hash:HashMap<&str, &str> = HashMap::new();
	
	for value in contents.split_whitespace()
	{
		let parts:Vec<&str> = value.split(')').collect();
		hash.insert(parts[1], parts[0]);
	}
	
	let mut result = 0;
	let mut you:Vec<&str> = Vec::new();
	let mut san:Vec<&str> = Vec::new();
	
	for key in hash.keys()
	{
		let mut orbiter = key;
		while orbiter != &"COM"
		{
			result += 1;
			orbiter = hash.get(orbiter).unwrap();
			if key == &"YOU"
			{	
				you.push(orbiter);
			}
			else if key == &"SAN"
			{	
				san.push(orbiter);
			}
		}
	}
	
	while you.last() == san.last()
	{
		you.pop();
		san.pop();
	}
	
	let second_result = you.len() + san.len();
	
	println!("First star: {}", result);
	println!("Second star: {}", second_result);
}
