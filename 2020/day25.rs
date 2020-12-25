use std::env;
use std::fs::File;
use std::io::prelude::*;

fn transform(value:usize, subject_number:usize) -> usize
{
	let result = value * subject_number;
	return result % 20201227;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let public_keys:Vec<usize> = contents.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
	
	let mut value = 1;
	let mut loops = 0;
	loop
	{
		loops += 1;
		value = transform(value, 7);
		if value == public_keys[0]
		{
			break;
		}
	}
	value = 1;
	for _j in 0..loops
	{
		value = transform(value, public_keys[1]);
	}
	println!("First star: {}", value);
}
