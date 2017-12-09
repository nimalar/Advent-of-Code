use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);
	let mut values = HashMap::new();
	let mut first_max:i32 = 0;
	let mut max:i32 = 0;
	
	// read a line a time
	for line in reader.lines()
	{
		// separate by whitespace
		let l = line.unwrap();
		let vec:Vec<&str> = l.split_whitespace().collect();

		// next is to inc or to dec
		let mut inc:i32 = 1;
		if vec[1] == "dec"
		{
			inc = -1;
		}
		
		// next is how much to inc or dec
		let mut change:i32 = vec[2].parse().unwrap();
		change = inc * change;
		
		// next is always if so skip it
		// next another value, look it up or insert it to hash
		let condition = vec[4];
		values.entry(condition.to_string()).or_insert(0 as i32);
		
		// check initial condition value
		let mut initial:i32 = 0;
		match values.get(condition)
		{
			Some(number) => initial = *number,
			None => println!("this shouldn't print")
		}
		
		// get compare value
		let compare:i32 = vec[6].parse().unwrap();
		
		// first is always the value to be modified		
		// check if it is already in the hash, otherwise put it there with initial value 0
		let value = values.entry(vec[0].to_string()).or_insert(0 as i32);
		
		// do the comparison and modify original value based on it
		if (vec[5] == "==" && initial == compare)
			|| (vec[5] == "!=" && initial != compare)
			|| (vec[5] == ">=" && initial >= compare)
			|| (vec[5] == ">" && initial > compare)
			|| (vec[5] == "<=" && initial <= compare)
			|| (vec[5] == "<" && initial < compare)
		{
			if *value > max
			{
				max = *value;
			}
			*value += change;
		}		
	}
	
	// iterate over everything.
	for (_dude, value) in &values 
	{
		if *value > first_max
		{
			first_max = *value;
		}
	}
	
	println!("osa1: {}", first_max);
	println!("osa2: {}", max);
}