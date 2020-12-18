use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_next_value(expression:&[char], index:&mut usize) -> usize
{
	let i = *index;
	match expression[i]
	{
		'(' => {
			let mut end_iterator = expression[i..].iter();
			let mut end = end_iterator.position(|&x| x == ')').unwrap();
			let mut start_iterator = expression[i + 1..].iter();
			let mut start = start_iterator.position(|&x| x == '(');
			let mut start_index = 0;
			loop
			{
				match start
				{
					Some(y) => {
						start_index += y + 1;
						if start_index < end
						{
							end += 1 + end_iterator.position(|&x| x == ')').unwrap();
							start = start_iterator.position(|&x| x == '(');
							continue;
						}
						break;
					}
					None => break,
				}
			}
			*index = i + end;
			return calculate(&expression[i + 1 .. i + end]);
		}
		_ => return expression[*index].to_digit(10).unwrap() as usize,
	}
}

fn calculate(expression:&[char]) -> usize
{
	let mut index = 0;
	let mut result = 0;
	while index < expression.len()
	{
		match expression[index]
		{
			'*' => {
				index += 1;
				result *= get_next_value(&expression[..], &mut index);
			},
			'+' => {
				index += 1;
				result += get_next_value(&expression[..], &mut index);
			}
			_ => result = get_next_value(&expression[..], &mut index),
		}
		index += 1;
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
	contents.retain(|x| x != ' ');
	let mut result_1 = 0;
	let mut result_2 = 0;
	
	for row in contents.lines()
	{	
		let mut expression:Vec<char> = row.chars().collect();
		result_1 += calculate(&expression[..]);
		
		let mut open_brackets = 0;
		let mut open_after:Vec<usize> = vec![0; 5];
		let mut i = 0;
		while i < expression.len()
		{
			if open_brackets > 0 && open_after[open_brackets] == 0 && (expression[i] == ')' || expression[i] == '*')
			{
				expression.insert(i, ')');
				open_brackets -= 1;
			}
			else if expression[i] == '*'
			{
				expression.insert(i + 1, '(');
				open_brackets += 1;
				i += 1;
			}
			else if expression[i] == '('
			{
				open_after[open_brackets] += 1;
			}
			else if expression[i] == ')'
			{
				open_after[open_brackets] -= 1;
			}
			i += 1;
		}

		while open_brackets > 0
		{
			expression.push(')');
			open_brackets -= 1;
		}
		result_2 += calculate(&expression[..]);
	}
	
	println!("First star: {}", result_1);
	println!("Second star: {}", result_2);
}
