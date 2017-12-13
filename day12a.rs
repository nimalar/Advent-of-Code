use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn check(program:u16, all:&[Vec<u16>], previous:&[u16]) -> i32
{
	let mut summa = 0;
	if program == 0
	{
		return 1;
	}
	let vec:Vec<u16> = all[program as usize].to_vec();
	for x in 0..vec.len()
	{	
		let mut prev:Vec<u16> = previous.to_vec();
		if !previous.contains(&vec[x])
		{
			prev.push(vec[x]);
			summa += check(vec[x], all, &prev);
		}
	}
	return summa;
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

    let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);
	let mut vec:Vec<Vec<u16>> = Vec::new();
	
	// read lines to vector
	for line in reader.lines()
	{
		let mut comms:Vec<u16> = Vec::new();
		let l = line.unwrap();
		let input:Vec<&str> = l.split_whitespace().collect();
		for x in 2..input.len()
		{
			comms.push(input[x].trim_matches(',').parse().unwrap());
		}
		vec.push(comms);
	}
	
	let mut summa = 0;
	// check how many are linked to 0
	for x in 0..vec.len()
	{
		let mut previous:Vec<u16> = Vec::new();
		previous.push(x as u16);
		if check(x as u16, &vec, &previous) > 0
		{
			summa += 1;
		}
	}
	println!("{}", summa);
}
