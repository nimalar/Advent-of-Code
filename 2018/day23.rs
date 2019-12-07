use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut nanobots:HashMap<(i32, i32, i32), i32> = HashMap::new(); // (x,y,z), r
	let mut max_rad = 0;
	let mut strongest_bot = (0,0,0);
	let mut min_coords = (0,0,0);
	let mut max_coords = (0,0,0);
	
	for line in contents.lines()
	{
		let words:Vec<&str> = line.split(&['<', ',', '>', '='][..]).collect();
		let x:i32 = words[2].trim().parse().unwrap();
		let y:i32 = words[3].trim().parse().unwrap();
		let z:i32 = words[4].trim().parse().unwrap();
		let r:i32 = words[7].trim().parse().unwrap();
		nanobots.insert((x, y, z), r);
		
		// find the strongest one
		if r > max_rad
		{
			max_rad = r;
			strongest_bot = (x, y, z);
		}
		if min_coords == (0,0,0) && max_coords == (0,0,0)
		{
			min_coords = (x, y, z);
			max_coords = (x, y, z);
		}
		min_coords = (cmp::min(min_coords.0, x), cmp::min(min_coords.1, y), cmp::min(min_coords.2, z));
		max_coords = (cmp::max(max_coords.0, x), cmp::max(max_coords.1, y), cmp::max(max_coords.2, z));
	}
	
	let mut in_range = 0;
	// find all bots in range
	for (nanobot, _radius) in &nanobots 
	{
		// manhattan distance
		if (nanobot.0 - strongest_bot.0).abs() + (nanobot.1 - strongest_bot.1).abs() + (nanobot.2 - strongest_bot.2).abs() <= max_rad
		{
			in_range += 1;
		}
	}
	
	println!("{:?}", in_range);
	
	let mut i = 10000000;
	let mut min_vec:Vec<(i32, i32, i32)> = vec![min_coords];
	let mut max_vec:Vec<(i32, i32, i32)> = vec![max_coords];
	let mut best_result = 0;
	
	while i >= 1
	{
		let mut current_best = 0;
		let min = min_vec.to_vec();
		let max = max_vec.to_vec();
		let mut best_coords:Vec<(i32, i32, i32)> = Vec::new();
		for c in 0..min.len()
		{
			for x in (min[c].0 / i) .. (max[c].0 / i) + 1
			{
				for y in (min[c].1 / i) .. (max[c].1 / i) + 1
				{
					for z in (min[c].2 / i) .. (max[c].2 / i) + 1
					{
						let mut bots = 0;
						// find all bots in range
						for (nanobot, radius) in &nanobots 
						{
							let mut extended_radius = 0;
							if *radius % i != 0
							{
								extended_radius += 1;
							}
							// manhattan distance
							if (nanobot.0 / i - x).abs() + (nanobot.1 / i - y).abs() + (nanobot.2 / i - z).abs() <= *radius / i + extended_radius
							{
								bots += 1;
							}
						}
						if bots >= current_best
						{	
							let best_coord = (x * i, y * i, z * i);
							if bots > current_best
							{
								min_vec = Vec::new();
								max_vec = Vec::new();
								best_result = 0;
								current_best = bots;
							}
							if !best_coords.contains(&best_coord)
							{
								if i == 1
								{
									let result = (best_coord.0).abs() + (best_coord.1).abs() + (best_coord.2).abs();
									if best_result == 0 || result < best_result
									{
										best_result = result;
									}
								}
								best_coords.push(best_coord);
								min_vec.push((best_coord.0 - i, best_coord.1 - i, best_coord.2 - i));
								max_vec.push((best_coord.0 + i, best_coord.1 + i, best_coord.2 + i));
							}
						}
					}
				}
			}
		}
		i = i / 10;
	}
	println!("{:?}", best_result);
}