use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashSet,HashMap};

fn cube_game(mut cubes:HashSet<(isize, isize, isize, isize)>, four_dim:bool) -> usize
{
	for _cycle in 0..6
	{
		let mut neighbours:HashMap<(isize, isize, isize, isize), usize> = HashMap::new();
		for key in cubes.iter()
		{
			for i_diff in 0..3
			{
				for j_diff in 0..3
				{
					for k_diff in 0..3
					{
						for l_diff in 0..3
						{
							if (four_dim || l_diff == 1) && (i_diff != 1 || j_diff != 1 || k_diff != 1 || l_diff != 1)
							{
								let counter = neighbours.entry((key.0 + i_diff - 1, key.1 + j_diff - 1, key.2 + k_diff - 1, key.3 + l_diff - 1)).or_insert(0);
								*counter += 1;
							}
						}
					}
				}
			}
		}
		let mut new_cubes:HashSet<(isize, isize, isize, isize)> = HashSet::new();
		for (key, val) in neighbours.iter()
		{
			if ((*val == 2 || *val == 3) && cubes.contains(&key)) || (*val == 3 && !cubes.contains(&key))
			{
				new_cubes.insert(*key);
			}
		}
		
		cubes = new_cubes;
	}
	return cubes.len();
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut cubes:HashSet<(isize, isize, isize, isize)> = HashSet::new();
	let mut y = 0;
	
	for row in contents.lines()
	{	
		let char_vec:Vec<char> = row.chars().collect();
		for i in 0..char_vec.len()
		{
			if char_vec[i] == '#'
			{
				cubes.insert((i as isize, y, 0, 0));
			}
		}
		y += 1;
	}
	
	println!("First star: {}", cube_game(cubes.clone(), false));
	println!("Second star: {}", cube_game(cubes, true));
}
