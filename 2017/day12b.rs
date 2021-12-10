use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn check(program:u16, all:&[Vec<u16>], previous:&[u16], group:u16) -> i32
{
	let mut summa = 0;
	if program == group
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
			summa += check(vec[x], all, &prev, group);
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
	
	let mut groups:Vec<u16> = Vec::new();
	
	// check how many are linked to 0
	for x in 0..vec.len()
	{
		let mut group_found = false;
		let mut previous:Vec<u16> = Vec::new();
		previous.push(x as u16);
		for y in 0..groups.len()
		{
			if check(x as u16, &vec, &previous, groups[y]) > 0
			{
				group_found = true;
			}
		}
		// create a new group if it doesn't fit any previous
		if !group_found
		{
			groups.push(x as u16);
		}
	}
	println!("{}", groups.len());
}
