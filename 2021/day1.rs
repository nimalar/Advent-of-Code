use std::env;
use std::fs::read_to_string;

fn main() 
{
	let contents = read_to_string(env::args().nth(1).unwrap()).expect("something went wrong reading the file");	
	let mut result = 0;
	let mut result_2 = -1;
	let mut previous = 0;
	let mut entries:Vec<usize> = Vec::new();
	
	for row in contents.lines()
	{
		entries.push(row.parse().unwrap());
	}
	for i in 1..entries.len()
	{
		if entries[i] > entries[i-1]
		{
			result += 1;
		}
		if i >= 2
		{
			let sum = entries[i] + entries[i-1] + entries[i-2];
			if sum > previous
			{
				result_2 += 1;
			}
			previous = sum;
		}
	}
	println!("First star: {}", result);
	println!("Second star: {}", result_2);
}
