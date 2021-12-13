use std::env;
use std::fs::read_to_string;

fn match_chunks(opener:char, closer:char) -> usize
{
	if closer == ')' && opener != '(' 
	{ 
		return 3;
	}
	if closer == ']' && opener != '[' 
	{ 
		return 57;
	}
	if closer == '}' && opener != '{' 
	{ 
		return 1197;
	}
	if closer == '>' && opener != '<' 
	{ 
		return 25137;
	}
	return 0;
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
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");	
	let mut scores:Vec<usize> = Vec::new();
	let mut result = 0;
	
	'lines: for row in contents.lines()
	{
		let mut openers:Vec<char> = Vec::new();
		let mut score = 0;
		for c in row.chars()
		{
			match c
			{
				'(' | '[' | '{' | '<' => openers.push(c),
				_ => score = match_chunks(openers.pop().unwrap(), c),
			}
			if score > 0
			{
				result += score;
				continue 'lines;
			}
		}
		while !openers.is_empty()
		{
			score *= 5;
			score += add_openers(openers.pop().unwrap());
		}
		scores.push(score);
	}
	scores.sort();
	
	println!("First star: {}", result);
	println!("Second star: {}", scores[scores.len()/2]);
}
