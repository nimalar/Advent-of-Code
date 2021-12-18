use std::env;
use std::fs::read_to_string;
use std::cmp;

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
	result
}

fn calculate_magnitude(row:String) -> usize
{
	let expression:Vec<char> = row.replace("[", "(3*").replace(",", "+(2*").replace("]", "))").chars().collect();
	calculate(&expression[..])
}

fn reduce(value:String) -> String
{
	let mut char_vec:Vec<char> = value.chars().collect();
	loop
	{
		let mut pairs = 0;
		let mut explosion_or_split = false;
		for i in 0..char_vec.len()
		{
			if char_vec[i] == '['
			{
				pairs += 1;
			}
			else if char_vec[i] == ']'
			{
				pairs -= 1;
			}
			else if pairs > 4 && char_vec[i].is_digit(10)
			{
				let mut number_as_chars = String::new();
				while char_vec[i].is_digit(10)
				{
					number_as_chars.push(char_vec.remove(i));
				}
				let left_number:usize = number_as_chars.parse().unwrap();
				number_as_chars = String::new();
				while char_vec[i+1].is_digit(10)
				{
					number_as_chars.push(char_vec.remove(i+1));
				}
				let right_number:usize = number_as_chars.parse().unwrap();
				
				char_vec[i-1] = '0';
				for _j in 0..2
				{
					char_vec.remove(i);
				}
				
				// find the next number to the right of explosion
				for j in i..char_vec.len()
				{
					if char_vec[j].is_digit(10)
					{
						number_as_chars = String::new();
						while char_vec[j].is_digit(10)
						{
							number_as_chars.push(char_vec.remove(j));
						}
						let sum = number_as_chars.parse::<usize>().unwrap() + right_number;
						for c in sum.to_string().chars().rev()
						{
							char_vec.insert(j, c);
						}
						break;
					}
				}
				
				// find the next number to the left of explosion
				for j in (0..i-1).rev()
				{
					if char_vec[j].is_digit(10)
					{
						let mut number_to_left_chars:Vec<char> = Vec::new();
						let mut k = j;
						while char_vec[k].is_digit(10)
						{
							number_to_left_chars.insert(0, char_vec.remove(k));
							k -= 1;
						}
						let sum = number_to_left_chars.into_iter().collect::<String>().parse::<usize>().unwrap() + left_number;
						for c in sum.to_string().chars().rev()
						{
							char_vec.insert(k+1, c);
						}
						break;
					}
				}
				explosion_or_split = true;
				break;
			}
		}
		if !explosion_or_split
		{
			for i in 0..char_vec.len()
			{
				if char_vec[i].is_digit(10) && char_vec[i+1].is_digit(10)
				{
					let mut split_chars = String::new();
					while char_vec[i].is_digit(10)
					{
						split_chars.push(char_vec.remove(i));
					}
					let split_number:usize = split_chars.parse().unwrap();
					let split_left = split_number / 2;
					let split_right = (split_number as f32 / 2.0).ceil() as usize;
					char_vec.insert(i, ']');
					for c in split_right.to_string().chars().rev()
					{
						char_vec.insert(i, c);
					}
					char_vec.insert(i, ',');
					for c in split_left.to_string().chars().rev()
					{
						char_vec.insert(i, c);
					}
					char_vec.insert(i, '[');
					explosion_or_split = true;
					break;
				}
			}
		}
		if !explosion_or_split
		{
			break;
		}
	}
	return char_vec.into_iter().collect();
}

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");
	let mut previous_row = String::new();
	let mut numbers:Vec<String> = Vec::new();
	for row	in contents.lines()
	{
		let mut sum = row.to_string();
		if !previous_row.is_empty()
		{
			sum = "[".to_string() + &previous_row + "," + row + "]";
			sum = reduce(sum);
		}
		previous_row = sum.to_string();
		numbers.push(row.to_string());
	}
	println!("First star: {}", calculate_magnitude(previous_row));
	
	let mut max = 0;
	for i in 0..numbers.len()
	{
		for j in 0..numbers.len()
		{
			if i != j
			{
				let sum = "[".to_string() + &numbers[i] + "," + &numbers[j] + "]";
				max = cmp::max(max, calculate_magnitude(reduce(sum)));
			}
		}
	}
	println!("Second star: {}", max);
}
