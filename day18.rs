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
	
	let mut area:Vec<Vec<char>> = Vec::new();
	
	for line in contents.lines()
	{	
		area.push(line.chars().collect());
	}	
	
	let mut wooded = 0;
	let mut lumberyard = 0;
	
	for _z in 0..10
	{
		wooded = 0;
		lumberyard = 0;
		let mut new_area:Vec<Vec<char>> = Vec::new();
		for y in 0..area.len()
		{
			let mut new_row:Vec<char> = Vec::new();
			for x in 0..area[y].len()
			{
				let mut adj_trees = 0;
				let mut adj_lumberyards = 0; 
				
				// adjacents
				for j in cmp::max(0, (y as i32) -1) as usize .. cmp::min(y+2, area.len())
				{
					for i in cmp::max(0, (x as i32) -1) as usize .. cmp::min(x+2, area[y].len())
					{
						if x != i || y != j
						{
							if area[j][i] == '|'
							{
								adj_trees += 1;
							}
							else if area[j][i] == '#'
							{
								adj_lumberyards += 1;
							}
						}
					}
				}
				let new_char;
				
				if area[y][x] == '.' && adj_trees >= 3
				{
					new_char = '|';
				}
				else if area[y][x] == '|' && adj_lumberyards >= 3
				{
					new_char = '#';
				}				
				else if area[y][x] == '#' && (adj_lumberyards == 0 || adj_trees == 0)
				{
					new_char = '.';
				}
				else
				{
					new_char = area[y][x];
				}
				
				if new_char == '|'
				{
					wooded += 1;
				}
				else if new_char == '#'
				{
					lumberyard += 1;
				}
				
				new_row.push(new_char);
			}
			new_area.push(new_row);
		}
		area = new_area.to_vec();
	}
	println!("{}", wooded * lumberyard);
}
