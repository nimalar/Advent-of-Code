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
	let mut step:i32 = 0;
	let mut severity = 0;
	let mut last_line = 0;
	
	// read lines to hash
	for line in reader.lines()
	{
		let l = line.unwrap();
		let vec:Vec<&str> = l.split(':').collect();
		let depth:i32 = vec[0].trim().parse().unwrap();
		let range:i32 = vec[1].trim().parse().unwrap();
		values.insert(depth, range);
		last_line = depth;
	}
	
	// start travelling
	while step <= last_line as i32
	{
		// check if caught
		// find out if there is a scanner and if it is on first layer 
		let mut range:i32 = 0;
		match values.get(&step)
		{
			Some(number) => range = *number,
			// if there is nothing in hash, we are safe
			None => {}
		}
		// add the severity to the sum
		if range == 1 || step % (range + (range-2)) == 0
		{
			severity += step * range;
		}
		step += 1;
	}
	
	println!("{}", severity);
}
