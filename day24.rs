use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn find_next_part(parts:&[Vec<usize>], previous_parts:&[usize], mut bridges:&mut Vec<Vec<usize>>, find:usize)
{
	// find all matching parts that are not yet used
	for i in 0..parts.len()
	{
		if !previous_parts.contains(&i) && parts[i].contains(&find)
		{
			let mut prev = previous_parts.to_vec();
			prev.push(i);
			bridges.push(prev.to_vec());
			let mut to_be_found = parts[i][0];
			if parts[i][0] == find
			{
				to_be_found = parts[i][1];
			}
			find_next_part(&parts, &prev, &mut bridges, to_be_found);
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);

	let mut parts:Vec<Vec<usize>> = Vec::new();
	// read the parts into vector
	for line in reader.lines()
	{
		let l = line.unwrap();
		let vec:Vec<&str> = l.split('/').collect();
		let mut part_vec:Vec<usize> = Vec::new();
		for i in 0..2
		{
			part_vec.push(vec[i].parse().unwrap());
		}
		parts.push(part_vec);
	}
	
	let mut bridges:Vec<Vec<usize>> = Vec::new();
	// find all possible bridges
	let previous_parts:Vec<usize> = Vec::new();
	let mut max = 0;
	let mut longest = 0;
	find_next_part(&parts, &previous_parts, &mut bridges, 0);
	for bridge in bridges
	{
		let length = bridge.len();
		if length > longest
		{
			longest = length;
			max = 0;
		}
		let mut strength = 0;
		for i in 0..bridge.len()
		{
			strength += parts[bridge[i]][0];
			strength += parts[bridge[i]][1];
		}
		if length == longest && strength > max
		{
			max = strength;
		}
	}
	println!("{}", max);
}
