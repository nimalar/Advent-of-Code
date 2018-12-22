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
	
	let mut roads:Vec<Vec<char>> = Vec::new();
	let mut carts:Vec<Vec<(char, i32, i32)>> = Vec::new(); // char representation, tick, turn counter
	
	for line in contents.lines()
	{
		let mut chars:Vec<char> = line.chars().collect();
		let mut cart_chars:Vec<(char, i32, i32)> = vec![(' ', 0, 0); chars.len()];
		for i in 0..chars.len()
		{
			if chars[i] == '>' || chars[i] == '<'
			{
				cart_chars[i] = (chars[i], 0, 1);
				chars[i] = '-';
			}
			else if chars[i] == '^' || chars[i] == 'v'
			{
				cart_chars[i] = (chars[i], 0, 1);
				chars[i] = '|';
			}
		}
		roads.push(chars);
		carts.push(cart_chars);
	}
	let mut tick = 0;
	
	'outer:loop
	{
		for j in 0..carts.len()
		{
			for i in 0..carts.len()
			{
				if carts[j][i].0 != ' ' && carts[j][i].1 <= tick
				{
					let mut new_place = (0,0);
					// cart, get new place
					
					if (carts[j][i].0 == '>' && (roads[j][i] == '-' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 2)))
						|| (carts[j][i].0 == '^' && (roads[j][i] == '/' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 0)))
						|| (carts[j][i].0 == 'v' && (roads[j][i] == '\\' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 1)))
					{
						new_place = (j, i+1);
						carts[j][i].0 = '>';
					}
					else if (carts[j][i].0 == '>' && (roads[j][i] == '\\' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 0)))
						|| (carts[j][i].0 == '<' && (roads[j][i] == '/' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 1)))
						|| (carts[j][i].0 == 'v' && (roads[j][i] == '|' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 2)))
					{
						new_place = (j+1, i);
						carts[j][i].0 = 'v';
					}
					else if (carts[j][i].0 == '<' && (roads[j][i] == '-' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 2)))
						|| (carts[j][i].0 == '^' && (roads[j][i] == '\\' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 1)))
						|| (carts[j][i].0 == 'v' && (roads[j][i] == '/' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 0)))
					{
						new_place = (j, i-1);
						carts[j][i].0 = '<';
					}
					else if (carts[j][i].0 == '<' && (roads[j][i] == '\\' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 0)))
						|| (carts[j][i].0 == '^' && (roads[j][i] == '|' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 2)))
						|| (carts[j][i].0 == '>' && (roads[j][i] == '/' || (roads[j][i] == '+' && carts[j][i].2 % 3 == 1)))
					{
						new_place = (j-1, i);
						carts[j][i].0 = '^';
					}
					
					
					if carts[new_place.0][new_place.1].0 != ' '
					{
						// crash!
						println!("{},{}", new_place.1, new_place.0);
						break 'outer;
					}
					
					// increase turn counter
					if roads[j][i] == '+'
					{
						carts[j][i].2 += 1;
					}
					
					// actual move
					carts[new_place.0][new_place.1] = carts[j][i];
					carts[new_place.0][new_place.1].1 += 1; // tick counter
					carts[j][i] = (' ', 0, 0);
				}
			}
		}
		tick += 1;
	}
}
