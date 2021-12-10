use std::env;
use std::fs::File;
use std::io::prelude::*;

fn match_chunks(opener:char, closer:char) -> usize
{
	let mut result = 0;
	if closer == ')' && opener != '(' 
	{ 
		result = 3;
	}
	else if closer == ']' && opener != '[' 
	{ 
		result = 57;
	}
	else if closer == '}' && opener != '{' 
	{ 
		result = 1197;
	}
	else if closer == '>' && opener != '<' 
	{ 
		result = 25137;
	}
	result
}

fn add_openers(opener:char) -> usize
{
	match opener
	{
		'(' => return 1,
		'[' => return 2,
		'{' => return 3,
		'<' => return 4,
		_ => return 0,
	}
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	let mut f = File::open(filename).expect("File not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("something went wrong reading the file");
	
	let mut scores:Vec<usize> = Vec::new();
	let mut result = 0;
	
	for row in contents.lines()
	{
		let mut openers:Vec<char> = Vec::new();
		let mut score = 0;
		for c in row.chars()
		{
			match c
			{
				'(' | '[' | '{' | '<' => openers.push(c),
				_ => { score = match_chunks(openers.pop().unwrap(), c) },
			}
			if score > 0
			{
				result += score;
				break;
			}
		}
		if score == 0
		{
			while !openers.is_empty()
			{
				score *= 5;
				score += add_openers(openers.pop().unwrap());
			}
			scores.push(score);
		}
	}
	scores.sort();
	
	println!("First star: {}", result);
	println!("Second star: {}", scores[scores.len()/2]);
}
