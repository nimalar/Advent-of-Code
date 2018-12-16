use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");

	let mut lights:Vec<(i32,i32,i32,i32)> = Vec::new();
	
	for line in contents.lines()
	{		
		let words:Vec<&str> = line.split(&['<', ',', '>'][..]).collect();
		let pos_x:i32 = words[1].trim().parse().unwrap();
		let pos_y:i32 = words[2].trim().parse().unwrap();
		let speed_x:i32 = words[4].trim().parse().unwrap();
		let speed_y:i32 = words[5].trim().parse().unwrap();
		lights.push((pos_x, pos_y, speed_x, speed_y));
	}
	let mut distribution_prev = -1;
	let mut sky:Vec<Vec<char>> = Vec::new();
	let mut time = 0;
	loop
	{
		// find borders, left right up down
		let mut borders = (lights[0].0,lights[0].0,lights[0].1,lights[0].1);
		for light in lights.to_vec()
		{
			borders.0 = cmp::min(borders.0, light.0);
			borders.1 = cmp::max(borders.1, light.0);
			borders.2 = cmp::min(borders.2, light.1);
			borders.3 = cmp::max(borders.3, light.1);
		}
		let distribution = borders.1 - borders.0 + borders.3 - borders.2;

		if distribution < distribution_prev || distribution_prev == -1
		{
			if distribution < 100 // don't save the overly large stuff in vec
			{
				sky = vec![vec![' '; (borders.1 - borders.0 + 1) as usize]; (borders.3 - borders.2 + 1) as usize];
				for i in 0..lights.len()
				{
					sky[(lights[i].1 - borders.2) as usize][(lights[i].0 - borders.0) as usize] = '#';
				}
			}
			for i in 0..lights.len()
			{
				lights[i].0 = lights[i].0 + lights[i].2;
				lights[i].1 = lights[i].1 + lights[i].3;
			}
			distribution_prev = distribution;
			time += 1;
		}
		else
		{
			// previous step was perfect
			for i in 0..sky.len()
			{
				println!("{}", sky[i].to_vec().into_iter().collect::<String>());
			}
			time -= 1;
			break;
		}
		
	}
	println!("{}", time);
}
