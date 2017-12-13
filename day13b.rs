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
	let mut last_line = 0;
	let mut delay = 0;
	
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
	
	loop
	{
		delay += 1;
		let mut step:i32 = 0;
		let mut caught = false;
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
				
			// check if caught
			if range > 0 && (range == 1 || (step+delay) % (range + (range-2)) == 0)
			{
				caught = true;
			}
			step += 1;
		}
		if !caught
		{
			break;
		}
	}
	println!("{}", delay);
}
