use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap,HashSet};

fn tile_game(mut tiles:HashSet<(isize, isize)>) -> usize
{
	for _cycle in 0..100
	{
		let mut neighbours:HashMap<(isize, isize), usize> = HashMap::new();
		for key in tiles.iter()
		{
			for i in -1..2
			{
				for j in -2..3 as isize
				{
					if (i != 0 || j != 0) && ((i == 0) != (j.abs() == 1)) 
					{
						let counter = neighbours.entry((key.0 + i, key.1 + j)).or_insert(0);
						*counter += 1;
					}
				}
			}
		}
		let mut new_tiles:HashSet<(isize, isize)> = HashSet::new();
		for (key, val) in neighbours.iter()
		{
			if ((*val == 1 || *val == 2) && tiles.contains(&key)) || (*val == 2 && !tiles.contains(&key))
			{
				new_tiles.insert(*key);
			}
		}
		tiles = new_tiles;
	}
	return tiles.len();
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut black_tiles:HashSet<(isize, isize)> = HashSet::new();
	
	for row in contents.lines()
	{
		let mut coordinate = (0, 0);
		let chars:Vec<char> = row.chars().collect();
		let mut i = 0;
		while i < chars.len()
		{
			match chars[i]
			{
				'n' => coordinate.0 += 1,
				's' => coordinate.0 -= 1,
				'e' => coordinate.1 -= 2,
				'w' => coordinate.1 += 2,
				_ => continue,
			}
			if chars[i] == 's' || chars[i] == 'n'
			{
				match chars[i + 1]
				{
					'e' => coordinate.1 -= 1,
					'w' => coordinate.1 += 1,
					_ => continue,
				}
				i += 2;
				continue;
			}
			i += 1;
		}
		if black_tiles.contains(&coordinate)
		{
			black_tiles.remove(&coordinate);
		}
		else
		{
			black_tiles.insert(coordinate);
		}
	}
	
	println!("First star: {}", black_tiles.len());
	println!("Second star: {}", tile_game(black_tiles));
}
