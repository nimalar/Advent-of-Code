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
	let mut seats:Vec<bool> = vec![false; 128 * 8];
	
	for line in contents.lines()
	{
		let row = &line.trim()[..7].replace("F", "0").replace("B", "1");
		let seat = &line.trim()[7..].replace("L", "0").replace("R", "1");
		
		let id = usize::from_str_radix(row, 2).unwrap() * 8 + usize::from_str_radix(seat, 2).unwrap();
		seats[id] = true;
		
		if id > result
		{
			result = id;
		}
	}
	
	println!("First star: {}", result);
	
	for i in 1..seats.len() - 1
	{
		if seats[i] == false && seats[i - 1] == true && seats[i + 1] == true
		{
			println!("Second star: {}", i);
			break;
		}
	}
}
