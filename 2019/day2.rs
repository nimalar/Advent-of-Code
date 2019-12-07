use std::env;
use std::fs::File;
use std::io::prelude::*;

fn program(mut p:Vec<usize>, noun:usize, verb:usize) -> usize
{
	p[1] = noun;
	p[2] = verb;
	let mut i = 0;
	while i < p.len()
	{
		if p[i] == 1
		{
			// add
			let index = p[i+3];
			p[index] = p[p[i+1]] + p[p[i+2]];
		}
		else if p[i] == 2
		{
			// multiply
			let index = p[i+3];
			p[index] = p[p[i+1]] * p[p[i+2]];
		}
		else if p[i] == 99
		{
			// halt
			break;
		}
		i += 4;
	}
	return p[0];
}


fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	let mut positions:Vec<usize> = Vec::new();
	
	for value in contents.split(',')
	{
		positions.push(value.parse().unwrap());
	}
	
	let result = program(positions.to_vec(), 12, 2);
	
	println!("First star: {}", result);
	
	'outer: for i in 0..99
	{
		for j in 0..99
		{
			if program(positions.to_vec(), i, j) == 19690720
			{
				println!("Second star: {}", 100 * i + j);
				break 'outer;
			}
		}
	}
}
