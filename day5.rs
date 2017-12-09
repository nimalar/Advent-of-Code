use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);
	let mut vec:Vec<i32> = Vec::new();
	
	// initial values to vec
	for line in reader.lines()
	{
		vec.push(line.unwrap().parse().unwrap());
	}
	
	// start jumping
	let mut i:i32 = 0;
	let mut summa:i32 = 0;
	
	loop
	{
		if vec[i as usize] < 3
		{
			vec[i as usize] += 1;
			i += vec[i as usize] -1;
		}
		else
		{
			vec[i as usize] -= 1;
			i += vec[i as usize] +1;
		}
		
		summa += 1;
		
		if i as usize >= vec.len() || i < 0
		{
			break;
		}
	}
	
	println!("summa: {}", summa);
}
