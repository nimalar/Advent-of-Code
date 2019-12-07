use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];

	let mut f = File::open(filename).expect("File not found");

	let mut contents:String = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	let mut y_minus:usize = 0;
	let mut y_plus:usize = 0;
	let mut ground:Vec<Vec<char>> = vec![vec!['.'; 3000]; 3000];
	ground[0][500] = '+';
	
	for line in contents.lines()
	{	
		let parts:Vec<&str> = line.split(", ").collect();
		let mut x_min:usize = 0;
		let mut x_max:usize = 0;
		let mut y_min:usize = 0;
		let mut y_max:usize = 0;
		for i in 0..parts.len()
		{
			let values:Vec<&str> = parts[i].split("=").collect();
			let value_limits:Vec<&str> = values[1].split("..").collect();
			
			if values[0] == "x"
			{
				x_min = value_limits[0].parse().unwrap();
				x_max = value_limits[value_limits.len()-1].parse().unwrap();
			}
			else
			{
				y_min = value_limits[0].parse().unwrap();
				y_max = value_limits[value_limits.len()-1].parse().unwrap();
				if y_minus == 0 || y_min < y_minus
				{
					y_minus = y_min;
				}
				else if y_plus == 0 || y_max > y_plus
				{
					y_plus = y_max;
				}
			}
		}
		
		for y in y_min..y_max+1
		{
			for x in x_min..x_max+1
			{
				ground[y][x] = '#';
			}
		}
	}	
		
	let mut prev_count = 0;
	loop
	{
		for y in 0..y_plus+1
		{
			for x in 0..ground[y].len()
			{
				// move down if possible
				if (ground[y][x] == '+' || ground[y][x] == '|') && ground[y+1][x] == '.'
				{
					ground[y+1][x] = '|';
				}
				// otherwise spread
				else if (ground[y][x] == '|') && (ground[y+1][x] == '#' || ground[y+1][x] == '~')
				{
					let mut left = x;
					let mut right = x;
					let mut unstable = false;
					// find borders if there are
					while left > 0
					{
						if ground[y][left] == '#'
						{
							break;
						}
						else if ground[y+1][left] != '#' && ground[y+1][left] != '~'
						{
							unstable = true;
							break;
						}
						left -= 1;
					}
					while right < ground[y].len()
					{
						if ground[y][right] == '#'
						{
							break;
						}
						else if ground[y+1][right] != '#' && ground[y+1][right] != '~'
						{							
							unstable = true;
							break;
						}
						right += 1;
					}
					if !unstable && (left > 0 && right < ground[y].len())
					{
						for i in left+1..right
						{
							ground[y][i] = '~';
						}
					}					
					else if unstable && (left > 0 && right < ground[y].len())
					{
						for i in left..right+1
						{
							if ground[y][i] == '.'
							{
								ground[y][i] = '|';
							}
						}
					}
				}
			}
		}
		let mut count = 0;
		let mut not_drained = 0;
		for y in y_minus..y_plus+1
		{
			for x in 0..ground[y].len()
			{		
				if ground[y][x] == '~' || ground[y][x] == '|'
				{
					count += 1;
				}
				if ground[y][x] == '~'
				{
					not_drained += 1;
				}
			}
		}
		
		if count == prev_count
		{
			println!("{}", count);
			println!("{}", not_drained);
			break;
		}
		prev_count = count;
	}
}
