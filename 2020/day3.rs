use std::env;
use std::fs::File;
use std::io::prelude::*;

fn slope(ref trees:&Vec<Vec<char>>, right:usize, down:usize) -> usize
{
	let mut result = 0;
	let mut x = 0;
	let mut y = 0;
	while y < trees.len()
	{
		if trees[y][x] == '#'
		{
			result += 1;
		}
		x = (x + right) % trees[0].len();
		y += down;
	}
	return result;
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut trees:Vec<Vec<char>> = Vec::new();
	
	for row in contents.lines()
	{
		trees.push(row.chars().collect());
	}
	
	let mut result = slope(&trees, 3, 1);
	println!("First star: {}", result);
	
	result *= slope(&trees, 1, 1);
	result *= slope(&trees, 5, 1);
	result *= slope(&trees, 7, 1);
	result *= slope(&trees, 1, 2);
	println!("Second star: {}", result);
}
