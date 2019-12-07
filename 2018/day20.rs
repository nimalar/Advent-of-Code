use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn calculate(x_start:i32, y_start:i32, distance:i32, start_index:usize, chars:&Vec<char>, distances:&mut HashMap<(i32, i32), i32>) -> usize
{
	let mut x = x_start;
	let mut y = y_start;
	let mut dist = distance;
	let mut i = start_index;
	while i < chars.len()
	{
		let c = chars[i];
		if c == '('
		{
			i = calculate(x, y, dist, i+1, chars, distances);
		}
		else if c == ')'
		{
			return i;
		}
		else if c == '|'
		{
			x = x_start;
			y = y_start;
			dist = distance;
		}
		else if "NEWS".contains(c) 
		{
			if c == 'N'
			{
				y += 1;
			}
			else if c == 'S'
			{
				y -= 1;
			}
			else if c == 'E'
			{
				x += 1;
			}
			else if c == 'W'
			{
				x -= 1;
			}
			dist += 1;
			let old_dist = distances.entry((x,y)).or_insert(dist);
			// if this route is shorter than previously found, update it 
			if *old_dist > dist
			{
				*old_dist = dist;
			}
		}
		i += 1;
	}
	return chars.len();
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut distances:HashMap<(i32, i32), i32> = HashMap::new(); // (x,y), dist
	// homesite
	distances.insert((0,0), 0);
	let chars:Vec<char> = contents.chars().collect();
	
	calculate(0, 0, 0, 0, &chars, &mut distances);
	
	// first star
	println!("{:?}", distances.values().max().unwrap());
	
	// another
	let mut sum = 0;
	for val in distances.values()
	{
		if val >= &1000
		{
			sum += 1;
		}
	}
	println!("{:?}", sum);
}
