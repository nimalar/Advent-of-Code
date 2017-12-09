use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let mut f = File::open(filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
		
	let mut vec:Vec<i32> = Vec::new();
	for number in contents.split_whitespace()
	{
		vec.push(number.parse().unwrap());
	}
	
	let mut layouts:Vec<Vec<i32>> = Vec::new();
	let mut summa:i32 = 0;
	let mut done = false;
	
	loop
	{
		// make a copy of the current layout
		let saved = vec.to_vec();
		
		// compare the current layout with previous ones
		for i in 0..layouts.len()
		{
			if saved == layouts[i]
			{	
				println!("Erotus: {}", layouts.len()-i);
				done = true;
				break;
			}
		}
		
		if done
		{
			break;
		}
		
		// stash the current layout in vector
		layouts.push(saved);
		
		let mut max_value = 0;
		let mut max_index = 0;
		// find the index with max value
		for i in 0..vec.len()
		{
			if i == 0 || vec[i] > max_value
			{
				max_value = vec[i];
				max_index = i;
			}
		}
		
		// redistribute the max heap
		vec[max_index] = 0;
		
		for i in 0..max_value
		{
			max_index += 1;
			if max_index >= vec.len()
			{
				max_index = 0;
			}
			vec[max_index] += 1;
		}
		
		summa += 1;
	}
	
	println!("summa: {}", summa);
}