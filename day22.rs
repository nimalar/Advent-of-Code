use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let f = File::open(filename).expect("File not found");
	let reader = BufReader::new(f);

	let mut map:Vec<Vec<char>> = Vec::new();
	// read the map into vec
	for line in reader.lines()
	{
		map.push(line.unwrap().chars().collect());
	}
	
	let mut location:Vec<i32> = vec![(map.len()/2) as i32, (map.len()/2) as i32];
	let mut facing:Vec<i32> = vec![-1, 0];
	let mut infections = 0;
	for i in 0..10000000
	{
		// check the location and expand map if necessary
		let x_len = map[0].len();
		if location[0] >= map.len() as i32
		{
			map.push(vec!['.'; x_len]);
		}
		else if location[0] < 0
		{
			map.insert(0, vec!['.'; x_len]);
			location[0] = 0;
		}
		else if location[1] < 0
		{
			for line in 0..map.len()
			{
				map[line].insert(0, '.');
			}
			location[1] = 0;
		}
		else if location[1] >= x_len as i32
		{
			for line in 0..map.len()
			{
				map[line].push('.');
			}
		}
	
		// clean -> weakened & to left
		if map[location[0] as usize][location[1] as usize] == '.'
		{
			if facing[0] != 0
			{
				facing = vec![0, facing[0]];
			}
			else
			{
				facing = vec![facing[1]*-1, 0];
			}
			map[location[0] as usize][location[1] as usize] = 'W';
		}
		// infected -> flagged & to right
		else if map[location[0] as usize][location[1] as usize] == '#'
		{
			if facing[0] != 0
			{
				facing = vec![0, facing[0]*-1];
			}
			else
			{
				facing = vec![facing[1], 0];
			}
			map[location[0] as usize][location[1] as usize] = 'F';
		}
		// weakened -> infected & same direction
		else if map[location[0] as usize][location[1] as usize] == 'W'
		{
			map[location[0] as usize][location[1] as usize] = '#';
			infections += 1;
		}
		// flagged -> clean & reverse direction
		else if map[location[0] as usize][location[1] as usize] == 'F'
		{
			facing = vec![facing[0]*-1, facing[1]*-1];
			map[location[0] as usize][location[1] as usize] = '.';
		}
		location = vec![location[0]+facing[0], location[1]+facing[1]];
	}
	println!("{}", infections);
}